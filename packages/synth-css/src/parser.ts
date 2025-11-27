/**
 * CSS Parser
 *
 * Hand-written CSS3 parser using Synth's universal AST
 * Converts CSS into Synth AST using language-agnostic BaseNode
 */

import type { NodeId, Plugin, Tree } from '@sylphx/synth'
import { addNode, createTree, SynthError } from '@sylphx/synth'
import { CSSTokenizer, type Token } from './tokenizer.js'

export interface CSSParseOptions {
  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]
}

export class CSSParser {
  private tokens: Token[] = []
  private pos = 0
  private plugins: Plugin[] = []
  private tree: Tree | null = null

  /**
   * Register a plugin
   */
  use(plugin: Plugin): this {
    this.plugins.push(plugin)
    return this
  }

  /**
   * Parse CSS synchronously
   */
  parse(source: string, options: CSSParseOptions = {}): Tree {
    const tree = createTree('css', source)
    this.tree = tree

    try {
      // Tokenize
      const tokenizer = new CSSTokenizer(source)
      this.tokens = tokenizer.tokenize()
      this.pos = 0

      // Parse stylesheet
      while (!this.isAtEnd()) {
        const ruleId = this.parseRule(tree.root)
        if (ruleId !== null) {
          tree.nodes[tree.root]?.children.push(ruleId)
        }
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`CSS parse error: ${error}`, 'PARSE_ERROR')
    }

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options.plugins || [])]

    const hasAsyncPlugin = allPlugins.some(
      (p) => 'transform' in p && p.transform.constructor.name === 'AsyncFunction'
    )

    if (hasAsyncPlugin) {
      throw new SynthError(
        'Detected async plugins. Use parseAsync() instead of parse()',
        'ASYNC_PLUGIN_IN_SYNC_PARSE'
      )
    }

    let result = tree
    for (const plugin of allPlugins) {
      if ('transform' in plugin) {
        result = plugin.transform(result) as Tree
      }
    }

    this.tree = result
    return result
  }

  /**
   * Parse CSS asynchronously
   */
  async parseAsync(source: string, options: CSSParseOptions = {}): Promise<Tree> {
    const tree = createTree('css', source)
    this.tree = tree

    try {
      const tokenizer = new CSSTokenizer(source)
      this.tokens = tokenizer.tokenize()
      this.pos = 0

      while (!this.isAtEnd()) {
        const ruleId = this.parseRule(tree.root)
        if (ruleId !== null) {
          tree.nodes[tree.root]?.children.push(ruleId)
        }
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`CSS parse error: ${error}`, 'PARSE_ERROR')
    }

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options.plugins || [])]
    let result = tree

    for (const plugin of allPlugins) {
      if ('transform' in plugin) {
        result = await plugin.transform(result)
      }
    }

    this.tree = result
    return result
  }

  /**
   * Get the last parsed tree
   */
  getTree(): Tree | null {
    return this.tree
  }

  private parseRule(parentId: NodeId): NodeId | null {
    this.skipComments()

    if (this.isAtEnd()) return null

    const token = this.current()

    // At-rule (@media, @keyframes, etc.)
    if (token.type === 'at-rule') {
      return this.parseAtRule(parentId)
    }

    // Style rule (selector { declarations })
    return this.parseStyleRule(parentId)
  }

  private parseAtRule(parentId: NodeId): NodeId {
    const tree = this.tree!
    const token = this.advance() // @rule

    const ruleId = addNode(tree, {
      type: 'AtRule',
      parent: parentId,
      children: [],
      span: {
        start: { offset: token.start, line: token.line, column: token.column },
        end: { offset: token.end, line: token.line, column: token.column },
      },
      data: {
        name: token.value.substring(1), // Remove @
      },
    })

    // Parse params (e.g., media query)
    const params: string[] = []
    while (!this.isAtEnd() && !this.check('open-brace')) {
      if (this.check('comment')) {
        this.advance()
        continue
      }
      params.push(this.advance().value)
    }

    const ruleNode = tree.nodes[ruleId]
    if (ruleNode?.data) {
      ruleNode.data.params = params.join(' ')
    }

    // Parse block
    if (this.check('open-brace')) {
      this.advance() // {

      while (!this.isAtEnd() && !this.check('close-brace')) {
        const childId = this.parseRule(ruleId)
        if (childId !== null) {
          tree.nodes[ruleId]?.children.push(childId)
        }
      }

      if (!this.isAtEnd()) {
        this.advance() // }
      }
    }

    return ruleId
  }

  private parseStyleRule(parentId: NodeId): NodeId {
    const tree = this.tree!

    // Parse selector
    const selectorTokens: string[] = []
    while (!this.isAtEnd() && !this.check('open-brace')) {
      if (this.check('comment')) {
        this.advance()
        continue
      }
      selectorTokens.push(this.advance().value)
    }

    const selector = selectorTokens.join('').trim()

    const ruleId = addNode(tree, {
      type: 'StyleRule',
      parent: parentId,
      children: [],
      data: {
        selector,
      },
    })

    // Parse declarations block
    if (!this.check('open-brace')) {
      return ruleId
    }

    this.advance() // {

    while (!this.isAtEnd() && !this.check('close-brace')) {
      this.skipComments()

      if (this.check('close-brace')) break

      const declId = this.parseDeclaration(ruleId)
      if (declId !== null) {
        tree.nodes[ruleId]?.children.push(declId)
      }
    }

    if (!this.isAtEnd()) {
      this.advance() // }
    }

    return ruleId
  }

  private parseDeclaration(parentId: NodeId): NodeId | null {
    const tree = this.tree!

    // Parse property
    const propertyTokens: string[] = []
    while (
      !this.isAtEnd() &&
      !this.check('colon') &&
      !this.check('semicolon') &&
      !this.check('close-brace')
    ) {
      if (this.check('comment')) {
        this.advance()
        continue
      }
      propertyTokens.push(this.advance().value)
    }

    const property = propertyTokens.join('').trim()

    if (!property) {
      // Skip empty declaration
      if (this.check('semicolon')) {
        this.advance()
      }
      return null
    }

    if (!this.check('colon')) {
      // Invalid declaration
      return null
    }

    this.advance() // :

    // Parse value
    const valueTokens: string[] = []
    while (!this.isAtEnd() && !this.check('semicolon') && !this.check('close-brace')) {
      if (this.check('comment')) {
        this.advance()
        continue
      }
      const token = this.advance()
      valueTokens.push(token.value)
    }

    const value = valueTokens.join('').trim()

    if (this.check('semicolon')) {
      this.advance() // ;
    }

    const declId = addNode(tree, {
      type: 'Declaration',
      parent: parentId,
      children: [],
      data: {
        property,
        value,
      },
    })

    return declId
  }

  private skipComments(): void {
    while (this.check('comment')) {
      this.advance()
    }
  }

  private current(): Token {
    return this.tokens[this.pos] || this.tokens[this.tokens.length - 1]!
  }

  private check(type: Token['type']): boolean {
    if (this.isAtEnd()) return false
    return this.current().type === type
  }

  private advance(): Token {
    const token = this.current()
    if (!this.isAtEnd()) {
      this.pos++
    }
    return token
  }

  private isAtEnd(): boolean {
    return this.pos >= this.tokens.length || this.current().type === 'eof'
  }
}

// Factory and standalone functions
export function createParser(): CSSParser {
  return new CSSParser()
}

export function parse(source: string, options?: CSSParseOptions): Tree {
  const parser = new CSSParser()
  return parser.parse(source, options)
}

export async function parseAsync(source: string, options?: CSSParseOptions): Promise<Tree> {
  const parser = new CSSParser()
  return parser.parseAsync(source, options)
}

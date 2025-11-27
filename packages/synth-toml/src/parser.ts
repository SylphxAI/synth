/**
 * TOML Parser
 *
 * Hand-written TOML 1.0 parser using Synth's universal AST
 * Converts TOML into Synth AST using language-agnostic BaseNode
 */

import type { NodeId, Plugin, Tree } from '@sylphx/synth'
import { addNode, createTree, SynthError } from '@sylphx/synth'
import { TOMLTokenizer, type Token } from './tokenizer.js'

export interface TOMLParseOptions {
  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]
}

export class TOMLParser {
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
   * Parse TOML synchronously
   */
  parse(source: string, options: TOMLParseOptions = {}): Tree {
    const tree = createTree('toml', source)
    this.tree = tree

    try {
      // Tokenize
      const tokenizer = new TOMLTokenizer(source)
      this.tokens = tokenizer.tokenize()
      this.pos = 0

      // Skip initial newlines and comments
      this.skipWhitespace()

      // Parse document
      let currentTable = tree.root

      while (!this.isAtEnd()) {
        // Table header
        if (this.check('open-bracket')) {
          const isArray = this.current().value === '[['
          const tableId = this.parseTable(tree.root, isArray)
          currentTable = tableId
          this.skipWhitespace()
          continue
        }

        // Key-value pair
        if (this.check('key')) {
          const kvId = this.parseKeyValue(currentTable)
          if (kvId !== null) {
            tree.nodes[currentTable]?.children.push(kvId)
          }
          this.skipWhitespace()
          continue
        }

        // Skip newlines and comments
        if (this.check('newline') || this.check('comment')) {
          this.advance()
          continue
        }

        // Unknown token
        this.advance()
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`TOML parse error: ${error}`, 'PARSE_ERROR')
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
   * Parse TOML asynchronously
   */
  async parseAsync(source: string, options: TOMLParseOptions = {}): Promise<Tree> {
    const tree = createTree('toml', source)
    this.tree = tree

    try {
      const tokenizer = new TOMLTokenizer(source)
      this.tokens = tokenizer.tokenize()
      this.pos = 0

      this.skipWhitespace()

      let currentTable = tree.root

      while (!this.isAtEnd()) {
        if (this.check('open-bracket')) {
          const isArray = this.current().value === '[['
          const tableId = this.parseTable(tree.root, isArray)
          currentTable = tableId
          this.skipWhitespace()
          continue
        }

        if (this.check('key')) {
          const kvId = this.parseKeyValue(currentTable)
          if (kvId !== null) {
            tree.nodes[currentTable]?.children.push(kvId)
          }
          this.skipWhitespace()
          continue
        }

        if (this.check('newline') || this.check('comment')) {
          this.advance()
          continue
        }

        this.advance()
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`TOML parse error: ${error}`, 'PARSE_ERROR')
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

  private parseTable(parentId: NodeId, isArray: boolean): NodeId {
    const tree = this.tree!
    const token = this.advance() // [ or [[

    // Parse table name (can be dotted: [parent.child])
    const nameParts: string[] = []

    while (!this.isAtEnd() && !this.check('close-bracket')) {
      if (this.check('key') || this.check('string')) {
        nameParts.push(this.advance().value)
      } else if (this.check('dot')) {
        this.advance()
      } else {
        this.advance()
      }
    }

    if (!this.isAtEnd()) {
      this.advance() // ] or ]]
    }

    const tableName = nameParts.join('.')

    const tableId = addNode(tree, {
      type: isArray ? 'TableArray' : 'Table',
      parent: parentId,
      children: [],
      span: {
        start: { offset: token.start, line: token.line, column: token.column },
        end: { offset: token.end, line: token.line, column: token.column },
      },
      data: {
        name: tableName,
      },
    })

    tree.nodes[parentId]?.children.push(tableId)

    return tableId
  }

  private parseKeyValue(parentId: NodeId): NodeId | null {
    const tree = this.tree!

    // Parse key (can be dotted: parent.child = value)
    const keyParts: string[] = []

    while (!this.isAtEnd() && !this.check('equals')) {
      if (this.check('key') || this.check('string')) {
        keyParts.push(this.advance().value)
      } else if (this.check('dot')) {
        this.advance()
      } else {
        break
      }
    }

    if (keyParts.length === 0) {
      return null
    }

    const key = keyParts.join('.')

    if (!this.check('equals')) {
      return null
    }

    this.advance() // =

    // Parse value
    const valueId = this.parseValue(parentId)

    if (valueId === null) {
      return null
    }

    const kvId = addNode(tree, {
      type: 'KeyValue',
      parent: parentId,
      children: [valueId],
      data: {
        key,
      },
    })

    return kvId
  }

  private parseValue(parentId: NodeId): NodeId | null {
    const tree = this.tree!

    // Skip whitespace before value
    while (this.check('comment') || this.current().value === ' ') {
      if (this.check('comment')) {
        this.advance()
      } else {
        this.advance()
      }
    }

    if (this.isAtEnd()) {
      return null
    }

    const token = this.current()

    // String
    if (token.type === 'string') {
      this.advance()
      return addNode(tree, {
        type: 'String',
        parent: parentId,
        children: [],
        data: { value: token.value },
      })
    }

    // Integer
    if (token.type === 'integer') {
      this.advance()
      return addNode(tree, {
        type: 'Integer',
        parent: parentId,
        children: [],
        data: { value: token.value },
      })
    }

    // Float
    if (token.type === 'float') {
      this.advance()
      return addNode(tree, {
        type: 'Float',
        parent: parentId,
        children: [],
        data: { value: token.value },
      })
    }

    // Boolean
    if (token.type === 'boolean') {
      this.advance()
      return addNode(tree, {
        type: 'Boolean',
        parent: parentId,
        children: [],
        data: { value: token.value },
      })
    }

    // DateTime
    if (token.type === 'datetime') {
      this.advance()
      return addNode(tree, {
        type: 'DateTime',
        parent: parentId,
        children: [],
        data: { value: token.value },
      })
    }

    // Array
    if (token.type === 'open-bracket') {
      return this.parseArray(parentId)
    }

    // Inline table
    if (token.type === 'open-brace') {
      return this.parseInlineTable(parentId)
    }

    return null
  }

  private parseArray(parentId: NodeId): NodeId {
    const tree = this.tree!

    this.advance() // [

    const arrayId = addNode(tree, {
      type: 'Array',
      parent: parentId,
      children: [],
    })

    while (!this.isAtEnd() && !this.check('close-bracket')) {
      // Skip newlines and comments in arrays
      if (this.check('newline') || this.check('comment')) {
        this.advance()
        continue
      }

      const valueId = this.parseValue(arrayId)
      if (valueId !== null) {
        tree.nodes[arrayId]?.children.push(valueId)
      }

      // Comma
      if (this.check('comma')) {
        this.advance()
      }

      // Skip trailing whitespace
      while (this.check('newline') || this.check('comment')) {
        this.advance()
      }
    }

    if (!this.isAtEnd()) {
      this.advance() // ]
    }

    return arrayId
  }

  private parseInlineTable(parentId: NodeId): NodeId {
    const tree = this.tree!

    this.advance() // {

    const tableId = addNode(tree, {
      type: 'InlineTable',
      parent: parentId,
      children: [],
    })

    while (!this.isAtEnd() && !this.check('close-brace')) {
      const kvId = this.parseKeyValue(tableId)
      if (kvId !== null) {
        tree.nodes[tableId]?.children.push(kvId)
      }

      if (this.check('comma')) {
        this.advance()
      }
    }

    if (!this.isAtEnd()) {
      this.advance() // }
    }

    return tableId
  }

  private skipWhitespace(): void {
    while (this.check('newline') || this.check('comment')) {
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
export function createParser(): TOMLParser {
  return new TOMLParser()
}

export function parse(source: string, options?: TOMLParseOptions): Tree {
  const parser = new TOMLParser()
  return parser.parse(source, options)
}

export async function parseAsync(source: string, options?: TOMLParseOptions): Promise<Tree> {
  const parser = new TOMLParser()
  return parser.parseAsync(source, options)
}

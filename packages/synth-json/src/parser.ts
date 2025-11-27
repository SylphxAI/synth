/**
 * JSON Parser
 *
 * Converts JSON into Synth AST using language-agnostic BaseNode
 * Hand-written recursive descent parser for JSON
 */

import type { NodeId, Plugin, Tree } from '@sylphx/synth'
import { addNode, createTree, SynthError } from '@sylphx/synth'

export interface JSONParseOptions {
  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]

  /** Allow trailing commas */
  allowTrailingCommas?: boolean

  /** Allow comments (non-standard) */
  allowComments?: boolean
}

export class JSONParser {
  private source = ''
  private pos = 0
  private line = 1
  private column = 0
  private plugins: Plugin[] = []
  private tree: Tree | null = null
  private options: JSONParseOptions = {}

  /**
   * Register a plugin
   */
  use(plugin: Plugin): this {
    this.plugins.push(plugin)
    return this
  }

  /**
   * Parse JSON synchronously
   */
  parse(source: string, options: JSONParseOptions = {}): Tree {
    this.source = source
    this.pos = 0
    this.line = 1
    this.column = 0
    this.options = options

    const tree = createTree('json', source)
    this.tree = tree

    try {
      this.skipWhitespace()
      if (this.pos >= this.source.length) {
        throw this.error('Empty JSON document')
      }

      const valueId = this.parseValue(tree.root)
      tree.nodes[tree.root]?.children.push(valueId)

      this.skipWhitespace()
      if (this.pos < this.source.length) {
        throw this.error('Unexpected content after JSON value')
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`JSON parse error: ${error}`, 'PARSE_ERROR')
    }

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options.plugins || [])]

    // Check for async plugins
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
   * Parse JSON asynchronously
   */
  async parseAsync(source: string, options: JSONParseOptions = {}): Promise<Tree> {
    this.source = source
    this.pos = 0
    this.line = 1
    this.column = 0
    this.options = options

    const tree = createTree('json', source)
    this.tree = tree

    try {
      this.skipWhitespace()
      if (this.pos >= this.source.length) {
        throw this.error('Empty JSON document')
      }

      const valueId = this.parseValue(tree.root)
      tree.nodes[tree.root]?.children.push(valueId)

      this.skipWhitespace()
      if (this.pos < this.source.length) {
        throw this.error('Unexpected content after JSON value')
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`JSON parse error: ${error}`, 'PARSE_ERROR')
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

  private parseValue(parentId: NodeId): NodeId {
    this.skipWhitespace()

    const char = this.source[this.pos]

    if (!char) {
      throw this.error('Unexpected end of input')
    }

    if (char === '{') {
      return this.parseObject(parentId)
    }
    if (char === '[') {
      return this.parseArray(parentId)
    }
    if (char === '"') {
      return this.parseString(parentId)
    }
    if (char === 't' || char === 'f') {
      return this.parseBoolean(parentId)
    }
    if (char === 'n') {
      return this.parseNull(parentId)
    }
    if (char === '-' || (char >= '0' && char <= '9')) {
      return this.parseNumber(parentId)
    }

    throw this.error(`Unexpected character: '${char}'`)
  }

  private parseObject(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.consume('{')
    this.skipWhitespace()

    const tree = this.tree!
    const objId = addNode(tree, {
      type: 'Object',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: start, line: startLine, column: startColumn }, // Will update
      },
      data: {},
    })

    // Empty object
    if (this.source[this.pos] === '}') {
      this.consume('}')
      const objNode = tree.nodes[objId]
      if (objNode?.span) {
        objNode.span.end = {
          offset: this.pos,
          line: this.line,
          column: this.column,
        }
      }
      return objId
    }

    // Parse properties
    while (true) {
      this.skipWhitespace()

      // Parse key (must be string)
      if (this.source[this.pos] !== '"') {
        throw this.error('Expected string key in object')
      }

      const propId = this.parseProperty(objId)
      tree.nodes[objId]?.children.push(propId)

      this.skipWhitespace()

      // Check for more properties
      if (this.source[this.pos] === ',') {
        this.consume(',')
        this.skipWhitespace()

        // Trailing comma check
        if (this.source[this.pos] === '}') {
          if (!this.options.allowTrailingCommas) {
            throw this.error('Trailing commas not allowed in JSON')
          }
          break
        }
      } else if (this.source[this.pos] === '}') {
        break
      } else {
        throw this.error('Expected comma or closing brace in object')
      }
    }

    this.consume('}')
    const objNode = tree.nodes[objId]
    if (objNode?.span) {
      objNode.span.end = {
        offset: this.pos,
        line: this.line,
        column: this.column,
      }
    }

    return objId
  }

  private parseProperty(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    const tree = this.tree!

    // Parse key
    const keyStart = this.pos
    const key = this.parseStringValue()
    const keyEnd = this.pos

    this.skipWhitespace()
    this.consume(':')
    this.skipWhitespace()

    // Parse value
    const propId = addNode(tree, {
      type: 'Property',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: start, line: startLine, column: startColumn }, // Will update
      },
      data: {
        key,
        keySpan: {
          start: { offset: keyStart, line: startLine, column: startColumn },
          end: { offset: keyEnd, line: this.line, column: this.column },
        },
      },
    })

    const valueId = this.parseValue(propId)
    tree.nodes[propId]?.children.push(valueId)

    const propNode = tree.nodes[propId]
    if (propNode?.span) {
      propNode.span.end = {
        offset: this.pos,
        line: this.line,
        column: this.column,
      }
    }

    return propId
  }

  private parseArray(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.consume('[')
    this.skipWhitespace()

    const tree = this.tree!
    const arrId = addNode(tree, {
      type: 'Array',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: start, line: startLine, column: startColumn }, // Will update
      },
      data: {},
    })

    // Empty array
    if (this.source[this.pos] === ']') {
      this.consume(']')
      const arrNode = tree.nodes[arrId]
      if (arrNode?.span) {
        arrNode.span.end = {
          offset: this.pos,
          line: this.line,
          column: this.column,
        }
      }
      return arrId
    }

    // Parse elements
    while (true) {
      const elemId = this.parseValue(arrId)
      tree.nodes[arrId]?.children.push(elemId)

      this.skipWhitespace()

      if (this.source[this.pos] === ',') {
        this.consume(',')
        this.skipWhitespace()

        // Trailing comma check
        if (this.source[this.pos] === ']') {
          if (!this.options.allowTrailingCommas) {
            throw this.error('Trailing commas not allowed in JSON')
          }
          break
        }
      } else if (this.source[this.pos] === ']') {
        break
      } else {
        throw this.error('Expected comma or closing bracket in array')
      }
    }

    this.consume(']')
    const arrNode = tree.nodes[arrId]
    if (arrNode?.span) {
      arrNode.span.end = {
        offset: this.pos,
        line: this.line,
        column: this.column,
      }
    }

    return arrId
  }

  private parseString(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    const value = this.parseStringValue()

    const tree = this.tree!
    return addNode(tree, {
      type: 'String',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: this.pos, line: this.line, column: this.column },
      },
      data: { value },
    })
  }

  private parseStringValue(): string {
    this.consume('"')

    let value = ''

    while (this.pos < this.source.length && this.source[this.pos] !== '"') {
      const char = this.source[this.pos]

      if (char === '\\') {
        this.advance()
        const escaped = this.source[this.pos]

        switch (escaped) {
          case '"':
            value += '"'
            break
          case '\\':
            value += '\\'
            break
          case '/':
            value += '/'
            break
          case 'b':
            value += '\b'
            break
          case 'f':
            value += '\f'
            break
          case 'n':
            value += '\n'
            break
          case 'r':
            value += '\r'
            break
          case 't':
            value += '\t'
            break
          case 'u': {
            // Unicode escape
            this.advance()
            const hex = this.source.substr(this.pos, 4)
            if (hex.length !== 4 || !/^[0-9a-fA-F]{4}$/.test(hex)) {
              throw this.error('Invalid unicode escape sequence')
            }
            value += String.fromCharCode(Number.parseInt(hex, 16))
            this.pos += 3 // Will advance one more at loop end
            this.column += 3
            break
          }
          default:
            throw this.error(`Invalid escape sequence: \\${escaped}`)
        }
        this.advance()
      } else {
        value += char
        this.advance()
      }
    }

    this.consume('"')
    return value
  }

  private parseNumber(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let numStr = ''

    // Optional minus
    if (this.source[this.pos] === '-') {
      numStr += '-'
      this.advance()
    }

    // Integer part
    const char1 = this.source[this.pos]
    if (char1 === '0') {
      numStr += '0'
      this.advance()
    } else if (char1 && char1 >= '1' && char1 <= '9') {
      while (true) {
        const ch = this.source[this.pos]
        if (ch && ch >= '0' && ch <= '9') {
          numStr += ch
          this.advance()
        } else {
          break
        }
      }
    } else {
      throw this.error('Invalid number')
    }

    // Fractional part
    if (this.source[this.pos] === '.') {
      numStr += '.'
      this.advance()

      const fracChar = this.source[this.pos]
      if (!(fracChar && fracChar >= '0' && fracChar <= '9')) {
        throw this.error('Expected digit after decimal point')
      }

      while (true) {
        const ch = this.source[this.pos]
        if (ch && ch >= '0' && ch <= '9') {
          numStr += ch
          this.advance()
        } else {
          break
        }
      }
    }

    // Exponent part
    const expChar = this.source[this.pos]
    if (expChar === 'e' || expChar === 'E') {
      numStr += expChar
      this.advance()

      const signChar = this.source[this.pos]
      if (signChar === '+' || signChar === '-') {
        numStr += signChar
        this.advance()
      }

      const expDigit = this.source[this.pos]
      if (!(expDigit && expDigit >= '0' && expDigit <= '9')) {
        throw this.error('Expected digit in exponent')
      }

      while (true) {
        const ch = this.source[this.pos]
        if (ch && ch >= '0' && ch <= '9') {
          numStr += ch
          this.advance()
        } else {
          break
        }
      }
    }

    const value = Number.parseFloat(numStr)

    const tree = this.tree!
    return addNode(tree, {
      type: 'Number',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: this.pos, line: this.line, column: this.column },
      },
      data: { value, raw: numStr },
    })
  }

  private parseBoolean(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let value: boolean

    if (this.source.substr(this.pos, 4) === 'true') {
      value = true
      this.pos += 4
      this.column += 4
    } else if (this.source.substr(this.pos, 5) === 'false') {
      value = false
      this.pos += 5
      this.column += 5
    } else {
      throw this.error('Invalid boolean value')
    }

    const tree = this.tree!
    return addNode(tree, {
      type: 'Boolean',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: this.pos, line: this.line, column: this.column },
      },
      data: { value },
    })
  }

  private parseNull(parentId: NodeId): NodeId {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    if (this.source.substr(this.pos, 4) !== 'null') {
      throw this.error('Invalid null value')
    }

    this.pos += 4
    this.column += 4

    const tree = this.tree!
    return addNode(tree, {
      type: 'Null',
      parent: parentId,
      children: [],
      span: {
        start: { offset: start, line: startLine, column: startColumn },
        end: { offset: this.pos, line: this.line, column: this.column },
      },
      data: { value: null },
    })
  }

  private skipWhitespace(): void {
    while (this.pos < this.source.length) {
      const char = this.source[this.pos]

      if (char === ' ' || char === '\t' || char === '\r') {
        this.advance()
      } else if (char === '\n') {
        this.advance()
        this.line++
        this.column = 0
      } else {
        break
      }
    }
  }

  private consume(expected: string): void {
    if (this.source[this.pos] !== expected) {
      throw this.error(`Expected '${expected}', got '${this.source[this.pos]}'`)
    }
    this.advance()
  }

  private advance(): void {
    this.pos++
    this.column++
  }

  private error(message: string): SynthError {
    return new SynthError(`${message} at line ${this.line}, column ${this.column}`, 'PARSE_ERROR')
  }
}

// Factory and standalone functions
export function createParser(): JSONParser {
  return new JSONParser()
}

export function parse(source: string, options?: JSONParseOptions): Tree {
  const parser = new JSONParser()
  return parser.parse(source, options)
}

export async function parseAsync(source: string, options?: JSONParseOptions): Promise<Tree> {
  const parser = new JSONParser()
  return parser.parseAsync(source, options)
}

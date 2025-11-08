/**
 * TOML Tokenizer
 *
 * Hand-written tokenizer for TOML 1.0
 * Converts TOML source into tokens
 */

export type TokenType =
  | 'key'
  | 'string'
  | 'integer'
  | 'float'
  | 'boolean'
  | 'datetime'
  | 'equals'
  | 'dot'
  | 'comma'
  | 'open-bracket'
  | 'close-bracket'
  | 'open-brace'
  | 'close-brace'
  | 'newline'
  | 'comment'
  | 'eof'

export interface Token {
  type: TokenType
  value: string
  start: number
  end: number
  line: number
  column: number
}

export class TOMLTokenizer {
  private source: string
  private pos = 0
  private line = 1
  private column = 0
  private tokens: Token[] = []

  constructor(source: string) {
    this.source = source
  }

  tokenize(): Token[] {
    this.tokens = []
    this.pos = 0
    this.line = 1
    this.column = 0

    while (this.pos < this.source.length) {
      const char = this.source[this.pos]

      // Whitespace (skip, but not newlines)
      if (char === ' ' || char === '\t' || char === '\r') {
        this.advance()
        continue
      }

      // Newline
      if (char === '\n') {
        this.addToken('newline', '\n')
        this.line++
        this.column = 0
        this.advance()
        continue
      }

      // Comment
      if (char === '#') {
        this.tokenizeComment()
        continue
      }

      // String (basic)
      if (char === '"') {
        this.tokenizeBasicString()
        continue
      }

      // String (literal)
      if (char === "'") {
        this.tokenizeLiteralString()
        continue
      }

      // Table array or table
      if (char === '[') {
        if (this.peek() === '[') {
          this.addToken('open-bracket', '[[')
          this.advance()
          this.advance()
        } else {
          this.addToken('open-bracket', '[')
          this.advance()
        }
        continue
      }

      if (char === ']') {
        if (this.peek() === ']') {
          this.addToken('close-bracket', ']]')
          this.advance()
          this.advance()
        } else {
          this.addToken('close-bracket', ']')
          this.advance()
        }
        continue
      }

      // Inline table
      if (char === '{') {
        this.addToken('open-brace', '{')
        this.advance()
        continue
      }

      if (char === '}') {
        this.addToken('close-brace', '}')
        this.advance()
        continue
      }

      // Assignment
      if (char === '=') {
        this.addToken('equals', '=')
        this.advance()
        continue
      }

      // Dot
      if (char === '.') {
        this.addToken('dot', '.')
        this.advance()
        continue
      }

      // Comma
      if (char === ',') {
        this.addToken('comma', ',')
        this.advance()
        continue
      }

      // Boolean true
      if (this.match('true')) {
        this.addToken('boolean', 'true')
        this.advance(4)
        continue
      }

      // Boolean false
      if (this.match('false')) {
        this.addToken('boolean', 'false')
        this.advance(5)
        continue
      }

      // Number or datetime
      if (this.isDigit(char) || char === '+' || char === '-') {
        this.tokenizeNumberOrDateTime()
        continue
      }

      // Key (bare key)
      if (this.isKeyChar(char)) {
        this.tokenizeKey()
        continue
      }

      // Unknown - skip
      this.advance()
    }

    this.addToken('eof', '')
    return this.tokens
  }

  private tokenizeComment(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.advance() // #

    let value = ''

    while (this.pos < this.source.length && this.source[this.pos] !== '\n') {
      value += this.source[this.pos]
      this.advance()
    }

    this.tokens.push({
      type: 'comment',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeBasicString(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.advance() // opening "

    // Multi-line string
    if (this.source[this.pos] === '"' && this.peek() === '"') {
      this.advance() // second "
      this.advance() // third "

      let value = ''

      while (this.pos < this.source.length) {
        if (
          this.source[this.pos] === '"' &&
          this.peek() === '"' &&
          this.source[this.pos + 2] === '"'
        ) {
          this.advance() // first "
          this.advance() // second "
          this.advance() // third "
          break
        }

        if (this.source[this.pos] === '\n') {
          this.line++
          this.column = 0
        }

        if (this.source[this.pos] === '\\') {
          this.advance()
          if (this.pos < this.source.length) {
            value += this.escapeChar(this.source[this.pos])
            this.advance()
          }
        } else {
          value += this.source[this.pos]
          this.advance()
        }
      }

      this.tokens.push({
        type: 'string',
        value,
        start,
        end: this.pos,
        line: startLine,
        column: startColumn,
      })
      return
    }

    // Single-line string
    let value = ''

    while (this.pos < this.source.length && this.source[this.pos] !== '"') {
      if (this.source[this.pos] === '\n') {
        break // Invalid in basic string
      }

      if (this.source[this.pos] === '\\') {
        this.advance()
        if (this.pos < this.source.length) {
          value += this.escapeChar(this.source[this.pos])
          this.advance()
        }
      } else {
        value += this.source[this.pos]
        this.advance()
      }
    }

    if (this.pos < this.source.length) {
      this.advance() // closing "
    }

    this.tokens.push({
      type: 'string',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeLiteralString(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.advance() // opening '

    // Multi-line literal string
    if (this.source[this.pos] === "'" && this.peek() === "'") {
      this.advance() // second '
      this.advance() // third '

      let value = ''

      while (this.pos < this.source.length) {
        if (
          this.source[this.pos] === "'" &&
          this.peek() === "'" &&
          this.source[this.pos + 2] === "'"
        ) {
          this.advance() // first '
          this.advance() // second '
          this.advance() // third '
          break
        }

        if (this.source[this.pos] === '\n') {
          this.line++
          this.column = 0
        }

        value += this.source[this.pos]
        this.advance()
      }

      this.tokens.push({
        type: 'string',
        value,
        start,
        end: this.pos,
        line: startLine,
        column: startColumn,
      })
      return
    }

    // Single-line literal string
    let value = ''

    while (this.pos < this.source.length && this.source[this.pos] !== "'") {
      if (this.source[this.pos] === '\n') {
        break // Invalid in literal string
      }

      value += this.source[this.pos]
      this.advance()
    }

    if (this.pos < this.source.length) {
      this.advance() // closing '
    }

    this.tokens.push({
      type: 'string',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeNumberOrDateTime(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let value = ''

    // Sign
    if (this.source[this.pos] === '+' || this.source[this.pos] === '-') {
      value += this.source[this.pos]
      this.advance()
    }

    // Check for datetime (YYYY-MM-DD)
    if (this.isDigit(this.source[this.pos])) {
      const preview = this.source.slice(this.pos, this.pos + 20)
      if (preview.match(/^\d{4}-\d{2}-\d{2}/)) {
        this.tokenizeDateTime(start, startLine, startColumn, value)
        return
      }
    }

    // Number
    let hasDecimal = false
    let hasExponent = false

    while (this.pos < this.source.length) {
      const char = this.source[this.pos]

      if (this.isDigit(char)) {
        value += char
        this.advance()
      } else if (char === '_') {
        this.advance() // Skip underscores in numbers
      } else if (char === '.' && !hasDecimal && !hasExponent) {
        hasDecimal = true
        value += char
        this.advance()
      } else if ((char === 'e' || char === 'E') && !hasExponent) {
        hasExponent = true
        value += char
        this.advance()
        if (this.source[this.pos] === '+' || this.source[this.pos] === '-') {
          value += this.source[this.pos]
          this.advance()
        }
      } else {
        break
      }
    }

    this.tokens.push({
      type: hasDecimal || hasExponent ? 'float' : 'integer',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeDateTime(
    start: number,
    startLine: number,
    startColumn: number,
    prefix: string
  ): void {
    let value = prefix

    while (this.pos < this.source.length) {
      const char = this.source[this.pos]

      if (
        this.isDigit(char) ||
        char === '-' ||
        char === ':' ||
        char === 'T' ||
        char === 'Z' ||
        char === '+' ||
        char === '.' ||
        char === ' '
      ) {
        value += char
        this.advance()
      } else {
        break
      }
    }

    this.tokens.push({
      type: 'datetime',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeKey(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let value = ''

    while (this.pos < this.source.length && this.isKeyChar(this.source[this.pos])) {
      value += this.source[this.pos]
      this.advance()
    }

    this.tokens.push({
      type: 'key',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private escapeChar(char: string | undefined): string {
    if (!char) return ''
    switch (char) {
      case 'n':
        return '\n'
      case 't':
        return '\t'
      case 'r':
        return '\r'
      case '\\':
        return '\\'
      case '"':
        return '"'
      case 'b':
        return '\b'
      case 'f':
        return '\f'
      default:
        return char
    }
  }

  private isDigit(char: string | undefined): boolean {
    if (!char) return false
    return char >= '0' && char <= '9'
  }

  private isKeyChar(char: string | undefined): boolean {
    if (!char) return false
    return (
      (char >= 'a' && char <= 'z') ||
      (char >= 'A' && char <= 'Z') ||
      (char >= '0' && char <= '9') ||
      char === '_' ||
      char === '-'
    )
  }

  private match(str: string): boolean {
    return this.source.slice(this.pos, this.pos + str.length) === str
  }

  private peek(): string | undefined {
    return this.source[this.pos + 1]
  }

  private advance(count: number = 1): void {
    for (let i = 0; i < count; i++) {
      this.pos++
      this.column++
    }
  }

  private addToken(type: TokenType, value: string): void {
    this.tokens.push({
      type,
      value,
      start: this.pos,
      end: this.pos + value.length,
      line: this.line,
      column: this.column,
    })
  }
}

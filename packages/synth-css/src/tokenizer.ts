/**
 * CSS Tokenizer
 *
 * Hand-written tokenizer for CSS3
 * Converts CSS source into tokens
 */

export type TokenType =
  | 'selector'
  | 'property'
  | 'value'
  | 'at-rule'
  | 'open-brace'
  | 'close-brace'
  | 'open-paren'
  | 'close-paren'
  | 'colon'
  | 'semicolon'
  | 'comma'
  | 'whitespace'
  | 'comment'
  | 'string'
  | 'number'
  | 'unit'
  | 'hash'
  | 'eof'

export interface Token {
  type: TokenType
  value: string
  start: number
  end: number
  line: number
  column: number
}

export class CSSTokenizer {
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
      this.skipWhitespace()

      if (this.pos >= this.source.length) break

      const char = this.source[this.pos]

      // Comment
      if (char === '/' && this.peek() === '*') {
        this.tokenizeComment()
        continue
      }

      // At-rule
      if (char === '@') {
        this.tokenizeAtRule()
        continue
      }

      // String
      if (char === '"' || char === "'") {
        this.tokenizeString()
        continue
      }

      // Number
      if (this.isDigit(char) || (char === '-' && this.isDigit(this.peek()))) {
        this.tokenizeNumber()
        continue
      }

      // Hash (color or ID)
      if (char === '#') {
        this.tokenizeHash()
        continue
      }

      // Punctuation
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

      if (char === '(') {
        this.addToken('open-paren', '(')
        this.advance()
        continue
      }

      if (char === ')') {
        this.addToken('close-paren', ')')
        this.advance()
        continue
      }

      if (char === ':') {
        this.addToken('colon', ':')
        this.advance()
        continue
      }

      if (char === ';') {
        this.addToken('semicolon', ';')
        this.advance()
        continue
      }

      if (char === ',') {
        this.addToken('comma', ',')
        this.advance()
        continue
      }

      // Identifier (property, value, selector)
      if (this.isIdentifierStart(char)) {
        this.tokenizeIdentifier()
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

    this.advance() // /
    this.advance() // *

    let value = ''

    while (this.pos < this.source.length) {
      if (this.source[this.pos] === '*' && this.peek() === '/') {
        this.advance() // *
        this.advance() // /
        break
      }

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

  private tokenizeAtRule(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.advance() // @

    let value = '@'

    while (this.pos < this.source.length && this.isIdentifier(this.source[this.pos])) {
      value += this.source[this.pos]
      this.advance()
    }

    this.tokens.push({
      type: 'at-rule',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeString(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column
    const quote = this.source[this.pos]

    this.advance() // opening quote

    let value = ''

    while (this.pos < this.source.length && this.source[this.pos] !== quote) {
      if (this.source[this.pos] === '\\') {
        this.advance()
        if (this.pos < this.source.length) {
          value += this.source[this.pos]
          this.advance()
        }
      } else {
        value += this.source[this.pos]
        this.advance()
      }
    }

    if (this.pos < this.source.length) {
      this.advance() // closing quote
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

  private tokenizeNumber(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let value = ''

    // Optional minus
    if (this.source[this.pos] === '-') {
      value += '-'
      this.advance()
    }

    // Digits
    while (this.pos < this.source.length && this.isDigit(this.source[this.pos])) {
      value += this.source[this.pos]
      this.advance()
    }

    // Decimal point
    if (this.source[this.pos] === '.') {
      value += '.'
      this.advance()

      while (this.pos < this.source.length && this.isDigit(this.source[this.pos])) {
        value += this.source[this.pos]
        this.advance()
      }
    }

    this.tokens.push({
      type: 'number',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })

    // Unit (px, em, %, etc.)
    if (this.isIdentifierStart(this.source[this.pos])) {
      this.tokenizeUnit()
    }
  }

  private tokenizeUnit(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let value = ''

    while (this.pos < this.source.length && this.isIdentifier(this.source[this.pos])) {
      value += this.source[this.pos]
      this.advance()
    }

    this.tokens.push({
      type: 'unit',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeHash(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    this.advance() // #

    let value = '#'

    while (
      this.pos < this.source.length &&
      (this.isIdentifier(this.source[this.pos]) || this.isDigit(this.source[this.pos]))
    ) {
      value += this.source[this.pos]
      this.advance()
    }

    this.tokens.push({
      type: 'hash',
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private tokenizeIdentifier(): void {
    const start = this.pos
    const startLine = this.line
    const startColumn = this.column

    let value = ''

    while (this.pos < this.source.length && this.isIdentifier(this.source[this.pos])) {
      value += this.source[this.pos]
      this.advance()
    }

    // Determine if it's property, value, or selector
    // For now, we'll use context in the parser
    this.tokens.push({
      type: 'value', // Will be refined by parser
      value,
      start,
      end: this.pos,
      line: startLine,
      column: startColumn,
    })
  }

  private skipWhitespace(): void {
    while (this.pos < this.source.length && this.isWhitespace(this.source[this.pos])) {
      if (this.source[this.pos] === '\n') {
        this.line++
        this.column = 0
      }
      this.advance()
    }
  }

  private isWhitespace(char: string | undefined): boolean {
    if (!char) return false
    return char === ' ' || char === '\t' || char === '\n' || char === '\r'
  }

  private isDigit(char: string | undefined): boolean {
    if (!char) return false
    return char >= '0' && char <= '9'
  }

  private isIdentifierStart(char: string | undefined): boolean {
    if (!char) return false
    return (
      (char >= 'a' && char <= 'z') ||
      (char >= 'A' && char <= 'Z') ||
      char === '_' ||
      char === '-' ||
      char === '.' ||
      char === '*' ||
      char === '>' ||
      char === '+' ||
      char === '~' ||
      char === '[' ||
      char === ']' ||
      char === '=' ||
      char === '!' ||
      char === '%'
    )
  }

  private isIdentifier(char: string | undefined): boolean {
    if (!char) return false
    return (
      this.isIdentifierStart(char) ||
      this.isDigit(char)
    )
  }

  private peek(): string {
    return this.source[this.pos + 1] || ''
  }

  private advance(): void {
    this.pos++
    this.column++
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

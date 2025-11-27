/**
 * HTML5 Tokenizer
 *
 * Fast HTML tokenization following HTML5 spec
 * Reference: https://html.spec.whatwg.org/multipage/parsing.html
 */

export interface Token {
  type: TokenType
  start: number
  end: number
  raw: string
}

export type TokenType =
  | 'doctype'
  | 'startTag'
  | 'endTag'
  | 'selfClosingTag'
  | 'text'
  | 'comment'
  | 'cdata'

export interface DoctypeToken extends Token {
  type: 'doctype'
  name: string
  publicId?: string
  systemId?: string
}

export interface StartTagToken extends Token {
  type: 'startTag'
  tagName: string
  attributes: Record<string, string>
  selfClosing: boolean
}

export interface EndTagToken extends Token {
  type: 'endTag'
  tagName: string
}

export interface SelfClosingTagToken extends Token {
  type: 'selfClosingTag'
  tagName: string
  attributes: Record<string, string>
}

export interface TextToken extends Token {
  type: 'text'
  value: string
}

export interface CommentToken extends Token {
  type: 'comment'
  value: string
}

export interface CDATAToken extends Token {
  type: 'cdata'
  value: string
}

export type HTMLToken =
  | DoctypeToken
  | StartTagToken
  | EndTagToken
  | SelfClosingTagToken
  | TextToken
  | CommentToken
  | CDATAToken

export class HTMLTokenizer {
  private input = ''
  private pos = 0
  private len = 0

  tokenize(html: string): HTMLToken[] {
    this.input = html
    this.pos = 0
    this.len = html.length

    const tokens: HTMLToken[] = []

    while (this.pos < this.len) {
      const token = this.nextToken()
      if (token) {
        tokens.push(token)
      }
    }

    return tokens
  }

  private nextToken(): HTMLToken | null {
    if (this.pos >= this.len) return null

    const ch = this.input[this.pos]

    if (ch === '<') {
      return this.parseTag()
    }

    return this.parseText()
  }

  private parseTag(): HTMLToken | null {
    const start = this.pos

    if (this.peek(4) === '<!--') {
      return this.parseComment(start)
    }

    if (this.peek(9).toUpperCase() === '<![CDATA[') {
      return this.parseCDATA(start)
    }

    if (this.peek(9).toLowerCase() === '<!doctype') {
      return this.parseDoctype(start)
    }

    this.pos++ // Skip '<'

    if (this.current() === '/') {
      return this.parseEndTag(start)
    }

    return this.parseStartTag(start)
  }

  private parseDoctype(start: number): DoctypeToken {
    this.pos += 9 // Skip '<!doctype'
    this.skipWhitespace()

    const nameStart = this.pos
    while (this.pos < this.len && !/[\s>]/.test(this.current())) {
      this.pos++
    }
    const name = this.input.slice(nameStart, this.pos)

    this.skipWhitespace()

    let publicId: string | undefined
    let systemId: string | undefined

    if (this.peek(6).toUpperCase() === 'PUBLIC') {
      this.pos += 6
      this.skipWhitespace()
      publicId = this.parseQuotedString()
      this.skipWhitespace()
      systemId = this.parseQuotedString()
    } else if (this.peek(6).toUpperCase() === 'SYSTEM') {
      this.pos += 6
      this.skipWhitespace()
      systemId = this.parseQuotedString()
    }

    while (this.pos < this.len && this.current() !== '>') {
      this.pos++
    }
    this.pos++ // Skip '>'

    return {
      type: 'doctype',
      name,
      publicId,
      systemId,
      start,
      end: this.pos,
      raw: this.input.slice(start, this.pos),
    }
  }

  private parseStartTag(start: number): StartTagToken | SelfClosingTagToken {
    const tagNameStart = this.pos
    while (this.pos < this.len && !/[\s/>]/.test(this.current())) {
      this.pos++
    }
    const tagName = this.input.slice(tagNameStart, this.pos).toLowerCase()

    this.skipWhitespace()

    const attributes = this.parseAttributes()

    this.skipWhitespace()

    const selfClosing = this.current() === '/' && this.peek(2) === '/>'

    if (selfClosing) {
      this.pos++ // Skip '/'
    }

    if (this.current() === '>') {
      this.pos++
    }

    const end = this.pos

    if (selfClosing) {
      return {
        type: 'selfClosingTag',
        tagName,
        attributes,
        start,
        end,
        raw: this.input.slice(start, end),
      }
    }

    return {
      type: 'startTag',
      tagName,
      attributes,
      selfClosing: false,
      start,
      end,
      raw: this.input.slice(start, end),
    }
  }

  private parseEndTag(start: number): EndTagToken {
    this.pos++ // Skip '/'

    const tagNameStart = this.pos
    while (this.pos < this.len && !/[\s>]/.test(this.current())) {
      this.pos++
    }
    const tagName = this.input.slice(tagNameStart, this.pos).toLowerCase()

    this.skipWhitespace()

    while (this.pos < this.len && this.current() !== '>') {
      this.pos++
    }
    this.pos++ // Skip '>'

    return {
      type: 'endTag',
      tagName,
      start,
      end: this.pos,
      raw: this.input.slice(start, this.pos),
    }
  }

  private parseAttributes(): Record<string, string> {
    const attrs: Record<string, string> = {}

    while (this.pos < this.len && this.current() !== '>' && this.current() !== '/') {
      this.skipWhitespace()

      if (this.current() === '>' || this.current() === '/') break

      const attrStart = this.pos
      while (this.pos < this.len && !/[\s=/>]/.test(this.current())) {
        this.pos++
      }
      const attrName = this.input.slice(attrStart, this.pos).toLowerCase()

      this.skipWhitespace()

      let attrValue = ''
      if (this.current() === '=') {
        this.pos++
        this.skipWhitespace()
        attrValue = this.parseAttributeValue()
      } else {
        attrValue = attrName // Boolean attribute
      }

      attrs[attrName] = attrValue
    }

    return attrs
  }

  private parseAttributeValue(): string {
    const quote = this.current()

    if (quote === '"' || quote === "'") {
      return this.parseQuotedString()
    }

    const start = this.pos
    while (this.pos < this.len && !/[\s>]/.test(this.current())) {
      this.pos++
    }
    return this.input.slice(start, this.pos)
  }

  private parseQuotedString(): string {
    const quote = this.current()
    this.pos++ // Skip opening quote

    const start = this.pos
    while (this.pos < this.len && this.current() !== quote) {
      this.pos++
    }
    const value = this.input.slice(start, this.pos)
    this.pos++ // Skip closing quote

    return value
  }

  private parseText(): TextToken {
    const start = this.pos
    while (this.pos < this.len && this.current() !== '<') {
      this.pos++
    }

    return {
      type: 'text',
      value: this.input.slice(start, this.pos),
      start,
      end: this.pos,
      raw: this.input.slice(start, this.pos),
    }
  }

  private parseComment(start: number): CommentToken {
    this.pos += 4 // Skip '<!--'

    const valueStart = this.pos
    while (this.pos < this.len - 2) {
      if (this.peek(3) === '-->') {
        break
      }
      this.pos++
    }

    const value = this.input.slice(valueStart, this.pos)
    this.pos += 3 // Skip '-->'

    return {
      type: 'comment',
      value,
      start,
      end: this.pos,
      raw: this.input.slice(start, this.pos),
    }
  }

  private parseCDATA(start: number): CDATAToken {
    this.pos += 9 // Skip '<![CDATA['

    const valueStart = this.pos
    while (this.pos < this.len - 2) {
      if (this.peek(3) === ']]>') {
        break
      }
      this.pos++
    }

    const value = this.input.slice(valueStart, this.pos)
    this.pos += 3 // Skip ']]>'

    return {
      type: 'cdata',
      value,
      start,
      end: this.pos,
      raw: this.input.slice(start, this.pos),
    }
  }

  private current(): string {
    return this.input[this.pos] || ''
  }

  private peek(n: number): string {
    return this.input.slice(this.pos, this.pos + n)
  }

  private skipWhitespace(): void {
    while (this.pos < this.len && /\s/.test(this.current())) {
      this.pos++
    }
  }
}

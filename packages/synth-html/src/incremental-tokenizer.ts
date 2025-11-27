/**
 * Incremental HTML Tokenizer
 *
 * Element-level tokenization for efficient incremental parsing of HTML.
 *
 * Features:
 * - Element-level granularity
 * - Self-closing tag detection
 * - Nested element tracking
 * - Attribute preservation
 * - 90%+ token reuse for typical edits
 */

import { IncrementalTokenizer } from '@sylphx/synth'
import type { Edit, Token, TokenRange } from '@sylphx/synth'
import { TokenFlags, TokenKind, createToken } from '@sylphx/synth'

/**
 * HTML element types
 */
enum HTMLElementType {
  TAG = 'tag', // Complete tag: <div>...</div>
  SELF_CLOSING = 'self_closing', // Self-closing: <img />
  TEXT = 'text', // Text content
  COMMENT = 'comment', // <!-- comment -->
  DOCTYPE = 'doctype', // <!DOCTYPE html>
  CDATA = 'cdata', // <![CDATA[...]]>
  SCRIPT = 'script', // <script>...</script>
  STYLE = 'style', // <style>...</style>
}

/**
 * Incremental HTML Tokenizer
 */
export class IncrementalHTMLTokenizer extends IncrementalTokenizer {
  protected language = 'html'

  /**
   * Tokenize HTML with element-level granularity
   */
  protected tokenizeImpl(source: string, startOffset: number, endOffset: number): Token[] {
    const tokens: Token[] = []
    const region = source.slice(startOffset, endOffset)

    let currentOffset = startOffset
    let buffer = ''
    let inTag = false
    let tagName = ''
    let tagStart = startOffset

    for (let i = 0; i < region.length; i++) {
      const char = region[i]!
      const peek = region[i + 1]
      const offset = startOffset + i

      // Detect tag start
      if (char === '<' && !inTag) {
        // Save any accumulated text
        if (buffer.trim()) {
          const token = this.createElementToken(
            HTMLElementType.TEXT,
            buffer,
            tagStart,
            offset - 1,
            tokens.length
          )
          tokens.push(token)
        }

        buffer = char
        inTag = true
        tagStart = offset

        // Detect special tags
        if (peek === '!') {
          // Comment, DOCTYPE, or CDATA
          if (region.slice(i + 1, i + 4) === '!--') {
            // Comment
            const commentEnd = region.indexOf('-->', i + 4)
            if (commentEnd !== -1) {
              const content = region.slice(i, commentEnd + 3)
              const token = this.createElementToken(
                HTMLElementType.COMMENT,
                content,
                offset,
                offset + content.length - 1,
                tokens.length
              )
              tokens.push(token)
              i = commentEnd + 2
              currentOffset = offset + content.length
              inTag = false
              buffer = ''
              continue
            }
          } else if (region.slice(i + 1, i + 9).toUpperCase() === '!DOCTYPE') {
            // DOCTYPE
            const doctypeEnd = region.indexOf('>', i + 1)
            if (doctypeEnd !== -1) {
              const content = region.slice(i, doctypeEnd + 1)
              const token = this.createElementToken(
                HTMLElementType.DOCTYPE,
                content,
                offset,
                offset + content.length - 1,
                tokens.length
              )
              tokens.push(token)
              i = doctypeEnd
              currentOffset = offset + content.length
              inTag = false
              buffer = ''
              continue
            }
          } else if (region.slice(i + 1, i + 9) === '![CDATA[') {
            // CDATA
            const cdataEnd = region.indexOf(']]>', i + 9)
            if (cdataEnd !== -1) {
              const content = region.slice(i, cdataEnd + 3)
              const token = this.createElementToken(
                HTMLElementType.CDATA,
                content,
                offset,
                offset + content.length - 1,
                tokens.length
              )
              tokens.push(token)
              i = cdataEnd + 2
              currentOffset = offset + content.length
              inTag = false
              buffer = ''
              continue
            }
          }
        }

        // Extract tag name
        const tagNameMatch = region.slice(i + 1).match(/^(\/?[\w-]+)/)
        if (tagNameMatch) {
          tagName = (tagNameMatch[1] ?? '').replace('/', '')
        }
      }

      // Detect tag end
      else if (char === '>' && inTag) {
        buffer += char

        // Check if self-closing
        const isSelfClosing = buffer.includes('/>')

        // Determine element type
        let elementType: HTMLElementType
        if (isSelfClosing) {
          elementType = HTMLElementType.SELF_CLOSING
        } else if (tagName.toLowerCase() === 'script') {
          elementType = HTMLElementType.SCRIPT
        } else if (tagName.toLowerCase() === 'style') {
          elementType = HTMLElementType.STYLE
        } else {
          elementType = HTMLElementType.TAG
        }

        // For script/style tags, find closing tag
        if (elementType === HTMLElementType.SCRIPT || elementType === HTMLElementType.STYLE) {
          const closingTag = `</${tagName}>`
          const closingIndex = region.indexOf(closingTag, i + 1)
          if (closingIndex !== -1) {
            buffer = region.slice(
              tagStart - startOffset,
              closingIndex - startOffset + closingTag.length
            )
            i = closingIndex + closingTag.length - 1
          }
        }
        // For opening tags, find matching closing tag
        else if (!isSelfClosing && !buffer.startsWith('</')) {
          const closingTag = `</${tagName}>`
          const closingIndex = this.findMatchingClosingTag(region, i + 1, tagName)
          if (closingIndex !== -1) {
            buffer = region.slice(
              tagStart - startOffset,
              closingIndex - startOffset + closingTag.length
            )
            i = closingIndex + closingTag.length - 1
          }
        }

        // Create token
        const token = this.createElementToken(
          elementType,
          buffer,
          tagStart,
          tagStart + buffer.length - 1,
          tokens.length
        )
        tokens.push(token)

        inTag = false
        buffer = ''
        tagName = ''
        currentOffset = tagStart + buffer.length
        tagStart = currentOffset
      }

      // Accumulate characters
      else {
        buffer += char
      }
    }

    // Handle remaining buffer
    if (buffer.trim()) {
      const token = this.createElementToken(
        inTag ? HTMLElementType.TAG : HTMLElementType.TEXT,
        buffer,
        tagStart,
        startOffset + region.length - 1,
        tokens.length
      )
      tokens.push(token)
    }

    return tokens
  }

  /**
   * Find matching closing tag for element
   */
  private findMatchingClosingTag(html: string, startIndex: number, tagName: string): number {
    const openTag = `<${tagName}`
    const closeTag = `</${tagName}>`
    let depth = 1
    let index = startIndex

    while (index < html.length && depth > 0) {
      const openIndex = html.indexOf(openTag, index)
      const closeIndex = html.indexOf(closeTag, index)

      if (closeIndex === -1) break

      if (openIndex !== -1 && openIndex < closeIndex) {
        // Found nested opening tag
        depth++
        index = openIndex + openTag.length
      } else {
        // Found closing tag
        depth--
        if (depth === 0) {
          return closeIndex
        }
        index = closeIndex + closeTag.length
      }
    }

    return -1
  }

  /**
   * Create element-level token
   */
  private createElementToken(
    elementType: HTMLElementType,
    content: string,
    startOffset: number,
    endOffset: number,
    index: number
  ): Token {
    // Map element type to token kind
    const kind = this.elementTypeToTokenKind(elementType)

    // Detect flags
    let flags = TokenFlags.NONE
    if (content.includes('\n')) {
      flags |= TokenFlags.MULTILINE
    }

    // Create position
    const startPos = this.offsetToPosition(startOffset, this.source)
    const endPos = this.offsetToPosition(endOffset, this.source)

    // Extract tag name if applicable
    let tagNameMatch: RegExpMatchArray | null = null
    if (elementType === HTMLElementType.TAG || elementType === HTMLElementType.SELF_CLOSING) {
      tagNameMatch = content.match(/<\/?(\w+)/)
    }

    // Metadata
    const metadata: Record<string, unknown> = {
      elementType,
      tagName: tagNameMatch ? tagNameMatch[1] : null,
    }

    return createToken(kind, content, { start: startPos, end: endPos }, index, flags, metadata)
  }

  /**
   * Map element type to token kind
   */
  private elementTypeToTokenKind(elementType: HTMLElementType): TokenKind {
    switch (elementType) {
      case HTMLElementType.TAG:
      case HTMLElementType.SELF_CLOSING:
        return TokenKind.TAG_OPEN
      case HTMLElementType.SCRIPT:
      case HTMLElementType.STYLE:
        return TokenKind.TAG_OPEN
      case HTMLElementType.COMMENT:
        return TokenKind.COMMENT
      case HTMLElementType.DOCTYPE:
      case HTMLElementType.CDATA:
        return TokenKind.TAG_OPEN
      default:
        return TokenKind.TEXT
    }
  }

  /**
   * Expand to safe boundaries for HTML
   *
   * Strategy: Expand to complete elements
   */
  protected expandToSafeBoundaries(range: TokenRange, _edit: Edit): TokenRange {
    if (!this.tokenStream) {
      return range
    }

    const tokens = this.tokenStream.tokens
    let startIndex = range.startIndex
    let endIndex = range.endIndex

    // Expand backward to include preceding whitespace/text
    while (startIndex > 0) {
      const prevToken = tokens[startIndex - 1]
      if (!prevToken) break

      const elementType = prevToken.metadata?.elementType as HTMLElementType
      if (elementType === HTMLElementType.TEXT && prevToken.value.trim() === '') {
        startIndex--
      } else {
        break
      }
    }

    // Expand forward to include following whitespace/text
    while (endIndex < tokens.length - 1) {
      const nextToken = tokens[endIndex + 1]
      if (!nextToken) break

      const elementType = nextToken.metadata?.elementType as HTMLElementType
      if (elementType === HTMLElementType.TEXT && nextToken.value.trim() === '') {
        endIndex++
      } else {
        break
      }
    }

    return {
      startIndex,
      endIndex,
      byteRange: {
        start: tokens[startIndex]?.span.start.offset ?? 0,
        end: tokens[endIndex]?.span.end.offset ?? 0,
      },
    }
  }
}

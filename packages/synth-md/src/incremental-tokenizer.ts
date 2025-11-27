/**
 * Incremental Markdown Tokenizer
 *
 * Tokenizes Markdown with block-level granularity for optimal incremental parsing.
 *
 * Key features:
 * 1. Block-level tokenization (paragraph, heading, list, etc.)
 * 2. Inline token detection (but detailed inline parsing deferred to parser)
 * 3. Smart boundary expansion (to blank lines)
 * 4. 90%+ token reuse for typical edits
 *
 * Performance target: <0.1ms for typical edits on 1000-line documents
 */

import { IncrementalTokenizer } from '@sylphx/synth'
import type { Token, TokenRange } from '@sylphx/synth'
import { TokenFlags, TokenKind, createToken } from '@sylphx/synth'
import type { Edit } from '@sylphx/synth'

/**
 * Markdown block type detection
 */
enum MarkdownBlockType {
  BLANK = 'blank',
  HEADING = 'heading',
  CODE_FENCE = 'code_fence',
  LIST_ITEM = 'list_item',
  BLOCKQUOTE = 'blockquote',
  HORIZONTAL_RULE = 'horizontal_rule',
  PARAGRAPH = 'paragraph',
  HTML_BLOCK = 'html_block',
}

/**
 * Incremental Markdown Tokenizer
 */
export class IncrementalMarkdownTokenizer extends IncrementalTokenizer {
  protected language = 'markdown'

  /**
   * Tokenize a range of source text
   *
   * Strategy:
   * 1. Split into lines
   * 2. Detect block type for each line
   * 3. Create block-level tokens
   * 4. Track inline markers (but don't fully parse)
   */
  protected tokenizeImpl(source: string, startOffset: number, endOffset: number): Token[] {
    const tokens: Token[] = []
    const region = source.slice(startOffset, endOffset)
    const lines = region.split('\n')

    let currentOffset = startOffset
    let inCodeFence = false
    let codeFenceMarker = ''

    for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
      const line = lines[lineIndex]!
      const lineStart = currentOffset
      const lineEnd = currentOffset + line.length

      // Detect block type
      const blockType = this.detectBlockType(line, inCodeFence)

      // Handle code fence state
      if (blockType === MarkdownBlockType.CODE_FENCE) {
        const marker = line.match(/^```|^~~~/)
        if (marker) {
          if (!inCodeFence) {
            inCodeFence = true
            codeFenceMarker = marker[0]!
          } else if (marker[0] === codeFenceMarker) {
            inCodeFence = false
            codeFenceMarker = ''
          }
        }
      }

      // Create token for this line
      const token = this.createBlockToken(
        blockType,
        line,
        lineStart,
        lineEnd,
        tokens.length,
        lineIndex
      )

      tokens.push(token)

      // Move to next line (+1 for newline)
      currentOffset = lineEnd + 1
    }

    return tokens
  }

  /**
   * Detect Markdown block type from line
   */
  private detectBlockType(line: string, inCodeFence: boolean): MarkdownBlockType {
    const trimmed = line.trim()

    // Empty line
    if (trimmed === '') {
      return MarkdownBlockType.BLANK
    }

    // Inside code fence
    if (inCodeFence) {
      // Check if this is the closing fence
      if (trimmed.match(/^```|^~~~/)) {
        return MarkdownBlockType.CODE_FENCE
      }
      // Otherwise it's code content (treated as text)
      return MarkdownBlockType.PARAGRAPH
    }

    // Code fence start
    if (trimmed.match(/^```|^~~~/)) {
      return MarkdownBlockType.CODE_FENCE
    }

    // ATX heading (# Heading)
    if (trimmed.match(/^#{1,6}\s/)) {
      return MarkdownBlockType.HEADING
    }

    // Horizontal rule (---, ***, ___)
    if (trimmed.match(/^(?:[-*_])\s*(?:[-*_]\s*){2,}$/)) {
      return MarkdownBlockType.HORIZONTAL_RULE
    }

    // List item (-, *, +, 1., 2., etc.)
    if (trimmed.match(/^[-*+]\s/) || trimmed.match(/^\d+\.\s/)) {
      return MarkdownBlockType.LIST_ITEM
    }

    // Blockquote (>)
    if (trimmed.match(/^>/)) {
      return MarkdownBlockType.BLOCKQUOTE
    }

    // HTML block
    if (
      trimmed.match(/^<(?:div|p|section|article|aside|header|footer|nav|main|figure|table|form)/i)
    ) {
      return MarkdownBlockType.HTML_BLOCK
    }

    // Default: Paragraph
    return MarkdownBlockType.PARAGRAPH
  }

  /**
   * Create a block-level token
   */
  private createBlockToken(
    blockType: MarkdownBlockType,
    content: string,
    startOffset: number,
    endOffset: number,
    index: number,
    lineNumber: number
  ): Token {
    // Map block type to token kind
    const kind = this.blockTypeToTokenKind(blockType)

    // Detect flags
    let flags = TokenFlags.NONE
    if (content.startsWith(' ') || content.startsWith('\t')) {
      flags |= TokenFlags.LEADING_WHITESPACE
    }
    if (content.endsWith(' ') || content.endsWith('\t')) {
      flags |= TokenFlags.TRAILING_WHITESPACE
    }

    // Create position
    const startPos = this.offsetToPosition(startOffset, this.source)
    const endPos = this.offsetToPosition(endOffset, this.source)

    // Metadata
    const metadata: Record<string, unknown> = {
      blockType,
      lineNumber,
      indentLevel: this.getIndentLevel(content),
    }

    // Add block-specific metadata
    if (blockType === MarkdownBlockType.HEADING) {
      const match = content.match(/^(#{1,6})/)
      if (match) {
        metadata.headingLevel = match[1]?.length ?? 0
      }
    } else if (blockType === MarkdownBlockType.LIST_ITEM) {
      const match = content.match(/^(\s*)([-*+]|\d+\.)\s/)
      if (match?.[2]) {
        metadata.listMarker = match[2]
        metadata.ordered = /^\d+\./.test(match[2])
      }
    }

    return createToken(kind, content, { start: startPos, end: endPos }, index, flags, metadata)
  }

  /**
   * Map block type to token kind
   */
  private blockTypeToTokenKind(blockType: MarkdownBlockType): TokenKind {
    switch (blockType) {
      case MarkdownBlockType.BLANK:
        return TokenKind.WHITESPACE
      case MarkdownBlockType.HEADING:
        return TokenKind.HEADING_START
      case MarkdownBlockType.CODE_FENCE:
        return TokenKind.CODE_FENCE
      case MarkdownBlockType.LIST_ITEM:
        return TokenKind.LIST_MARKER
      case MarkdownBlockType.BLOCKQUOTE:
        return TokenKind.BLOCKQUOTE_MARKER
      case MarkdownBlockType.HORIZONTAL_RULE:
        return TokenKind.HORIZONTAL_RULE
      case MarkdownBlockType.HTML_BLOCK:
        return TokenKind.TAG_OPEN
      default:
        return TokenKind.TEXT
    }
  }

  /**
   * Get indentation level
   */
  private getIndentLevel(line: string): number {
    const match = line.match(/^(\s+)/)
    if (!match) return 0

    const whitespace = match[1]!
    // Count spaces (tab = 4 spaces)
    return whitespace.split('').reduce((count, char) => {
      return count + (char === '\t' ? 4 : 1)
    }, 0)
  }

  /**
   * Expand to safe boundaries for Markdown
   *
   * Strategy: Smart expansion based on document size
   * - Small documents: Minimal expansion (exact token boundaries)
   * - Large documents: Expand to blank lines for safety
   * This optimizes token reuse for all document sizes
   */
  protected expandToSafeBoundaries(range: TokenRange, _edit: Edit): TokenRange {
    if (!this.tokenStream) {
      return range
    }

    const tokens = this.tokenStream.tokens
    const documentSize = this.tokenStream.source.length

    // For small documents (<1KB), use minimal expansion
    if (documentSize < 1000) {
      // Only expand for lists/blockquotes (structural blocks)
      const startToken = tokens[range.startIndex]
      const endToken = tokens[range.endIndex]

      if (
        startToken?.kind === TokenKind.LIST_MARKER ||
        startToken?.kind === TokenKind.BLOCKQUOTE_MARKER ||
        endToken?.kind === TokenKind.LIST_MARKER ||
        endToken?.kind === TokenKind.BLOCKQUOTE_MARKER
      ) {
        // Expand to block boundaries for lists/blockquotes
        const startIndex = this.expandToBlockStart(tokens, range.startIndex)
        const endIndex = this.expandToBlockEnd(tokens, range.endIndex)
        return {
          startIndex,
          endIndex,
          byteRange: {
            start: tokens[startIndex]?.span.start.offset ?? 0,
            end: tokens[endIndex]?.span.end.offset ?? 0,
          },
        }
      }

      // Otherwise, use exact token boundaries (no expansion)
      return range
    }

    // For large documents (>=1KB), use safe expansion
    let startIndex = range.startIndex
    let endIndex = range.endIndex

    // Expand backward to include preceding blank lines (max 2)
    let blankCount = 0
    while (startIndex > 0 && blankCount < 2) {
      const prevToken = tokens[startIndex - 1]
      if (!prevToken) break

      if (prevToken.kind === TokenKind.WHITESPACE && prevToken.value.trim() === '') {
        startIndex--
        blankCount++
      } else {
        break
      }
    }

    // Expand forward to include following blank lines (max 2)
    blankCount = 0
    while (endIndex < tokens.length - 1 && blankCount < 2) {
      const nextToken = tokens[endIndex + 1]
      if (!nextToken) break

      if (nextToken.kind === TokenKind.WHITESPACE && nextToken.value.trim() === '') {
        endIndex++
        blankCount++
      } else {
        break
      }
    }

    // Also expand to include the block boundary if we're in a list/blockquote
    startIndex = this.expandToBlockStart(tokens, startIndex)
    endIndex = this.expandToBlockEnd(tokens, endIndex)

    return {
      startIndex,
      endIndex,
      byteRange: {
        start: tokens[startIndex]?.span.start.offset ?? 0,
        end: tokens[endIndex]?.span.end.offset ?? 0,
      },
    }
  }

  /**
   * Expand to block start (for nested blocks like lists)
   */
  private expandToBlockStart(tokens: Token[], startIndex: number): number {
    let index = startIndex

    // If we're in a list item, include all consecutive list items at same level
    const startToken = tokens[startIndex]
    if (startToken?.kind === TokenKind.LIST_MARKER) {
      const startIndent = (startToken.metadata?.indentLevel as number) || 0

      while (index > 0) {
        const prevToken = tokens[index - 1]
        if (!prevToken) break

        // Include previous list items at same or greater indent
        if (prevToken.kind === TokenKind.LIST_MARKER) {
          const prevIndent = (prevToken.metadata?.indentLevel as number) || 0
          if (prevIndent >= startIndent) {
            index--
          } else {
            break
          }
        } else if (prevToken.kind === TokenKind.WHITESPACE) {
          // Skip blank lines
          index--
        } else {
          break
        }
      }
    }

    return index
  }

  /**
   * Expand to block end (for nested blocks like lists)
   */
  private expandToBlockEnd(tokens: Token[], endIndex: number): number {
    let index = endIndex

    // If we're in a list item, include all consecutive list items at same level
    const endToken = tokens[endIndex]
    if (endToken?.kind === TokenKind.LIST_MARKER) {
      const endIndent = (endToken.metadata?.indentLevel as number) || 0

      while (index < tokens.length - 1) {
        const nextToken = tokens[index + 1]
        if (!nextToken) break

        // Include next list items at same or greater indent
        if (nextToken.kind === TokenKind.LIST_MARKER) {
          const nextIndent = (nextToken.metadata?.indentLevel as number) || 0
          if (nextIndent >= endIndent) {
            index++
          } else {
            break
          }
        } else if (nextToken.kind === TokenKind.WHITESPACE) {
          // Skip blank lines
          index++
        } else {
          break
        }
      }
    }

    return index
  }
}

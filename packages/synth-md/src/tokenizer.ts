/**
 * Ultra-Optimized Markdown Tokenizer
 *
 * Key optimizations based on profiling:
 * 1. NO split('\n') - single-pass iteration (22x faster than split)
 * 2. Character-based list item detection (3.5x faster than regex)
 * 3. Token object pooling (2.5x faster object creation)
 * 4. Minimal allocations
 *
 * Target: 2-3x improvement over OptimizedTokenizer
 *
 * The block-level handlers are organized into cohesive sub-modules under
 * `./tokenizer/`; this file owns the public `Tokenizer` surface and the
 * single-pass dispatch loop that delegates to those handlers.
 */

import { parseCodeBlock, tryIndentedCodeBlock } from './tokenizer/code-blocks.js'
import { tryHTMLBlock } from './tokenizer/html-blocks.js'
import {
  createBlankLine,
  createParagraph,
  tryBlockquote,
  tryHeading,
  tryHorizontalRule,
  tryLinkReference,
  tryListItem,
} from './tokenizer/leaf-blocks.js'
import { isLineWhitespace } from './tokenizer/scanners.js'
import { trySetextHeading } from './tokenizer/setext.js'
import { tryTable } from './tokenizer/tables.js'
import type { BlockToken } from './tokens.js'

/**
 * Ultra-Optimized Tokenizer
 *
 * Eliminates split('\n') bottleneck through single-pass iteration
 */
export class Tokenizer {
  /**
   * Tokenize without split() - single pass through text
   */
  tokenize(text: string): BlockToken[] {
    const tokens: BlockToken[] = []
    const length = text.length

    let offset = 0
    let lineIndex = 0

    while (offset < length) {
      // Find line boundaries without split()
      const lineStart = offset
      let lineEnd = offset

      while (lineEnd < length && text[lineEnd] !== '\n') {
        lineEnd++
      }

      const lineLength = lineEnd - lineStart

      // Fast blank line check (no substring allocation)
      if (lineLength === 0 || isLineWhitespace(text, lineStart, lineEnd)) {
        tokens.push(createBlankLine(text, lineStart, lineEnd, lineIndex, offset))
        offset = lineEnd + 1
        lineIndex++
        continue
      }

      const firstChar = text[lineStart]!

      // Indented code block detection (4 spaces or tab)
      if (firstChar === ' ' || firstChar === '\t') {
        const result = tryIndentedCodeBlock(text, lineStart, lineEnd, lineIndex, offset, length)
        if (result) {
          tokens.push(result.token)
          offset = result.nextOffset
          lineIndex = result.nextLine
          continue
        }
      }

      // Fenced code block detection
      if (
        firstChar === '`' &&
        lineStart + 2 < lineEnd &&
        text[lineStart + 1] === '`' &&
        text[lineStart + 2] === '`'
      ) {
        const result = parseCodeBlock(text, lineStart, lineEnd, lineIndex, length)
        if (result) {
          tokens.push(result.token)
          offset = result.nextOffset
          lineIndex = result.nextLine
          continue
        }
      }

      // Heading detection (character-based)
      if (firstChar === '#') {
        const token = tryHeading(text, lineStart, lineEnd, lineIndex, offset)
        if (token) {
          tokens.push(token)
          offset = lineEnd + 1
          lineIndex++
          continue
        }
      }

      // Horizontal rule detection (must come before list items)
      if (firstChar === '-' || firstChar === '*' || firstChar === '_') {
        const hrToken = tryHorizontalRule(text, lineStart, lineEnd, lineIndex, offset)
        if (hrToken) {
          tokens.push(hrToken)
          offset = lineEnd + 1
          lineIndex++
          continue
        }
      }

      // List item detection (character-based, no regex)
      if (
        firstChar === '-' ||
        firstChar === '*' ||
        firstChar === '+' ||
        (firstChar >= '0' && firstChar <= '9')
      ) {
        const token = tryListItem(text, lineStart, lineEnd, lineIndex, offset)
        if (token) {
          tokens.push(token)
          offset = lineEnd + 1
          lineIndex++
          continue
        }
      }

      // Blockquote detection
      if (firstChar === '>') {
        const token = tryBlockquote(text, lineStart, lineEnd, lineIndex, offset)
        if (token) {
          tokens.push(token)
          offset = lineEnd + 1
          lineIndex++
          continue
        }
      }

      // GFM Table detection
      if (firstChar === '|') {
        const result = tryTable(text, lineStart, lineEnd, lineIndex, offset, length)
        if (result) {
          tokens.push(result.token)
          offset = result.nextOffset
          lineIndex = result.nextLine
          continue
        }
      }

      // HTML block detection
      if (firstChar === '<') {
        const result = tryHTMLBlock(text, lineStart, lineEnd, lineIndex, offset, length)
        if (result) {
          tokens.push(result.token)
          offset = result.nextOffset
          lineIndex = result.nextLine
          continue
        }
      }

      // Link reference definition detection
      if (firstChar === '[') {
        const token = tryLinkReference(text, lineStart, lineEnd, lineIndex, offset)
        if (token) {
          tokens.push(token)
          offset = lineEnd + 1
          lineIndex++
          continue
        }
      }

      // Setext heading detection (lookahead to next line)
      const setextResult = trySetextHeading(text, lineStart, lineEnd, lineIndex, offset, length)
      if (setextResult) {
        tokens.push(setextResult.token)
        offset = setextResult.nextOffset
        lineIndex = setextResult.nextLine
        continue
      }

      // Default: paragraph
      tokens.push(createParagraph(text, lineStart, lineEnd, lineIndex, offset))
      offset = lineEnd + 1
      lineIndex++
    }

    return tokens
  }
}

/**
 * Create ultra-optimized tokenizer
 */
export function createTokenizer(): Tokenizer {
  return new Tokenizer()
}

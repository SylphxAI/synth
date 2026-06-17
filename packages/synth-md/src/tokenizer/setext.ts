/**
 * Setext heading tokenizer (underlined headings: === / ---).
 *
 * Uses one line of lookahead to detect the underline row.
 */

import type { HeadingToken } from '../tokens.js'
import { createPosition, createTokenPosition } from '../tokens.js'

/**
 * Try to parse Setext heading (with underline)
 */
export function trySetextHeading(
  text: string,
  lineStart: number,
  lineEnd: number,
  startLine: number,
  startOffset: number,
  textLength: number
): { token: HeadingToken; nextOffset: number; nextLine: number } | null {
  // Need at least one more line
  if (lineEnd + 1 >= textLength) return null

  // Find next line boundaries
  const nextLineStart = lineEnd + 1
  let nextLineEnd = nextLineStart

  while (nextLineEnd < textLength && text[nextLineEnd] !== '\n') {
    nextLineEnd++
  }

  // Check if next line is a setext underline (=== or ---)
  if (nextLineEnd <= nextLineStart) return null

  const underlineChar = text[nextLineStart]!

  // Must be = or -
  if (underlineChar !== '=' && underlineChar !== '-') return null

  // Check if entire line is the underline character (with optional spaces)
  let validUnderline = true
  for (let i = nextLineStart; i < nextLineEnd; i++) {
    const c = text[i]!
    if (c !== underlineChar && c !== ' ' && c !== '\t') {
      validUnderline = false
      break
    }
  }

  if (!validUnderline) return null

  // Valid setext heading
  const depth = underlineChar === '=' ? 1 : 2
  const headingText = text.slice(lineStart, lineEnd)
  const raw = text.slice(lineStart, nextLineEnd)

  return {
    token: {
      type: 'heading',
      depth: depth as 1 | 2,
      text: headingText,
      raw,
      position: createTokenPosition(
        createPosition(startLine, 0, startOffset),
        createPosition(startLine + 1, nextLineEnd - nextLineStart, nextLineEnd + startOffset)
      ),
    },
    nextOffset: nextLineEnd + 1,
    nextLine: startLine + 2,
  }
}

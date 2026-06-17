/**
 * Code block tokenizers: fenced (```) and indented (4 spaces / tab).
 *
 * Both consume multiple lines and return the next offset/line for the caller.
 */

import type { CodeBlockToken } from '../tokens.js'
import { createPosition, createTokenPosition } from '../tokens.js'
import { getIndentLevel, isLineWhitespace } from './scanners.js'

/**
 * Parse code block without split()
 */
export function parseCodeBlock(
  text: string,
  lineStart: number,
  lineEnd: number,
  startLine: number,
  textLength: number
): { token: CodeBlockToken; nextOffset: number; nextLine: number } | null {
  // Parse opening fence
  let i = lineStart + 3 // Skip ```

  // Skip spaces
  while (i < lineEnd && text[i] === ' ') {
    i++
  }

  // Extract language
  const langStart = i
  while (i < lineEnd && text[i] !== ' ' && text[i] !== '\n') {
    i++
  }
  const lang = langStart < i ? text.slice(langStart, i) : undefined

  // Extract meta
  while (i < lineEnd && text[i] === ' ') {
    i++
  }
  const meta = i < lineEnd ? text.slice(i, lineEnd) : undefined

  // Find code content and closing fence
  let offset = lineEnd + 1
  let currentLine = startLine + 1
  const codeStart = offset

  while (offset < textLength) {
    const fenceLineStart = offset

    // Find end of line
    while (offset < textLength && text[offset] !== '\n') {
      offset++
    }

    // Check for closing fence
    if (
      offset - fenceLineStart >= 3 &&
      text[fenceLineStart] === '`' &&
      text[fenceLineStart + 1] === '`' &&
      text[fenceLineStart + 2] === '`'
    ) {
      // Found closing fence
      const code = codeStart < fenceLineStart - 1 ? text.slice(codeStart, fenceLineStart - 1) : ''

      return {
        token: {
          type: 'codeBlock',
          lang,
          meta,
          code,
          raw: text.slice(lineStart, offset),
          position: createTokenPosition(
            createPosition(startLine, 0, lineStart),
            createPosition(currentLine, offset - fenceLineStart, offset)
          ),
        },
        nextOffset: offset + 1,
        nextLine: currentLine + 1,
      }
    }

    offset++
    currentLine++
  }

  // No closing fence
  const code = text.slice(codeStart)

  return {
    token: {
      type: 'codeBlock',
      lang,
      meta,
      code,
      raw: text.slice(lineStart),
      position: createTokenPosition(
        createPosition(startLine, 0, lineStart),
        createPosition(currentLine, 0, textLength)
      ),
    },
    nextOffset: textLength,
    nextLine: currentLine,
  }
}

/**
 * Try to parse indented code block (4 spaces or 1 tab)
 */
export function tryIndentedCodeBlock(
  text: string,
  lineStart: number,
  lineEnd: number,
  startLine: number,
  startOffset: number,
  textLength: number
): { token: CodeBlockToken; nextOffset: number; nextLine: number } | null {
  // Check if line has proper indentation (4 spaces or 1 tab)
  const firstLineIndent = getIndentLevel(text, lineStart, lineEnd)
  if (firstLineIndent < 4) return null

  // Collect code lines
  const codeLines: string[] = []
  let currentOffset = lineStart
  let currentLine = startLine

  while (currentOffset < textLength) {
    const rowStart = currentOffset
    let rowEnd = currentOffset

    // Find end of line
    while (rowEnd < textLength && text[rowEnd] !== '\n') {
      rowEnd++
    }

    const lineIndent = getIndentLevel(text, rowStart, rowEnd)

    // Empty line - include it and continue
    if (rowStart === rowEnd || isLineWhitespace(text, rowStart, rowEnd)) {
      codeLines.push('')
      currentOffset = rowEnd + 1
      currentLine++
      continue
    }

    // Indented line - include it
    if (lineIndent >= 4) {
      // Remove 4 spaces of indentation
      let codeStart = rowStart
      let removed = 0
      while (removed < 4 && codeStart < rowEnd) {
        if (text[codeStart] === '\t') {
          removed = 4
          codeStart++
        } else if (text[codeStart] === ' ') {
          removed++
          codeStart++
        } else {
          break
        }
      }

      const codeLine = text.slice(codeStart, rowEnd)
      codeLines.push(codeLine)
      currentOffset = rowEnd + 1
      currentLine++
      continue
    }

    // Non-indented line, end of code block
    break
  }

  // Remove trailing blank lines
  while (codeLines.length > 0 && codeLines[codeLines.length - 1] === '') {
    codeLines.pop()
    currentLine--
  }

  if (codeLines.length === 0) {
    return null
  }

  const code = codeLines.join('\n')
  const endOffset = currentOffset > 0 ? currentOffset - 1 : currentOffset
  const raw = text.slice(lineStart, endOffset)

  return {
    token: {
      type: 'codeBlock',
      lang: undefined,
      meta: undefined,
      code,
      raw,
      position: createTokenPosition(
        createPosition(startLine, 0, startOffset),
        createPosition(currentLine - 1, 0, endOffset)
      ),
    },
    nextOffset: currentOffset,
    nextLine: currentLine,
  }
}

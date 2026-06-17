/**
 * HTML block tokenizer covering CommonMark HTML block types 1-7.
 *
 * Detects the opening condition for each type and consumes through the
 * appropriate closing condition (matching tag, marker, blank line, or
 * single-line complete tag).
 */

import type { HTMLBlockToken } from '../tokens.js'
import { createPosition, createTokenPosition } from '../tokens.js'
import { isLineWhitespace } from './scanners.js'

/**
 * Try to parse HTML block
 */
export function tryHTMLBlock(
  text: string,
  lineStart: number,
  lineEnd: number,
  startLine: number,
  startOffset: number,
  textLength: number
): { token: HTMLBlockToken; nextOffset: number; nextLine: number } | null {
  if (text[lineStart] !== '<') return null

  // Type 1: Script/pre/style/textarea tags
  const scriptTags = ['<script', '<pre', '<style', '<textarea']
  for (const tag of scriptTags) {
    if (startsWith(text, lineStart, lineEnd, tag)) {
      return parseHTMLBlockWithClosingTag(
        text,
        lineStart,
        startLine,
        startOffset,
        textLength,
        tag.slice(1) // Remove <
      )
    }
  }

  // Type 2: HTML comment <!-- -->
  if (startsWith(text, lineStart, lineEnd, '<!--')) {
    return parseHTMLBlockUntil(text, lineStart, startLine, startOffset, textLength, '-->')
  }

  // Type 3: Processing instruction <? ?>
  if (startsWith(text, lineStart, lineEnd, '<?')) {
    return parseHTMLBlockUntil(text, lineStart, startLine, startOffset, textLength, '?>')
  }

  // Type 4: Declaration <!UPPER>
  if (startsWith(text, lineStart, lineEnd, '<!') && lineStart + 2 < lineEnd) {
    const thirdChar = text[lineStart + 2]!
    if (thirdChar >= 'A' && thirdChar <= 'Z') {
      return parseHTMLBlockUntil(text, lineStart, startLine, startOffset, textLength, '>')
    }
  }

  // Type 5: CDATA <![CDATA[ ]]>
  if (startsWith(text, lineStart, lineEnd, '<![CDATA[')) {
    return parseHTMLBlockUntil(text, lineStart, startLine, startOffset, textLength, ']]>')
  }

  // Type 6: Block-level tags
  const blockTags = [
    'address',
    'article',
    'aside',
    'base',
    'basefont',
    'blockquote',
    'body',
    'caption',
    'center',
    'col',
    'colgroup',
    'dd',
    'details',
    'dialog',
    'dir',
    'div',
    'dl',
    'dt',
    'fieldset',
    'figcaption',
    'figure',
    'footer',
    'form',
    'frame',
    'frameset',
    'h1',
    'h2',
    'h3',
    'h4',
    'h5',
    'h6',
    'head',
    'header',
    'hr',
    'html',
    'iframe',
    'legend',
    'li',
    'link',
    'main',
    'menu',
    'menuitem',
    'nav',
    'noframes',
    'ol',
    'optgroup',
    'option',
    'p',
    'param',
    'section',
    'source',
    'summary',
    'table',
    'tbody',
    'td',
    'tfoot',
    'th',
    'thead',
    'title',
    'tr',
    'track',
    'ul',
  ]

  for (const tag of blockTags) {
    if (startsWithTag(text, lineStart, lineEnd, tag)) {
      return parseHTMLBlockUntilBlankLine(text, lineStart, startLine, startOffset, textLength)
    }
  }

  // Type 7: Complete tag on single line
  if (isCompleteHTMLTag(text, lineStart, lineEnd)) {
    const content = text.slice(lineStart, lineEnd)
    const raw = content
    return {
      token: {
        type: 'htmlBlock',
        content,
        raw,
        position: createTokenPosition(
          createPosition(startLine, 0, startOffset),
          createPosition(startLine, lineEnd - lineStart, startOffset + (lineEnd - lineStart))
        ),
      },
      nextOffset: lineEnd + 1,
      nextLine: startLine + 1,
    }
  }

  return null
}

/**
 * Check if text starts with a specific string
 */
function startsWith(text: string, lineStart: number, lineEnd: number, prefix: string): boolean {
  if (lineStart + prefix.length > lineEnd) return false

  for (let i = 0; i < prefix.length; i++) {
    if (text[lineStart + i] !== prefix[i]) return false
  }

  return true
}

/**
 * Check if text starts with an HTML tag
 */
function startsWithTag(text: string, lineStart: number, lineEnd: number, tagName: string): boolean {
  if (!startsWith(text, lineStart, lineEnd, `<${tagName}`)) return false

  const afterTag = lineStart + tagName.length + 1
  if (afterTag >= lineEnd) return true

  const nextChar = text[afterTag]!
  // Tag must be followed by space, >, or /
  return (
    nextChar === ' ' ||
    nextChar === '>' ||
    nextChar === '/' ||
    nextChar === '\t' ||
    nextChar === '\n'
  )
}

/**
 * Parse HTML block with specific closing tag
 */
function parseHTMLBlockWithClosingTag(
  text: string,
  lineStart: number,
  startLine: number,
  startOffset: number,
  textLength: number,
  tagName: string
): { token: HTMLBlockToken; nextOffset: number; nextLine: number } | null {
  const closingTag = `</${tagName}>`
  let currentOffset = lineStart
  let currentLine = startLine

  // Find closing tag
  while (currentOffset < textLength) {
    // Find next >
    const gtIndex = text.indexOf('>', currentOffset)
    if (gtIndex === -1) break

    // Check if this is the closing tag
    const possibleClosing = text.slice(Math.max(0, gtIndex - closingTag.length + 1), gtIndex + 1)
    if (possibleClosing.toLowerCase() === closingTag.toLowerCase()) {
      // Found closing tag
      const endOffset = gtIndex + 1

      // Count newlines to update line index
      for (let i = lineStart; i < endOffset; i++) {
        if (text[i] === '\n') currentLine++
      }

      const content = text.slice(lineStart, endOffset)
      return {
        token: {
          type: 'htmlBlock',
          content,
          raw: content,
          position: createTokenPosition(
            createPosition(startLine, 0, startOffset),
            createPosition(currentLine, 0, endOffset)
          ),
        },
        nextOffset: endOffset + (text[endOffset] === '\n' ? 1 : 0),
        nextLine: currentLine + (text[endOffset] === '\n' ? 1 : 0),
      }
    }

    currentOffset = gtIndex + 1
  }

  return null
}

/**
 * Parse HTML block until specific end marker
 */
function parseHTMLBlockUntil(
  text: string,
  lineStart: number,
  startLine: number,
  startOffset: number,
  textLength: number,
  endMarker: string
): { token: HTMLBlockToken; nextOffset: number; nextLine: number } | null {
  const endIndex = text.indexOf(endMarker, lineStart)
  if (endIndex === -1) {
    // No end marker found, consume rest of document
    const content = text.slice(lineStart)
    let currentLine = startLine

    for (let i = lineStart; i < textLength; i++) {
      if (text[i] === '\n') currentLine++
    }

    return {
      token: {
        type: 'htmlBlock',
        content,
        raw: content,
        position: createTokenPosition(
          createPosition(startLine, 0, startOffset),
          createPosition(currentLine, 0, textLength)
        ),
      },
      nextOffset: textLength,
      nextLine: currentLine,
    }
  }

  const endOffset = endIndex + endMarker.length
  let currentLine = startLine

  for (let i = lineStart; i < endOffset; i++) {
    if (text[i] === '\n') currentLine++
  }

  const content = text.slice(lineStart, endOffset)
  return {
    token: {
      type: 'htmlBlock',
      content,
      raw: content,
      position: createTokenPosition(
        createPosition(startLine, 0, startOffset),
        createPosition(currentLine, 0, endOffset)
      ),
    },
    nextOffset: endOffset + (text[endOffset] === '\n' ? 1 : 0),
    nextLine: currentLine + (text[endOffset] === '\n' ? 1 : 0),
  }
}

/**
 * Parse HTML block until blank line
 */
function parseHTMLBlockUntilBlankLine(
  text: string,
  lineStart: number,
  startLine: number,
  startOffset: number,
  textLength: number
): { token: HTMLBlockToken; nextOffset: number; nextLine: number } | null {
  let currentOffset = lineStart
  let currentLine = startLine
  let lastLineEnd = lineStart

  while (currentOffset < textLength) {
    const rowStart = currentOffset
    let rowEnd = currentOffset

    // Find end of line
    while (rowEnd < textLength && text[rowEnd] !== '\n') {
      rowEnd++
    }

    // Check if blank line
    if (rowStart === rowEnd || isLineWhitespace(text, rowStart, rowEnd)) {
      // Found blank line, end of HTML block
      break
    }

    lastLineEnd = rowEnd
    currentOffset = rowEnd + 1
    currentLine++
  }

  const content = text.slice(lineStart, lastLineEnd)
  return {
    token: {
      type: 'htmlBlock',
      content,
      raw: content,
      position: createTokenPosition(
        createPosition(startLine, 0, startOffset),
        createPosition(currentLine, 0, lastLineEnd)
      ),
    },
    nextOffset: currentOffset,
    nextLine: currentLine + 1,
  }
}

/**
 * Check if line contains a complete HTML tag
 */
function isCompleteHTMLTag(text: string, lineStart: number, lineEnd: number): boolean {
  // Must start with <
  if (text[lineStart] !== '<') return false

  // Must end with >
  let actualEnd = lineEnd - 1
  while (actualEnd > lineStart && (text[actualEnd] === ' ' || text[actualEnd] === '\t')) {
    actualEnd--
  }

  if (text[actualEnd] !== '>') return false

  // Check for valid tag structure: <tagname> or </tagname> or <tagname />
  let i = lineStart + 1
  const isClosing = text[i] === '/'
  if (isClosing) i++

  // Tag name must start with letter
  if (i >= actualEnd) return false
  const firstChar = text[i]!
  if (!((firstChar >= 'a' && firstChar <= 'z') || (firstChar >= 'A' && firstChar <= 'Z'))) {
    return false
  }

  return true
}

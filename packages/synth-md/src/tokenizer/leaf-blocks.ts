/**
 * Single-line leaf block tokenizers.
 *
 * Each handler inspects a single already-bounded line (lineStart..lineEnd) and
 * returns a token or null. They are pure functions of their arguments.
 */

import type {
  BlankLineToken,
  BlockquoteToken,
  HeadingToken,
  HorizontalRuleToken,
  LinkReferenceToken,
  ListItemToken,
  ParagraphToken,
} from '../tokens.js'
import { createPosition, createTokenPosition } from '../tokens.js'

/**
 * Character-based heading detection (no regex)
 */
export function tryHeading(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): HeadingToken | null {
  let depth = 0
  let i = lineStart

  // Count # symbols
  while (i < lineEnd && text[i] === '#' && depth < 6) {
    depth++
    i++
  }

  if (depth === 0 || depth > 6) return null

  // Must have space after #
  if (i >= lineEnd || text[i] !== ' ') return null

  i++ // Skip space

  // Extract heading text (no substring until needed)
  if (i >= lineEnd) return null

  const headingText = text.slice(i, lineEnd)
  const raw = text.slice(lineStart, lineEnd)

  return {
    type: 'heading',
    depth: depth as 1 | 2 | 3 | 4 | 5 | 6,
    text: headingText,
    raw,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

/**
 * Character-based list item detection (no regex)
 */
export function tryListItem(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): ListItemToken | null {
  let i = lineStart

  // Count leading spaces
  let indent = 0
  while (i < lineEnd && (text[i] === ' ' || text[i] === '\t')) {
    indent++
    i++
  }

  if (i >= lineEnd) return null

  const markerChar = text[i]!

  // Check for bullet markers (-, *, +)
  if (markerChar === '-' || markerChar === '*' || markerChar === '+') {
    i++
  }
  // Check for numbered list (1., 2., etc.)
  else if (markerChar >= '0' && markerChar <= '9') {
    // Skip digits
    while (i < lineEnd && text[i]! >= '0' && text[i]! <= '9') {
      i++
    }
    // Must have .
    if (i >= lineEnd || text[i] !== '.') return null
    i++ // Skip .
  } else {
    return null
  }

  // Must have space after marker
  if (i >= lineEnd || text[i] !== ' ') return null
  i++ // Skip space

  // Extract text
  const itemText = i < lineEnd ? text.slice(i, lineEnd) : ''

  // Check for task list
  let checked: boolean | undefined
  if (itemText.startsWith('[') && itemText[2] === ']') {
    const checkChar = itemText[1]
    if (checkChar === 'x' || checkChar === ' ') {
      checked = checkChar === 'x'
    }
  }

  const raw = text.slice(lineStart, lineEnd)

  return {
    type: 'listItem',
    indent,
    text: itemText,
    checked,
    raw,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

/**
 * Character-based horizontal rule detection
 */
export function tryHorizontalRule(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): HorizontalRuleToken | null {
  const firstChar = text[lineStart]!
  if (firstChar !== '-' && firstChar !== '*' && firstChar !== '_') {
    return null
  }

  let count = 0
  for (let i = lineStart; i < lineEnd; i++) {
    const c = text[i]!
    if (c === firstChar) {
      count++
    } else if (c !== ' ' && c !== '\t') {
      return null
    }
  }

  if (count < 3) return null

  const raw = text.slice(lineStart, lineEnd)

  return {
    type: 'horizontalRule',
    raw,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

/**
 * Character-based blockquote detection
 */
export function tryBlockquote(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): BlockquoteToken | null {
  if (text[lineStart] !== '>') return null

  let i = lineStart + 1

  // Skip optional space
  if (i < lineEnd && text[i] === ' ') {
    i++
  }

  const quoteText = i < lineEnd ? text.slice(i, lineEnd) : ''
  const raw = text.slice(lineStart, lineEnd)

  return {
    type: 'blockquote',
    text: quoteText,
    raw,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

/**
 * Create paragraph token
 */
export function createParagraph(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): ParagraphToken {
  const paragraphText = text.slice(lineStart, lineEnd)

  return {
    type: 'paragraph',
    text: paragraphText,
    raw: paragraphText,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

/**
 * Create blank line token
 */
export function createBlankLine(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): BlankLineToken {
  const raw = text.slice(lineStart, lineEnd)

  return {
    type: 'blankLine',
    raw,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

/**
 * Try to parse link reference definition: [label]: url "title"
 */
export function tryLinkReference(
  text: string,
  lineStart: number,
  lineEnd: number,
  lineIndex: number,
  offset: number
): LinkReferenceToken | null {
  if (text[lineStart] !== '[') return null

  let i = lineStart + 1

  // Find closing ]
  while (i < lineEnd && text[i] !== ']') {
    i++
  }

  if (i >= lineEnd) return null

  const label = text
    .slice(lineStart + 1, i)
    .toLowerCase()
    .trim()
  if (label.length === 0) return null

  i++ // Skip ]

  // Must have :
  if (i >= lineEnd || text[i] !== ':') return null
  i++ // Skip :

  // Skip whitespace
  while (i < lineEnd && (text[i] === ' ' || text[i] === '\t')) {
    i++
  }

  if (i >= lineEnd) return null

  // Parse URL
  const urlStart = i

  // URL can be in angle brackets or bare
  let url: string
  if (text[i] === '<') {
    i++ // Skip <
    const urlContentStart = i
    while (i < lineEnd && text[i] !== '>') {
      i++
    }
    if (i >= lineEnd) return null
    url = text.slice(urlContentStart, i)
    i++ // Skip >
  } else {
    // Bare URL - until whitespace or end
    while (i < lineEnd && text[i] !== ' ' && text[i] !== '\t') {
      i++
    }
    url = text.slice(urlStart, i)
  }

  if (url.length === 0) return null

  // Skip whitespace
  while (i < lineEnd && (text[i] === ' ' || text[i] === '\t')) {
    i++
  }

  // Parse optional title
  let title: string | undefined
  if (i < lineEnd) {
    const quoteChar = text[i]
    if (quoteChar === '"' || quoteChar === "'" || quoteChar === '(') {
      const closingQuote = quoteChar === '(' ? ')' : quoteChar
      i++ // Skip opening quote
      const titleStart = i
      while (i < lineEnd && text[i] !== closingQuote) {
        i++
      }
      if (i < lineEnd) {
        title = text.slice(titleStart, i)
        i++ // Skip closing quote
      }
    }
  }

  const raw = text.slice(lineStart, lineEnd)

  return {
    type: 'linkReference',
    label,
    url,
    title,
    raw,
    position: createTokenPosition(
      createPosition(lineIndex, 0, offset),
      createPosition(lineIndex, lineEnd - lineStart, offset + (lineEnd - lineStart))
    ),
  }
}

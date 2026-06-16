/**
 * Low-level line scanning helpers shared across block tokenizers.
 *
 * These are pure, allocation-free utilities operating on offsets into the
 * source string (no substring allocation).
 */

/**
 * Check if line is whitespace without substring allocation
 */
export function isLineWhitespace(text: string, start: number, end: number): boolean {
  for (let i = start; i < end; i++) {
    const c = text[i]!
    if (c !== ' ' && c !== '\t' && c !== '\r') {
      return false
    }
  }
  return true
}

/**
 * Get indentation level of a line
 */
export function getIndentLevel(text: string, lineStart: number, lineEnd: number): number {
  let indent = 0
  let i = lineStart

  while (i < lineEnd && (text[i] === ' ' || text[i] === '\t')) {
    if (text[i] === '\t') {
      return 4 // Tab counts as 4 spaces
    }
    indent++
    i++
  }

  return indent
}

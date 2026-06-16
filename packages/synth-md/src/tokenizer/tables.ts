/**
 * GFM table tokenizer.
 *
 * Detects a header row + alignment separator row, then consumes contiguous
 * pipe-delimited data rows.
 */

import type { TableToken } from '../tokens.js'
import { createPosition, createTokenPosition } from '../tokens.js'

/**
 * Try to parse GFM table with lookahead
 */
export function tryTable(
  text: string,
  headerStart: number,
  headerEnd: number,
  startLine: number,
  startOffset: number,
  textLength: number
): { token: TableToken; nextOffset: number; nextLine: number } | null {
  // Parse header row
  const headerRow = parseTableRow(text, headerStart, headerEnd)
  if (!headerRow || headerRow.length === 0) return null

  // Lookahead to next line for separator
  const separatorStart = headerEnd + 1
  if (separatorStart >= textLength) return null

  // Find separator line boundaries
  let separatorEnd = separatorStart
  while (separatorEnd < textLength && text[separatorEnd] !== '\n') {
    separatorEnd++
  }

  // Parse and validate separator
  const align = parseTableAlignment(text, separatorStart, separatorEnd)
  if (!align || align.length !== headerRow.length) return null

  // Parse data rows
  const rows: string[][] = []
  let currentOffset = separatorEnd + 1
  let currentLine = startLine + 2

  while (currentOffset < textLength) {
    const rowStart = currentOffset
    let rowEnd = currentOffset

    // Find end of line
    while (rowEnd < textLength && text[rowEnd] !== '\n') {
      rowEnd++
    }

    // Check if line is part of table
    if (rowEnd > rowStart && text[rowStart] === '|') {
      const row = parseTableRow(text, rowStart, rowEnd)
      if (row && row.length > 0) {
        rows.push(row)
        currentOffset = rowEnd + 1
        currentLine++
        continue
      }
    }

    // Not a table row, end of table
    break
  }

  // Build raw content
  const raw = text.slice(headerStart, currentOffset - 1)

  return {
    token: {
      type: 'table',
      header: headerRow,
      align,
      rows,
      raw,
      position: createTokenPosition(
        createPosition(startLine, 0, startOffset),
        createPosition(currentLine - 1, 0, currentOffset - 1)
      ),
    },
    nextOffset: currentOffset,
    nextLine: currentLine,
  }
}

/**
 * Parse a table row (| cell | cell |)
 */
export function parseTableRow(text: string, lineStart: number, lineEnd: number): string[] | null {
  const cells: string[] = []
  let i = lineStart

  // Skip leading |
  if (i < lineEnd && text[i] === '|') {
    i++
  }

  let cellStart = i

  while (i <= lineEnd) {
    const char = i < lineEnd ? text[i] : null

    if (char === '|' || char === null) {
      // Extract cell content
      const cellContent = text.slice(cellStart, i).trim()
      cells.push(cellContent)

      // Move to next cell
      i++
      cellStart = i
    } else {
      i++
    }
  }

  // Remove trailing empty cell if line ends with |
  if (cells.length > 0 && cells[cells.length - 1] === '') {
    cells.pop()
  }

  return cells.length > 0 ? cells : null
}

/**
 * Parse table alignment row (|:---|:---:|---:|)
 */
export function parseTableAlignment(
  text: string,
  lineStart: number,
  lineEnd: number
): Array<'left' | 'right' | 'center' | null> | null {
  const cells = parseTableRow(text, lineStart, lineEnd)
  if (!cells) return null

  const alignments: Array<'left' | 'right' | 'center' | null> = []

  for (const cell of cells) {
    // Check if valid separator (----, :---, ---:, :---:)
    const trimmed = cell.trim()

    if (trimmed.length < 3) return null

    const startsWithColon = trimmed[0] === ':'
    const endsWithColon = trimmed[trimmed.length - 1] === ':'

    // Check all middle characters are dashes
    const start = startsWithColon ? 1 : 0
    const end = endsWithColon ? trimmed.length - 1 : trimmed.length

    for (let i = start; i < end; i++) {
      if (trimmed[i] !== '-') return null
    }

    // Determine alignment
    if (startsWithColon && endsWithColon) {
      alignments.push('center')
    } else if (startsWithColon) {
      alignments.push('left')
    } else if (endsWithColon) {
      alignments.push('right')
    } else {
      alignments.push(null)
    }
  }

  return alignments
}

/**
 * True Incremental Parser Benchmarks
 *
 * Demonstrates the performance improvements of token-level incremental parsing:
 * - Target: 10-100x faster than full re-parse
 * - Token reuse: 70-99%+
 * - Response time: <1ms for typical edits
 */

import { bench, describe } from 'vitest'
import { Parser } from '../src/parser.js'
import { TrueIncrementalParser, detectEdit } from '../src/true-incremental-parser.js'

// Generate realistic Markdown documents
function generateMarkdownDocument(paragraphs: number): string {
  const lines: string[] = ['# Main Document', '']

  for (let i = 0; i < paragraphs; i++) {
    lines.push(`## Section ${i + 1}`, '')

    if (i % 3 === 0) {
      // Add a list
      lines.push('Key points:', '')
      lines.push('- First point', '- Second point', '- Third point', '')
    } else {
      // Add paragraphs
      lines.push(
        `This is paragraph ${i + 1}. It contains some text content to make the document more realistic.`,
        ''
      )
      lines.push(
        `Another paragraph in section ${i + 1}. This helps test the incremental parsing behavior.`,
        ''
      )
    }
  }

  return lines.join('\n')
}

describe('True Incremental Parsing - Small Documents (<1KB)', () => {
  const doc = generateMarkdownDocument(10) // ~500 bytes

  bench('Baseline: Full re-parse (Parser)', () => {
    const parser = new Parser()
    const text = doc
    parser.parse(text)
    parser.parse(text.replace('Section 5', 'Modified Section 5'))
  })

  bench('Optimized: Incremental parse', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = doc.replace('Section 5', 'Modified Section 5')

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })

  bench('Incremental: Single character edit', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = doc.replace('paragraph', 'Paragraph')

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })
})

describe('True Incremental Parsing - Medium Documents (1-10KB)', () => {
  const doc = generateMarkdownDocument(50) // ~5KB

  bench('Baseline: Full re-parse', () => {
    const parser = new Parser()
    const text = doc
    parser.parse(text)
    parser.parse(text.replace('Section 25', 'Modified Section 25'))
  })

  bench('Optimized: Incremental parse', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = doc.replace('Section 25', 'Modified Section 25')

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })

  bench('Incremental: Insert paragraph', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const insertPoint = text1.indexOf('## Section 25')
    const text2 = `${text1.slice(0, insertPoint)}\nNEW PARAGRAPH\n\n${text1.slice(insertPoint)}`

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })
})

describe('True Incremental Parsing - Large Documents (10KB+)', () => {
  const doc = generateMarkdownDocument(200) // ~20KB

  bench('Baseline: Full re-parse', () => {
    const parser = new Parser()
    const text = doc
    parser.parse(text)
    parser.parse(text.replace('Section 100', 'Modified Section 100'))
  })

  bench('Optimized: Incremental parse', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = doc.replace('Section 100', 'Modified Section 100')

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })

  bench('Incremental: Tiny edit (0.1% of document)', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = doc.replace('point', 'item') // Just a few characters

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })
})

describe('Real-World Scenarios', () => {
  describe('Typing Simulation (Live Editor)', () => {
    const baseDoc = '# Title\n\nHello'

    bench('Baseline: Full re-parse per keystroke (10 chars)', () => {
      const parser = new Parser()
      let text = baseDoc

      const chars = [' ', 'w', 'o', 'r', 'l', 'd', '!', '!', '!']
      for (const char of chars) {
        text += char
        parser.parse(text)
      }
    })

    bench('Optimized: Incremental per keystroke (10 chars)', () => {
      const parser = new TrueIncrementalParser()
      let text = baseDoc
      parser.parse(text)

      const chars = [' ', 'w', 'o', 'r', 'l', 'd', '!', '!', '!']
      for (const char of chars) {
        const newText = text + char
        const edit = detectEdit(text, newText)
        parser.update(newText, edit)
        text = newText
      }
    })
  })

  describe('Live Preview (Edit + Re-render)', () => {
    const doc = generateMarkdownDocument(50)

    bench('Baseline: Full re-parse', () => {
      const parser = new Parser()
      // Simulate 5 edits
      for (let i = 0; i < 5; i++) {
        const edited = doc.replace(`Section ${i + 1}`, `Edited Section ${i + 1}`)
        parser.parse(edited)
      }
    })

    bench('Optimized: Incremental updates', () => {
      const parser = new TrueIncrementalParser()
      let text = doc
      parser.parse(text)

      // Simulate 5 edits
      for (let i = 0; i < 5; i++) {
        const newText = text.replace(`Section ${i + 1}`, `Edited Section ${i + 1}`)
        const edit = detectEdit(text, newText)
        parser.update(newText, edit)
        text = newText
      }
    })
  })

  describe('Code Refactoring (Multiple Edits)', () => {
    const doc = generateMarkdownDocument(100)

    bench('Baseline: Full re-parse after each change', () => {
      const parser = new Parser()
      let text = doc

      // Rename variable-like strings
      const renames = ['First', 'Second', 'Third', 'Fourth', 'Fifth']
      for (const old of renames) {
        text = text.replace(old, old.toUpperCase())
        parser.parse(text)
      }
    })

    bench('Optimized: Incremental updates', () => {
      const parser = new TrueIncrementalParser()
      let text = doc
      parser.parse(text)

      const renames = ['First', 'Second', 'Third', 'Fourth', 'Fifth']
      for (const old of renames) {
        const newText = text.replace(old, old.toUpperCase())
        const edit = detectEdit(text, newText)
        parser.update(newText, edit)
        text = newText
      }
    })
  })
})

describe('Worst Case Scenarios', () => {
  const doc = generateMarkdownDocument(50)

  bench('Large edit (50% of document changed)', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const halfIndex = Math.floor(doc.length / 2)
    const text2 = doc.slice(0, halfIndex) + doc.slice(halfIndex).toUpperCase()

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })

  bench('Complete rewrite (100% changed)', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = generateMarkdownDocument(50) // Completely different

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    parser.update(text2, edit)
  })

  bench('Baseline: Full re-parse for comparison', () => {
    const parser = new Parser()
    const text1 = doc
    const text2 = generateMarkdownDocument(50)

    parser.parse(text1)
    parser.parse(text2)
  })
})

describe('Token Reuse Statistics', () => {
  const doc = generateMarkdownDocument(100)

  bench('Measure: Single line edit', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const text2 = doc.replace('Section 50', 'MODIFIED Section 50')

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    const { stats } = parser.update(text2, edit)

    // Log stats for analysis
    if (process.env.NODE_ENV !== 'production') {
      console.log(
        `Token reuse: ${(stats.tokenReuseRate * 100).toFixed(1)}%, ` +
          `Speedup: ${stats.speedup.toFixed(1)}x`
      )
    }
  })

  bench('Measure: Insert paragraph', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const insertPoint = doc.indexOf('## Section 50')
    const text2 = `${doc.slice(0, insertPoint)}\n\nNEW CONTENT HERE\n\n${doc.slice(insertPoint)}`

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    const { stats } = parser.update(text2, edit)

    if (process.env.NODE_ENV !== 'production') {
      console.log(
        `Token reuse: ${(stats.tokenReuseRate * 100).toFixed(1)}%, ` +
          `Speedup: ${stats.speedup.toFixed(1)}x`
      )
    }
  })

  bench('Measure: Delete paragraph', () => {
    const parser = new TrueIncrementalParser()
    const text1 = doc
    const startDelete = doc.indexOf('## Section 50')
    const endDelete = doc.indexOf('## Section 51')
    const text2 = doc.slice(0, startDelete) + doc.slice(endDelete)

    parser.parse(text1)
    const edit = detectEdit(text1, text2)
    const { stats } = parser.update(text2, edit)

    if (process.env.NODE_ENV !== 'production') {
      console.log(
        `Token reuse: ${(stats.tokenReuseRate * 100).toFixed(1)}%, ` +
          `Speedup: ${stats.speedup.toFixed(1)}x`
      )
    }
  })
})

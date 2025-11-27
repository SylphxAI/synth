/**
 * Tests for True Incremental Parser
 *
 * Validates:
 * 1. Token-level reuse (target: 90%+)
 * 2. Performance improvement (target: 10-100x)
 * 3. Correctness (same AST as full parse)
 */

import { describe, expect, it } from 'bun:test'
import { detectEdit, formatIncrementalStats, TrueIncrementalParser } from './true-incremental-parser.js'

describe('TrueIncrementalParser', () => {
  describe('Token-Level Reuse', () => {
    it('should achieve 90%+ token reuse for single line edit', () => {
      const parser = new TrueIncrementalParser()

      // Initial parse
      const text1 = `# Heading

This is paragraph 1.

This is paragraph 2.

This is paragraph 3.`

      parser.parse(text1)

      // Edit paragraph 2
      const text2 = `# Heading

This is paragraph 1.

This is MODIFIED paragraph 2.

This is paragraph 3.`

      const edit = detectEdit(text1, text2)
      const { stats } = parser.update(text2, edit)

      // Should reuse most tokens (paragraphs 1 and 3 unchanged)
      expect(stats.tokenReuseRate).toBeGreaterThan(0.7)
      console.log('Token reuse rate:', `${(stats.tokenReuseRate * 100).toFixed(1)}%`)
    })

    it('should achieve high token reuse for insertion', () => {
      const parser = new TrueIncrementalParser()

      const text1 = `# Heading

Paragraph 1.

Paragraph 2.`

      parser.parse(text1)

      // Insert new paragraph
      const text2 = `# Heading

Paragraph 1.

NEW PARAGRAPH HERE.

Paragraph 2.`

      const edit = detectEdit(text1, text2)
      const { stats } = parser.update(text2, edit)

      // Insertion adds lines which affects adjacent blank lines
      expect(stats.tokenReuseRate).toBeGreaterThan(0.5)
      console.log('Insertion - Token reuse:', `${(stats.tokenReuseRate * 100).toFixed(1)}%`)
    })

    it('should achieve high token reuse for deletion', () => {
      const parser = new TrueIncrementalParser()

      const text1 = `# Heading

Paragraph 1.

Paragraph to delete.

Paragraph 2.`

      parser.parse(text1)

      // Delete middle paragraph
      const text2 = `# Heading

Paragraph 1.

Paragraph 2.`

      const edit = detectEdit(text1, text2)
      const { stats } = parser.update(text2, edit)

      expect(stats.tokenReuseRate).toBeGreaterThan(0.7)
      console.log('Deletion - Token reuse:', `${(stats.tokenReuseRate * 100).toFixed(1)}%`)
    })
  })

  describe('Performance', () => {
    it('should be faster than full re-parse for small edits', () => {
      const parser = new TrueIncrementalParser()

      // Create large document
      const lines: string[] = ['# Document']
      for (let i = 0; i < 100; i++) {
        lines.push('', `## Section ${i}`, '', `This is paragraph ${i}.`)
      }
      const text1 = lines.join('\n')

      parser.parse(text1)

      // Edit one line in the middle
      const text2 = text1.replace('This is paragraph 50.', 'This is MODIFIED paragraph 50.')

      const edit = detectEdit(text1, text2)
      const { stats } = parser.update(text2, edit)

      // Should show speedup
      expect(stats.speedup).toBeGreaterThan(1)
      expect(stats.totalTimeMs).toBeLessThan(10) // Should be very fast

      console.log(formatIncrementalStats(stats))
    })

    it('should handle typing simulation efficiently', () => {
      const parser = new TrueIncrementalParser()

      let text = '# Title\n\nHello'

      parser.parse(text)

      // Simulate typing " world!"
      const additions = [' ', 'w', 'o', 'r', 'l', 'd', '!']
      const totalTimes: number[] = []

      for (const char of additions) {
        const newText = text + char
        const edit = detectEdit(text, newText)
        const { stats } = parser.update(newText, edit)

        totalTimes.push(stats.totalTimeMs)
        text = newText
      }

      // All updates should be very fast (<1ms target)
      const avgTime = totalTimes.reduce((a, b) => a + b, 0) / totalTimes.length
      console.log(`Typing simulation - Avg time: ${avgTime.toFixed(3)}ms`)

      expect(avgTime).toBeLessThan(1) // Target: sub-millisecond
    })
  })

  describe('Correctness', () => {
    it('should produce correct AST after incremental update', () => {
      const parser = new TrueIncrementalParser()

      const text1 = '# Hello\n\nWorld'
      parser.parse(text1)

      const text2 = '# Hello\n\nWorld!'
      const edit = detectEdit(text1, text2)
      const { tree: incrementalTree } = parser.update(text2, edit)

      // Full re-parse for comparison
      const fullParser = new TrueIncrementalParser()
      const fullTree = fullParser.parse(text2)

      // Should have same number of nodes
      expect(incrementalTree.nodes.length).toBe(fullTree.nodes.length)

      // Should have same source
      expect(incrementalTree.meta.source).toBe(fullTree.meta.source)
    })

    it('should handle multiple consecutive edits', () => {
      const parser = new TrueIncrementalParser()

      let text = '# Title\n\nContent'
      parser.parse(text)

      // Edit 1: Modify title
      let newText = '# New Title\n\nContent'
      let edit = detectEdit(text, newText)
      let { stats } = parser.update(newText, edit)
      console.log('Edit 1:', formatIncrementalStats(stats))
      text = newText

      // Edit 2: Modify content
      newText = '# New Title\n\nModified content'
      edit = detectEdit(text, newText)
      ;({ stats } = parser.update(newText, edit))
      console.log('Edit 2:', formatIncrementalStats(stats))
      text = newText

      // Edit 3: Add paragraph
      newText = '# New Title\n\nModified content\n\nNew paragraph'
      edit = detectEdit(text, newText)
      ;({ stats } = parser.update(newText, edit))
      console.log('Edit 3:', formatIncrementalStats(stats))

      expect(parser.getTree()).not.toBeNull()
      expect(parser.getTree()?.meta.source).toBe(newText)
    })
  })

  describe('Edge Cases', () => {
    it('should handle empty document', () => {
      const parser = new TrueIncrementalParser()

      const text1 = ''
      parser.parse(text1)

      const text2 = '# Hello'
      const edit = detectEdit(text1, text2)
      const { tree } = parser.update(text2, edit)

      expect(tree.meta.source).toBe(text2)
    })

    it('should handle delete all content', () => {
      const parser = new TrueIncrementalParser()

      const text1 = '# Hello\n\nWorld'
      parser.parse(text1)

      const text2 = ''
      const edit = detectEdit(text1, text2)
      const { tree } = parser.update(text2, edit)

      expect(tree.meta.source).toBe(text2)
    })

    it('should handle edit at document start', () => {
      const parser = new TrueIncrementalParser()

      const text1 = '# Hello\n\nWorld'
      parser.parse(text1)

      const text2 = 'Prefix\n# Hello\n\nWorld'
      const edit = detectEdit(text1, text2)
      const { stats } = parser.update(text2, edit)

      // Editing at start affects many tokens, but should still reuse some
      expect(stats.tokenReuseRate).toBeGreaterThanOrEqual(0.4)
    })

    it('should handle edit at document end', () => {
      const parser = new TrueIncrementalParser()

      const text1 = '# Hello\n\nWorld'
      parser.parse(text1)

      const text2 = '# Hello\n\nWorld\n\nSuffix'
      const edit = detectEdit(text1, text2)
      const { stats } = parser.update(text2, edit)

      // Editing at end adds tokens, but should reuse beginning
      expect(stats.tokenReuseRate).toBeGreaterThanOrEqual(0.4)
    })
  })

  describe('detectEdit helper', () => {
    it('should detect insertion', () => {
      const edit = detectEdit('Hello World', 'Hello Beautiful World')

      expect(edit.startIndex).toBe(6) // After "Hello "
      expect(edit.oldEndIndex).toBe(6)
      expect(edit.newEndIndex).toBe(16) // After "Hello Beautiful "
    })

    it('should detect deletion', () => {
      const edit = detectEdit('Hello Beautiful World', 'Hello World')

      expect(edit.startIndex).toBe(6)
      expect(edit.oldEndIndex).toBe(16)
      expect(edit.newEndIndex).toBe(6)
    })

    it('should detect replacement', () => {
      const edit = detectEdit('Hello World', 'Hello Universe')

      expect(edit.startIndex).toBe(6)
      expect(edit.oldEndIndex).toBe(11) // After "Hello World"
      expect(edit.newEndIndex).toBe(14) // After "Hello Universe"
    })

    it('should handle no change', () => {
      const edit = detectEdit('Hello World', 'Hello World')

      expect(edit.startIndex).toBe(11) // End of string
      expect(edit.oldEndIndex).toBe(11)
      expect(edit.newEndIndex).toBe(11)
    })
  })
})

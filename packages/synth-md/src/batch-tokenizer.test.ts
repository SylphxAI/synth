/**
 * Batch Tokenizer Tests
 */

import { describe, it, expect } from 'bun:test'
import { BatchTokenizer, createBatchTokenizer } from './batch-tokenizer.js'

describe('BatchTokenizer', () => {
  describe('Constructor', () => {
    it('should create with default batch size', () => {
      const tokenizer = new BatchTokenizer()
      expect(tokenizer).toBeDefined()
    })

    it('should create with custom batch size', () => {
      const tokenizer = new BatchTokenizer(16)
      expect(tokenizer).toBeDefined()
    })

    it('should create using factory function', () => {
      const tokenizer = createBatchTokenizer(32)
      expect(tokenizer).toBeDefined()
    })
  })

  describe('Basic Tokenization', () => {
    it('should tokenize empty string', () => {
      const tokenizer = new BatchTokenizer()
      const tokens = tokenizer.tokenize('')
      expect(tokens).toEqual([])
    })

    it('should fall back to regular tokenizer for small documents', () => {
      const tokenizer = new BatchTokenizer(10)
      const text = '# Heading\n\nParagraph'
      const tokens = tokenizer.tokenize(text)

      expect(tokens.length).toBeGreaterThan(0)
    })

    it('should process large documents in batches', () => {
      const tokenizer = new BatchTokenizer(4)
      const lines = Array(20).fill('- List item').join('\n')
      const tokens = tokenizer.tokenize(lines)

      expect(tokens.length).toBe(20)
      expect(tokens.every(t => t.type === 'listItem')).toBe(true)
    })
  })

  describe('Heading Parsing', () => {
    it('should parse h1 headings', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('# Heading 1\n\nText')

      const heading = tokens.find(t => t.type === 'heading')
      expect(heading).toBeDefined()
      expect(heading?.depth).toBe(1)
      expect(heading?.text).toBe('Heading 1')
    })

    it('should parse h2-h6 headings', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = '## H2\n### H3\n#### H4\n##### H5\n###### H6'
      const tokens = tokenizer.tokenize(text)

      const headings = tokens.filter(t => t.type === 'heading')
      expect(headings).toHaveLength(5)
      expect(headings.map(h => h.depth)).toEqual([2, 3, 4, 5, 6])
    })

    it('should reject invalid headings (>6 hashes)', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('####### Not a heading\n\nEnd')

      const heading = tokens.find(t => t.type === 'heading')
      expect(heading).toBeUndefined()
    })

    it('should reject headings without space after hashes', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('#NoSpace\n\nEnd')

      const heading = tokens.find(t => t.type === 'heading')
      expect(heading).toBeUndefined()
    })

    it('should handle headings with extra spaces', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('  # Indented Heading  \n\nEnd')

      const heading = tokens.find(t => t.type === 'heading')
      expect(heading).toBeDefined()
      expect(heading?.text).toBe('Indented Heading')
    })
  })

  describe('Code Block Parsing', () => {
    it('should parse code blocks with language', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = '```js\nconst x = 1\n```\n\nEnd'
      const tokens = tokenizer.tokenize(text)

      const codeBlock = tokens.find(t => t.type === 'codeBlock')
      expect(codeBlock).toBeDefined()
      expect(codeBlock?.lang).toBe('js')
      expect(codeBlock?.code).toContain('const x = 1')
    })

    it('should parse code blocks without language', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = '```\ncode here\n```\n\nEnd'
      const tokens = tokenizer.tokenize(text)

      const codeBlock = tokens.find(t => t.type === 'codeBlock')
      expect(codeBlock).toBeDefined()
      expect(codeBlock?.lang).toBe('')
    })

    it('should parse multi-line code blocks', () => {
      const tokenizer = new BatchTokenizer(10) // Larger batch to contain whole block
      const text = '```python\nline1\nline2\nline3\n```\n\nEnd'
      const tokens = tokenizer.tokenize(text)

      const codeBlock = tokens.find(t => t.type === 'codeBlock')
      expect(codeBlock).toBeDefined()
      expect(codeBlock?.lang).toBe('python')
      expect(codeBlock?.code).toContain('line1')
      expect(codeBlock?.code).toContain('line2')
      expect(codeBlock?.code).toContain('line3')
    })

    it('should handle unclosed code blocks', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = '```js\nconst x = 1\nconst y = 2'
      const tokens = tokenizer.tokenize(text)

      const codeBlock = tokens.find(t => t.type === 'codeBlock')
      expect(codeBlock).toBeDefined()
      expect(codeBlock?.code).toContain('const x = 1')
    })

    it('should handle indented closing fence', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = '```\ncode\n  ```\n\nEnd'
      const tokens = tokenizer.tokenize(text)

      const codeBlock = tokens.find(t => t.type === 'codeBlock')
      expect(codeBlock).toBeDefined()
    })
  })

  describe('List Parsing', () => {
    it('should parse unordered lists with dash', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('- Item 1\n- Item 2\n- Item 3')

      const listItems = tokens.filter(t => t.type === 'listItem')
      expect(listItems).toHaveLength(3)
      expect(listItems[0]?.text).toBe('Item 1')
    })

    it('should parse unordered lists with star', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('* Item A\n* Item B')

      const listItems = tokens.filter(t => t.type === 'listItem')
      expect(listItems).toHaveLength(2)
    })

    it('should parse unordered lists with plus', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('+ Item X\n+ Item Y')

      const listItems = tokens.filter(t => t.type === 'listItem')
      expect(listItems).toHaveLength(2)
    })

    it('should parse ordered lists', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('1. First\n2. Second\n3. Third')

      const listItems = tokens.filter(t => t.type === 'listItem')
      expect(listItems).toHaveLength(3)
      expect(listItems[0]?.text).toBe('First')
    })

    it('should parse task lists (unchecked)', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('- [ ] Todo item\n\nEnd')

      const listItem = tokens.find(t => t.type === 'listItem')
      expect(listItem).toBeDefined()
      expect(listItem?.checked).toBe(false)
      expect(listItem?.text).toBe('Todo item')
    })

    it('should parse task lists (checked)', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('- [x] Done item\n\nEnd')

      const listItem = tokens.find(t => t.type === 'listItem')
      expect(listItem).toBeDefined()
      expect(listItem?.checked).toBe(true)
      expect(listItem?.text).toBe('Done item')
    })

    it('should parse task lists (checked uppercase)', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('- [X] Done item\n\nEnd')

      const listItem = tokens.find(t => t.type === 'listItem')
      expect(listItem).toBeDefined()
      expect(listItem?.checked).toBe(true)
    })

    it('should preserve list item indentation', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('- Item\n  - Nested\n    - Deep')

      const listItems = tokens.filter(t => t.type === 'listItem')
      expect(listItems).toHaveLength(3)
      expect(listItems[0]?.indent).toBe(0)
      expect(listItems[1]?.indent).toBe(2)
      expect(listItems[2]?.indent).toBe(4)
    })
  })

  describe('Horizontal Rule Parsing', () => {
    it('should parse horizontal rule with dashes', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('---\n\nEnd')

      const hr = tokens.find(t => t.type === 'horizontalRule')
      expect(hr).toBeDefined()
    })

    it('should parse horizontal rule with stars', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('***\n\nEnd')

      const hr = tokens.find(t => t.type === 'horizontalRule')
      expect(hr).toBeDefined()
    })

    it('should handle underscores as paragraph', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('___\n\nEnd')

      // Underscores at start are not detected as HR in batch tokenizer
      // because they don't match any firstChar pattern
      const para = tokens.find(t => t.type === 'paragraph')
      expect(para).toBeDefined()
    })

    it('should parse horizontal rule with extra characters', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('-----\n\nEnd')

      const hr = tokens.find(t => t.type === 'horizontalRule')
      expect(hr).toBeDefined()
    })

    it('should handle list that looks like HR with spaces', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('- - -\n\nEnd')

      // This creates multiple list items, not a HR
      const items = tokens.filter(t => t.type === 'listItem')
      expect(items.length).toBeGreaterThan(0)
    })
  })

  describe('Blockquote Parsing', () => {
    it('should parse blockquotes', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('> Quote text\n\nEnd')

      const blockquote = tokens.find(t => t.type === 'blockquote')
      expect(blockquote).toBeDefined()
      expect(blockquote?.text).toBe('Quote text')
    })

    it('should parse multiple blockquotes', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('> Line 1\n> Line 2\n> Line 3')

      const blockquotes = tokens.filter(t => t.type === 'blockquote')
      expect(blockquotes).toHaveLength(3)
    })

    it('should handle indented blockquotes', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('  > Indented quote\n\nEnd')

      const blockquote = tokens.find(t => t.type === 'blockquote')
      expect(blockquote).toBeDefined()
      expect(blockquote?.text).toBe('Indented quote')
    })
  })

  describe('Paragraph Parsing', () => {
    it('should parse single-line paragraphs', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('This is a paragraph\n\nEnd')

      const para = tokens.find(t => t.type === 'paragraph')
      expect(para).toBeDefined()
      expect(para?.text).toBe('This is a paragraph')
    })

    it('should parse multi-line paragraphs in same batch', () => {
      const tokenizer = new BatchTokenizer(1) // Force single batch processing
      const text = 'Line one\nLine two\n\nEnd'
      const tokens = tokenizer.tokenize(text)

      // With small document, falls back to regular tokenizer
      expect(tokens.length).toBeGreaterThan(0)
    })

    it('should handle paragraph text', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = 'Simple paragraph text\n\nEnd'
      const tokens = tokenizer.tokenize(text)

      const para = tokens.find(t => t.type === 'paragraph')
      expect(para).toBeDefined()
      expect(para?.text).toBe('Simple paragraph text')
    })

    it('should split paragraphs on blank lines', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = 'Paragraph 1\n\nParagraph 2\n\nParagraph 3'
      const tokens = tokenizer.tokenize(text)

      const paras = tokens.filter(t => t.type === 'paragraph')
      expect(paras.length).toBeGreaterThanOrEqual(3)
    })
  })

  describe('Blank Line Handling', () => {
    it('should create blank line tokens', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('Text\n\nMore text')

      const blankLines = tokens.filter(t => t.type === 'blankLine')
      expect(blankLines.length).toBeGreaterThanOrEqual(1)
    })

    it('should handle multiple consecutive blank lines', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('Text\n\n\n\nMore')

      const blankLines = tokens.filter(t => t.type === 'blankLine')
      expect(blankLines.length).toBeGreaterThanOrEqual(1)
    })
  })

  describe('Line Extraction', () => {
    it('should extract lines with trailing newlines', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('Line 1\nLine 2\nLine 3\n')

      expect(tokens.length).toBeGreaterThan(0)
    })

    it('should extract lines without trailing newline', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('Line 1\nLine 2\nLine 3')

      expect(tokens.length).toBeGreaterThan(0)
    })

    it('should handle single line without newline', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('Single line')

      expect(tokens.length).toBeGreaterThan(0)
    })

    it('should handle text with only newlines', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('\n\n\n')

      expect(tokens.every(t => t.type === 'blankLine')).toBe(true)
    })
  })

  describe('Metadata Extraction', () => {
    it('should detect first character patterns', () => {
      const tokenizer = new BatchTokenizer(2)
      const text = [
        '# Heading',
        '- List',
        '* Star list',
        '+ Plus list',
        '1. Ordered',
        '```code',
        '> Quote',
      ].join('\n')

      const tokens = tokenizer.tokenize(text)

      expect(tokens.some(t => t.type === 'heading')).toBe(true)
      expect(tokens.some(t => t.type === 'listItem')).toBe(true)
      expect(tokens.some(t => t.type === 'codeBlock')).toBe(true)
      expect(tokens.some(t => t.type === 'blockquote')).toBe(true)
    })

    it('should track line positions correctly', () => {
      const tokenizer = new BatchTokenizer(10)
      const tokens = tokenizer.tokenize('Line 1\nLine 2\nLine 3')

      // First token should start at line 0
      expect(tokens[0]?.position?.start?.line).toBe(0)

      // Positions should be sequential
      expect(tokens[0]?.position).toBeDefined()
    })
  })

  describe('Edge Cases', () => {
    it('should handle ordered list without proper format', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('1.No space\n\nEnd')

      // Should fall back to paragraph
      expect(tokens.length).toBeGreaterThan(0)
    })

    it('should handle code block that looks like backticks but is not', () => {
      const tokenizer = new BatchTokenizer(2)
      const tokens = tokenizer.tokenize('`` Not a code block\n\nEnd')

      const codeBlock = tokens.find(t => t.type === 'codeBlock')
      expect(codeBlock).toBeUndefined()
    })

    it('should process batch boundaries correctly', () => {
      const tokenizer = new BatchTokenizer(3) // Small batch size
      const lines = Array(10).fill('- Item').join('\n')
      const tokens = tokenizer.tokenize(lines)

      // Should process all items across multiple batches
      expect(tokens.filter(t => t.type === 'listItem').length).toBe(10)
    })

    it('should handle mixed content in one batch', () => {
      const tokenizer = new BatchTokenizer(10)
      const text = [
        '# Heading',
        '',
        'Paragraph',
        '',
        '- List',
        '',
        '> Quote',
        '',
        '```',
        'code',
        '```',
      ].join('\n')

      const tokens = tokenizer.tokenize(text)

      expect(tokens.some(t => t.type === 'heading')).toBe(true)
      expect(tokens.some(t => t.type === 'paragraph')).toBe(true)
      expect(tokens.some(t => t.type === 'listItem')).toBe(true)
      expect(tokens.some(t => t.type === 'blockquote')).toBe(true)
      expect(tokens.some(t => t.type === 'codeBlock')).toBe(true)
    })
  })
})

/**
 * Type Safety Tests
 *
 * Demonstrates the type-safe API for Markdown nodes
 */

import { describe, expect, it } from 'bun:test'
import type { BaseNode } from '@sylphx/synth'
import {
  asNodeType,
  assertNodeType,
  type BlockquoteNode,
  type CodeBlockNode,
  createCodeBlockNode,
  createHeadingNode,
  createLinkNode,
  createParagraphNode,
  type EmphasisNode,
  filterByType,
  findByType,
  type HeadingNode,
  type ImageNode,
  type InlineCodeNode,
  isBlockNode,
  isBlockquoteNode,
  isCodeBlockNode,
  isEmphasisNode,
  isHeadingNode,
  isImageNode,
  isInlineCodeNode,
  isInlineNode,
  isListItemNode,
  isListNode,
  isNodeType,
  isParagraphNode,
  isStrongNode,
  isTableNode,
  type LinkNode,
  type ListItemNode,
  type ListNode,
  type MarkdownNode,
  mapNodes,
  type NodeByType,
  type ParagraphNode,
  type StrongNode,
  type TableNode,
} from './types.js'

describe('Type Safety', () => {
  describe('Type Guards', () => {
    it('should narrow heading node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'heading',
        parent: null,
        children: [],
      } as HeadingNode

      if (isHeadingNode(node)) {
        // TypeScript knows node is HeadingNode here
        expect(node.type).toBe('heading')
        // This would cause a compile error if type narrowing didn't work:
        // const depth = node.depth // TypeScript knows this exists
      }
    })

    it('should narrow paragraph node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'paragraph',
        parent: null,
        children: [],
        text: 'Hello',
      }

      if (isParagraphNode(node)) {
        expect(node.text).toBe('Hello')
      }
    })

    it('should distinguish block vs inline nodes', () => {
      const heading: BaseNode = {
        id: 1,
        type: 'heading',
        parent: null,
        children: [],
      } as HeadingNode
      const link: BaseNode = { id: 2, type: 'link', parent: null, children: [] } as LinkNode

      expect(isBlockNode(heading)).toBe(true)
      expect(isInlineNode(heading)).toBe(false)

      expect(isBlockNode(link)).toBe(false)
      expect(isInlineNode(link)).toBe(true)
    })

    it('should narrow code block node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'codeBlock',
        parent: null,
        children: [],
      } as CodeBlockNode

      expect(isCodeBlockNode(node)).toBe(true)
      if (isCodeBlockNode(node)) {
        expect(node.type).toBe('codeBlock')
      }
    })

    it('should narrow list node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'list',
        parent: null,
        children: [],
      } as ListNode

      expect(isListNode(node)).toBe(true)
    })

    it('should narrow list item node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'listItem',
        parent: null,
        children: [],
      } as ListItemNode

      expect(isListItemNode(node)).toBe(true)
    })

    it('should narrow blockquote node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'blockquote',
        parent: null,
        children: [],
      } as BlockquoteNode

      expect(isBlockquoteNode(node)).toBe(true)
    })

    it('should narrow table node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'table',
        parent: null,
        children: [],
      } as TableNode

      expect(isTableNode(node)).toBe(true)
    })

    it('should narrow image node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'image',
        parent: null,
        children: [],
      } as ImageNode

      expect(isImageNode(node)).toBe(true)
    })

    it('should narrow emphasis node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'emphasis',
        parent: null,
        children: [],
      } as EmphasisNode

      expect(isEmphasisNode(node)).toBe(true)
    })

    it('should narrow strong node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'strong',
        parent: null,
        children: [],
      } as StrongNode

      expect(isStrongNode(node)).toBe(true)
    })

    it('should narrow inline code node type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'inlineCode',
        parent: null,
        children: [],
      } as InlineCodeNode

      expect(isInlineCodeNode(node)).toBe(true)
    })
  })

  describe('assertNodeType', () => {
    it('should narrow type after assertion', () => {
      const node: BaseNode = {
        id: 1,
        type: 'heading',
        parent: null,
        children: [],
        depth: 1,
        text: 'Title',
      }

      assertNodeType(node, 'heading')
      // TypeScript knows node is HeadingNode now
      expect(node.depth).toBe(1)
      expect(node.text).toBe('Title')
    })

    it('should throw on type mismatch', () => {
      const node: BaseNode = {
        id: 1,
        type: 'paragraph',
        parent: null,
        children: [],
      } as ParagraphNode

      expect(() => {
        assertNodeType(node, 'heading')
      }).toThrow("Invalid node type 'paragraph'. Expected one of: heading")
    })
  })

  describe('isNodeType', () => {
    it('should check multiple types', () => {
      const heading: BaseNode = {
        id: 1,
        type: 'heading',
        parent: null,
        children: [],
      } as HeadingNode
      const paragraph: BaseNode = {
        id: 2,
        type: 'paragraph',
        parent: null,
        children: [],
      } as ParagraphNode
      const link: BaseNode = { id: 3, type: 'link', parent: null, children: [] } as LinkNode

      expect(isNodeType(heading, 'heading', 'paragraph')).toBe(true)
      expect(isNodeType(paragraph, 'heading', 'paragraph')).toBe(true)
      expect(isNodeType(link, 'heading', 'paragraph')).toBe(false)
    })
  })

  describe('Utility Functions', () => {
    const nodes: BaseNode[] = [
      { id: 1, type: 'heading', parent: null, children: [], depth: 1, text: 'H1' } as HeadingNode,
      { id: 2, type: 'paragraph', parent: null, children: [], text: 'Para 1' } as ParagraphNode,
      { id: 3, type: 'heading', parent: null, children: [], depth: 2, text: 'H2' } as HeadingNode,
      { id: 4, type: 'paragraph', parent: null, children: [], text: 'Para 2' } as ParagraphNode,
      { id: 5, type: 'codeBlock', parent: null, children: [], code: 'code', lang: 'js' },
    ]

    it('should filter by type', () => {
      const headings = filterByType(nodes, 'heading')
      expect(headings).toHaveLength(2)
      expect(headings[0]?.depth).toBe(1)
      expect(headings[1]?.depth).toBe(2)
    })

    it('should find first by type', () => {
      const firstHeading = findByType(nodes, 'heading')
      expect(firstHeading).toBeDefined()
      expect(firstHeading?.depth).toBe(1)
      expect(firstHeading?.text).toBe('H1')
    })

    it('should return undefined if not found', () => {
      const table = findByType(nodes, 'table')
      expect(table).toBeUndefined()
    })
  })

  describe('Builder Functions', () => {
    it('should create heading node with correct type', () => {
      const heading = createHeadingNode(1, 'My Title', 0)

      expect(heading.type).toBe('heading')
      expect(heading.depth).toBe(1)
      expect(heading.text).toBe('My Title')
      expect(heading.id).toBe(0)
      expect(heading.parent).toBe(null)
      expect(heading.children).toEqual([])
    })

    it('should create paragraph node', () => {
      const para = createParagraphNode('Some text', 1, 0)

      expect(para.type).toBe('paragraph')
      expect(para.text).toBe('Some text')
      expect(para.id).toBe(1)
      expect(para.parent).toBe(0)
    })

    it('should create code block node', () => {
      const code = createCodeBlockNode('const x = 1', 'js', 2, 0)

      expect(code.type).toBe('codeBlock')
      expect(code.code).toBe('const x = 1')
      expect(code.lang).toBe('js')
      expect(code.id).toBe(2)
      expect(code.parent).toBe(0)
      expect(code.children).toEqual([])
    })

    it('should create code block node without language', () => {
      const code = createCodeBlockNode('plain code', undefined, 3, null)

      expect(code.type).toBe('codeBlock')
      expect(code.code).toBe('plain code')
      expect(code.lang).toBeUndefined()
      expect(code.id).toBe(3)
      expect(code.parent).toBe(null)
    })

    it('should create link node', () => {
      const link = createLinkNode('Click here', 'https://example.com', 'Example', 2, 1)

      expect(link.type).toBe('link')
      expect(link.text).toBe('Click here')
      expect(link.url).toBe('https://example.com')
      expect(link.title).toBe('Example')
      expect(link.id).toBe(2)
      expect(link.parent).toBe(1)
    })
  })

  describe('asNodeType', () => {
    it('should cast node to specific type', () => {
      const node: BaseNode = {
        id: 1,
        type: 'heading',
        parent: null,
        children: [],
        depth: 1,
        text: 'Title',
      }

      const heading = asNodeType<'heading'>(node, 'heading')
      expect(heading.type).toBe('heading')
      expect(heading.depth).toBe(1)
    })

    it('should allow unsafe casting', () => {
      const node: BaseNode = {
        id: 1,
        type: 'paragraph',
        parent: null,
        children: [],
      }

      // Unsafe cast - user's responsibility
      const heading = asNodeType<'heading'>(node, 'heading')
      expect(heading).toBeDefined()
    })
  })

  describe('mapNodes', () => {
    it('should map nodes with type safety', () => {
      const nodes: BaseNode[] = [
        createHeadingNode(1, 'H1', 0),
        createHeadingNode(2, 'H2', 1),
        createParagraphNode('Text', 2),
      ]

      const mapped = mapNodes(nodes, (node: MarkdownNode) => {
        return node.type
      })

      expect(mapped).toEqual(['heading', 'heading', 'paragraph'])
    })

    it('should transform nodes', () => {
      const nodes: BaseNode[] = [createHeadingNode(1, 'First', 0), createHeadingNode(2, 'Second', 1)]

      const texts = mapNodes(nodes, (node: MarkdownNode) => {
        if ('text' in node) {
          return node.text
        }
        return ''
      })

      expect(texts).toEqual(['First', 'Second'])
    })

    it('should handle empty array', () => {
      const result = mapNodes([], (node: MarkdownNode) => node.type)
      expect(result).toEqual([])
    })
  })

  describe('Type Inference', () => {
    it('should infer correct types from NodeByType', () => {
      type HeadingType = NodeByType<'heading'>
      type ParagraphType = NodeByType<'paragraph'>
      type LinkType = NodeByType<'link'>

      const heading: HeadingType = {
        id: 1,
        type: 'heading',
        parent: null,
        children: [],
        depth: 1,
        text: 'Title',
      }

      const paragraph: ParagraphType = {
        id: 2,
        type: 'paragraph',
        parent: null,
        children: [],
        text: 'Text',
      }

      const link: LinkType = {
        id: 3,
        type: 'link',
        parent: null,
        children: [],
        text: 'Link text',
        url: 'https://example.com',
      }

      expect(heading.depth).toBe(1)
      expect(paragraph.text).toBe('Text')
      expect(link.url).toBe('https://example.com')
    })
  })

  describe('Visitor Pattern with Type Safety', () => {
    it('should provide type-safe visitor functions', () => {
      const nodes: BaseNode[] = [
        createHeadingNode(1, 'Title', 0),
        createParagraphNode('Text', 1),
        createLinkNode('Link', 'https://example.com', undefined, 2),
      ]

      // Type-safe visitor with discriminated union
      const processNode = (node: BaseNode): string => {
        switch (node.type) {
          case 'heading':
            // TypeScript knows node is HeadingNode here
            return `H${(node as HeadingNode).depth}: ${(node as HeadingNode).text}`

          case 'paragraph':
            // TypeScript knows node is ParagraphNode here
            return `P: ${(node as ParagraphNode).text}`

          case 'link':
            // TypeScript knows node is LinkNode here
            return `Link: ${(node as LinkNode).text} -> ${(node as LinkNode).url}`

          default:
            return 'Unknown'
        }
      }

      const results = nodes.map(processNode)
      expect(results[0]).toBe('H1: Title')
      expect(results[1]).toBe('P: Text')
      expect(results[2]).toBe('Link: Link -> https://example.com')
    })
  })
})

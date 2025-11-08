/**
 * Incremental Markdown Parser
 *
 * Parses Markdown tokens into Synth AST with support for incremental re-parsing.
 * Integrates with node pool and query index for maximum performance.
 */

import type { Tree, NodeId, BaseNode } from '../../types/index.js'
import { createTree, addNode } from '../../types/tree.js'
import type { Edit } from '../../core/incremental.js'
import { createIndex, type ASTIndex } from '../../core/query-index.js'
import { globalNodePool } from '../../core/node-pool.js'
import { IncrementalTokenizer } from './tokenizer.js'
import type { BlockToken } from './tokens.js'

/**
 * Incremental Markdown Parser
 *
 * Provides complete Markdown parsing with incremental re-parsing support.
 */
export class IncrementalMarkdownParser {
  private tokenizer: IncrementalTokenizer
  private tree: Tree | null = null
  private tokens: BlockToken[] = []
  private index: ASTIndex | null = null
  private source: string = ''

  constructor() {
    this.tokenizer = new IncrementalTokenizer()
  }

  /**
   * Full parse of Markdown text
   */
  parse(text: string): Tree {
    this.source = text

    // 1. Tokenize
    this.tokens = this.tokenizer.tokenize(text)

    // 2. Build AST
    this.tree = this.buildTree(this.tokens, text)

    // 3. Build index
    this.index = createIndex(this.tree)
    this.index.build()

    return this.tree
  }

  /**
   * Incremental parse after edit
   *
   * Strategy:
   * 1. Incremental tokenize
   * 2. Find affected nodes
   * 3. Re-parse affected region
   * 4. Structural sharing (reuse unchanged nodes)
   * 5. Rebuild index
   */
  parseIncremental(text: string, edit: Edit): Tree {
    if (!this.tree || !this.index) {
      throw new Error('Must call parse() before parseIncremental()')
    }

    this.source = text

    // 1. Incremental tokenize
    this.tokens = this.tokenizer.retokenize(text, edit, this.tokens)

    // 2. Find affected nodes using index
    const affectedNodeIds = this.findAffectedNodes(edit)

    // 3. Release affected nodes to pool
    for (const nodeId of affectedNodeIds) {
      const node = this.tree.nodes[nodeId]
      if (node) {
        globalNodePool.release(node)
      }
    }

    // 4. Re-build entire tree (for now - will optimize later)
    this.tree = this.buildTree(this.tokens, text)

    // 5. Rebuild index
    this.index = createIndex(this.tree)
    this.index.build()

    return this.tree
  }

  /**
   * Build AST from tokens
   */
  private buildTree(tokens: BlockToken[], source: string): Tree {
    const tree = createTree('markdown', source)

    // Convert each token to a node
    for (const token of tokens) {
      const nodeId = this.buildNode(tree, token, tree.root)
      if (nodeId !== null) {
        tree.nodes[tree.root]!.children.push(nodeId)
      }
    }

    return tree
  }

  /**
   * Build a single node from token
   */
  private buildNode(tree: Tree, token: BlockToken, parent: NodeId): NodeId | null {
    // Skip blank lines (don't create nodes for them)
    if (token.type === 'blankLine') {
      return null
    }

    switch (token.type) {
      case 'heading': {
        const headingId = addNode(tree, {
          type: 'heading',
          parent,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: { depth: token.depth },
        } as BaseNode)

        // Add text child
        const textId = addNode(tree, {
          type: 'text',
          parent: headingId,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: { value: token.text },
        } as BaseNode)

        tree.nodes[headingId]!.children.push(textId)
        return headingId
      }

      case 'paragraph': {
        const paragraphId = addNode(tree, {
          type: 'paragraph',
          parent,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
        } as BaseNode)

        // Add text child
        const textId = addNode(tree, {
          type: 'text',
          parent: paragraphId,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: { value: token.text },
        } as BaseNode)

        tree.nodes[paragraphId]!.children.push(textId)
        return paragraphId
      }

      case 'codeBlock': {
        return addNode(tree, {
          type: 'code',
          parent,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: {
            value: token.code,
            lang: token.lang,
            meta: token.meta,
          },
        } as BaseNode)
      }

      case 'listItem': {
        const listItemId = addNode(tree, {
          type: 'listItem',
          parent,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: {
            checked: token.checked,
          },
        } as BaseNode)

        // Add text child
        const textId = addNode(tree, {
          type: 'text',
          parent: listItemId,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: { value: token.text },
        } as BaseNode)

        tree.nodes[listItemId]!.children.push(textId)
        return listItemId
      }

      case 'blockquote': {
        const blockquoteId = addNode(tree, {
          type: 'blockquote',
          parent,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
        } as BaseNode)

        // Add text child
        const textId = addNode(tree, {
          type: 'text',
          parent: blockquoteId,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
          data: { value: token.text },
        } as BaseNode)

        tree.nodes[blockquoteId]!.children.push(textId)
        return blockquoteId
      }

      case 'horizontalRule': {
        return addNode(tree, {
          type: 'thematicBreak',
          parent,
          children: [],
          span: {
            start: token.position.start,
            end: token.position.end,
          },
        } as BaseNode)
      }

      default: {
        console.warn(`Unknown token type: ${(token as any).type}`)
        return null
      }
    }
  }

  /**
   * Find affected nodes based on edit
   */
  private findAffectedNodes(edit: Edit): NodeId[] {
    if (!this.index) return []

    const affected = new Set<NodeId>()

    // Find nodes that overlap with the edit range
    for (let i = 0; i < this.tree!.nodes.length; i++) {
      const node = this.tree!.nodes[i]
      if (!node || !node.span) continue

      const nodeStart = node.span.start.offset
      const nodeEnd = node.span.end.offset

      // Check if node overlaps with edit range
      if (
        (nodeStart <= edit.startByte && nodeEnd >= edit.startByte) ||
        (nodeStart <= edit.oldEndByte && nodeEnd >= edit.oldEndByte) ||
        (nodeStart >= edit.startByte && nodeEnd <= edit.oldEndByte)
      ) {
        affected.add(i)

        // Also mark parent as affected
        if (node.parent !== null) {
          affected.add(node.parent)
        }
      }
    }

    return Array.from(affected)
  }

  /**
   * Get current tree
   */
  getTree(): Tree {
    if (!this.tree) {
      throw new Error('No tree available. Call parse() first.')
    }
    return this.tree
  }

  /**
   * Get current tokens
   */
  getTokens(): BlockToken[] {
    return this.tokens
  }

  /**
   * Get index
   */
  getIndex(): ASTIndex | null {
    return this.index
  }
}

/**
 * Create an incremental Markdown parser
 */
export function createMarkdownParser(): IncrementalMarkdownParser {
  return new IncrementalMarkdownParser()
}

/**
 * Parse Markdown text (one-shot)
 */
export function parseMarkdown(text: string): Tree {
  const parser = createMarkdownParser()
  return parser.parse(text)
}

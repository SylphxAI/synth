/**
 * C Parser (WASM-based)
 *
 * Converts C source to Synth's universal AST
 * Uses web-tree-sitter (WASM) for cross-platform compatibility
 */

import type { NodeId, Plugin, Tree } from '@sylphx/synth'
import { addNode, createTree, SynthError } from '@sylphx/synth'
import Parser, { type Language, type SyntaxNode } from 'web-tree-sitter'
import type { CParseOptions } from './types.js'

// Singleton parser instance (reused across calls)
let parserPromise: Promise<Parser> | null = null

/**
 * Initialize the WASM parser (called automatically, cached)
 */
async function initParser(): Promise<Parser> {
  if (parserPromise) {
    return parserPromise
  }

  parserPromise = (async () => {
    await Parser.init()
    const parser = new Parser()

    // Load C language from tree-sitter-wasms
    const wasmPath = new URL(
      '../../node_modules/tree-sitter-wasms/out/tree-sitter-c.wasm',
      import.meta.url
    )

    // Try multiple paths for the WASM file
    let language: Language
    try {
      // Try direct path first (works in most bundlers)
      language = await Parser.Language.load(wasmPath.pathname)
    } catch {
      try {
        // Try relative path (works in Node.js)
        const { createRequire } = await import('node:module')
        const require = createRequire(import.meta.url)
        const wasmFile = require.resolve('tree-sitter-wasms/out/tree-sitter-c.wasm')
        language = await Parser.Language.load(wasmFile)
      } catch {
        // Fallback: try import from package directly
        const { default: cWasm } = await import('tree-sitter-wasms/out/tree-sitter-c.wasm')
        language = await Parser.Language.load(cWasm)
      }
    }

    parser.setLanguage(language)
    return parser
  })()

  return parserPromise
}

export class CParser {
  private plugins: Plugin[] = []
  private tree: Tree | null = null
  private parser: Parser | null = null

  /**
   * Register a plugin
   */
  use(plugin: Plugin): this {
    this.plugins.push(plugin)
    return this
  }

  /**
   * Parse C synchronously
   * @deprecated Use parseAsync() instead - WASM requires async initialization
   */
  parse(_source: string, _options: CParseOptions = {}): Tree {
    throw new SynthError(
      'Synchronous parse() is not supported with WASM. Use parseAsync() instead.',
      'SYNC_NOT_SUPPORTED'
    )
  }

  /**
   * Parse C asynchronously
   */
  async parseAsync(source: string, options: CParseOptions = {}): Promise<Tree> {
    // Initialize parser if needed
    if (!this.parser) {
      this.parser = await initParser()
    }

    const tree = createTree('c', source)
    this.tree = tree

    try {
      const tsTree = this.parser.parse(source)
      const rootNode = tsTree.rootNode

      this.convertNode(tree, rootNode, tree.root)
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`C parse error: ${error}`, 'PARSE_ERROR')
    }

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options.plugins || [])]
    let result = tree

    for (const plugin of allPlugins) {
      if ('transform' in plugin) {
        result = await plugin.transform(result)
      }
    }

    this.tree = result
    return result
  }

  /**
   * Get the last parsed tree
   */
  getTree(): Tree | null {
    return this.tree
  }

  private convertNode(tree: Tree, tsNode: SyntaxNode, parentId: NodeId): NodeId {
    // Create Synth node from tree-sitter node
    const nodeId = addNode(tree, {
      type: this.mapNodeType(tsNode.type),
      parent: parentId,
      children: [],
      span: {
        start: {
          offset: tsNode.startIndex,
          line: tsNode.startPosition.row + 1,
          column: tsNode.startPosition.column,
        },
        end: {
          offset: tsNode.endIndex,
          line: tsNode.endPosition.row + 1,
          column: tsNode.endPosition.column,
        },
      },
      data: {
        text: tsNode.text,
        isNamed: tsNode.isNamed,
        originalType: tsNode.type,
      },
    })

    // Add to parent's children
    tree.nodes[parentId]?.children.push(nodeId)

    // Recursively convert children
    for (let i = 0; i < tsNode.childCount; i++) {
      const child = tsNode.child(i)
      if (child) {
        this.convertNode(tree, child, nodeId)
      }
    }

    return nodeId
  }

  private mapNodeType(tsType: string): string {
    // Map tree-sitter node types to more readable names
    // Keep the tree-sitter types but make them PascalCase for consistency
    return tsType
      .split('_')
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join('')
  }
}

// Factory and standalone functions
export function createParser(): CParser {
  return new CParser()
}

/**
 * @deprecated Use parseAsync() instead - WASM requires async initialization
 */
export function parse(_source: string, _options?: CParseOptions): Tree {
  throw new SynthError(
    'Synchronous parse() is not supported with WASM. Use parseAsync() instead.',
    'SYNC_NOT_SUPPORTED'
  )
}

export async function parseAsync(source: string, options?: CParseOptions): Promise<Tree> {
  const parser = new CParser()
  return parser.parseAsync(source, options)
}

/**
 * Pre-initialize the parser (optional, for faster first parse)
 */
export async function init(): Promise<void> {
  await initParser()
}

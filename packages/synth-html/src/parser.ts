/**
 * HTML5 Parser
 *
 * Converts HTML into Synth AST using language-agnostic BaseNode
 */

import type { NodeId, Plugin, Tree } from '@sylphx/synth'
import { addNode, createTree } from '@sylphx/synth'
import { SynthError } from '@sylphx/synth'
import { type HTMLToken, HTMLTokenizer } from './tokenizer.js'
import { VOID_ELEMENTS } from './types.js'

export interface HTMLParseOptions {
  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]
}

export class HTMLParser {
  private tokenizer = new HTMLTokenizer()
  private plugins: Plugin[] = []
  private tree: Tree | null = null

  /**
   * Register a plugin
   */
  use(plugin: Plugin): this {
    this.plugins.push(plugin)
    return this
  }

  /**
   * Parse HTML synchronously
   */
  parse(html: string, options?: HTMLParseOptions): Tree {
    const tokens = this.tokenizer.tokenize(html)
    const tree = this.buildTree(tokens, html)

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options?.plugins || [])]

    // Check for async plugins
    const hasAsyncPlugin = allPlugins.some(
      (p) => 'transform' in p && p.transform.constructor.name === 'AsyncFunction'
    )

    if (hasAsyncPlugin) {
      throw new SynthError(
        'Detected async plugins. Use parseAsync() instead of parse()',
        'ASYNC_PLUGIN_IN_SYNC_PARSE'
      )
    }

    let result = tree
    for (const plugin of allPlugins) {
      if ('transform' in plugin) {
        result = plugin.transform(result) as Tree
      }
    }

    this.tree = result
    return result
  }

  /**
   * Parse HTML asynchronously
   */
  async parseAsync(html: string, options?: HTMLParseOptions): Promise<Tree> {
    const tokens = this.tokenizer.tokenize(html)
    const tree = this.buildTree(tokens, html)

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options?.plugins || [])]
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

  private buildTree(tokens: HTMLToken[], source: string): Tree {
    const tree = createTree('html', source)

    // Stack for tracking open elements
    interface StackItem {
      id: NodeId
      tagName: string
    }
    const stack: StackItem[] = []
    let currentParent = tree.root

    for (const token of tokens) {
      switch (token.type) {
        case 'doctype': {
          const id = addNode(tree, {
            type: 'doctype',
            parent: currentParent,
            children: [],
            data: {
              name: token.name,
              publicId: token.publicId,
              systemId: token.systemId,
            },
          })
          tree.nodes[currentParent]?.children.push(id)
          break
        }

        case 'startTag': {
          const id = addNode(tree, {
            type: 'element',
            parent: currentParent,
            children: [],
            data: {
              tagName: token.tagName,
              attributes: token.attributes,
              selfClosing: false,
              void: VOID_ELEMENTS.has(token.tagName),
            },
          })
          tree.nodes[currentParent]?.children.push(id)

          // Void elements don't have children
          if (!VOID_ELEMENTS.has(token.tagName)) {
            stack.push({ id, tagName: token.tagName })
            currentParent = id
          }
          break
        }

        case 'endTag': {
          // Pop from stack and restore parent
          const lastElement = stack[stack.length - 1]
          if (lastElement && lastElement.tagName === token.tagName) {
            stack.pop()
            currentParent =
              stack.length > 0 ? (stack[stack.length - 1]?.id ?? tree.root) : tree.root
          }
          break
        }

        case 'selfClosingTag': {
          const id = addNode(tree, {
            type: 'element',
            parent: currentParent,
            children: [],
            data: {
              tagName: token.tagName,
              attributes: token.attributes,
              selfClosing: true,
              void: VOID_ELEMENTS.has(token.tagName),
            },
          })
          tree.nodes[currentParent]?.children.push(id)
          break
        }

        case 'text': {
          const value = token.value.trim()
          if (value) {
            const id = addNode(tree, {
              type: 'text',
              parent: currentParent,
              children: [],
              data: { value },
            })
            tree.nodes[currentParent]?.children.push(id)
          }
          break
        }

        case 'comment': {
          const id = addNode(tree, {
            type: 'comment',
            parent: currentParent,
            children: [],
            data: { value: token.value },
          })
          tree.nodes[currentParent]?.children.push(id)
          break
        }

        case 'cdata': {
          const id = addNode(tree, {
            type: 'cdata',
            parent: currentParent,
            children: [],
            data: { value: token.value },
          })
          tree.nodes[currentParent]?.children.push(id)
          break
        }
      }
    }

    return tree
  }
}

// Factory and standalone functions
export function createParser(): HTMLParser {
  return new HTMLParser()
}

export function parse(html: string, options?: HTMLParseOptions): Tree {
  const parser = new HTMLParser()
  return parser.parse(html, options)
}

export async function parseAsync(html: string, options?: HTMLParseOptions): Promise<Tree> {
  const parser = new HTMLParser()
  return parser.parseAsync(html, options)
}

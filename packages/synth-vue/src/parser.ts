/**
 * Vue SFC Parser
 *
 * Converts Vue Single File Component source to Synth's universal AST
 * Uses @vue/compiler-sfc for parsing, then converts to Synth format
 */

import type { Tree, Plugin } from '@sylphx/synth'
import { createTree, addNode } from '@sylphx/synth'
import { SynthError } from '@sylphx/synth'
import { parse as parseVueSFC } from '@vue/compiler-sfc'
import type { NodeId } from '@sylphx/synth'

export interface VueParseOptions {
  /** Source filename (for better error messages) */
  filename?: string

  /** Source map support */
  sourceMap?: boolean

  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]
}

export class VueParser {
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
   * Parse Vue SFC synchronously
   */
  parse(source: string, options: VueParseOptions = {}): Tree {
    const tree = createTree('vue', source)
    this.tree = tree

    try {
      // Parse with @vue/compiler-sfc
      const { descriptor, errors } = parseVueSFC(source, {
        filename: options.filename || 'anonymous.vue',
        sourceMap: options.sourceMap ?? false,
      })

      if (errors.length > 0) {
        throw new SynthError(
          `Vue parse errors: ${errors.map((e) => e.message).join(', ')}`,
          'PARSE_ERROR'
        )
      }

      // Convert Vue descriptor to Synth AST
      this.convertDescriptor(tree, descriptor, tree.root, source)
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`Vue parse error: ${error}`, 'PARSE_ERROR')
    }

    // Apply plugins
    const allPlugins = [...this.plugins, ...(options.plugins || [])]

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
   * Parse Vue SFC asynchronously
   */
  async parseAsync(source: string, options: VueParseOptions = {}): Promise<Tree> {
    const tree = createTree('vue', source)
    this.tree = tree

    try {
      const { descriptor, errors } = parseVueSFC(source, {
        filename: options.filename || 'anonymous.vue',
        sourceMap: options.sourceMap ?? false,
      })

      if (errors.length > 0) {
        throw new SynthError(
          `Vue parse errors: ${errors.map((e) => e.message).join(', ')}`,
          'PARSE_ERROR'
        )
      }

      this.convertDescriptor(tree, descriptor, tree.root, source)
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`Vue parse error: ${error}`, 'PARSE_ERROR')
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

  private convertDescriptor(
    tree: Tree,
    descriptor: any,
    parentId: NodeId,
    source: string
  ): void {
    // Create SFC root node
    const sfcId = addNode(tree, {
      type: 'VueSFC',
      parent: parentId,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: source.length, line: source.split('\n').length, column: 0 },
      },
      data: {
        filename: descriptor.filename,
      },
    })

    tree.nodes[parentId]!.children.push(sfcId)

    // Parse template block
    if (descriptor.template) {
      this.convertBlock(tree, descriptor.template, sfcId, source, 'template')
    }

    // Parse script block
    if (descriptor.script) {
      this.convertBlock(tree, descriptor.script, sfcId, source, 'script')
    }

    // Parse script setup block
    if (descriptor.scriptSetup) {
      this.convertBlock(tree, descriptor.scriptSetup, sfcId, source, 'scriptSetup')
    }

    // Parse style blocks
    if (descriptor.styles && descriptor.styles.length > 0) {
      for (const style of descriptor.styles) {
        this.convertBlock(tree, style, sfcId, source, 'style')
      }
    }

    // Parse custom blocks
    if (descriptor.customBlocks && descriptor.customBlocks.length > 0) {
      for (const customBlock of descriptor.customBlocks) {
        this.convertBlock(tree, customBlock, sfcId, source, 'customBlock')
      }
    }
  }

  private convertBlock(
    tree: Tree,
    block: any,
    parentId: NodeId,
    source: string,
    blockType: string
  ): void {
    const loc = block.loc
    const content = block.content || ''

    const blockId = addNode(tree, {
      type: this.getBlockType(blockType, block),
      parent: parentId,
      children: [],
      span: {
        start: {
          offset: loc?.start.offset || 0,
          line: loc?.start.line || 1,
          column: loc?.start.column || 0,
        },
        end: {
          offset: loc?.end.offset || source.length,
          line: loc?.end.line || 1,
          column: loc?.end.column || 0,
        },
      },
      data: {
        content,
        lang: block.lang || this.getDefaultLang(blockType),
        scoped: block.scoped || false,
        module: block.module || false,
        attrs: block.attrs || {},
        blockType,
      },
    })

    tree.nodes[parentId]!.children.push(blockId)

    // Add content as text node
    if (content) {
      const textId = addNode(tree, {
        type: 'Text',
        parent: blockId,
        children: [],
        span: {
          start: {
            offset: loc?.start.offset || 0,
            line: loc?.start.line || 1,
            column: loc?.start.column || 0,
          },
          end: {
            offset: loc?.end.offset || source.length,
            line: loc?.end.line || 1,
            column: loc?.end.column || 0,
          },
        },
        data: {
          text: content,
        },
      })

      tree.nodes[blockId]!.children.push(textId)
    }
  }

  private getBlockType(blockType: string, block: any): string {
    switch (blockType) {
      case 'template':
        return 'VueTemplate'
      case 'script':
        return 'VueScript'
      case 'scriptSetup':
        return 'VueScriptSetup'
      case 'style':
        return 'VueStyle'
      case 'customBlock':
        return `VueCustomBlock_${block.type || 'unknown'}`
      default:
        return 'VueBlock'
    }
  }

  private getDefaultLang(blockType: string): string {
    switch (blockType) {
      case 'template':
        return 'html'
      case 'script':
      case 'scriptSetup':
        return 'js'
      case 'style':
        return 'css'
      default:
        return 'text'
    }
  }
}

// Factory and standalone functions
export function createParser(): VueParser {
  return new VueParser()
}

export function parse(source: string, options?: VueParseOptions): Tree {
  const parser = new VueParser()
  return parser.parse(source, options)
}

export async function parseAsync(
  source: string,
  options?: VueParseOptions
): Promise<Tree> {
  const parser = new VueParser()
  return parser.parseAsync(source, options)
}

/**
 * @sylphx/synth-md-gfm
 *
 * GitHub Flavored Markdown (GFM) extensions for Synth
 *
 * Features:
 * - Tables with alignment
 * - Strikethrough (~~text~~)
 * - Autolinks (URLs and email)
 * - Task lists ([x] and [ ])
 */

import { createTransformPlugin } from '@sylphx/synth'

// Re-export GFM tokenizer utilities
export * from './gfm-tokenizer.js'

export interface GFMPluginOptions {
  /**
   * Enable tables (default: true)
   */
  tables?: boolean

  /**
   * Enable strikethrough (default: true)
   */
  strikethrough?: boolean

  /**
   * Enable autolinks (default: true)
   */
  autolinks?: boolean

  /**
   * Enable task lists (default: true)
   */
  taskLists?: boolean
}

/**
 * GFM plugin for Synth Markdown
 *
 * Adds GitHub Flavored Markdown extensions to the parser
 *
 * @example
 * ```typescript
 * import { UltraOptimizedMarkdownParser } from '@sylphx/synth-md'
 * import { gfmPlugin } from '@sylphx/synth-md-gfm'
 *
 * const parser = new UltraOptimizedMarkdownParser()
 * const tree = parser.parse(markdown, {
 *   plugins: [gfmPlugin()]
 * })
 * ```
 */
export function gfmPlugin(options: GFMPluginOptions = {}) {
  const {
    tables = true,
    strikethrough = true,
    autolinks = true,
    taskLists = true,
  } = options

  return createTransformPlugin(
    {
      name: 'gfm',
      version: '0.1.0',
      description: 'GitHub Flavored Markdown extensions (tables, strikethrough, autolinks, task lists)'
    },
    (tree) => {
      // GFM transformations would be applied here
      // For now, this is a placeholder that marks the tree as GFM-enabled

      if (!tree.meta?.data) {
        tree.meta = {
          ...tree.meta,
          data: {
            ...tree.meta?.data,
            gfm: {
              enabled: true,
              features: {
                tables,
                strikethrough,
                autolinks,
                taskLists
              }
            }
          }
        }
      } else {
        tree.meta.data.gfm = {
          enabled: true,
          features: {
            tables,
            strikethrough,
            autolinks,
            taskLists
          }
        }
      }

      return tree
    }
  )
}

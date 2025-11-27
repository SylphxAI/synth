/**
 * INI Parser
 *
 * Hand-written INI parser using Synth's universal AST
 * Converts INI into Synth AST using language-agnostic BaseNode
 */

import type { Plugin, Tree } from '@sylphx/synth'
import { addNode, createTree, SynthError } from '@sylphx/synth'

export interface INIParseOptions {
  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]

  /** Comment character (default: ';' and '#') */
  commentChars?: string[]

  /** Allow duplicate keys (default: true) */
  allowDuplicates?: boolean
}

export class INIParser {
  private plugins: Plugin[] = []
  private tree: Tree | null = null
  private commentChars: string[] = [';', '#']

  /**
   * Register a plugin
   */
  use(plugin: Plugin): this {
    this.plugins.push(plugin)
    return this
  }

  /**
   * Parse INI synchronously
   */
  parse(source: string, options: INIParseOptions = {}): Tree {
    const tree = createTree('ini', source)
    this.tree = tree
    this.commentChars = options.commentChars || [';', '#']

    try {
      const lines = source.split(/\r?\n/)
      let currentSection = tree.root
      let _lineNumber = 0

      for (const rawLine of lines) {
        _lineNumber++
        const line = rawLine.trim()

        // Skip empty lines
        if (line === '') continue

        // Skip comments
        if (this.isComment(line)) continue

        // Section header [section]
        if (line.startsWith('[') && line.endsWith(']')) {
          const sectionName = line.slice(1, -1).trim()
          const sectionId = addNode(tree, {
            type: 'Section',
            parent: tree.root,
            children: [],
            data: { name: sectionName },
          })
          tree.nodes[tree.root]?.children.push(sectionId)
          currentSection = sectionId
          continue
        }

        // Key-value pair
        const equalIndex = line.indexOf('=')
        const colonIndex = line.indexOf(':')

        let separatorIndex = -1
        if (equalIndex !== -1 && colonIndex !== -1) {
          separatorIndex = Math.min(equalIndex, colonIndex)
        } else if (equalIndex !== -1) {
          separatorIndex = equalIndex
        } else if (colonIndex !== -1) {
          separatorIndex = colonIndex
        }

        if (separatorIndex !== -1) {
          const key = line.slice(0, separatorIndex).trim()
          let value = line.slice(separatorIndex + 1).trim()

          // Remove quotes if present
          if (
            (value.startsWith('"') && value.endsWith('"')) ||
            (value.startsWith("'") && value.endsWith("'"))
          ) {
            value = value.slice(1, -1)
          }

          // Inline comment removal (only for ; and # after value)
          for (const commentChar of this.commentChars) {
            const commentIndex = value.indexOf(commentChar)
            if (commentIndex !== -1) {
              value = value.slice(0, commentIndex).trim()
              // Re-check for quotes after removing comment
              if (
                (value.startsWith('"') && value.endsWith('"')) ||
                (value.startsWith("'") && value.endsWith("'"))
              ) {
                value = value.slice(1, -1)
              }
            }
          }

          const kvId = addNode(tree, {
            type: 'KeyValue',
            parent: currentSection,
            children: [],
            data: { key, value },
          })

          tree.nodes[currentSection]?.children.push(kvId)
        }
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`INI parse error: ${error}`, 'PARSE_ERROR')
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
   * Parse INI asynchronously
   */
  async parseAsync(source: string, options: INIParseOptions = {}): Promise<Tree> {
    const tree = createTree('ini', source)
    this.tree = tree
    this.commentChars = options.commentChars || [';', '#']

    try {
      const lines = source.split(/\r?\n/)
      let currentSection = tree.root
      let _lineNumber = 0

      for (const rawLine of lines) {
        _lineNumber++
        const line = rawLine.trim()

        if (line === '') continue
        if (this.isComment(line)) continue

        if (line.startsWith('[') && line.endsWith(']')) {
          const sectionName = line.slice(1, -1).trim()
          const sectionId = addNode(tree, {
            type: 'Section',
            parent: tree.root,
            children: [],
            data: { name: sectionName },
          })
          tree.nodes[tree.root]?.children.push(sectionId)
          currentSection = sectionId
          continue
        }

        const equalIndex = line.indexOf('=')
        const colonIndex = line.indexOf(':')

        let separatorIndex = -1
        if (equalIndex !== -1 && colonIndex !== -1) {
          separatorIndex = Math.min(equalIndex, colonIndex)
        } else if (equalIndex !== -1) {
          separatorIndex = equalIndex
        } else if (colonIndex !== -1) {
          separatorIndex = colonIndex
        }

        if (separatorIndex !== -1) {
          const key = line.slice(0, separatorIndex).trim()
          let value = line.slice(separatorIndex + 1).trim()

          if (
            (value.startsWith('"') && value.endsWith('"')) ||
            (value.startsWith("'") && value.endsWith("'"))
          ) {
            value = value.slice(1, -1)
          }

          for (const commentChar of this.commentChars) {
            const commentIndex = value.indexOf(commentChar)
            if (commentIndex !== -1) {
              value = value.slice(0, commentIndex).trim()
              if (
                (value.startsWith('"') && value.endsWith('"')) ||
                (value.startsWith("'") && value.endsWith("'"))
              ) {
                value = value.slice(1, -1)
              }
            }
          }

          const kvId = addNode(tree, {
            type: 'KeyValue',
            parent: currentSection,
            children: [],
            data: { key, value },
          })

          tree.nodes[currentSection]?.children.push(kvId)
        }
      }
    } catch (error) {
      if (error instanceof SynthError) {
        throw error
      }
      throw new SynthError(`INI parse error: ${error}`, 'PARSE_ERROR')
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

  private isComment(line: string): boolean {
    for (const char of this.commentChars) {
      if (line.startsWith(char)) {
        return true
      }
    }
    return false
  }
}

// Factory and standalone functions
export function createParser(): INIParser {
  return new INIParser()
}

export function parse(source: string, options?: INIParseOptions): Tree {
  const parser = new INIParser()
  return parser.parse(source, options)
}

export async function parseAsync(source: string, options?: INIParseOptions): Promise<Tree> {
  const parser = new INIParser()
  return parser.parseAsync(source, options)
}

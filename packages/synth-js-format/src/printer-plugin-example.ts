/**
 * Example: Extending Printer with custom node handlers
 *
 * This shows how to handle new JavaScript syntax without modifying core code
 */

import type { BaseNode, Tree } from '@sylphx/synth'
import { Printer } from './printer.js'

// Custom printer handler type
export type PrinterHandler = (
  tree: Tree,
  node: BaseNode,
  printer: Printer
) => string | null

// Plugin interface for printer extensions
export interface PrinterPlugin {
  name: string
  handlers: Record<string, PrinterHandler>
}

/**
 * Example: ES2025 Deferred Import plugin
 */
export const deferredImportPlugin: PrinterPlugin = {
  name: 'deferred-import',
  handlers: {
    ImportDeclaration: (tree, node, printer) => {
      // Handle new "import defer" syntax
      if (node.data?.defer) {
        return `import defer * as ${node.data.name} from "${node.data.source}";`
      }
      return null // Fall back to default handler
    },
  },
}

/**
 * Example: Pipeline operator plugin (TC39 Stage 2)
 */
export const pipelineOperatorPlugin: PrinterPlugin = {
  name: 'pipeline-operator',
  handlers: {
    PipelineExpression: (tree, node, printer) => {
      // value |> transform |> finalTransform
      const children = node.children.map(id => tree.nodes[id]!)
      const parts: string[] = []

      // Recursively print each part
      // (simplified - real implementation would use printer methods)
      return parts.join(' |> ')
    },
  },
}

/**
 * Example: Extensible Printer with plugin support
 */
export class ExtensiblePrinter extends Printer {
  private plugins: PrinterPlugin[] = []

  use(plugin: PrinterPlugin): this {
    this.plugins.push(plugin)
    return this
  }

  // Override printNode to check plugins first
  protected printNodeWithPlugins(tree: Tree, node: BaseNode): void {
    // Try plugins first
    for (const plugin of this.plugins) {
      const handler = plugin.handlers[node.type]
      if (handler) {
        const result = handler(tree, node, this)
        if (result !== null) {
          // Plugin handled it
          return
        }
      }
    }

    // Fall back to default implementation
    // super.printNode(tree, node)  // Would call parent method
  }
}

/**
 * Usage example:
 *
 * const printer = new ExtensiblePrinter()
 *   .use(deferredImportPlugin)
 *   .use(pipelineOperatorPlugin)
 *
 * const code = printer.print(tree)
 */

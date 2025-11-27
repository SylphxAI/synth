/**
 * WASM bindings wrapper
 */

import { initWasm } from './init.js'
import type { WasmTree } from './types.js'

// Re-export Tree and Position classes
export { Position, Tree } from '../wasm/synth_wasm_md.js'

// Import the raw parse function
import { parse as wasmParse, version as wasmVersion } from '../wasm/synth_wasm_md.js'

/**
 * Parse Markdown text into an AST
 *
 * @param markdown - Markdown source text
 * @returns Tree structure compatible with @sylphx/synth
 *
 * @example
 * ```typescript
 * import { parse } from '@sylphx/synth-wasm-md'
 *
 * const tree = await parse('# Hello World')
 * console.log(tree.toJSON())
 * ```
 */
export async function parse(markdown: string): Promise<WasmTree> {
  await initWasm()
  const tree = wasmParse(markdown)
  return tree.toJSON() as WasmTree
}

/**
 * Parse Markdown synchronously (requires WASM to be pre-initialized)
 *
 * @throws Error if WASM is not initialized
 *
 * @example
 * ```typescript
 * import { initWasm, parseSync } from '@sylphx/synth-wasm-md'
 *
 * await initWasm()
 * const tree = parseSync('# Hello World')
 * ```
 */
export function parseSync(markdown: string): WasmTree {
  const tree = wasmParse(markdown)
  return tree.toJSON() as WasmTree
}

/**
 * Get the version of the WASM parser
 */
export async function version(): Promise<string> {
  await initWasm()
  return wasmVersion()
}

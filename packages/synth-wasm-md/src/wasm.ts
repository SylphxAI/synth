/**
 * WASM bindings wrapper
 */

import { initWasm } from './init.js'
import type { WasmTree } from './types.js'

// Re-export Tree and Position classes
export { Position, Tree } from '../wasm/synth_wasm_md.js'

// Import the raw functions
import {
  parseBinary as wasmParseBinary,
  parseToJson as wasmParseToJson,
  version as wasmVersion,
} from '../wasm/synth_wasm_md.js'

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
 * console.log(tree.nodes)
 * ```
 */
export async function parse(markdown: string): Promise<WasmTree> {
  await initWasm()
  const json = wasmParseToJson(markdown)
  return JSON.parse(json) as WasmTree
}

/**
 * Parse Markdown to compact binary format (maximum performance, 10-18x faster)
 *
 * Use this when you need maximum speed and can decode the binary yourself.
 *
 * Binary format:
 * - Header: [node_count: u32, source_len: u32]
 * - Nodes: 24 bytes each (node_type, flags, parent, text_start, text_len, span_start, span_end)
 *
 * @example
 * ```typescript
 * import { parseBinary } from '@sylphx/synth-wasm-md'
 *
 * const buffer = await parseBinary('# Hello World')
 * const view = new DataView(buffer.buffer)
 * const nodeCount = view.getUint32(0, true)
 * ```
 */
export async function parseBinary(markdown: string): Promise<Uint8Array> {
  await initWasm()
  return wasmParseBinary(markdown)
}

/**
 * Get the version of the WASM parser
 */
export async function version(): Promise<string> {
  await initWasm()
  return wasmVersion()
}

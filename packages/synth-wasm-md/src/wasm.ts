/**
 * WASM bindings wrapper
 */

import { initWasm } from './init.js'
import type { WasmTree } from './types.js'

// Re-export Tree and Position classes
export { Position, Tree } from '../wasm/synth_wasm_md.js'

// Import the raw functions
import {
  parse as wasmParse,
  parseBinary as wasmParseBinary,
  parseCount as wasmParseCount,
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
 * console.log(tree)
 * ```
 */
export async function parse(markdown: string): Promise<WasmTree> {
  await initWasm()
  const json = wasmParseToJson(markdown)
  return JSON.parse(json) as WasmTree
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
  const json = wasmParseToJson(markdown)
  return JSON.parse(json) as WasmTree
}

/**
 * Parse Markdown to compact binary format (maximum performance)
 *
 * Returns a Uint8Array containing the binary tree structure.
 * This is the fastest parsing option - no JSON, no string copies.
 *
 * Binary format:
 * - Header: [node_count: u32, source_len: u32]
 * - Nodes: 24 bytes each
 *
 * @example
 * ```typescript
 * import { initWasm, parseBinary } from '@sylphx/synth-wasm-md'
 *
 * await initWasm()
 * const buffer = parseBinary('# Hello World')
 * const view = new DataView(buffer.buffer)
 * const nodeCount = view.getUint32(0, true)
 * ```
 */
export async function parseBinary(markdown: string): Promise<Uint8Array> {
  await initWasm()
  return wasmParseBinary(markdown)
}

/**
 * Parse Markdown to binary synchronously (requires WASM to be pre-initialized)
 */
export function parseBinarySync(markdown: string): Uint8Array {
  return wasmParseBinary(markdown)
}

/**
 * Count nodes in parsed markdown (for benchmarking)
 *
 * This measures pure parsing performance without any serialization overhead.
 */
export async function parseCount(markdown: string): Promise<number> {
  await initWasm()
  return wasmParseCount(markdown)
}

/**
 * Count nodes synchronously (requires WASM to be pre-initialized)
 */
export function parseCountSync(markdown: string): number {
  return wasmParseCount(markdown)
}

/**
 * Parse using the original Tree object (allows direct WASM object access)
 *
 * Note: For most use cases, parse() is preferred as it returns a plain JS object.
 */
export async function parseToTree(markdown: string) {
  await initWasm()
  return wasmParse(markdown)
}

/**
 * Get the version of the WASM parser
 */
export async function version(): Promise<string> {
  await initWasm()
  return wasmVersion()
}

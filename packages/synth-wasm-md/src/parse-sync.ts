/**
 * Synchronous parse path for Rust WASM authority routing.
 */

import { parseToJson } from '../wasm/synth_wasm_md.js'
import { initWasmSync } from './init-sync.js'
import type { WasmTree } from './types.js'

/**
 * Parse Markdown synchronously via Rust WASM (requires Node/Bun).
 *
 * @param markdown - Markdown source text
 * @returns AST tree compatible with @sylphx/synth
 */
export function parseSync(markdown: string): WasmTree {
  initWasmSync()
  const json = parseToJson(markdown)
  return JSON.parse(json) as WasmTree
}

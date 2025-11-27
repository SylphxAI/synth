/**
 * @sylphx/synth-wasm-md
 *
 * High-performance Markdown parser compiled to WebAssembly.
 *
 * Performance vs JavaScript:
 * - parseBinary(): 10-18x faster (returns Uint8Array)
 * - parse(): 2-4x faster (returns JSON object)
 */

// Export initialization helper
export { initWasm, isWasmInitialized } from './init.js'

// Re-export types
export type { ParseOptions, WasmNode, WasmTree } from './types.js'

// Re-export WASM bindings
export {
  parse,
  parseBinary,
  parseBinarySync,
  parseCount,
  parseCountSync,
  parseSync,
  parseToTree,
  Position,
  Tree,
  version,
} from './wasm.js'

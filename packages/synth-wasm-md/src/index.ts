/**
 * @sylphx/synth-wasm-md
 *
 * High-performance Markdown parser compiled to WebAssembly.
 *
 * Performance vs JavaScript:
 * - parse(): 2-4x faster (returns JSON object)
 * - parseBinary(): 10-18x faster (returns Uint8Array)
 */

// Initialization
export { initWasm, isWasmInitialized } from './init.js'

// Types
export type { ParseOptions, WasmNode, WasmTree } from './types.js'

// Parse functions
export { parse, parseBinary, Position, Tree, version } from './wasm.js'

/**
 * @module @sylphx/synth-wasm-md
 *
 * High-performance Markdown parser compiled to WebAssembly.
 *
 * Performance vs JavaScript:
 * - parse(): 2-4x faster (returns JSON object)
 * - parseBinary(): 10-18x faster (returns Uint8Array)
 *
 * @since 0.1.0
 * @packageDocumentation
 */

// Initialization
export { initWasm, isWasmInitialized } from './init.js'

// Types
export type { ParseOptions, WasmNode, WasmTree } from './types.js'

// Parse functions
export { Position, parse, parseBinary, Tree, version } from './wasm.js'

/**
 * @sylphx/synth-wasm-md
 *
 * High-performance Markdown parser compiled to WebAssembly.
 * Provides ~2-5x speedup over pure JavaScript implementation.
 */

// Export initialization helper
export { initWasm, isWasmInitialized } from './init.js'

// Re-export types
export type { ParseOptions, WasmTree } from './types.js'
// Re-export WASM bindings
export { Position, parse, Tree, version } from './wasm.js'

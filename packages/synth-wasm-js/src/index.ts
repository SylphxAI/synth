/**
 * @sylphx/synth-wasm-js
 *
 * High-performance JavaScript/TypeScript parser compiled to WebAssembly.
 * Supports ES2024 syntax.
 *
 * Performance:
 * - parse(): Returns structured AST with decoded nodes
 * - parseBinary(): Maximum performance (returns raw Uint8Array)
 * - parseCount(): Node count only (for benchmarking)
 */

// Initialization
export { initWasm, isWasmInitialized } from './init.js'

// Types
export type { ASTNode, ParseResult } from './types.js'
export { NodeFlags, NodeKind } from './types.js'

// Parse functions
export { parse, parseBinary, parseCount, tokenize, version } from './wasm.js'

/**
 * Synchronous parse path for Rust WASM authority routing.
 */

import { parseBinary } from '../wasm/synth_wasm_js.js'
import { initWasmSync } from './init-sync.js'
import { binaryToTree } from './tree-bridge.js'
import type { Tree } from '@sylphx/synth'

/**
 * Parse JavaScript synchronously via Rust WASM (requires Node/Bun).
 *
 * @param source - JavaScript/TypeScript source text
 * @returns AST tree compatible with @sylphx/synth
 */
export function parseSync(source: string): Tree {
	initWasmSync()
	const binary = parseBinary(source)
	return binaryToTree(source, binary)
}
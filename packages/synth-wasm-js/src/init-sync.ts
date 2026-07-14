/**
 * Synchronous WASM initialization for Node/Bun authority routing.
 */

import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { initSync } from '../wasm/synth_wasm_js.js'

let syncInitialized = false

/**
 * Initialize WASM synchronously (Node/Bun only).
 *
 * Used by @sylphx/synth-js default parse() authority routing so the sync API
 * can delegate to Rust without awaiting init.
 */
export function initWasmSync(): void {
	if (syncInitialized) {
		return
	}

	const here = dirname(fileURLToPath(import.meta.url))
	const wasmPath = join(here, '../wasm/synth_wasm_js_bg.wasm')
	const bytes = readFileSync(wasmPath)
	initSync({ module: bytes })
	syncInitialized = true
}

/**
 * Whether synchronous WASM init has completed.
 */
export function isWasmSyncInitialized(): boolean {
	return syncInitialized
}
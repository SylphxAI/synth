/**
 * Rust WASM authority routing for default @sylphx/synth-js consumers.
 *
 * Default parse() / parseAsync() delegate to @sylphx/synth-wasm-js when options
 * are baseline (no plugins). Advanced TS paths remain for plugin/incremental
 * slices until ts_deleted. Baseline consumers require explicit opt-in
 * (useTsParser or SYNTH_JS_AUTHORITY=ts) to use TS.
 */

import type { Tree } from '@sylphx/synth'
import {
	initWasm,
	parse as parseWasmAsync,
	parseSync as parseWasmSync,
} from '@sylphx/synth-wasm-js'
import type { JSParseOptions } from './parser.js'

function wasmAuthorityEnabled(): boolean {
	const override = typeof process !== 'undefined' ? process.env?.SYNTH_JS_AUTHORITY : undefined
	return override !== 'ts'
}

/**
 * Whether the given options qualify for Rust WASM authority routing.
 */
export function isWasmAuthorityEligible(options?: JSParseOptions): boolean {
	if (!wasmAuthorityEnabled()) {
		return false
	}

	if (!options) {
		return true
	}

	const plugins = options.plugins ?? []
	if (plugins.length > 0) {
		return false
	}

	if (options.useTsParser === true) {
		return false
	}

	return true
}

/**
 * Parse via Rust WASM authority (sync).
 */
export function parseViaWasmAuthority(code: string): Tree {
	return parseWasmSync(code) as Tree
}

/**
 * Parse via Rust WASM authority (async init for environments without sync loader).
 */
export async function parseViaWasmAuthorityAsync(code: string): Promise<Tree> {
	await initWasm()
	const tree = await parseWasmAsync(code)
	return tree as Tree
}
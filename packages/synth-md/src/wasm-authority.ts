/**
 * Rust WASM authority routing for default @sylphx/synth-md consumers.
 *
 * Default parse() / parseAsync() delegate to @sylphx/synth-wasm-md when options
 * are baseline (no plugins, no batch tokenizer). Advanced TS paths remain for
 * incremental/streaming/plugin slices until ts_deleted. Baseline consumers
 * require explicit opt-in (useTsParser or SYNTH_MD_AUTHORITY=ts) to use TS.
 */

import type { Tree } from '@sylphx/synth'
import {
  initWasm,
  parse as parseWasmAsync,
  parseSync as parseWasmSync,
} from '@sylphx/synth-wasm-md'
import type { ParseOptions } from './parser.js'

function wasmAuthorityEnabled(): boolean {
  const override = typeof process !== 'undefined' ? process.env?.SYNTH_MD_AUTHORITY : undefined
  return override !== 'ts'
}

/**
 * Whether the given options qualify for Rust WASM authority routing.
 */
export function isWasmAuthorityEligible(options?: ParseOptions): boolean {
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

  if (options.useBatchTokenizer === true) {
    return false
  }

  if (options.batchSize !== undefined && options.batchSize !== 16) {
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
export function parseViaWasmAuthority(markdown: string): Tree {
  return parseWasmSync(markdown) as Tree
}

/**
 * Parse via Rust WASM authority (async init for environments without sync loader).
 */
export async function parseViaWasmAuthorityAsync(markdown: string): Promise<Tree> {
  await initWasm()
  const tree = await parseWasmAsync(markdown)
  return tree as Tree
}

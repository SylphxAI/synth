/**
 * WASM initialization helpers
 */

let initialized = false
let initPromise: Promise<void> | null = null

/**
 * Initialize the WASM module
 *
 * This is called automatically when you first call parse(),
 * but you can call it manually for eager initialization.
 *
 * @example
 * ```typescript
 * import { initWasm } from '@sylphx/synth-wasm-js'
 *
 * // Eager initialization (optional)
 * await initWasm()
 *
 * // Now parsing is ready
 * const count = parseCount('const x = 1;')
 * ```
 */
export async function initWasm(): Promise<void> {
  if (initialized) return

  if (initPromise) {
    return initPromise
  }

  initPromise = (async () => {
    // Dynamic import to avoid bundler issues
    const wasm = await import('../wasm/synth_wasm_js.js')
    await wasm.default()
    initialized = true
  })()

  return initPromise
}

/**
 * Check if WASM is initialized
 */
export function isWasmInitialized(): boolean {
  return initialized
}

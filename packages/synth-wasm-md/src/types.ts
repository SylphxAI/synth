/**
 * Type definitions for synth-wasm-md
 */

/**
 * Parse options for the WASM parser
 */
export type ParseOptions = Record<string, never>

/**
 * WASM Tree structure (matches @sylphx/synth Tree)
 */
export interface WasmTree {
  meta: {
    language: string
    source: string
    created: number
    modified: number
  }
  root: number
  nodes: WasmNode[]
}

/**
 * WASM Node structure
 */
export interface WasmNode {
  id: number
  type: string
  parent: number | null
  children: number[]
  span?: {
    start: { line: number; column: number; offset: number }
    end: { line: number; column: number; offset: number }
  }
  data?: Record<string, unknown>
}

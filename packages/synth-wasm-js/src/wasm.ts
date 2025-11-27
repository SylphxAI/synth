/**
 * WASM bindings wrapper
 */

// Import the raw functions
import {
  parseBinary as wasmParseBinary,
  parseCount as wasmParseCount,
  tokenize as wasmTokenize,
  version as wasmVersion,
} from '../wasm/synth_wasm_js.js'
import { initWasm } from './init.js'
import type { ASTNode, NodeKind, ParseResult } from './types.js'

/**
 * Parse JavaScript and return the node count
 *
 * Useful for benchmarking and quick validation.
 *
 * @param source - JavaScript source code
 * @returns Number of AST nodes
 *
 * @example
 * ```typescript
 * import { parseCount } from '@sylphx/synth-wasm-js'
 *
 * const count = await parseCount('const x = 1;')
 * console.log(count) // 5
 * ```
 */
export async function parseCount(source: string): Promise<number> {
  await initWasm()
  return wasmParseCount(source)
}

/**
 * Parse JavaScript to compact binary format (maximum performance)
 *
 * Binary format:
 * - Header: [node_count: u32]
 * - Nodes: 16 bytes each (kind: u8, flags: u8, pad: u16, start: u32, end: u32, extra: u32)
 *
 * @param source - JavaScript source code
 * @returns Binary buffer containing AST
 *
 * @example
 * ```typescript
 * import { parseBinary } from '@sylphx/synth-wasm-js'
 *
 * const buffer = await parseBinary('const x = 1;')
 * const view = new DataView(buffer.buffer)
 * const nodeCount = view.getUint32(0, true)
 * ```
 */
export async function parseBinary(source: string): Promise<Uint8Array> {
  await initWasm()
  return wasmParseBinary(source)
}

/**
 * Parse JavaScript and decode the binary AST into a structured result
 *
 * @param source - JavaScript source code
 * @returns ParseResult with nodes array
 *
 * @example
 * ```typescript
 * import { parse } from '@sylphx/synth-wasm-js'
 *
 * const result = await parse('const x = 1;')
 * console.log(result.nodes[0].kind) // NodeKind.Program
 * ```
 */
export async function parse(source: string): Promise<ParseResult> {
  const binary = await parseBinary(source)
  const view = new DataView(binary.buffer)

  const nodeCount = view.getUint32(0, true)
  const nodes: ASTNode[] = []

  for (let i = 0; i < nodeCount; i++) {
    const offset = 4 + i * 16
    nodes.push({
      kind: view.getUint8(offset) as NodeKind,
      flags: view.getUint8(offset + 1),
      start: view.getUint32(offset + 4, true),
      end: view.getUint32(offset + 8, true),
      extra: view.getUint32(offset + 12, true),
    })
  }

  return { nodes, source }
}

/**
 * Tokenize JavaScript and return the token count
 *
 * Useful for benchmarking the lexer.
 *
 * @param source - JavaScript source code
 * @returns Number of tokens
 */
export async function tokenize(source: string): Promise<number> {
  await initWasm()
  return wasmTokenize(source)
}

/**
 * Get the version of the WASM parser
 */
export async function version(): Promise<string> {
  await initWasm()
  return wasmVersion()
}

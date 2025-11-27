/* tslint:disable */
/* eslint-disable */
/**
 * Parse Markdown text directly to JSON string
 *
 * This is faster than `parse().toJSON()` because it avoids
 * the intermediate JsValue conversion.
 *
 * # Example (JavaScript)
 * ```javascript
 * import { parseToJson } from '@sylphx/synth-wasm-md';
 *
 * const json = parseToJson('# Hello World');
 * const tree = JSON.parse(json);
 * ```
 */
export function parseToJson(markdown: string): string;
/**
 * Parse Markdown to compact binary format (maximum performance)
 *
 * Returns a Uint8Array containing the binary tree structure.
 * This is the fastest parsing option - no JSON, no string copies.
 *
 * Binary format:
 * - Header: [node_count: u32, source_len: u32]
 * - Nodes: 24 bytes each
 *   - node_type: u8 (1=heading, 2=para, 3=code, 4=hr, 5=quote, 6=list)
 *   - flags: u8 (depth for heading, ordered/checked for list)
 *   - _pad: [u8; 2]
 *   - parent: u32
 *   - text_start: u32
 *   - text_len: u32
 *   - span_start: u32
 *   - span_end: u32
 *
 * # Example (JavaScript)
 * ```javascript
 * import { parseBinary } from '@sylphx/synth-wasm-md';
 *
 * const buffer = parseBinary('# Hello World');
 * const view = new DataView(buffer.buffer);
 * const nodeCount = view.getUint32(0, true);
 * ```
 */
export function parseBinary(markdown: string): Uint8Array;
/**
 * Count nodes in parsed markdown (for benchmarking)
 *
 * This measures pure parsing performance without any serialization overhead.
 */
export function parseCount(markdown: string): number;
/**
 * Parse Markdown text into an AST Tree
 *
 * # Example (JavaScript)
 * ```javascript
 * import { parse } from '@sylphx/synth-wasm-md';
 *
 * const tree = parse('# Hello World');
 * console.log(tree.toJSON());
 * ```
 */
export function parse(markdown: string): Tree;
/**
 * Get the version of the Markdown parser
 */
export function version(): string;
/**
 * Get the version of the WASM core
 */
export function coreVersion(): string;
/**
 * Initialize the WASM module (called automatically)
 */
export function init(): void;
/**
 * A position in the source text
 */
export class Position {
  free(): void;
  [Symbol.dispose](): void;
  constructor(line: number, column: number, offset: number);
  /**
   * Line number (1-indexed)
   */
  line: number;
  /**
   * Column number (0-indexed)
   */
  column: number;
  /**
   * Byte offset from start of source
   */
  offset: number;
}
/**
 * AST Tree structure
 *
 * Uses arena-based storage for efficient memory layout:
 * - All nodes stored in a flat Vec
 * - Node IDs are array indices
 * - Cache-friendly iteration
 */
export class Tree {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create a new empty tree
   */
  constructor(language: string, source: string);
  /**
   * Serialize tree to JSON
   *
   * Note: Uses serde_json::to_string instead of serde_wasm_bindgen::to_value
   * because the latter doesn't correctly serialize HashMap<String, serde_json::Value>
   */
  toJSON(): any;
  /**
   * Deserialize tree from JSON
   */
  static fromJSON(json: any): Tree;
  /**
   * Get the number of nodes
   */
  readonly node_count: number;
  /**
   * Get the source
   */
  readonly source: string;
  /**
   * Get the root node ID
   */
  readonly root_id: number;
  /**
   * Get the language
   */
  readonly language: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly parse: (a: number, b: number) => [number, number, number];
  readonly parseBinary: (a: number, b: number) => [number, number];
  readonly parseCount: (a: number, b: number) => number;
  readonly parseToJson: (a: number, b: number) => [number, number, number, number];
  readonly version: () => [number, number];
  readonly coreVersion: () => [number, number];
  readonly init: () => void;
  readonly __wbg_tree_free: (a: number, b: number) => void;
  readonly tree_fromJSON: (a: any) => [number, number, number];
  readonly tree_language: (a: number) => [number, number];
  readonly tree_new: (a: number, b: number, c: number, d: number) => number;
  readonly tree_node_count: (a: number) => number;
  readonly tree_root_id: (a: number) => number;
  readonly tree_source: (a: number) => [number, number];
  readonly tree_toJSON: (a: number) => [number, number, number];
  readonly __wbg_get_position_column: (a: number) => number;
  readonly __wbg_get_position_line: (a: number) => number;
  readonly __wbg_get_position_offset: (a: number) => number;
  readonly __wbg_position_free: (a: number, b: number) => void;
  readonly __wbg_set_position_column: (a: number, b: number) => void;
  readonly __wbg_set_position_line: (a: number, b: number) => void;
  readonly __wbg_set_position_offset: (a: number, b: number) => void;
  readonly position_new: (a: number, b: number, c: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

/* tslint:disable */
/* eslint-disable */
/**
 * Get the version of the Markdown parser
 */
export function version(): string;
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
 * Fast parse and count nodes (for benchmarking)
 */
export function fastParseCount(markdown: string): number;
/**
 * Count nodes in the parsed markdown (for benchmarking without serialization)
 *
 * This measures pure parsing performance without JSON overhead.
 */
export function parseAndCount(markdown: string): number;
/**
 * TURBO parse - maximum performance, 16-byte nodes
 *
 * Returns a Uint8Array with compact binary format:
 * - Header: [node_count: u32]
 * - Nodes: 16 bytes each (node_type, depth, parent, text_start, text_len, span)
 */
export function turboParseBinary(markdown: string): Uint8Array;
/**
 * TURBO parse count (for benchmarking)
 */
export function turboParseCount(markdown: string): number;
/**
 * Fast tokenize-only (zero-copy, for benchmarking)
 *
 * Returns the number of tokens found. Uses SIMD-accelerated parsing.
 */
export function fastTokenize(markdown: string): number;
/**
 * Parse Markdown text into an AST
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
 * Ultra-fast parse to binary format
 *
 * Returns a Uint8Array containing the binary tree structure.
 * This is the fastest parsing option - no JSON, no string copies.
 *
 * Binary format (28 bytes per node):
 * - node_type: u8, depth: u8, flags: u8, _pad: u8
 * - parent: u32, first_child: u32, next_sibling: u32
 * - start_offset: u32, end_offset: u32
 * - text_start: u32, text_len: u32
 */
export function fastParseBinary(markdown: string): Uint8Array;
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
  readonly fastParseBinary: (a: number, b: number) => [number, number];
  readonly fastParseCount: (a: number, b: number) => number;
  readonly fastTokenize: (a: number, b: number) => number;
  readonly parse: (a: number, b: number) => [number, number, number];
  readonly parseAndCount: (a: number, b: number) => [number, number, number];
  readonly parseToJson: (a: number, b: number) => [number, number, number, number];
  readonly turboParseBinary: (a: number, b: number) => [number, number];
  readonly turboParseCount: (a: number, b: number) => number;
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
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
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

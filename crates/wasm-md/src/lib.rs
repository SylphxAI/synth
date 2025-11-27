//! Synth WASM Markdown Parser
//!
//! High-performance Markdown parser compiled to WebAssembly.
//! Compatible with @sylphx/synth-md TypeScript package.
//!
//! ## API
//!
//! - `parse(markdown)` → Returns Tree object (compatible with JS API)
//! - `parseBinary(markdown)` → Returns Uint8Array (maximum performance)
//! - `parseToJson(markdown)` → Returns JSON string
//! - `parseCount(markdown)` → Returns node count (for benchmarking)

mod parser_v2;

use parser_v2::MarkdownParserV2;
use synth_wasm_core::Tree;
use wasm_bindgen::prelude::*;

/// Parse Markdown text into an AST Tree
///
/// # Example (JavaScript)
/// ```javascript
/// import { parse } from '@sylphx/synth-wasm-md';
///
/// const tree = parse('# Hello World');
/// console.log(tree.toJSON());
/// ```
#[wasm_bindgen]
pub fn parse(markdown: &str) -> Result<Tree, JsValue> {
    let mut parser = MarkdownParserV2::new(markdown);
    parser
        .parse()
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse Markdown text directly to JSON string
///
/// This is faster than `parse().toJSON()` because it avoids
/// the intermediate JsValue conversion.
///
/// # Example (JavaScript)
/// ```javascript
/// import { parseToJson } from '@sylphx/synth-wasm-md';
///
/// const json = parseToJson('# Hello World');
/// const tree = JSON.parse(json);
/// ```
#[wasm_bindgen(js_name = parseToJson)]
pub fn parse_to_json(markdown: &str) -> Result<String, JsValue> {
    let mut parser = MarkdownParserV2::new(markdown);
    let tree = parser
        .parse()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&tree).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse Markdown to compact binary format (maximum performance)
///
/// Returns a Uint8Array containing the binary tree structure.
/// This is the fastest parsing option - no JSON, no string copies.
///
/// Binary format:
/// - Header: [node_count: u32, source_len: u32]
/// - Nodes: 24 bytes each
///   - node_type: u8 (1=heading, 2=para, 3=code, 4=hr, 5=quote, 6=list)
///   - flags: u8 (depth for heading, ordered/checked for list)
///   - _pad: [u8; 2]
///   - parent: u32
///   - text_start: u32
///   - text_len: u32
///   - span_start: u32
///   - span_end: u32
///
/// # Example (JavaScript)
/// ```javascript
/// import { parseBinary } from '@sylphx/synth-wasm-md';
///
/// const buffer = parseBinary('# Hello World');
/// const view = new DataView(buffer.buffer);
/// const nodeCount = view.getUint32(0, true);
/// ```
#[wasm_bindgen(js_name = parseBinary)]
pub fn parse_binary(markdown: &str) -> Vec<u8> {
    let mut parser = MarkdownParserV2::new(markdown);
    parser.parse_binary()
}

/// Count nodes in parsed markdown (for benchmarking)
///
/// This measures pure parsing performance without any serialization overhead.
#[wasm_bindgen(js_name = parseCount)]
pub fn parse_count(markdown: &str) -> usize {
    let mut parser = MarkdownParserV2::new(markdown);
    parser.parse_count()
}

/// Get the version of the Markdown parser
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

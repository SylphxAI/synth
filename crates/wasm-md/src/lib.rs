//! Synth WASM Markdown Parser
//!
//! High-performance Markdown parser compiled to WebAssembly.
//! Compatible with @sylphx/synth-md TypeScript package.

mod fast_parser;
mod fast_tokenizer;
mod parser;
mod tokenizer;

pub use parser::*;

use fast_parser::FastParser;
use fast_tokenizer::FastTokenizer;
use wasm_bindgen::prelude::*;
use synth_wasm_core::Tree;

/// Parse Markdown text into an AST
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
    let parser = MarkdownParser::new();
    parser.parse(markdown).map_err(|e| JsValue::from_str(&e.to_string()))
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
    let parser = MarkdownParser::new();
    let tree = parser.parse(markdown).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&tree).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Count nodes in the parsed markdown (for benchmarking without serialization)
///
/// This measures pure parsing performance without JSON overhead.
#[wasm_bindgen(js_name = parseAndCount)]
pub fn parse_and_count(markdown: &str) -> Result<usize, JsValue> {
    let parser = MarkdownParser::new();
    let tree = parser.parse(markdown).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(tree.node_count())
}

/// Fast tokenize-only (zero-copy, for benchmarking)
///
/// Returns the number of tokens found. Uses SIMD-accelerated parsing.
#[wasm_bindgen(js_name = fastTokenize)]
pub fn fast_tokenize(markdown: &str) -> usize {
    let mut tokenizer = FastTokenizer::new(markdown);
    let tokens = tokenizer.tokenize();
    tokens.len()
}

/// Ultra-fast parse to binary format
///
/// Returns a Uint8Array containing the binary tree structure.
/// This is the fastest parsing option - no JSON, no string copies.
///
/// Binary format (28 bytes per node):
/// - node_type: u8, depth: u8, flags: u8, _pad: u8
/// - parent: u32, first_child: u32, next_sibling: u32
/// - start_offset: u32, end_offset: u32
/// - text_start: u32, text_len: u32
#[wasm_bindgen(js_name = fastParseBinary)]
pub fn fast_parse_binary(markdown: &str) -> Vec<u8> {
    let mut parser = FastParser::new(markdown);
    parser.parse_to_binary()
}

/// Fast parse and count nodes (for benchmarking)
#[wasm_bindgen(js_name = fastParseCount)]
pub fn fast_parse_count(markdown: &str) -> usize {
    let mut parser = FastParser::new(markdown);
    parser.parse_count()
}

/// Get the version of the Markdown parser
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

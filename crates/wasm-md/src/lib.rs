//! Synth WASM Markdown Parser
//!
//! High-performance Markdown parser compiled to WebAssembly.
//! Compatible with @sylphx/synth-md TypeScript package.

mod parser;
mod tokenizer;

pub use parser::*;

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

/// Get the version of the Markdown parser
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

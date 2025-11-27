//! Synth WASM JavaScript/TypeScript Parser
//!
//! High-performance JS/TS parser compiled to WebAssembly.
//! Compatible with @sylphx/synth-js TypeScript package.
//!
//! Note: This is a placeholder. Full implementation will use tree-sitter-javascript.

use wasm_bindgen::prelude::*;
use synth_wasm_core::Tree;

/// Parse JavaScript/TypeScript source into an AST
///
/// # Example (JavaScript)
/// ```javascript
/// import { parse } from '@sylphx/synth-wasm-js';
///
/// const tree = parse('const x = 1;');
/// console.log(tree.toJSON());
/// ```
#[wasm_bindgen]
pub fn parse(source: &str) -> Result<Tree, JsValue> {
    // Placeholder implementation
    // TODO: Implement full JS/TS parser using tree-sitter-javascript
    let tree = Tree::new("javascript", source);
    Ok(tree)
}

/// Parse with options
#[wasm_bindgen]
pub fn parse_with_options(source: &str, _typescript: bool) -> Result<Tree, JsValue> {
    // Placeholder - options will be used when tree-sitter is integrated
    parse(source)
}

/// Get the version of the JavaScript parser
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

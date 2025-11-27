//! Synth WASM JavaScript/TypeScript Parser
//!
//! High-performance JS/TS parser compiled to WebAssembly.
//! Supports ES2024 syntax.

mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use wasm_bindgen::prelude::*;

/// Count tokens (for benchmarking)
#[wasm_bindgen(js_name = tokenize)]
pub fn tokenize(source: &str) -> usize {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    tokens.len()
}

/// Parse JavaScript and return node count (for benchmarking)
#[wasm_bindgen(js_name = parseCount)]
pub fn parse_count(source: &str) -> usize {
    let mut parser = Parser::new(source);
    parser.parse_count()
}

/// Parse JavaScript and return binary AST
#[wasm_bindgen(js_name = parseBinary)]
pub fn parse_binary(source: &str) -> Vec<u8> {
    let mut parser = Parser::new(source);
    parser.parse_binary()
}

/// Get the version of the JavaScript parser
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let count = tokenize("const x = 1;");
        assert!(count > 0);
    }

    #[test]
    fn test_parse_count() {
        let count = parse_count("const x = 1;");
        assert!(count >= 3); // Program, VariableDeclaration, VariableDeclarator, etc.
    }

    #[test]
    fn test_parse_binary() {
        let buf = parse_binary("function foo() { return 42; }");
        assert!(buf.len() > 4); // At least header + some nodes
        let node_count = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        assert!(node_count >= 4);
    }
}

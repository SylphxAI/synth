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
    use parser::NodeKind;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    fn golden_fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../test/fixtures/javascript-parity/golden.json")
    }

    fn node_kind_name(kind: NodeKind) -> &'static str {
        match kind {
            NodeKind::Program => "Program",
            NodeKind::VariableDeclaration => "VariableDeclaration",
            NodeKind::VariableDeclarator => "VariableDeclarator",
            NodeKind::FunctionDeclaration => "FunctionDeclaration",
            NodeKind::ClassDeclaration => "ClassDeclaration",
            NodeKind::ImportDeclaration => "ImportDeclaration",
            NodeKind::ExportDeclaration => "ExportDeclaration",
            NodeKind::BlockStatement => "BlockStatement",
            NodeKind::ExpressionStatement => "ExpressionStatement",
            NodeKind::IfStatement => "IfStatement",
            NodeKind::ForStatement => "ForStatement",
            NodeKind::ForInStatement => "ForInStatement",
            NodeKind::ForOfStatement => "ForOfStatement",
            NodeKind::WhileStatement => "WhileStatement",
            NodeKind::DoWhileStatement => "DoWhileStatement",
            NodeKind::SwitchStatement => "SwitchStatement",
            NodeKind::SwitchCase => "SwitchCase",
            NodeKind::ReturnStatement => "ReturnStatement",
            NodeKind::ThrowStatement => "ThrowStatement",
            NodeKind::TryStatement => "TryStatement",
            NodeKind::CatchClause => "CatchClause",
            NodeKind::BreakStatement => "BreakStatement",
            NodeKind::ContinueStatement => "ContinueStatement",
            NodeKind::EmptyStatement => "EmptyStatement",
            NodeKind::Identifier => "Identifier",
            NodeKind::Literal => "Literal",
            NodeKind::ArrayExpression => "ArrayExpression",
            NodeKind::ObjectExpression => "ObjectExpression",
            NodeKind::Property => "Property",
            NodeKind::FunctionExpression => "FunctionExpression",
            NodeKind::ArrowFunctionExpression => "ArrowFunctionExpression",
            NodeKind::ClassExpression => "ClassExpression",
            NodeKind::CallExpression => "CallExpression",
            NodeKind::NewExpression => "NewExpression",
            NodeKind::MemberExpression => "MemberExpression",
            NodeKind::BinaryExpression => "BinaryExpression",
            NodeKind::UnaryExpression => "UnaryExpression",
            NodeKind::UpdateExpression => "UpdateExpression",
            NodeKind::AssignmentExpression => "AssignmentExpression",
            NodeKind::LogicalExpression => "LogicalExpression",
            NodeKind::ConditionalExpression => "ConditionalExpression",
            NodeKind::SequenceExpression => "SequenceExpression",
            NodeKind::SpreadElement => "SpreadElement",
            NodeKind::TemplateLiteral => "TemplateLiteral",
            NodeKind::TaggedTemplateExpression => "TaggedTemplateExpression",
            NodeKind::ThisExpression => "ThisExpression",
            NodeKind::Super => "Super",
            NodeKind::AwaitExpression => "AwaitExpression",
            NodeKind::YieldExpression => "YieldExpression",
            NodeKind::ArrayPattern => "ArrayPattern",
            NodeKind::ObjectPattern => "ObjectPattern",
            NodeKind::AssignmentPattern => "AssignmentPattern",
            NodeKind::RestElement => "RestElement",
            NodeKind::ImportSpecifier => "ImportSpecifier",
            NodeKind::ImportDefaultSpecifier => "ImportDefaultSpecifier",
            NodeKind::ImportNamespaceSpecifier => "ImportNamespaceSpecifier",
            NodeKind::ExportSpecifier => "ExportSpecifier",
            NodeKind::MethodDefinition => "MethodDefinition",
            NodeKind::PropertyDefinition => "PropertyDefinition",
            NodeKind::Comment => "Comment",
            NodeKind::ClassBody => "ClassBody",
            NodeKind::TemplateElement => "TemplateElement",
        }
    }

    fn wasm_kind_counts(nodes: &[parser::Node]) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        for node in nodes {
            *counts
                .entry(node_kind_name(node.kind).to_string())
                .or_insert(0) += 1;
        }
        counts
    }

    fn normalize_wasm_counts_for_ts_parity(mut counts: HashMap<String, u32>) -> HashMap<String, u32> {
        let name_identifiers = counts.get("FunctionDeclaration").copied().unwrap_or(0)
            + counts.get("ClassDeclaration").copied().unwrap_or(0);
        if name_identifiers > 0
            && let Some(ids) = counts.get_mut("Identifier")
        {
            *ids = ids.saturating_sub(name_identifiers);
            if *ids == 0 {
                counts.remove("Identifier");
            }
        }
        counts
    }

    #[derive(serde::Deserialize)]
    struct GoldenFixture {
        source: String,
        #[serde(rename = "kindCounts")]
        kind_counts: HashMap<String, u32>,
    }

    #[test]
    fn golden_fixtures_match_ts_baseline() {
        let raw = fs::read_to_string(golden_fixture_path()).expect("golden.json must exist");
        let fixtures: HashMap<String, GoldenFixture> =
            serde_json::from_str(&raw).expect("golden.json must parse");

        assert!(!fixtures.is_empty(), "golden fixtures must not be empty");

        for (id, fixture) in &fixtures {
            let mut parser = Parser::new(&fixture.source);
            parser.parse_count();
            let got = normalize_wasm_counts_for_ts_parity(wasm_kind_counts(parser.nodes()));
            assert_eq!(
                got, fixture.kind_counts,
                "fixture {id}: Rust JS parser must match TS baseline kind histogram"
            );
        }
    }

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

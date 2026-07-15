//! Pure block-node type classifier — mirrors
//! `packages/synth-lint/src/rules/no-empty-blocks.ts#isBlockNode`.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full linter engine, NO authority_rust / ts_deleted.

/// Block/container type fragments (lowercase substring match; TS includes parity).
const BLOCK_TYPES: &[&str] = &[
    // Generic
    "block",
    "body",
    "blockstatement",
    // JavaScript/TypeScript
    "functionbody",
    "classbody",
    "objectexpression",
    "arrayexpression",
    // CSS
    "rule",
    // HTML/XML
    "element",
    // Markdown
    "blockquote",
    "list",
    // Go
    "blockstmt",
    // Rust
    "block_expr",
    // Python
    "suite",
];

/// True when node type represents a block/container (isBlockNode).
#[must_use]
pub fn is_block_node(node_type: &str) -> bool {
    let lower = node_type.to_ascii_lowercase();
    BLOCK_TYPES.iter().any(|t| lower.contains(t))
}

/// Whether an empty block should report (children empty + is block type).
#[must_use]
pub fn is_empty_block(child_count: usize, node_type: &str) -> bool {
    child_count == 0 && is_block_node(node_type)
}

/// Warning message parity with TS no-empty-blocks rule.
#[must_use]
pub fn empty_block_message(node_type: &str) -> String {
    format!("Empty {node_type} block")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_types_match_ts() {
        assert!(is_block_node("BlockStatement"));
        assert!(is_block_node("FunctionBody"));
        assert!(is_block_node("ClassBody"));
        assert!(is_block_node("ObjectExpression"));
        assert!(is_block_node("ArrayExpression"));
        assert!(is_block_node("Block"));
        assert!(is_block_node("Rule"));
        assert!(is_block_node("Element"));
        assert!(is_block_node("blockquote"));
        assert!(is_block_node("list"));
        assert!(is_block_node("BlockStmt"));
        assert!(is_block_node("block_expr"));
        assert!(is_block_node("suite"));
        assert!(!is_block_node("Identifier"));
        assert!(!is_block_node("Literal"));
        assert!(!is_block_node("CallExpression"));
    }

    #[test]
    fn empty_block_gate() {
        assert!(is_empty_block(0, "BlockStatement"));
        assert!(!is_empty_block(1, "BlockStatement"));
        assert!(!is_empty_block(0, "Identifier"));
        assert_eq!(empty_block_message("BlockStatement"), "Empty BlockStatement block");
    }
}

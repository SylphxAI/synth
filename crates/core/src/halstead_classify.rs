//! Pure Halstead operator/operand type classification —
//! mirrors `packages/synth-metrics/src/analyzer.ts` isOperatorNode / isOperandNode.
//! Residual pure deepen for tooling/metrics fragment.
//! AST traversal remains TS; this is the pure type-name kernel only.
//! NO authority_rust / ts_deleted.

/// Operator node type fragments (TS: node.type.includes(type)).
const OPERATOR_TYPES: &[&str] = &[
    "BinaryExpression",
    "UnaryExpression",
    "LogicalExpression",
    "AssignmentExpression",
    "UpdateExpression",
    "CallExpression",
    "MemberExpression",
];

/// Operand node type fragments (TS: node.type.includes(type)).
const OPERAND_TYPES: &[&str] = &[
    "Identifier",
    "Literal",
    "NumericLiteral",
    "StringLiteral",
    "BooleanLiteral",
];

/// True when node type is an operator for Halstead (isOperatorNode).
#[must_use]
pub fn is_operator_node_type(node_type: &str) -> bool {
    OPERATOR_TYPES.iter().any(|t| node_type.contains(t))
}

/// True when node type is an operand for Halstead (isOperandNode).
#[must_use]
pub fn is_operand_node_type(node_type: &str) -> bool {
    OPERAND_TYPES.iter().any(|t| node_type.contains(t))
}

/// Resolve operand value string (TS getNodeValue pure branches).
/// Prefers `value`, then `name`, else node type.
#[must_use]
pub fn operand_value(node_type: &str, value: Option<&str>, name: Option<&str>) -> String {
    if let Some(v) = value {
        return v.to_string();
    }
    if let Some(n) = name {
        return n.to_string();
    }
    node_type.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_types() {
        assert!(is_operator_node_type("BinaryExpression"));
        assert!(is_operator_node_type("UnaryExpression"));
        assert!(is_operator_node_type("LogicalExpression"));
        assert!(is_operator_node_type("AssignmentExpression"));
        assert!(is_operator_node_type("UpdateExpression"));
        assert!(is_operator_node_type("CallExpression"));
        assert!(is_operator_node_type("MemberExpression"));
        assert!(is_operator_node_type("TSCallExpression")); // includes
        assert!(!is_operator_node_type("Identifier"));
        assert!(!is_operator_node_type("Literal"));
        assert!(!is_operator_node_type("Program"));
    }

    #[test]
    fn operand_types() {
        assert!(is_operand_node_type("Identifier"));
        assert!(is_operand_node_type("Literal"));
        assert!(is_operand_node_type("NumericLiteral"));
        assert!(is_operand_node_type("StringLiteral"));
        assert!(is_operand_node_type("BooleanLiteral"));
        assert!(is_operand_node_type("TSStringLiteral")); // includes
        assert!(!is_operand_node_type("BinaryExpression"));
        assert!(!is_operand_node_type("Program"));
    }

    #[test]
    fn operand_value_prefers_value_then_name() {
        assert_eq!(operand_value("Identifier", Some("42"), Some("x")), "42");
        assert_eq!(operand_value("Identifier", None, Some("foo")), "foo");
        assert_eq!(operand_value("Identifier", None, None), "Identifier");
    }
}

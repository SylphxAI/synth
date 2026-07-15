//! Pure function-node type classification + depth aggregates —
//! mirrors `packages/synth-metrics/src/analyzer.ts` isFunctionNode / depth stats.
//! Residual pure deepen for tooling/metrics fragment.
//! AST traversal remains TS; this is the pure type-name + arithmetic kernel only.
//! NO authority_rust / ts_deleted.

/// Function-like type fragments (lowercase substring match; TS isFunctionNode).
const FUNCTION_TYPES: &[&str] = &[
    "functiondeclaration",
    "functionexpression",
    "arrowfunctionexpression",
    "methoddefinition",
    "function_definition",
    "function_declaration",
    "method_declaration",
    "func_decl",
    "funcdecl",
];

/// True when node type string is a function-like node (isFunctionNode).
#[must_use]
pub fn is_function_node_type(node_type: &str) -> bool {
    let lower = node_type.to_ascii_lowercase();
    FUNCTION_TYPES.iter().any(|t| lower.contains(t))
}

/// Max depth from a depths list (TS: Math.max(...depths, 0)).
#[must_use]
pub fn max_depth_of(depths: &[u32]) -> u32 {
    depths.iter().copied().max().unwrap_or(0)
}

/// Average depth (TS: reduce / length; empty → 0.0).
#[must_use]
pub fn avg_depth_of(depths: &[u32]) -> f64 {
    if depths.is_empty() {
        return 0.0;
    }
    let sum: u64 = depths.iter().map(|&d| u64::from(d)).sum();
    sum as f64 / depths.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_types_match_ts() {
        assert!(is_function_node_type("FunctionDeclaration"));
        assert!(is_function_node_type("FunctionExpression"));
        assert!(is_function_node_type("ArrowFunctionExpression"));
        assert!(is_function_node_type("MethodDefinition"));
        assert!(is_function_node_type("function_definition"));
        assert!(is_function_node_type("func_decl"));
        assert!(is_function_node_type("FuncDecl"));
        assert!(!is_function_node_type("VariableDeclaration"));
        assert!(!is_function_node_type("Identifier"));
        assert!(!is_function_node_type("ClassDeclaration"));
    }

    #[test]
    fn depth_aggregates() {
        assert_eq!(max_depth_of(&[]), 0);
        assert_eq!(max_depth_of(&[1, 3, 2]), 3);
        assert!((avg_depth_of(&[]) - 0.0).abs() < 1e-12);
        assert!((avg_depth_of(&[1, 2, 3]) - 2.0).abs() < 1e-12);
        assert!((avg_depth_of(&[0, 0, 4]) - 4.0 / 3.0).abs() < 1e-12);
    }
}

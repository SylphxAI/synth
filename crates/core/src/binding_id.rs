//! Pure binding-id / pattern type classification —
//! mirrors VariableDeclarator id selection in printer/compressor
//! (`type === 'Identifier' || type.includes('Pattern')`).
//! Residual pure deepen for tooling/format-minify-lint + metrics fragment.
//! NO full printer/analyzer engine, NO authority_rust / ts_deleted.

/// True when a node type can serve as a binding target (lhs of declarator).
/// TS: `n.type === 'Identifier' || n.type.includes('Pattern')`.
#[must_use]
pub fn is_binding_id_type(node_type: &str) -> bool {
    node_type == "Identifier" || node_type.contains("Pattern")
}

/// True when a node type is an object/array/assignment/rest pattern fragment.
/// Subset of `is_binding_id_type` excluding bare Identifier.
#[must_use]
pub fn is_pattern_type(node_type: &str) -> bool {
    node_type.contains("Pattern")
}

/// Select binding-id vs init among two optional child type tags (pure decision).
/// Returns `"id"` when the first child is a binding type, `"init"` when only the
/// second qualifies as non-id, else `"none"`.
/// Mirrors TS: find Identifier|Pattern as id; find sibling as init.
#[must_use]
pub fn classify_declarator_child(child_type: &str, is_first_binding: bool) -> &'static str {
    if is_binding_id_type(child_type) && is_first_binding {
        "id"
    } else if is_binding_id_type(child_type) {
        // later binding-like nodes are unusual; treat as init candidate only if
        // caller already claimed id — still report as binding for inspection
        "binding"
    } else {
        "init"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_types() {
        assert!(is_binding_id_type("Identifier"));
        assert!(is_binding_id_type("ObjectPattern"));
        assert!(is_binding_id_type("ArrayPattern"));
        assert!(is_binding_id_type("AssignmentPattern"));
        // RestElement does NOT contain "Pattern" and is not Identifier — TS includes() parity
        assert!(!is_binding_id_type("RestElement"));
        assert!(!is_binding_id_type("Literal"));
        assert!(!is_binding_id_type("CallExpression"));
    }

    #[test]
    fn pattern_subset() {
        assert!(is_pattern_type("ObjectPattern"));
        assert!(is_pattern_type("ArrayPattern"));
        assert!(!is_pattern_type("Identifier"));
        assert!(!is_pattern_type("Literal"));
    }

    #[test]
    fn classify_children() {
        assert_eq!(classify_declarator_child("Identifier", true), "id");
        assert_eq!(classify_declarator_child("ObjectPattern", true), "id");
        assert_eq!(classify_declarator_child("Literal", true), "init");
        assert_eq!(classify_declarator_child("CallExpression", false), "init");
        assert_eq!(classify_declarator_child("Identifier", false), "binding");
    }
}

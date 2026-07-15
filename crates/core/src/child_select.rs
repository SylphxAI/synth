//! Pure AST child-role selection helpers —
//! mirrors printer/compressor `getChildren` + `find`/`filter` branches
//! (`BlockStatement` body, `ClassBody`, `FunctionExpression` method value,
//! callee vs args, Program root).
//! Residual pure continue for tooling/format-minify-lint + metrics fragment.
//! NO full printer/compressor/analyzer engine, NO authority_rust / ts_deleted.

/// True when node type is a Program root (TS: `n.type === 'Program'`).
#[must_use]
pub fn is_program_type(node_type: &str) -> bool {
    node_type == "Program"
}

/// True when node type is a BlockStatement body.
#[must_use]
pub fn is_block_statement_type(node_type: &str) -> bool {
    node_type == "BlockStatement"
}

/// True when node type is a ClassBody container.
#[must_use]
pub fn is_class_body_type(node_type: &str) -> bool {
    node_type == "ClassBody"
}

/// True when node type is a FunctionExpression (method value).
#[must_use]
pub fn is_function_expression_type(node_type: &str) -> bool {
    node_type == "FunctionExpression"
}

/// True when node type is a bare Identifier (method key / name).
#[must_use]
pub fn is_identifier_type(node_type: &str) -> bool {
    node_type == "Identifier"
}

/// Index of the first child whose type matches `pred`, if any.
#[must_use]
pub fn find_first_index(child_types: &[&str], pred: impl Fn(&str) -> bool) -> Option<usize> {
    child_types.iter().position(|t| pred(t))
}

/// Index of the first `BlockStatement` among children (function body).
/// TS: `children.find((n) => n.type === 'BlockStatement')`.
#[must_use]
pub fn body_index(child_types: &[&str]) -> Option<usize> {
    find_first_index(child_types, is_block_statement_type)
}

/// Index of the first `ClassBody` among children.
#[must_use]
pub fn class_body_index(child_types: &[&str]) -> Option<usize> {
    find_first_index(child_types, is_class_body_type)
}

/// Index of the first `FunctionExpression` (method value).
#[must_use]
pub fn method_value_index(child_types: &[&str]) -> Option<usize> {
    find_first_index(child_types, is_function_expression_type)
}

/// Index of the first `Identifier` (method key / class name child).
#[must_use]
pub fn identifier_index(child_types: &[&str]) -> Option<usize> {
    find_first_index(child_types, is_identifier_type)
}

/// Arrow-function body index = last child when present (TS: `children[children.length - 1]`).
#[must_use]
pub fn arrow_body_index(child_count: usize) -> Option<usize> {
    if child_count == 0 {
        None
    } else {
        Some(child_count - 1)
    }
}

/// CallExpression: callee is child 0 when present; args start at 1.
#[must_use]
pub fn has_callee(child_count: usize) -> bool {
    child_count > 0
}

/// Number of call arguments given total children (callee + args).
#[must_use]
pub fn call_arg_count(child_count: usize) -> usize {
    child_count.saturating_sub(1)
}

/// BinaryExpression left/right presence (children[0], children[1]).
#[must_use]
pub fn binary_has_left(child_count: usize) -> bool {
    child_count > 0
}

/// True when a binary expression has a right operand child.
#[must_use]
pub fn binary_has_right(child_count: usize) -> bool {
    child_count > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_predicates() {
        assert!(is_program_type("Program"));
        assert!(!is_program_type("BlockStatement"));
        assert!(is_block_statement_type("BlockStatement"));
        assert!(is_class_body_type("ClassBody"));
        assert!(is_function_expression_type("FunctionExpression"));
        assert!(is_identifier_type("Identifier"));
        assert!(!is_identifier_type("Literal"));
    }

    #[test]
    fn find_and_body_indices() {
        let kids = ["Identifier", "BlockStatement", "Literal"];
        assert_eq!(body_index(&kids), Some(1));
        assert_eq!(identifier_index(&kids), Some(0));
        assert_eq!(class_body_index(&kids), None);
        assert_eq!(method_value_index(&["Identifier", "FunctionExpression"]), Some(1));
        assert_eq!(body_index(&["Identifier"]), None);
    }

    #[test]
    fn arrow_call_binary_splits() {
        assert_eq!(arrow_body_index(0), None);
        assert_eq!(arrow_body_index(1), Some(0));
        assert_eq!(arrow_body_index(3), Some(2));
        assert!(has_callee(2));
        assert!(!has_callee(0));
        assert_eq!(call_arg_count(0), 0);
        assert_eq!(call_arg_count(1), 0);
        assert_eq!(call_arg_count(3), 2);
        assert!(binary_has_left(1));
        assert!(!binary_has_right(1));
        assert!(binary_has_right(2));
    }
}

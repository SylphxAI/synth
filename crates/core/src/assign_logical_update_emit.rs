//! Pure Assignment / Logical / Update / This / Empty / Debugger emission —
//! residual pure continue15 for tooling/format-minify-lint fragment.
//! Mirrors printer/compressor spacing for expression forms not covered by
//! dedicated skeleton modules (unary_binary/conditional_spread cover tokens
//! only). Full engines remain product dens. NO authority_rust / ts_deleted.

use crate::unary_binary_emit::{
    binary_operator_token, is_assignment_operator, is_logical_operator, update_prefix_token,
};

/// Whether node type is AssignmentExpression.
#[must_use]
pub fn is_assignment_expression_type(t: &str) -> bool {
    t == "AssignmentExpression"
}

/// Whether node type is LogicalExpression.
#[must_use]
pub fn is_logical_expression_type(t: &str) -> bool {
    t == "LogicalExpression"
}

/// Whether node type is UpdateExpression (`++` / `--`).
#[must_use]
pub fn is_update_expression_type(t: &str) -> bool {
    t == "UpdateExpression"
}

/// Whether node type is ThisExpression.
#[must_use]
pub fn is_this_expression_type(t: &str) -> bool {
    t == "ThisExpression"
}

/// Whether node type is SequenceExpression.
#[must_use]
pub fn is_sequence_expression_type(t: &str) -> bool {
    t == "SequenceExpression"
}

/// Whether node type is EmptyStatement.
#[must_use]
pub fn is_empty_statement_type(t: &str) -> bool {
    t == "EmptyStatement"
}

/// Whether node type is DebuggerStatement.
#[must_use]
pub fn is_debugger_statement_type(t: &str) -> bool {
    t == "DebuggerStatement"
}

/// Whether node type is Super.
#[must_use]
pub fn is_super_type(t: &str) -> bool {
    t == "Super"
}

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_assign_logical_update_related_type(t: &str) -> bool {
    matches!(
        t,
        "AssignmentExpression"
            | "LogicalExpression"
            | "UpdateExpression"
            | "ThisExpression"
            | "SequenceExpression"
            | "EmptyStatement"
            | "DebuggerStatement"
            | "Super"
            | "MetaProperty"
    )
}

/// `this` keyword token.
#[must_use]
pub fn this_token() -> &'static str {
    "this"
}

/// `super` keyword token.
#[must_use]
pub fn super_token() -> &'static str {
    "super"
}

/// Empty statement emission (`;` always — pretty and minify).
#[must_use]
pub fn empty_statement_token() -> &'static str {
    ";"
}

/// Debugger statement: pretty → `debugger;` minify → `debugger;` (semi always).
#[must_use]
pub fn debugger_statement_token(semi: bool) -> &'static str {
    if semi {
        "debugger;"
    } else {
        "debugger"
    }
}

/// Assignment expression skeleton: `lhs` + op + `rhs` with pretty/minify spacing.
/// Operator must be an assignment operator (`=`, `+=`, …); falls back to `=`.
#[must_use]
pub fn assignment_expression_skeleton(lhs: &str, op: &str, rhs: &str, pretty: bool) -> String {
    let op_tok = if is_assignment_operator(op) {
        binary_operator_token(op, pretty)
    } else {
        binary_operator_token("=", pretty)
    };
    format!("{lhs}{op_tok}{rhs}")
}

/// Logical expression skeleton: `left` + `&&`/`||`/`??` + `right`.
/// Non-logical ops still emit with binary spacing for dual-oracle completeness.
#[must_use]
pub fn logical_expression_skeleton(left: &str, op: &str, right: &str, pretty: bool) -> String {
    let _ = is_logical_operator(op);
    let op_tok = binary_operator_token(op, pretty);
    format!("{left}{op_tok}{right}")
}

/// Update expression skeleton: prefix `++x` / postfix `x++`.
#[must_use]
pub fn update_expression_skeleton(operand: &str, op: &str, prefix: bool) -> String {
    let (before, after) = update_prefix_token(prefix, op);
    format!("{before}{operand}{after}")
}

/// Sequence expression: join expressions with comma sep.
#[must_use]
pub fn sequence_expression_skeleton(parts: &[&str], pretty: bool) -> String {
    let sep = if pretty { ", " } else { "," };
    parts.join(sep)
}

/// MetaProperty fragment (`new.target`, `import.meta`) — `meta` + `.` + `property`.
#[must_use]
pub fn meta_property_skeleton(meta: &str, property: &str) -> String {
    format!("{meta}.{property}")
}

/// Grouped parenthesized expression (pretty/minify identical for explicit parens).
/// TS engines rarely re-parenthesize; this models explicit ParenExpression residual.
#[must_use]
pub fn paren_group(inner: &str, _pretty: bool) -> String {
    format!("({inner})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_predicates() {
        assert!(is_assignment_expression_type("AssignmentExpression"));
        assert!(is_logical_expression_type("LogicalExpression"));
        assert!(is_update_expression_type("UpdateExpression"));
        assert!(is_this_expression_type("ThisExpression"));
        assert!(is_sequence_expression_type("SequenceExpression"));
        assert!(is_empty_statement_type("EmptyStatement"));
        assert!(is_debugger_statement_type("DebuggerStatement"));
        assert!(is_super_type("Super"));
        assert!(is_assign_logical_update_related_type("MetaProperty"));
        assert!(!is_assign_logical_update_related_type("Identifier"));
    }

    #[test]
    fn this_super_empty_debugger() {
        assert_eq!(this_token(), "this");
        assert_eq!(super_token(), "super");
        assert_eq!(empty_statement_token(), ";");
        assert_eq!(debugger_statement_token(true), "debugger;");
        assert_eq!(debugger_statement_token(false), "debugger");
    }

    #[test]
    fn assignment_pretty_minify() {
        assert_eq!(
            assignment_expression_skeleton("a", "=", "1", true),
            "a = 1"
        );
        assert_eq!(
            assignment_expression_skeleton("a", "=", "1", false),
            "a=1"
        );
        assert_eq!(
            assignment_expression_skeleton("x", "+=", "y", true),
            "x += y"
        );
        assert_eq!(
            assignment_expression_skeleton("x", "+=", "y", false),
            "x+=y"
        );
        // unknown op falls back to `=`
        assert_eq!(
            assignment_expression_skeleton("a", "??", "b", false),
            "a=b"
        );
    }

    #[test]
    fn logical_and_update() {
        assert_eq!(
            logical_expression_skeleton("a", "&&", "b", true),
            "a && b"
        );
        assert_eq!(
            logical_expression_skeleton("a", "||", "b", false),
            "a||b"
        );
        assert_eq!(
            logical_expression_skeleton("a", "??", "b", true),
            "a ?? b"
        );
        assert_eq!(update_expression_skeleton("i", "++", true), "++i");
        assert_eq!(update_expression_skeleton("i", "++", false), "i++");
        assert_eq!(update_expression_skeleton("n", "--", true), "--n");
        assert_eq!(update_expression_skeleton("n", "--", false), "n--");
    }

    #[test]
    fn sequence_meta_paren() {
        assert_eq!(
            sequence_expression_skeleton(&["a", "b", "c"], true),
            "a, b, c"
        );
        assert_eq!(
            sequence_expression_skeleton(&["a", "b", "c"], false),
            "a,b,c"
        );
        assert_eq!(meta_property_skeleton("new", "target"), "new.target");
        assert_eq!(meta_property_skeleton("import", "meta"), "import.meta");
        assert_eq!(paren_group("a + b", true), "(a + b)");
        assert_eq!(paren_group("a+b", false), "(a+b)");
    }
}

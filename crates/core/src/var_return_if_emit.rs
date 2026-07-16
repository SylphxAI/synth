//! Pure VariableDeclaration + VariableDeclarator + ExpressionStatement +
//! ReturnStatement + IfStatement dual-oracle emission — residual pure
//! **continue79** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules:
//! - VariableDeclaration dual-oracle pretty/minify declarator seps + semi policy
//!   driving real `variable_declaration_skeleton` (continue11)
//! - VariableDeclarator dual-oracle `id = init` / `id=init`
//! - ExpressionStatement dual-oracle semi policy
//! - ReturnStatement dual-oracle arg spacing + semi
//! - IfStatement dual-oracle pretty/minify open/close/else driving real
//!   `if_statement_skeleton`
//!
//! Intentionally does **not** re-wrap continue64–78 partition shells
//! (assignment/sequence continue78 stays separate; call/member continue77 stays
//! separate). Composes real shipped pure helpers from `var_stmt_emit`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::var_stmt_emit::{
    expression_statement_skeleton, if_statement_skeleton, return_statement_skeleton,
    variable_declaration_skeleton, variable_declarator_skeleton,
};

/// Dual-oracle residual: continue79 related AST type catalog.
pub const CONTINUE79_RELATED_TYPES: &[&str] = &[
    "VariableDeclaration",
    "VariableDeclarator",
    "ExpressionStatement",
    "ReturnStatement",
    "IfStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_var_return_if_related_type(t: &str) -> bool {
    CONTINUE79_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue79_variable_declaration_type(t: &str) -> bool {
    t == "VariableDeclaration"
}

#[must_use]
pub fn is_continue79_variable_declarator_type(t: &str) -> bool {
    t == "VariableDeclarator"
}

#[must_use]
pub fn is_continue79_expression_statement_type(t: &str) -> bool {
    t == "ExpressionStatement"
}

#[must_use]
pub fn is_continue79_return_type(t: &str) -> bool {
    t == "ReturnStatement"
}

#[must_use]
pub fn is_continue79_if_type(t: &str) -> bool {
    t == "IfStatement"
}

// ── VariableDeclarator dual-oracle ──────────────────────────────────────────

/// Dual-oracle VariableDeclarator skeleton: `id` / `id = init` / `id=init`.
///
/// Drives real shipped [`variable_declarator_skeleton`].
#[must_use]
pub fn continue79_variable_declarator_skeleton(
    id: &str,
    init: Option<&str>,
    pretty: bool,
) -> String {
    variable_declarator_skeleton(id, init, pretty)
}

/// Convenience: pretty declarator.
#[must_use]
pub fn variable_declarator_pretty(id: &str, init: Option<&str>) -> String {
    continue79_variable_declarator_skeleton(id, init, true)
}

/// Convenience: minify declarator.
#[must_use]
pub fn variable_declarator_minify(id: &str, init: Option<&str>) -> String {
    continue79_variable_declarator_skeleton(id, init, false)
}

// ── VariableDeclaration dual-oracle ─────────────────────────────────────────

/// Dual-oracle VariableDeclaration skeleton composing real
/// [`variable_declaration_skeleton`].
///
/// Pretty + semi: `const x = 1, y = 2;`; minify: `const x=1,y=2;`.
#[must_use]
pub fn continue79_variable_declaration_skeleton(
    kind: Option<&str>,
    declarators: &[&str],
    pretty: bool,
    semi: bool,
) -> String {
    variable_declaration_skeleton(kind, declarators, pretty, semi)
}

/// Convenience: pretty const declaration with semi.
#[must_use]
pub fn variable_declaration_pretty(kind: Option<&str>, declarators: &[&str]) -> String {
    continue79_variable_declaration_skeleton(kind, declarators, true, true)
}

/// Convenience: minify declaration (always semi).
#[must_use]
pub fn variable_declaration_minify(kind: Option<&str>, declarators: &[&str]) -> String {
    continue79_variable_declaration_skeleton(kind, declarators, false, false)
}

// ── ExpressionStatement dual-oracle ─────────────────────────────────────────

/// Dual-oracle ExpressionStatement: `expr;` / `expr` (semi policy).
///
/// Drives real shipped [`expression_statement_skeleton`].
#[must_use]
pub fn continue79_expression_statement_skeleton(
    expr: Option<&str>,
    pretty: bool,
    semi: bool,
) -> String {
    expression_statement_skeleton(expr, pretty, semi)
}

/// Convenience: pretty expression statement with semi.
#[must_use]
pub fn expression_statement_pretty(expr: &str) -> String {
    continue79_expression_statement_skeleton(Some(expr), true, true)
}

/// Convenience: minify expression statement (always semi).
#[must_use]
pub fn expression_statement_minify(expr: &str) -> String {
    continue79_expression_statement_skeleton(Some(expr), false, false)
}

// ── ReturnStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ReturnStatement: `return` / `return arg` + semi policy.
///
/// Drives real shipped [`return_statement_skeleton`].
#[must_use]
pub fn continue79_return_statement_skeleton(
    arg: Option<&str>,
    pretty: bool,
    semi: bool,
) -> String {
    return_statement_skeleton(arg, pretty, semi)
}

/// Convenience: pretty return with semi.
#[must_use]
pub fn return_statement_pretty(arg: Option<&str>) -> String {
    continue79_return_statement_skeleton(arg, true, true)
}

/// Convenience: minify return (always semi).
#[must_use]
pub fn return_statement_minify(arg: Option<&str>) -> String {
    continue79_return_statement_skeleton(arg, false, false)
}

// ── IfStatement dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle IfStatement skeleton composing real [`if_statement_skeleton`].
///
/// Pretty: `if (x) a() else b()`; minify: `if(x)a()else b()`.
#[must_use]
pub fn continue79_if_statement_skeleton(
    test: &str,
    consequent: &str,
    alternate: Option<&str>,
    pretty: bool,
) -> String {
    if_statement_skeleton(test, consequent, alternate, pretty)
}

/// Convenience: pretty if without else.
#[must_use]
pub fn if_statement_pretty(test: &str, consequent: &str) -> String {
    continue79_if_statement_skeleton(test, consequent, None, true)
}

/// Convenience: minify if without else.
#[must_use]
pub fn if_statement_minify(test: &str, consequent: &str) -> String {
    continue79_if_statement_skeleton(test, consequent, None, false)
}

/// Convenience: pretty if/else.
#[must_use]
pub fn if_else_statement_pretty(test: &str, consequent: &str, alternate: &str) -> String {
    continue79_if_statement_skeleton(test, consequent, Some(alternate), true)
}

/// Convenience: minify if/else.
#[must_use]
pub fn if_else_statement_minify(test: &str, consequent: &str, alternate: &str) -> String {
    continue79_if_statement_skeleton(test, consequent, Some(alternate), false)
}

/// Dual-oracle residual: compose return inside if consequent.
#[must_use]
pub fn continue79_if_return(
    test: &str,
    ret_arg: Option<&str>,
    pretty: bool,
    semi: bool,
) -> String {
    let body = continue79_return_statement_skeleton(ret_arg, pretty, semi);
    continue79_if_statement_skeleton(test, &body, None, pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::var_stmt_emit::{
        expression_statement_skeleton, if_statement_skeleton, return_statement_skeleton,
        variable_declaration_skeleton, variable_declarator_skeleton,
    };

    #[test]
    fn continue79_type_catalog() {
        assert_eq!(CONTINUE79_RELATED_TYPES.len(), 5);
        assert!(is_var_return_if_related_type("VariableDeclaration"));
        assert!(is_var_return_if_related_type("VariableDeclarator"));
        assert!(is_var_return_if_related_type("ExpressionStatement"));
        assert!(is_var_return_if_related_type("ReturnStatement"));
        assert!(is_var_return_if_related_type("IfStatement"));
        assert!(!is_var_return_if_related_type("AssignmentExpression"));
        assert!(!is_var_return_if_related_type("CallExpression"));
        assert!(!is_var_return_if_related_type("BinaryExpression"));
        assert!(is_continue79_variable_declaration_type("VariableDeclaration"));
        assert!(is_continue79_variable_declarator_type("VariableDeclarator"));
        assert!(is_continue79_expression_statement_type("ExpressionStatement"));
        assert!(is_continue79_return_type("ReturnStatement"));
        assert!(is_continue79_if_type("IfStatement"));
        assert!(!is_continue79_return_type("IfStatement"));
        assert!(!is_continue79_if_type("ReturnStatement"));
    }

    #[test]
    fn continue79_variable_declarator_dual_oracle() {
        assert_eq!(variable_declarator_pretty("x", Some("1")), "x = 1");
        assert_eq!(variable_declarator_minify("x", Some("1")), "x=1");
        assert_eq!(variable_declarator_pretty("x", None), "x");
        assert_eq!(variable_declarator_minify("x", None), "x");
        assert_eq!(
            continue79_variable_declarator_skeleton("a", Some("b"), true),
            variable_declarator_skeleton("a", Some("b"), true)
        );
        assert_eq!(
            continue79_variable_declarator_skeleton("a", Some("b"), false),
            variable_declarator_skeleton("a", Some("b"), false)
        );
        assert_ne!(
            variable_declarator_pretty("x", Some("1")),
            variable_declarator_minify("x", Some("1"))
        );
    }

    #[test]
    fn continue79_variable_declaration_dual_oracle() {
        let d1 = variable_declarator_pretty("x", Some("1"));
        let d2 = variable_declarator_pretty("y", Some("2"));
        assert_eq!(
            variable_declaration_pretty(Some("const"), &[d1.as_str(), d2.as_str()]),
            "const x = 1, y = 2;"
        );

        let m1 = variable_declarator_minify("x", Some("1"));
        let m2 = variable_declarator_minify("y", Some("2"));
        assert_eq!(
            variable_declaration_minify(Some("let"), &[m1.as_str(), m2.as_str()]),
            "let x=1,y=2;"
        );

        assert_eq!(
            continue79_variable_declaration_skeleton(None, &["a"], true, false),
            "const a"
        );
        assert_eq!(
            continue79_variable_declaration_skeleton(Some("var"), &["z"], false, false),
            "var z;"
        );
        // drives shipped continue11 skeleton
        assert_eq!(
            continue79_variable_declaration_skeleton(Some("const"), &["x = 1"], true, true),
            variable_declaration_skeleton(Some("const"), &["x = 1"], true, true)
        );
        assert_eq!(
            continue79_variable_declaration_skeleton(Some("const"), &["x=1"], false, false),
            variable_declaration_skeleton(Some("const"), &["x=1"], false, false)
        );
    }

    #[test]
    fn continue79_expression_and_return_dual_oracle() {
        assert_eq!(expression_statement_pretty("foo()"), "foo();");
        assert_eq!(expression_statement_minify("foo()"), "foo();");
        assert_eq!(
            continue79_expression_statement_skeleton(Some("foo()"), true, false),
            "foo()"
        );
        assert_eq!(
            continue79_expression_statement_skeleton(None, false, true),
            ";"
        );
        assert_eq!(
            continue79_expression_statement_skeleton(Some("x"), true, true),
            expression_statement_skeleton(Some("x"), true, true)
        );

        assert_eq!(return_statement_pretty(Some("x")), "return x;");
        assert_eq!(return_statement_pretty(None), "return;");
        assert_eq!(return_statement_minify(Some("x")), "return x;");
        assert_eq!(return_statement_minify(None), "return;");
        assert_eq!(
            continue79_return_statement_skeleton(None, true, false),
            "return"
        );
        assert_eq!(
            continue79_return_statement_skeleton(Some("x"), true, true),
            return_statement_skeleton(Some("x"), true, true)
        );
        assert_eq!(
            continue79_return_statement_skeleton(Some("x"), false, false),
            return_statement_skeleton(Some("x"), false, false)
        );
    }

    #[test]
    fn continue79_if_statement_dual_oracle() {
        assert_eq!(if_statement_pretty("x", "a()"), "if (x) a()");
        assert_eq!(if_statement_minify("x", "a()"), "if(x)a()");
        assert_eq!(
            if_else_statement_pretty("ok", "a()", "b()"),
            "if (ok) a() else b()"
        );
        assert_eq!(
            if_else_statement_minify("ok", "a()", "b()"),
            "if(ok)a()else b()"
        );
        assert_eq!(
            continue79_if_statement_skeleton("x", "{ return 1; }", None, true),
            if_statement_skeleton("x", "{ return 1; }", None, true)
        );
        assert_eq!(
            continue79_if_statement_skeleton("x", "{return 1;}", Some("{return 0;}"), false),
            if_statement_skeleton("x", "{return 1;}", Some("{return 0;}"), false)
        );
        // pretty ≠ minify spacing residual
        assert_ne!(
            if_statement_pretty("x", "a()"),
            if_statement_minify("x", "a()")
        );

        assert_eq!(
            continue79_if_return("ready", Some("1"), true, true),
            "if (ready) return 1;"
        );
        assert_eq!(
            continue79_if_return("ready", None, false, false),
            "if(ready)return;"
        );
    }
}

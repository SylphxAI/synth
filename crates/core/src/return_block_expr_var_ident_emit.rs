//! Pure ReturnStatement + BlockStatement + ExpressionStatement +
//! VariableDeclaration + VariableDeclarator + Identifier dual-oracle emission
//! — residual pure **continue107** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–106:
//! - ReturnStatement dual-oracle composing real `continue40_return_skeleton`
//! - BlockStatement dual-oracle composing real `continue40_block_skeleton`
//! - ExpressionStatement dual-oracle composing real
//!   `continue40_expression_statement_skeleton`
//! - VariableDeclaration dual-oracle composing real
//!   `continue40_variable_declaration_skeleton`
//! - Identifier dual-oracle composing real `continue40_identifier_skeleton`
//! - Composed return/block/expr/var/ident residual shells
//!
//! Intentionally does **not** re-wrap continue101 for/var/assignment/program
//! poles (continue34 bases), continue103 return via continue36, continue106
//! switch-case/break/continue/for-logical poles, or continue32–39 bases.
//! Composes real shipped pure helpers from continue40 bases. Full engines
//! remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue40_block_skeleton, continue40_expression_statement_skeleton,
    continue40_identifier_skeleton, continue40_return_skeleton,
    continue40_variable_declaration_skeleton,
};

/// Dual-oracle residual: continue107 related AST type catalog.
pub const CONTINUE107_RELATED_TYPES: &[&str] = &[
    "ReturnStatement",
    "BlockStatement",
    "ExpressionStatement",
    "VariableDeclaration",
    "VariableDeclarator",
    "Identifier",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_return_block_expr_var_ident_related_type(t: &str) -> bool {
    CONTINUE107_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue107_return_type(t: &str) -> bool {
    t == "ReturnStatement"
}

#[must_use]
pub fn is_continue107_block_type(t: &str) -> bool {
    t == "BlockStatement"
}

#[must_use]
pub fn is_continue107_expression_statement_type(t: &str) -> bool {
    t == "ExpressionStatement"
}

#[must_use]
pub fn is_continue107_variable_declaration_type(t: &str) -> bool {
    t == "VariableDeclaration"
}

#[must_use]
pub fn is_continue107_variable_declarator_type(t: &str) -> bool {
    t == "VariableDeclarator"
}

#[must_use]
pub fn is_continue107_identifier_type(t: &str) -> bool {
    t == "Identifier"
}

#[must_use]
pub fn is_continue107_stmt_type(t: &str) -> bool {
    matches!(
        t,
        "ReturnStatement" | "BlockStatement" | "ExpressionStatement" | "VariableDeclaration"
    )
}

#[must_use]
pub fn is_continue107_var_family_type(t: &str) -> bool {
    matches!(t, "VariableDeclaration" | "VariableDeclarator")
}

// ── ReturnStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ReturnStatement skeleton composing real
/// [`continue40_return_skeleton`].
#[must_use]
pub fn continue107_return_skeleton(arg: Option<&str>) -> String {
    continue40_return_skeleton(arg)
}

/// Dual-oracle ReturnStatement pretty alias.
#[must_use]
pub fn continue107_return_pretty(arg: Option<&str>) -> String {
    continue107_return_skeleton(arg)
}

/// Dual-oracle ReturnStatement minify alias.
#[must_use]
pub fn continue107_return_minify(arg: Option<&str>) -> String {
    continue107_return_skeleton(arg)
}

// ── BlockStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BlockStatement skeleton composing real
/// [`continue40_block_skeleton`].
#[must_use]
pub fn continue107_block_skeleton(body: &str) -> String {
    continue40_block_skeleton(body)
}

/// Dual-oracle BlockStatement pretty alias.
#[must_use]
pub fn continue107_block_pretty(body: &str) -> String {
    continue107_block_skeleton(body)
}

/// Dual-oracle BlockStatement minify alias.
#[must_use]
pub fn continue107_block_minify(body: &str) -> String {
    continue107_block_skeleton(body)
}

// ── ExpressionStatement dual-oracle ─────────────────────────────────────────

/// Dual-oracle ExpressionStatement skeleton composing real
/// [`continue40_expression_statement_skeleton`].
#[must_use]
pub fn continue107_expression_statement_skeleton(expr: &str) -> String {
    continue40_expression_statement_skeleton(expr)
}

/// Dual-oracle ExpressionStatement pretty alias.
#[must_use]
pub fn continue107_expression_statement_pretty(expr: &str) -> String {
    continue107_expression_statement_skeleton(expr)
}

/// Dual-oracle ExpressionStatement minify alias.
#[must_use]
pub fn continue107_expression_statement_minify(expr: &str) -> String {
    continue107_expression_statement_skeleton(expr)
}

// ── VariableDeclaration dual-oracle ─────────────────────────────────────────

/// Dual-oracle VariableDeclaration skeleton composing real
/// [`continue40_variable_declaration_skeleton`].
#[must_use]
pub fn continue107_variable_declaration_skeleton(
    kind: &str,
    id: &str,
    init: Option<&str>,
) -> String {
    continue40_variable_declaration_skeleton(kind, id, init)
}

/// Dual-oracle VariableDeclaration pretty alias.
#[must_use]
pub fn continue107_variable_declaration_pretty(
    kind: &str,
    id: &str,
    init: Option<&str>,
) -> String {
    continue107_variable_declaration_skeleton(kind, id, init)
}

/// Dual-oracle VariableDeclaration minify alias.
#[must_use]
pub fn continue107_variable_declaration_minify(
    kind: &str,
    id: &str,
    init: Option<&str>,
) -> String {
    continue107_variable_declaration_skeleton(kind, id, init)
}

// ── Identifier dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle Identifier skeleton composing real
/// [`continue40_identifier_skeleton`].
#[must_use]
pub fn continue107_identifier_skeleton(name: &str) -> String {
    continue40_identifier_skeleton(name)
}

/// Dual-oracle Identifier pretty alias.
#[must_use]
pub fn continue107_identifier_pretty(name: &str) -> String {
    continue107_identifier_skeleton(name)
}

/// Dual-oracle Identifier minify alias.
#[must_use]
pub fn continue107_identifier_minify(name: &str) -> String {
    continue107_identifier_skeleton(name)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: bare return.
#[must_use]
pub fn continue107_return_bare() -> String {
    continue107_return_skeleton(None)
}

/// Dual-oracle residual: return with argument.
#[must_use]
pub fn continue107_return_value(arg: &str) -> String {
    continue107_return_skeleton(Some(arg))
}

/// Dual-oracle residual: return of an identifier.
#[must_use]
pub fn continue107_return_ident(name: &str) -> String {
    let id = continue107_identifier_skeleton(name);
    continue107_return_value(&id)
}

/// Dual-oracle residual: block whose body is bare return.
#[must_use]
pub fn continue107_block_return_bare() -> String {
    let r = continue107_return_bare();
    continue107_block_skeleton(&r)
}

/// Dual-oracle residual: block whose body returns an identifier.
#[must_use]
pub fn continue107_block_return_ident(name: &str) -> String {
    let r = continue107_return_ident(name);
    continue107_block_skeleton(&r)
}

/// Dual-oracle residual: const declaration with init.
#[must_use]
pub fn continue107_const_init(id: &str, init: &str) -> String {
    continue107_variable_declaration_skeleton("const", id, Some(init))
}

/// Dual-oracle residual: let declaration without init.
#[must_use]
pub fn continue107_let_bare(id: &str) -> String {
    continue107_variable_declaration_skeleton("let", id, None)
}

/// Dual-oracle residual: expression statement of a call-shaped expr.
#[must_use]
pub fn continue107_expr_call(callee: &str) -> String {
    continue107_expression_statement_skeleton(&format!("{callee}()"))
}

/// Dual-oracle residual: block of const + return of that id.
#[must_use]
pub fn continue107_block_const_return(id: &str, init: &str) -> String {
    let decl = continue107_const_init(id, init);
    let ret = continue107_return_ident(id);
    let body = format!("{decl} {ret}");
    continue107_block_skeleton(&body)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue107_stmt_sep(pretty: bool) -> &'static str {
    if pretty {
        " "
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue40_block_skeleton, continue40_expression_statement_skeleton,
        continue40_identifier_skeleton, continue40_return_skeleton,
        continue40_variable_declaration_skeleton,
    };

    #[test]
    fn continue107_type_catalog() {
        assert_eq!(CONTINUE107_RELATED_TYPES.len(), 6);
        assert!(is_return_block_expr_var_ident_related_type(
            "ReturnStatement"
        ));
        assert!(is_return_block_expr_var_ident_related_type(
            "BlockStatement"
        ));
        assert!(is_return_block_expr_var_ident_related_type(
            "ExpressionStatement"
        ));
        assert!(is_return_block_expr_var_ident_related_type(
            "VariableDeclaration"
        ));
        assert!(is_return_block_expr_var_ident_related_type(
            "VariableDeclarator"
        ));
        assert!(is_return_block_expr_var_ident_related_type("Identifier"));
        assert!(!is_return_block_expr_var_ident_related_type("SwitchCase"));
        assert!(!is_return_block_expr_var_ident_related_type(
            "ForInStatement"
        ));

        assert!(is_continue107_return_type("ReturnStatement"));
        assert!(!is_continue107_return_type("BlockStatement"));
        assert!(is_continue107_block_type("BlockStatement"));
        assert!(is_continue107_expression_statement_type(
            "ExpressionStatement"
        ));
        assert!(is_continue107_variable_declaration_type(
            "VariableDeclaration"
        ));
        assert!(is_continue107_variable_declarator_type(
            "VariableDeclarator"
        ));
        assert!(is_continue107_identifier_type("Identifier"));
        assert!(is_continue107_stmt_type("ReturnStatement"));
        assert!(is_continue107_stmt_type("BlockStatement"));
        assert!(is_continue107_stmt_type("ExpressionStatement"));
        assert!(is_continue107_stmt_type("VariableDeclaration"));
        assert!(!is_continue107_stmt_type("Identifier"));
        assert!(is_continue107_var_family_type("VariableDeclaration"));
        assert!(is_continue107_var_family_type("VariableDeclarator"));
        assert!(!is_continue107_var_family_type("Identifier"));
    }

    #[test]
    fn continue107_return_block_expr_emit() {
        assert_eq!(continue107_return_skeleton(None), "return;");
        assert_eq!(continue107_return_skeleton(Some("1")), "return 1;");
        assert_eq!(
            continue107_return_skeleton(Some("1")),
            continue40_return_skeleton(Some("1"))
        );
        assert_eq!(
            continue107_return_pretty(None),
            continue107_return_minify(None)
        );

        assert_eq!(continue107_block_skeleton("x;"), "{ x; }");
        assert_eq!(
            continue107_block_skeleton("x;"),
            continue40_block_skeleton("x;")
        );
        assert_eq!(
            continue107_block_pretty("a;"),
            continue107_block_minify("a;")
        );

        assert_eq!(
            continue107_expression_statement_skeleton("foo()"),
            "foo();"
        );
        assert_eq!(
            continue107_expression_statement_skeleton("foo()"),
            continue40_expression_statement_skeleton("foo()")
        );
        assert_eq!(
            continue107_expression_statement_pretty("x"),
            continue107_expression_statement_minify("x")
        );
    }

    #[test]
    fn continue107_var_ident_emit() {
        assert_eq!(
            continue107_variable_declaration_skeleton("const", "x", Some("1")),
            "const x = 1;"
        );
        assert_eq!(
            continue107_variable_declaration_skeleton("let", "y", None),
            "let y;"
        );
        assert_eq!(
            continue107_variable_declaration_skeleton("const", "x", Some("1")),
            continue40_variable_declaration_skeleton("const", "x", Some("1"))
        );
        assert_eq!(
            continue107_variable_declaration_pretty("var", "z", Some("0")),
            continue107_variable_declaration_minify("var", "z", Some("0"))
        );

        assert_eq!(continue107_identifier_skeleton("foo"), "foo");
        assert_eq!(
            continue107_identifier_skeleton("foo"),
            continue40_identifier_skeleton("foo")
        );
        assert_eq!(
            continue107_identifier_pretty("bar"),
            continue107_identifier_minify("bar")
        );
    }

    #[test]
    fn continue107_composed_residual_shells() {
        assert_eq!(continue107_return_bare(), "return;");
        assert_eq!(continue107_return_value("1"), "return 1;");
        assert_eq!(continue107_return_ident("x"), "return x;");
        assert_eq!(continue107_block_return_bare(), "{ return; }");
        assert_eq!(continue107_block_return_ident("x"), "{ return x; }");
        assert_eq!(continue107_const_init("x", "1"), "const x = 1;");
        assert_eq!(continue107_let_bare("y"), "let y;");
        assert_eq!(continue107_expr_call("foo"), "foo();");
        assert_eq!(
            continue107_block_const_return("x", "1"),
            "{ const x = 1; return x; }"
        );
        assert_eq!(continue107_stmt_sep(true), " ");
        assert_eq!(continue107_stmt_sep(false), "");
        assert_ne!(continue107_stmt_sep(true), continue107_stmt_sep(false));
    }
}

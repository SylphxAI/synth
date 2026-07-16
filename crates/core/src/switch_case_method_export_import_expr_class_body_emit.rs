//! Pure SwitchCase + MethodDefinition + ExportSpecifier + ImportSpecifier +
//! ExpressionStatement + ClassBody dual-oracle emission — residual pure
//! **continue105** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–104:
//! - SwitchCase dual-oracle composing real `continue38_switch_case_skeleton`
//! - Switch default dual-oracle composing real
//!   `continue38_switch_default_skeleton`
//! - MethodDefinition dual-oracle composing real
//!   `continue38_method_definition_skeleton`
//! - ExportSpecifier dual-oracle composing real
//!   `continue38_export_specifier_skeleton`
//! - ImportSpecifier dual-oracle composing real
//!   `continue38_import_specifier_skeleton`
//! - ExpressionStatement dual-oracle composing real
//!   `continue38_expression_statement_skeleton`
//! - ClassBody dual-oracle composing real `continue38_class_body_skeleton`
//! - Composed switch-case/method/export/import/expr/class-body residual shells
//!
//! Intentionally does **not** re-wrap continue87 import/export specifier poles
//! as the sole surface, continue99 switch/try poles, continue103 class/return
//! poles, continue104 try/throw/debugger/empty/switch poles, or continue32–37
//! bases. Composes real shipped pure helpers from continue38 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue38_class_body_skeleton, continue38_export_specifier_skeleton,
    continue38_expression_statement_skeleton, continue38_import_specifier_skeleton,
    continue38_method_definition_skeleton, continue38_switch_case_skeleton,
    continue38_switch_default_skeleton,
};

/// Dual-oracle residual: continue105 related AST type catalog.
pub const CONTINUE105_RELATED_TYPES: &[&str] = &[
    "SwitchCase",
    "MethodDefinition",
    "ExportSpecifier",
    "ImportSpecifier",
    "ExpressionStatement",
    "ClassBody",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_switch_case_method_export_import_expr_class_body_related_type(t: &str) -> bool {
    CONTINUE105_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue105_switch_case_type(t: &str) -> bool {
    t == "SwitchCase"
}

#[must_use]
pub fn is_continue105_method_definition_type(t: &str) -> bool {
    t == "MethodDefinition"
}

#[must_use]
pub fn is_continue105_export_specifier_type(t: &str) -> bool {
    t == "ExportSpecifier"
}

#[must_use]
pub fn is_continue105_import_specifier_type(t: &str) -> bool {
    t == "ImportSpecifier"
}

#[must_use]
pub fn is_continue105_expression_statement_type(t: &str) -> bool {
    t == "ExpressionStatement"
}

#[must_use]
pub fn is_continue105_class_body_type(t: &str) -> bool {
    t == "ClassBody"
}

#[must_use]
pub fn is_continue105_specifier_type(t: &str) -> bool {
    matches!(t, "ExportSpecifier" | "ImportSpecifier")
}

#[must_use]
pub fn is_continue105_class_member_type(t: &str) -> bool {
    matches!(t, "MethodDefinition" | "ClassBody")
}

// ── SwitchCase dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle SwitchCase skeleton composing real
/// [`continue38_switch_case_skeleton`].
#[must_use]
pub fn continue105_switch_case_skeleton(test: &str, body: &str) -> String {
    continue38_switch_case_skeleton(test, body)
}

/// Dual-oracle SwitchCase pretty alias.
#[must_use]
pub fn continue105_switch_case_pretty(test: &str, body: &str) -> String {
    continue105_switch_case_skeleton(test, body)
}

/// Dual-oracle SwitchCase minify alias.
#[must_use]
pub fn continue105_switch_case_minify(test: &str, body: &str) -> String {
    continue105_switch_case_skeleton(test, body)
}

// ── Switch default dual-oracle ──────────────────────────────────────────────

/// Dual-oracle switch default skeleton composing real
/// [`continue38_switch_default_skeleton`].
#[must_use]
pub fn continue105_switch_default_skeleton(body: &str) -> String {
    continue38_switch_default_skeleton(body)
}

/// Dual-oracle switch default pretty alias.
#[must_use]
pub fn continue105_switch_default_pretty(body: &str) -> String {
    continue105_switch_default_skeleton(body)
}

/// Dual-oracle switch default minify alias.
#[must_use]
pub fn continue105_switch_default_minify(body: &str) -> String {
    continue105_switch_default_skeleton(body)
}

// ── MethodDefinition dual-oracle ────────────────────────────────────────────

/// Dual-oracle MethodDefinition skeleton composing real
/// [`continue38_method_definition_skeleton`].
#[must_use]
pub fn continue105_method_definition_skeleton(name: &str, params: &str, body: &str) -> String {
    continue38_method_definition_skeleton(name, params, body)
}

/// Dual-oracle MethodDefinition pretty alias.
#[must_use]
pub fn continue105_method_definition_pretty(name: &str, params: &str, body: &str) -> String {
    continue105_method_definition_skeleton(name, params, body)
}

/// Dual-oracle MethodDefinition minify alias.
#[must_use]
pub fn continue105_method_definition_minify(name: &str, params: &str, body: &str) -> String {
    continue105_method_definition_skeleton(name, params, body)
}

// ── ExportSpecifier dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ExportSpecifier skeleton composing real
/// [`continue38_export_specifier_skeleton`].
#[must_use]
pub fn continue105_export_specifier_skeleton(local: &str, exported: Option<&str>) -> String {
    continue38_export_specifier_skeleton(local, exported)
}

/// Dual-oracle ExportSpecifier pretty alias.
#[must_use]
pub fn continue105_export_specifier_pretty(local: &str, exported: Option<&str>) -> String {
    continue105_export_specifier_skeleton(local, exported)
}

/// Dual-oracle ExportSpecifier minify alias.
#[must_use]
pub fn continue105_export_specifier_minify(local: &str, exported: Option<&str>) -> String {
    continue105_export_specifier_skeleton(local, exported)
}

// ── ImportSpecifier dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ImportSpecifier skeleton composing real
/// [`continue38_import_specifier_skeleton`].
#[must_use]
pub fn continue105_import_specifier_skeleton(imported: &str, local: Option<&str>) -> String {
    continue38_import_specifier_skeleton(imported, local)
}

/// Dual-oracle ImportSpecifier pretty alias.
#[must_use]
pub fn continue105_import_specifier_pretty(imported: &str, local: Option<&str>) -> String {
    continue105_import_specifier_skeleton(imported, local)
}

/// Dual-oracle ImportSpecifier minify alias.
#[must_use]
pub fn continue105_import_specifier_minify(imported: &str, local: Option<&str>) -> String {
    continue105_import_specifier_skeleton(imported, local)
}

// ── ExpressionStatement dual-oracle ─────────────────────────────────────────

/// Dual-oracle ExpressionStatement skeleton composing real
/// [`continue38_expression_statement_skeleton`].
#[must_use]
pub fn continue105_expression_statement_skeleton(expr: &str) -> String {
    continue38_expression_statement_skeleton(expr)
}

/// Dual-oracle ExpressionStatement pretty alias.
#[must_use]
pub fn continue105_expression_statement_pretty(expr: &str) -> String {
    continue105_expression_statement_skeleton(expr)
}

/// Dual-oracle ExpressionStatement minify alias.
#[must_use]
pub fn continue105_expression_statement_minify(expr: &str) -> String {
    continue105_expression_statement_skeleton(expr)
}

// ── ClassBody dual-oracle ───────────────────────────────────────────────────

/// Dual-oracle ClassBody skeleton composing real
/// [`continue38_class_body_skeleton`].
#[must_use]
pub fn continue105_class_body_skeleton(members: &str) -> String {
    continue38_class_body_skeleton(members)
}

/// Dual-oracle ClassBody pretty alias.
#[must_use]
pub fn continue105_class_body_pretty(members: &str) -> String {
    continue105_class_body_skeleton(members)
}

/// Dual-oracle ClassBody minify alias.
#[must_use]
pub fn continue105_class_body_minify(members: &str) -> String {
    continue105_class_body_skeleton(members)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: switch case whose body is an expression statement.
#[must_use]
pub fn continue105_case_expr_stmt(test: &str, expr: &str) -> String {
    let stmt = continue105_expression_statement_skeleton(expr);
    continue105_switch_case_skeleton(test, &stmt)
}

/// Dual-oracle residual: switch default whose body is an expression statement.
#[must_use]
pub fn continue105_default_expr_stmt(expr: &str) -> String {
    let stmt = continue105_expression_statement_skeleton(expr);
    continue105_switch_default_skeleton(&stmt)
}

/// Dual-oracle residual: class body wrapping a single method definition.
#[must_use]
pub fn continue105_class_body_method(name: &str, params: &str, body: &str) -> String {
    let method = continue105_method_definition_skeleton(name, params, body);
    continue105_class_body_skeleton(&method)
}

/// Dual-oracle residual: bare export specifier (no rename).
#[must_use]
pub fn continue105_export_bare(local: &str) -> String {
    continue105_export_specifier_skeleton(local, None)
}

/// Dual-oracle residual: renamed export specifier (`local as exported`).
#[must_use]
pub fn continue105_export_as(local: &str, exported: &str) -> String {
    continue105_export_specifier_skeleton(local, Some(exported))
}

/// Dual-oracle residual: bare import specifier (no rename).
#[must_use]
pub fn continue105_import_bare(imported: &str) -> String {
    continue105_import_specifier_skeleton(imported, None)
}

/// Dual-oracle residual: renamed import specifier (`imported as local`).
#[must_use]
pub fn continue105_import_as(imported: &str, local: &str) -> String {
    continue105_import_specifier_skeleton(imported, Some(local))
}

/// Dual-oracle residual: method that is a single expression statement body.
#[must_use]
pub fn continue105_method_expr_body(name: &str, params: &str, expr: &str) -> String {
    let stmt = continue105_expression_statement_skeleton(expr);
    continue105_method_definition_skeleton(name, params, &stmt)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue105_stmt_sep(pretty: bool) -> &'static str {
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
        continue38_class_body_skeleton, continue38_export_specifier_skeleton,
        continue38_expression_statement_skeleton, continue38_import_specifier_skeleton,
        continue38_method_definition_skeleton, continue38_switch_case_skeleton,
        continue38_switch_default_skeleton,
    };

    #[test]
    fn continue105_type_catalog() {
        assert_eq!(CONTINUE105_RELATED_TYPES.len(), 6);
        assert!(is_switch_case_method_export_import_expr_class_body_related_type(
            "SwitchCase"
        ));
        assert!(is_switch_case_method_export_import_expr_class_body_related_type(
            "MethodDefinition"
        ));
        assert!(is_switch_case_method_export_import_expr_class_body_related_type(
            "ExportSpecifier"
        ));
        assert!(is_switch_case_method_export_import_expr_class_body_related_type(
            "ImportSpecifier"
        ));
        assert!(is_switch_case_method_export_import_expr_class_body_related_type(
            "ExpressionStatement"
        ));
        assert!(is_switch_case_method_export_import_expr_class_body_related_type(
            "ClassBody"
        ));
        assert!(!is_switch_case_method_export_import_expr_class_body_related_type(
            "TryStatement"
        ));
        assert!(!is_switch_case_method_export_import_expr_class_body_related_type(
            "SwitchStatement"
        ));

        assert!(is_continue105_switch_case_type("SwitchCase"));
        assert!(!is_continue105_switch_case_type("SwitchStatement"));
        assert!(is_continue105_method_definition_type("MethodDefinition"));
        assert!(is_continue105_export_specifier_type("ExportSpecifier"));
        assert!(is_continue105_import_specifier_type("ImportSpecifier"));
        assert!(is_continue105_expression_statement_type(
            "ExpressionStatement"
        ));
        assert!(is_continue105_class_body_type("ClassBody"));
        assert!(is_continue105_specifier_type("ExportSpecifier"));
        assert!(is_continue105_specifier_type("ImportSpecifier"));
        assert!(!is_continue105_specifier_type("MethodDefinition"));
        assert!(is_continue105_class_member_type("MethodDefinition"));
        assert!(is_continue105_class_member_type("ClassBody"));
        assert!(!is_continue105_class_member_type("SwitchCase"));
    }

    #[test]
    fn continue105_switch_case_method_emit() {
        assert_eq!(
            continue105_switch_case_skeleton("1", "break;"),
            "case 1: break;"
        );
        assert_eq!(
            continue105_switch_case_skeleton("1", "break;"),
            continue38_switch_case_skeleton("1", "break;")
        );
        assert_eq!(
            continue105_switch_case_pretty("x", "return;"),
            continue105_switch_case_minify("x", "return;")
        );

        assert_eq!(
            continue105_switch_default_skeleton("break;"),
            "default: break;"
        );
        assert_eq!(
            continue105_switch_default_skeleton("break;"),
            continue38_switch_default_skeleton("break;")
        );
        assert_eq!(
            continue105_switch_default_pretty("return;"),
            continue105_switch_default_minify("return;")
        );

        assert_eq!(
            continue105_method_definition_skeleton("run", "x", "return x;"),
            "run(x) { return x; }"
        );
        assert_eq!(
            continue105_method_definition_skeleton("run", "x", "return x;"),
            continue38_method_definition_skeleton("run", "x", "return x;")
        );
        assert_eq!(
            continue105_method_definition_pretty("f", "", "0;"),
            continue105_method_definition_minify("f", "", "0;")
        );
    }

    #[test]
    fn continue105_export_import_expr_class_body_emit() {
        assert_eq!(
            continue105_export_specifier_skeleton("foo", Some("bar")),
            "foo as bar"
        );
        assert_eq!(
            continue105_export_specifier_skeleton("foo", None),
            "foo"
        );
        assert_eq!(
            continue105_export_specifier_skeleton("foo", Some("bar")),
            continue38_export_specifier_skeleton("foo", Some("bar"))
        );
        assert_eq!(
            continue105_export_specifier_pretty("a", None),
            continue105_export_specifier_minify("a", None)
        );

        assert_eq!(
            continue105_import_specifier_skeleton("a", Some("b")),
            "a as b"
        );
        assert_eq!(continue105_import_specifier_skeleton("a", None), "a");
        assert_eq!(
            continue105_import_specifier_skeleton("a", Some("b")),
            continue38_import_specifier_skeleton("a", Some("b"))
        );
        assert_eq!(
            continue105_import_specifier_pretty("x", None),
            continue105_import_specifier_minify("x", None)
        );

        assert_eq!(
            continue105_expression_statement_skeleton("foo()"),
            "foo();"
        );
        assert_eq!(
            continue105_expression_statement_skeleton("foo()"),
            continue38_expression_statement_skeleton("foo()")
        );
        assert_eq!(
            continue105_expression_statement_pretty("x"),
            continue105_expression_statement_minify("x")
        );

        assert_eq!(
            continue105_class_body_skeleton("x() {}"),
            "{ x() {} }"
        );
        assert_eq!(
            continue105_class_body_skeleton("x() {}"),
            continue38_class_body_skeleton("x() {}")
        );
        assert_eq!(
            continue105_class_body_pretty("m() {}"),
            continue105_class_body_minify("m() {}")
        );
    }

    #[test]
    fn continue105_composed_residual_shells() {
        assert_eq!(
            continue105_case_expr_stmt("1", "doWork()"),
            "case 1: doWork();"
        );
        assert_eq!(
            continue105_default_expr_stmt("fallback()"),
            "default: fallback();"
        );
        assert_eq!(
            continue105_class_body_method("run", "x", "return x;"),
            "{ run(x) { return x; } }"
        );
        assert_eq!(continue105_export_bare("foo"), "foo");
        assert_eq!(continue105_export_as("foo", "bar"), "foo as bar");
        assert_eq!(continue105_import_bare("a"), "a");
        assert_eq!(continue105_import_as("a", "b"), "a as b");
        assert_eq!(
            continue105_method_expr_body("log", "msg", "console.log(msg)"),
            "log(msg) { console.log(msg); }"
        );
        assert_eq!(continue105_stmt_sep(true), " ");
        assert_eq!(continue105_stmt_sep(false), "");
        assert_ne!(continue105_stmt_sep(true), continue105_stmt_sep(false));
    }
}

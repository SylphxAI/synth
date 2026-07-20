//! Pure ClassDeclaration + FunctionDeclaration + ArrowFunctionExpression +
//! ImportDeclaration + ExportNamedDeclaration + ExportDefaultDeclaration
//! dual-oracle emission — residual pure **continue96** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–95:
//! - ClassDeclaration dual-oracle composing real `continue29_class_skeleton`
//! - FunctionDeclaration dual-oracle composing real
//!   `continue29_function_skeleton`
//! - ArrowFunctionExpression dual-oracle composing real
//!   `continue29_arrow_skeleton`
//! - ImportDeclaration dual-oracle composing real
//!   `continue29_import_named_skeleton`
//! - ExportNamedDeclaration dual-oracle composing real
//!   `continue29_export_named_skeleton`
//! - ExportDefaultDeclaration dual-oracle composing real
//!   `continue29_export_default_skeleton`
//! - Composed class/function/arrow/import/export residual shells
//!
//! Intentionally does **not** re-wrap continue71 class-extends/export-from,
//! continue72 arrow/method/export-default pretty-minify poles, continue86
//! function/class/this dual-oracle, continue87 import/export specifier, or
//! continue95 if/while/return/throw poles. Composes real shipped pure helpers
//! from continue29 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue29_arrow_skeleton, continue29_class_skeleton, continue29_export_default_skeleton,
    continue29_export_named_skeleton, continue29_function_skeleton,
    continue29_import_named_skeleton,
};

/// Dual-oracle residual: continue96 related AST type catalog.
pub const CONTINUE96_RELATED_TYPES: &[&str] = &[
    "ClassDeclaration",
    "ClassExpression",
    "ClassBody",
    "MethodDefinition",
    "FunctionDeclaration",
    "FunctionExpression",
    "ArrowFunctionExpression",
    "ImportDeclaration",
    "ImportSpecifier",
    "ExportNamedDeclaration",
    "ExportDefaultDeclaration",
    "ExportAllDeclaration",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_class_function_import_export_related_type(t: &str) -> bool {
    CONTINUE96_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue96_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue96_function_declaration_type(t: &str) -> bool {
    t == "FunctionDeclaration"
}

#[must_use]
pub fn is_continue96_arrow_function_type(t: &str) -> bool {
    t == "ArrowFunctionExpression"
}

#[must_use]
pub fn is_continue96_import_declaration_type(t: &str) -> bool {
    t == "ImportDeclaration"
}

#[must_use]
pub fn is_continue96_export_named_type(t: &str) -> bool {
    t == "ExportNamedDeclaration"
}

#[must_use]
pub fn is_continue96_export_default_type(t: &str) -> bool {
    t == "ExportDefaultDeclaration"
}

// ── ClassDeclaration dual-oracle ────────────────────────────────────────────

/// Dual-oracle ClassDeclaration skeleton composing real
/// [`continue29_class_skeleton`].
#[must_use]
pub fn continue96_class_declaration_skeleton(name: &str, body: &str) -> String {
    continue29_class_skeleton(name, body)
}

/// Dual-oracle ClassDeclaration pretty alias.
#[must_use]
pub fn continue96_class_pretty(name: &str, body: &str) -> String {
    continue96_class_declaration_skeleton(name, body)
}

/// Dual-oracle ClassDeclaration minify alias.
#[must_use]
pub fn continue96_class_minify(name: &str, body: &str) -> String {
    continue96_class_declaration_skeleton(name, body)
}

// ── FunctionDeclaration dual-oracle ─────────────────────────────────────────

/// Dual-oracle FunctionDeclaration skeleton composing real
/// [`continue29_function_skeleton`].
#[must_use]
pub fn continue96_function_declaration_skeleton(name: &str, params: &str, body: &str) -> String {
    continue29_function_skeleton(name, params, body)
}

/// Dual-oracle FunctionDeclaration pretty alias.
#[must_use]
pub fn continue96_function_pretty(name: &str, params: &str, body: &str) -> String {
    continue96_function_declaration_skeleton(name, params, body)
}

/// Dual-oracle FunctionDeclaration minify alias.
#[must_use]
pub fn continue96_function_minify(name: &str, params: &str, body: &str) -> String {
    continue96_function_declaration_skeleton(name, params, body)
}

// ── ArrowFunctionExpression dual-oracle ─────────────────────────────────────

/// Dual-oracle ArrowFunctionExpression skeleton composing real
/// [`continue29_arrow_skeleton`].
#[must_use]
pub fn continue96_arrow_function_skeleton(params: &str, body: &str) -> String {
    continue29_arrow_skeleton(params, body)
}

/// Dual-oracle ArrowFunctionExpression pretty alias.
#[must_use]
pub fn continue96_arrow_pretty(params: &str, body: &str) -> String {
    continue96_arrow_function_skeleton(params, body)
}

/// Dual-oracle ArrowFunctionExpression minify alias.
#[must_use]
pub fn continue96_arrow_minify(params: &str, body: &str) -> String {
    continue96_arrow_function_skeleton(params, body)
}

// ── ImportDeclaration dual-oracle ───────────────────────────────────────────

/// Dual-oracle ImportDeclaration skeleton composing real
/// [`continue29_import_named_skeleton`].
#[must_use]
pub fn continue96_import_named_skeleton(names: &str, source: &str) -> String {
    continue29_import_named_skeleton(names, source)
}

/// Dual-oracle ImportDeclaration pretty alias.
#[must_use]
pub fn continue96_import_pretty(names: &str, source: &str) -> String {
    continue96_import_named_skeleton(names, source)
}

/// Dual-oracle ImportDeclaration minify alias.
#[must_use]
pub fn continue96_import_minify(names: &str, source: &str) -> String {
    continue96_import_named_skeleton(names, source)
}

// ── ExportNamedDeclaration dual-oracle ──────────────────────────────────────

/// Dual-oracle ExportNamedDeclaration skeleton composing real
/// [`continue29_export_named_skeleton`].
#[must_use]
pub fn continue96_export_named_skeleton(names: &str) -> String {
    continue29_export_named_skeleton(names)
}

/// Dual-oracle ExportNamedDeclaration pretty alias.
#[must_use]
pub fn continue96_export_named_pretty(names: &str) -> String {
    continue96_export_named_skeleton(names)
}

/// Dual-oracle ExportNamedDeclaration minify alias.
#[must_use]
pub fn continue96_export_named_minify(names: &str) -> String {
    continue96_export_named_skeleton(names)
}

// ── ExportDefaultDeclaration dual-oracle ────────────────────────────────────

/// Dual-oracle ExportDefaultDeclaration skeleton composing real
/// [`continue29_export_default_skeleton`].
#[must_use]
pub fn continue96_export_default_skeleton(expr: &str) -> String {
    continue29_export_default_skeleton(expr)
}

/// Dual-oracle ExportDefaultDeclaration pretty alias.
#[must_use]
pub fn continue96_export_default_pretty(expr: &str) -> String {
    continue96_export_default_skeleton(expr)
}

/// Dual-oracle ExportDefaultDeclaration minify alias.
#[must_use]
pub fn continue96_export_default_minify(expr: &str) -> String {
    continue96_export_default_skeleton(expr)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: class with empty body.
#[must_use]
pub fn continue96_class_empty(name: &str) -> String {
    continue96_class_declaration_skeleton(name, "{}")
}

/// Dual-oracle residual: function with empty body.
#[must_use]
pub fn continue96_function_empty(name: &str) -> String {
    continue96_function_declaration_skeleton(name, "", "{}")
}

/// Dual-oracle residual: arrow with empty block body.
#[must_use]
pub fn continue96_arrow_empty_block(params: &str) -> String {
    continue96_arrow_function_skeleton(params, "{}")
}

/// Dual-oracle residual: export default of a class declaration.
#[must_use]
pub fn continue96_export_default_class(name: &str, body: &str) -> String {
    let class = continue96_class_declaration_skeleton(name, body);
    continue96_export_default_skeleton(&class)
}

/// Dual-oracle residual: export default of a function declaration.
#[must_use]
pub fn continue96_export_default_function(name: &str, params: &str, body: &str) -> String {
    let func = continue96_function_declaration_skeleton(name, params, body);
    continue96_export_default_skeleton(&func)
}

/// Dual-oracle residual: export default of an arrow function.
#[must_use]
pub fn continue96_export_default_arrow(params: &str, body: &str) -> String {
    let arrow = continue96_arrow_function_skeleton(params, body);
    continue96_export_default_skeleton(&arrow)
}

/// Dual-oracle residual: named import then named export of same names.
#[must_use]
pub fn continue96_import_then_export(names: &str, source: &str) -> String {
    format!(
        "{}; {}",
        continue96_import_named_skeleton(names, source),
        continue96_export_named_skeleton(names)
    )
}

/// Dual-oracle residual: class method-like body with function assignment shell.
#[must_use]
pub fn continue96_class_with_method_fn(name: &str, method: &str, params: &str) -> String {
    let method_fn = continue96_function_declaration_skeleton(method, params, "{}");
    let body = format!("{{ {method_fn} }}");
    continue96_class_declaration_skeleton(name, &body)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue96_stmt_sep(pretty: bool) -> &'static str {
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
        continue29_arrow_skeleton, continue29_class_skeleton, continue29_export_default_skeleton,
        continue29_export_named_skeleton, continue29_function_skeleton,
        continue29_import_named_skeleton,
    };

    #[test]
    fn continue96_type_catalog() {
        assert_eq!(CONTINUE96_RELATED_TYPES.len(), 12);
        assert!(is_class_function_import_export_related_type(
            "ClassDeclaration"
        ));
        assert!(is_class_function_import_export_related_type(
            "FunctionDeclaration"
        ));
        assert!(is_class_function_import_export_related_type(
            "ArrowFunctionExpression"
        ));
        assert!(is_class_function_import_export_related_type(
            "ImportDeclaration"
        ));
        assert!(is_class_function_import_export_related_type(
            "ExportNamedDeclaration"
        ));
        assert!(is_class_function_import_export_related_type(
            "ExportDefaultDeclaration"
        ));
        assert!(is_class_function_import_export_related_type("ClassBody"));
        assert!(is_class_function_import_export_related_type(
            "MethodDefinition"
        ));
        assert!(is_class_function_import_export_related_type(
            "FunctionExpression"
        ));
        assert!(is_class_function_import_export_related_type(
            "ImportSpecifier"
        ));
        assert!(is_class_function_import_export_related_type(
            "ExportAllDeclaration"
        ));
        assert!(!is_class_function_import_export_related_type("IfStatement"));
        assert!(!is_class_function_import_export_related_type(
            "WhileStatement"
        ));
        assert!(is_continue96_class_declaration_type("ClassDeclaration"));
        assert!(is_continue96_function_declaration_type(
            "FunctionDeclaration"
        ));
        assert!(is_continue96_arrow_function_type(
            "ArrowFunctionExpression"
        ));
        assert!(is_continue96_import_declaration_type("ImportDeclaration"));
        assert!(is_continue96_export_named_type("ExportNamedDeclaration"));
        assert!(is_continue96_export_default_type(
            "ExportDefaultDeclaration"
        ));
        assert!(!is_continue96_class_declaration_type(
            "FunctionDeclaration"
        ));
    }

    #[test]
    fn continue96_class_function_arrow_import_export_emit() {
        assert_eq!(
            continue96_class_declaration_skeleton("A", "{}"),
            "class A {}"
        );
        assert_eq!(
            continue96_class_declaration_skeleton("A", "{}"),
            continue29_class_skeleton("A", "{}")
        );
        assert_eq!(
            continue96_class_pretty("A", "{}"),
            continue96_class_minify("A", "{}")
        );

        assert_eq!(
            continue96_function_declaration_skeleton("f", "x", "{}"),
            "function f(x) {}"
        );
        assert_eq!(
            continue96_function_declaration_skeleton("f", "x", "{}"),
            continue29_function_skeleton("f", "x", "{}")
        );
        assert_eq!(
            continue96_function_pretty("f", "", "{}"),
            continue96_function_minify("f", "", "{}")
        );

        assert_eq!(
            continue96_arrow_function_skeleton("x", "x"),
            "(x) => x"
        );
        assert_eq!(
            continue96_arrow_function_skeleton("x", "x"),
            continue29_arrow_skeleton("x", "x")
        );
        assert_eq!(
            continue96_arrow_pretty("a, b", "{}"),
            continue96_arrow_minify("a, b", "{}")
        );

        assert_eq!(
            continue96_import_named_skeleton("a, b", "./m"),
            "import { a, b } from \"./m\""
        );
        assert_eq!(
            continue96_import_named_skeleton("a", "./m"),
            continue29_import_named_skeleton("a", "./m")
        );
        assert_eq!(
            continue96_import_pretty("x", "y"),
            continue96_import_minify("x", "y")
        );

        assert_eq!(
            continue96_export_named_skeleton("a, b"),
            "export { a, b }"
        );
        assert_eq!(
            continue96_export_named_skeleton("a"),
            continue29_export_named_skeleton("a")
        );
        assert_eq!(
            continue96_export_named_pretty("z"),
            continue96_export_named_minify("z")
        );

        assert_eq!(
            continue96_export_default_skeleton("Foo"),
            "export default Foo"
        );
        assert_eq!(
            continue96_export_default_skeleton("1"),
            continue29_export_default_skeleton("1")
        );
        assert_eq!(
            continue96_export_default_pretty("x"),
            continue96_export_default_minify("x")
        );
    }

    #[test]
    fn continue96_composed_residual_shells() {
        assert_eq!(continue96_class_empty("C"), "class C {}");
        assert_eq!(continue96_function_empty("g"), "function g() {}");
        assert_eq!(continue96_arrow_empty_block("x"), "(x) => {}");
        assert_eq!(
            continue96_export_default_class("App", "{}"),
            "export default class App {}"
        );
        assert_eq!(
            continue96_export_default_function("main", "", "{}"),
            "export default function main() {}"
        );
        assert_eq!(
            continue96_export_default_arrow("", "null"),
            "export default () => null"
        );
        assert_eq!(
            continue96_import_then_export("x", "./x"),
            "import { x } from \"./x\"; export { x }"
        );
        assert_eq!(
            continue96_class_with_method_fn("C", "m", ""),
            "class C { function m() {} }"
        );
        assert_eq!(continue96_stmt_sep(true), " ");
        assert_eq!(continue96_stmt_sep(false), "");
        assert_ne!(continue96_stmt_sep(true), continue96_stmt_sep(false));
    }
}

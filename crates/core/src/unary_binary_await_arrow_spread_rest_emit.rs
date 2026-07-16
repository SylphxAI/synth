//! Pure UnaryExpression + BinaryExpression + AwaitExpression +
//! ArrowFunctionExpression + SpreadElement + RestElement dual-oracle emission
//! — residual pure **continue108** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–107:
//! - UnaryExpression dual-oracle composing real `continue41_unary_skeleton`
//! - BinaryExpression dual-oracle composing real `continue41_binary_skeleton`
//! - AwaitExpression dual-oracle composing real `continue41_await_skeleton`
//! - ArrowFunctionExpression dual-oracle composing real
//!   `continue41_arrow_skeleton`
//! - SpreadElement dual-oracle composing real `continue41_spread_skeleton`
//! - RestElement dual-oracle composing real `continue41_rest_skeleton`
//! - Composed unary/binary/await/arrow/spread/rest residual shells
//!
//! Intentionally does **not** re-wrap continue98 unary/binary/await/jump poles
//! (continue31 bases), continue97 assignment/update/member/call, continue107
//! return/block/expr/var/ident poles, or continue40 bases. Composes real
//! shipped pure helpers from continue41 bases. Full engines remain product dens.
//! NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue41_arrow_skeleton, continue41_await_skeleton, continue41_binary_skeleton,
    continue41_rest_skeleton, continue41_spread_skeleton, continue41_unary_skeleton,
};

/// Dual-oracle residual: continue108 related AST type catalog.
pub const CONTINUE108_RELATED_TYPES: &[&str] = &[
    "UnaryExpression",
    "BinaryExpression",
    "AwaitExpression",
    "ArrowFunctionExpression",
    "SpreadElement",
    "RestElement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_unary_binary_await_arrow_spread_rest_related_type(t: &str) -> bool {
    CONTINUE108_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue108_unary_type(t: &str) -> bool {
    t == "UnaryExpression"
}

#[must_use]
pub fn is_continue108_binary_type(t: &str) -> bool {
    t == "BinaryExpression"
}

#[must_use]
pub fn is_continue108_await_type(t: &str) -> bool {
    t == "AwaitExpression"
}

#[must_use]
pub fn is_continue108_arrow_type(t: &str) -> bool {
    t == "ArrowFunctionExpression"
}

#[must_use]
pub fn is_continue108_spread_type(t: &str) -> bool {
    t == "SpreadElement"
}

#[must_use]
pub fn is_continue108_rest_type(t: &str) -> bool {
    t == "RestElement"
}

#[must_use]
pub fn is_continue108_expr_type(t: &str) -> bool {
    matches!(
        t,
        "UnaryExpression"
            | "BinaryExpression"
            | "AwaitExpression"
            | "ArrowFunctionExpression"
    )
}

#[must_use]
pub fn is_continue108_pattern_element_type(t: &str) -> bool {
    matches!(t, "SpreadElement" | "RestElement")
}

// ── UnaryExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle UnaryExpression skeleton composing real
/// [`continue41_unary_skeleton`].
#[must_use]
pub fn continue108_unary_skeleton(op: &str, arg: &str, prefix: bool) -> String {
    continue41_unary_skeleton(op, arg, prefix)
}

/// Dual-oracle UnaryExpression pretty alias.
#[must_use]
pub fn continue108_unary_pretty(op: &str, arg: &str, prefix: bool) -> String {
    continue108_unary_skeleton(op, arg, prefix)
}

/// Dual-oracle UnaryExpression minify alias.
#[must_use]
pub fn continue108_unary_minify(op: &str, arg: &str, prefix: bool) -> String {
    continue108_unary_skeleton(op, arg, prefix)
}

// ── BinaryExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle BinaryExpression skeleton composing real
/// [`continue41_binary_skeleton`].
#[must_use]
pub fn continue108_binary_skeleton(left: &str, op: &str, right: &str) -> String {
    continue41_binary_skeleton(left, op, right)
}

/// Dual-oracle BinaryExpression pretty alias.
#[must_use]
pub fn continue108_binary_pretty(left: &str, op: &str, right: &str) -> String {
    continue108_binary_skeleton(left, op, right)
}

/// Dual-oracle BinaryExpression minify alias.
#[must_use]
pub fn continue108_binary_minify(left: &str, op: &str, right: &str) -> String {
    continue108_binary_skeleton(left, op, right)
}

// ── AwaitExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle AwaitExpression skeleton composing real
/// [`continue41_await_skeleton`].
#[must_use]
pub fn continue108_await_skeleton(arg: &str) -> String {
    continue41_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression pretty alias.
#[must_use]
pub fn continue108_await_pretty(arg: &str) -> String {
    continue108_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression minify alias.
#[must_use]
pub fn continue108_await_minify(arg: &str) -> String {
    continue108_await_skeleton(arg)
}

// ── ArrowFunctionExpression dual-oracle ─────────────────────────────────────

/// Dual-oracle ArrowFunctionExpression skeleton composing real
/// [`continue41_arrow_skeleton`].
#[must_use]
pub fn continue108_arrow_skeleton(params: &str, body: &str, expression: bool) -> String {
    continue41_arrow_skeleton(params, body, expression)
}

/// Dual-oracle ArrowFunctionExpression pretty alias.
#[must_use]
pub fn continue108_arrow_pretty(params: &str, body: &str, expression: bool) -> String {
    continue108_arrow_skeleton(params, body, expression)
}

/// Dual-oracle ArrowFunctionExpression minify alias.
#[must_use]
pub fn continue108_arrow_minify(params: &str, body: &str, expression: bool) -> String {
    continue108_arrow_skeleton(params, body, expression)
}

// ── SpreadElement dual-oracle ───────────────────────────────────────────────

/// Dual-oracle SpreadElement skeleton composing real
/// [`continue41_spread_skeleton`].
#[must_use]
pub fn continue108_spread_skeleton(arg: &str) -> String {
    continue41_spread_skeleton(arg)
}

/// Dual-oracle SpreadElement pretty alias.
#[must_use]
pub fn continue108_spread_pretty(arg: &str) -> String {
    continue108_spread_skeleton(arg)
}

/// Dual-oracle SpreadElement minify alias.
#[must_use]
pub fn continue108_spread_minify(arg: &str) -> String {
    continue108_spread_skeleton(arg)
}

// ── RestElement dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle RestElement skeleton composing real
/// [`continue41_rest_skeleton`].
#[must_use]
pub fn continue108_rest_skeleton(arg: &str) -> String {
    continue41_rest_skeleton(arg)
}

/// Dual-oracle RestElement pretty alias.
#[must_use]
pub fn continue108_rest_pretty(arg: &str) -> String {
    continue108_rest_skeleton(arg)
}

/// Dual-oracle RestElement minify alias.
#[must_use]
pub fn continue108_rest_minify(arg: &str) -> String {
    continue108_rest_skeleton(arg)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: logical not.
#[must_use]
pub fn continue108_not(arg: &str) -> String {
    continue108_unary_skeleton("!", arg, true)
}

/// Dual-oracle residual: typeof.
#[must_use]
pub fn continue108_typeof(arg: &str) -> String {
    // unary typeof needs a space before arg for valid JS; continue41 prefix
    // concatenates op+arg, so pass "typeof " as op for the keyword form.
    continue108_unary_skeleton("typeof ", arg, true)
}

/// Dual-oracle residual: postfix increment.
#[must_use]
pub fn continue108_postfix_inc(arg: &str) -> String {
    continue108_unary_skeleton("++", arg, false)
}

/// Dual-oracle residual: strict equality.
#[must_use]
pub fn continue108_strict_eq(left: &str, right: &str) -> String {
    continue108_binary_skeleton(left, "===", right)
}

/// Dual-oracle residual: addition.
#[must_use]
pub fn continue108_add(left: &str, right: &str) -> String {
    continue108_binary_skeleton(left, "+", right)
}

/// Dual-oracle residual: await of an identifier-shaped arg.
#[must_use]
pub fn continue108_await_ident(name: &str) -> String {
    continue108_await_skeleton(name)
}

/// Dual-oracle residual: expression-body arrow.
#[must_use]
pub fn continue108_arrow_expr(params: &str, body: &str) -> String {
    continue108_arrow_skeleton(params, body, true)
}

/// Dual-oracle residual: block-body arrow.
#[must_use]
pub fn continue108_arrow_block(params: &str, body: &str) -> String {
    continue108_arrow_skeleton(params, body, false)
}

/// Dual-oracle residual: spread of an identifier.
#[must_use]
pub fn continue108_spread_ident(name: &str) -> String {
    continue108_spread_skeleton(name)
}

/// Dual-oracle residual: rest of an identifier.
#[must_use]
pub fn continue108_rest_ident(name: &str) -> String {
    continue108_rest_skeleton(name)
}

/// Dual-oracle residual: arrow that returns awaited arg (`(x) => await x`).
#[must_use]
pub fn continue108_arrow_await(params: &str, arg: &str) -> String {
    let awaited = continue108_await_skeleton(arg);
    continue108_arrow_expr(params, &awaited)
}

/// Dual-oracle residual: call-args spread list fragment (`a, ...rest`).
#[must_use]
pub fn continue108_args_with_spread(head: &str, rest: &str) -> String {
    let spread = continue108_spread_skeleton(rest);
    format!("{head}, {spread}")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue108_expr_sep(pretty: bool) -> &'static str {
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
        continue41_arrow_skeleton, continue41_await_skeleton, continue41_binary_skeleton,
        continue41_rest_skeleton, continue41_spread_skeleton, continue41_unary_skeleton,
    };

    #[test]
    fn continue108_type_catalog() {
        assert_eq!(CONTINUE108_RELATED_TYPES.len(), 6);
        assert!(is_unary_binary_await_arrow_spread_rest_related_type(
            "UnaryExpression"
        ));
        assert!(is_unary_binary_await_arrow_spread_rest_related_type(
            "BinaryExpression"
        ));
        assert!(is_unary_binary_await_arrow_spread_rest_related_type(
            "AwaitExpression"
        ));
        assert!(is_unary_binary_await_arrow_spread_rest_related_type(
            "ArrowFunctionExpression"
        ));
        assert!(is_unary_binary_await_arrow_spread_rest_related_type(
            "SpreadElement"
        ));
        assert!(is_unary_binary_await_arrow_spread_rest_related_type(
            "RestElement"
        ));
        assert!(!is_unary_binary_await_arrow_spread_rest_related_type(
            "ReturnStatement"
        ));
        assert!(!is_unary_binary_await_arrow_spread_rest_related_type(
            "SwitchCase"
        ));

        assert!(is_continue108_unary_type("UnaryExpression"));
        assert!(!is_continue108_unary_type("BinaryExpression"));
        assert!(is_continue108_binary_type("BinaryExpression"));
        assert!(is_continue108_await_type("AwaitExpression"));
        assert!(is_continue108_arrow_type("ArrowFunctionExpression"));
        assert!(is_continue108_spread_type("SpreadElement"));
        assert!(is_continue108_rest_type("RestElement"));
        assert!(is_continue108_expr_type("UnaryExpression"));
        assert!(is_continue108_expr_type("BinaryExpression"));
        assert!(is_continue108_expr_type("AwaitExpression"));
        assert!(is_continue108_expr_type("ArrowFunctionExpression"));
        assert!(!is_continue108_expr_type("SpreadElement"));
        assert!(is_continue108_pattern_element_type("SpreadElement"));
        assert!(is_continue108_pattern_element_type("RestElement"));
        assert!(!is_continue108_pattern_element_type("UnaryExpression"));
    }

    #[test]
    fn continue108_unary_binary_await_emit() {
        assert_eq!(continue108_unary_skeleton("!", "x", true), "!x");
        assert_eq!(continue108_unary_skeleton("++", "i", false), "i++");
        assert_eq!(
            continue108_unary_skeleton("!", "x", true),
            continue41_unary_skeleton("!", "x", true)
        );
        assert_eq!(
            continue108_unary_pretty("!", "x", true),
            continue108_unary_minify("!", "x", true)
        );

        assert_eq!(continue108_binary_skeleton("a", "+", "b"), "a + b");
        assert_eq!(
            continue108_binary_skeleton("a", "===", "b"),
            "a === b"
        );
        assert_eq!(
            continue108_binary_skeleton("a", "+", "b"),
            continue41_binary_skeleton("a", "+", "b")
        );
        assert_eq!(
            continue108_binary_pretty("a", "-", "b"),
            continue108_binary_minify("a", "-", "b")
        );

        assert_eq!(continue108_await_skeleton("fetch()"), "await fetch()");
        assert_eq!(
            continue108_await_skeleton("x"),
            continue41_await_skeleton("x")
        );
        assert_eq!(
            continue108_await_pretty("x"),
            continue108_await_minify("x")
        );
    }

    #[test]
    fn continue108_arrow_spread_rest_emit() {
        assert_eq!(
            continue108_arrow_skeleton("x", "x + 1", true),
            "(x) => x + 1"
        );
        assert_eq!(
            continue108_arrow_skeleton("x", "return x;", false),
            "(x) => { return x; }"
        );
        assert_eq!(
            continue108_arrow_skeleton("x", "x", true),
            continue41_arrow_skeleton("x", "x", true)
        );
        assert_eq!(
            continue108_arrow_pretty("a", "a", true),
            continue108_arrow_minify("a", "a", true)
        );

        assert_eq!(continue108_spread_skeleton("xs"), "...xs");
        assert_eq!(
            continue108_spread_skeleton("xs"),
            continue41_spread_skeleton("xs")
        );
        assert_eq!(
            continue108_spread_pretty("ys"),
            continue108_spread_minify("ys")
        );

        assert_eq!(continue108_rest_skeleton("rest"), "...rest");
        assert_eq!(
            continue108_rest_skeleton("rest"),
            continue41_rest_skeleton("rest")
        );
        assert_eq!(
            continue108_rest_pretty("r"),
            continue108_rest_minify("r")
        );
    }

    #[test]
    fn continue108_composed_residual_shells() {
        assert_eq!(continue108_not("x"), "!x");
        assert_eq!(continue108_typeof("x"), "typeof x");
        assert_eq!(continue108_postfix_inc("i"), "i++");
        assert_eq!(continue108_strict_eq("a", "b"), "a === b");
        assert_eq!(continue108_add("1", "2"), "1 + 2");
        assert_eq!(continue108_await_ident("p"), "await p");
        assert_eq!(continue108_arrow_expr("x", "x + 1"), "(x) => x + 1");
        assert_eq!(
            continue108_arrow_block("x", "return x;"),
            "(x) => { return x; }"
        );
        assert_eq!(continue108_spread_ident("xs"), "...xs");
        assert_eq!(continue108_rest_ident("rest"), "...rest");
        assert_eq!(
            continue108_arrow_await("x", "x"),
            "(x) => await x"
        );
        assert_eq!(
            continue108_args_with_spread("a", "rest"),
            "a, ...rest"
        );
        assert_eq!(continue108_expr_sep(true), " ");
        assert_eq!(continue108_expr_sep(false), "");
        assert_ne!(continue108_expr_sep(true), continue108_expr_sep(false));
    }
}

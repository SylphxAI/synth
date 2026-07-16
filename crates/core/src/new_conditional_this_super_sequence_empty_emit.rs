//! Pure NewExpression + ConditionalExpression + ThisExpression + Super +
//! SequenceExpression + EmptyStatement dual-oracle emission
//! — residual pure **continue109** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–108:
//! - NewExpression dual-oracle composing real `continue42_new_skeleton`
//! - ConditionalExpression dual-oracle composing real
//!   `continue42_conditional_skeleton`
//! - ThisExpression dual-oracle composing real `continue42_this_skeleton`
//! - Super dual-oracle composing real `continue42_super_skeleton`
//! - SequenceExpression dual-oracle composing real
//!   `continue42_sequence_skeleton`
//! - EmptyStatement dual-oracle composing real `continue42_empty_skeleton`
//! - Composed new/conditional/this/super/sequence/empty residual shells
//!
//! Intentionally does **not** re-wrap continue108 unary/binary/await/arrow/
//! spread/rest poles (continue41 bases), continue100 meta/import/chain poles
//! (continue33 bases), continue93 sequence/update/yield, or continue40–41
//! bases. Composes real shipped pure helpers from continue42 bases. Full
//! engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue42_conditional_skeleton, continue42_empty_skeleton, continue42_new_skeleton,
    continue42_sequence_skeleton, continue42_super_skeleton, continue42_this_skeleton,
};

/// Dual-oracle residual: continue109 related AST type catalog.
pub const CONTINUE109_RELATED_TYPES: &[&str] = &[
    "NewExpression",
    "ConditionalExpression",
    "ThisExpression",
    "Super",
    "SequenceExpression",
    "EmptyStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_new_conditional_this_super_sequence_empty_related_type(t: &str) -> bool {
    CONTINUE109_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue109_new_type(t: &str) -> bool {
    t == "NewExpression"
}

#[must_use]
pub fn is_continue109_conditional_type(t: &str) -> bool {
    t == "ConditionalExpression"
}

#[must_use]
pub fn is_continue109_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue109_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue109_sequence_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue109_empty_type(t: &str) -> bool {
    t == "EmptyStatement"
}

#[must_use]
pub fn is_continue109_expr_type(t: &str) -> bool {
    matches!(
        t,
        "NewExpression"
            | "ConditionalExpression"
            | "ThisExpression"
            | "Super"
            | "SequenceExpression"
    )
}

#[must_use]
pub fn is_continue109_stmt_type(t: &str) -> bool {
    t == "EmptyStatement"
}

// ── NewExpression dual-oracle ───────────────────────────────────────────────

/// Dual-oracle NewExpression skeleton composing real
/// [`continue42_new_skeleton`].
#[must_use]
pub fn continue109_new_skeleton(ctor: &str, args: &str) -> String {
    continue42_new_skeleton(ctor, args)
}

/// Dual-oracle NewExpression pretty alias.
#[must_use]
pub fn continue109_new_pretty(ctor: &str, args: &str) -> String {
    continue109_new_skeleton(ctor, args)
}

/// Dual-oracle NewExpression minify alias.
#[must_use]
pub fn continue109_new_minify(ctor: &str, args: &str) -> String {
    continue109_new_skeleton(ctor, args)
}

// ── ConditionalExpression dual-oracle ───────────────────────────────────────

/// Dual-oracle ConditionalExpression skeleton composing real
/// [`continue42_conditional_skeleton`].
#[must_use]
pub fn continue109_conditional_skeleton(test: &str, cons: &str, alt: &str) -> String {
    continue42_conditional_skeleton(test, cons, alt)
}

/// Dual-oracle ConditionalExpression pretty alias.
#[must_use]
pub fn continue109_conditional_pretty(test: &str, cons: &str, alt: &str) -> String {
    continue109_conditional_skeleton(test, cons, alt)
}

/// Dual-oracle ConditionalExpression minify alias.
#[must_use]
pub fn continue109_conditional_minify(test: &str, cons: &str, alt: &str) -> String {
    continue109_conditional_skeleton(test, cons, alt)
}

// ── ThisExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThisExpression skeleton composing real
/// [`continue42_this_skeleton`].
#[must_use]
pub fn continue109_this_skeleton() -> &'static str {
    continue42_this_skeleton()
}

/// Dual-oracle ThisExpression pretty alias.
#[must_use]
pub fn continue109_this_pretty() -> &'static str {
    continue109_this_skeleton()
}

/// Dual-oracle ThisExpression minify alias.
#[must_use]
pub fn continue109_this_minify() -> &'static str {
    continue109_this_skeleton()
}

// ── Super dual-oracle ───────────────────────────────────────────────────────

/// Dual-oracle Super skeleton composing real [`continue42_super_skeleton`].
#[must_use]
pub fn continue109_super_skeleton() -> &'static str {
    continue42_super_skeleton()
}

/// Dual-oracle Super pretty alias.
#[must_use]
pub fn continue109_super_pretty() -> &'static str {
    continue109_super_skeleton()
}

/// Dual-oracle Super minify alias.
#[must_use]
pub fn continue109_super_minify() -> &'static str {
    continue109_super_skeleton()
}

// ── SequenceExpression dual-oracle ──────────────────────────────────────────

/// Dual-oracle SequenceExpression skeleton composing real
/// [`continue42_sequence_skeleton`].
#[must_use]
pub fn continue109_sequence_skeleton(left: &str, right: &str) -> String {
    continue42_sequence_skeleton(left, right)
}

/// Dual-oracle SequenceExpression pretty alias.
#[must_use]
pub fn continue109_sequence_pretty(left: &str, right: &str) -> String {
    continue109_sequence_skeleton(left, right)
}

/// Dual-oracle SequenceExpression minify alias.
#[must_use]
pub fn continue109_sequence_minify(left: &str, right: &str) -> String {
    continue109_sequence_skeleton(left, right)
}

// ── EmptyStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle EmptyStatement skeleton composing real
/// [`continue42_empty_skeleton`].
#[must_use]
pub fn continue109_empty_skeleton() -> &'static str {
    continue42_empty_skeleton()
}

/// Dual-oracle EmptyStatement pretty alias.
#[must_use]
pub fn continue109_empty_pretty() -> &'static str {
    continue109_empty_skeleton()
}

/// Dual-oracle EmptyStatement minify alias.
#[must_use]
pub fn continue109_empty_minify() -> &'static str {
    continue109_empty_skeleton()
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: `new Map()`.
#[must_use]
pub fn continue109_new_empty(ctor: &str) -> String {
    continue109_new_skeleton(ctor, "")
}

/// Dual-oracle residual: `new Error(msg)`.
#[must_use]
pub fn continue109_new_error(msg: &str) -> String {
    continue109_new_skeleton("Error", msg)
}

/// Dual-oracle residual: nullish-style ternary fallback.
#[must_use]
pub fn continue109_or_else(test: &str, cons: &str, alt: &str) -> String {
    continue109_conditional_skeleton(test, cons, alt)
}

/// Dual-oracle residual: identity ternary (`t ? a : a`).
#[must_use]
pub fn continue109_conditional_same(test: &str, value: &str) -> String {
    continue109_conditional_skeleton(test, value, value)
}

/// Dual-oracle residual: `this` member seed (`this` alone).
#[must_use]
pub fn continue109_this_ref() -> &'static str {
    continue109_this_skeleton()
}

/// Dual-oracle residual: `super` ref seed.
#[must_use]
pub fn continue109_super_ref() -> &'static str {
    continue109_super_skeleton()
}

/// Dual-oracle residual: three-element sequence via nested pairs.
#[must_use]
pub fn continue109_sequence_three(a: &str, b: &str, c: &str) -> String {
    let left = continue109_sequence_skeleton(a, b);
    continue109_sequence_skeleton(&left, c)
}

/// Dual-oracle residual: sequence ending in empty-statement semi.
#[must_use]
pub fn continue109_sequence_then_empty(left: &str, right: &str) -> String {
    let seq = continue109_sequence_skeleton(left, right);
    format!("{seq}{}", continue109_empty_skeleton())
}

/// Dual-oracle residual: `new Ctor()` chosen by ternary.
#[must_use]
pub fn continue109_conditional_new(test: &str, ctor_a: &str, ctor_b: &str) -> String {
    let a = continue109_new_empty(ctor_a);
    let b = continue109_new_empty(ctor_b);
    continue109_conditional_skeleton(test, &a, &b)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue109_expr_sep(pretty: bool) -> &'static str {
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
        continue42_conditional_skeleton, continue42_empty_skeleton, continue42_new_skeleton,
        continue42_sequence_skeleton, continue42_super_skeleton, continue42_this_skeleton,
    };

    #[test]
    fn continue109_type_catalog() {
        assert_eq!(CONTINUE109_RELATED_TYPES.len(), 6);
        assert!(is_new_conditional_this_super_sequence_empty_related_type(
            "NewExpression"
        ));
        assert!(is_new_conditional_this_super_sequence_empty_related_type(
            "ConditionalExpression"
        ));
        assert!(is_new_conditional_this_super_sequence_empty_related_type(
            "ThisExpression"
        ));
        assert!(is_new_conditional_this_super_sequence_empty_related_type(
            "Super"
        ));
        assert!(is_new_conditional_this_super_sequence_empty_related_type(
            "SequenceExpression"
        ));
        assert!(is_new_conditional_this_super_sequence_empty_related_type(
            "EmptyStatement"
        ));
        assert!(!is_new_conditional_this_super_sequence_empty_related_type(
            "UnaryExpression"
        ));
        assert!(!is_new_conditional_this_super_sequence_empty_related_type(
            "ArrowFunctionExpression"
        ));

        assert!(is_continue109_new_type("NewExpression"));
        assert!(!is_continue109_new_type("ConditionalExpression"));
        assert!(is_continue109_conditional_type("ConditionalExpression"));
        assert!(is_continue109_this_type("ThisExpression"));
        assert!(is_continue109_super_type("Super"));
        assert!(is_continue109_sequence_type("SequenceExpression"));
        assert!(is_continue109_empty_type("EmptyStatement"));
        assert!(is_continue109_expr_type("NewExpression"));
        assert!(is_continue109_expr_type("ConditionalExpression"));
        assert!(is_continue109_expr_type("ThisExpression"));
        assert!(is_continue109_expr_type("Super"));
        assert!(is_continue109_expr_type("SequenceExpression"));
        assert!(!is_continue109_expr_type("EmptyStatement"));
        assert!(is_continue109_stmt_type("EmptyStatement"));
        assert!(!is_continue109_stmt_type("NewExpression"));
    }

    #[test]
    fn continue109_new_conditional_emit() {
        assert_eq!(
            continue109_new_skeleton("Map", "entries"),
            "new Map(entries)"
        );
        assert_eq!(
            continue109_new_skeleton("Map", "entries"),
            continue42_new_skeleton("Map", "entries")
        );
        assert_eq!(
            continue109_new_pretty("Set", ""),
            continue109_new_minify("Set", "")
        );

        assert_eq!(
            continue109_conditional_skeleton("ok", "a", "b"),
            "ok ? a : b"
        );
        assert_eq!(
            continue109_conditional_skeleton("ok", "a", "b"),
            continue42_conditional_skeleton("ok", "a", "b")
        );
        assert_eq!(
            continue109_conditional_pretty("t", "1", "0"),
            continue109_conditional_minify("t", "1", "0")
        );
    }

    #[test]
    fn continue109_this_super_sequence_empty_emit() {
        assert_eq!(continue109_this_skeleton(), "this");
        assert_eq!(continue109_this_skeleton(), continue42_this_skeleton());
        assert_eq!(continue109_this_pretty(), continue109_this_minify());

        assert_eq!(continue109_super_skeleton(), "super");
        assert_eq!(continue109_super_skeleton(), continue42_super_skeleton());
        assert_eq!(continue109_super_pretty(), continue109_super_minify());

        assert_eq!(continue109_sequence_skeleton("a", "b"), "a, b");
        assert_eq!(
            continue109_sequence_skeleton("a", "b"),
            continue42_sequence_skeleton("a", "b")
        );
        assert_eq!(
            continue109_sequence_pretty("x", "y"),
            continue109_sequence_minify("x", "y")
        );

        assert_eq!(continue109_empty_skeleton(), ";");
        assert_eq!(continue109_empty_skeleton(), continue42_empty_skeleton());
        assert_eq!(continue109_empty_pretty(), continue109_empty_minify());
    }

    #[test]
    fn continue109_composed_residual_shells() {
        assert_eq!(continue109_new_empty("Map"), "new Map()");
        assert_eq!(continue109_new_error("\"boom\""), "new Error(\"boom\")");
        assert_eq!(continue109_or_else("x", "y", "z"), "x ? y : z");
        assert_eq!(
            continue109_conditional_same("flag", "v"),
            "flag ? v : v"
        );
        assert_eq!(continue109_this_ref(), "this");
        assert_eq!(continue109_super_ref(), "super");
        assert_eq!(continue109_sequence_three("a", "b", "c"), "a, b, c");
        assert_eq!(continue109_sequence_then_empty("a", "b"), "a, b;");
        assert_eq!(
            continue109_conditional_new("useMap", "Map", "Set"),
            "useMap ? new Map() : new Set()"
        );
        assert_eq!(continue109_expr_sep(true), " ");
        assert_eq!(continue109_expr_sep(false), "");
        assert_ne!(continue109_expr_sep(true), continue109_expr_sep(false));
    }
}

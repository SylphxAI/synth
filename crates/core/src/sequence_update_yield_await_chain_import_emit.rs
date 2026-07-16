//! Pure SequenceExpression + UpdateExpression + YieldExpression +
//! AwaitExpression + ChainExpression + ImportExpression dual-oracle emission —
//! residual pure **continue119** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–118:
//! - SequenceExpression dual-oracle composing real
//!   `continue53_sequence_skeleton`
//! - UpdateExpression prefix dual-oracle composing real
//!   `continue53_update_prefix_skeleton`
//! - UpdateExpression postfix dual-oracle composing real
//!   `continue53_update_postfix_skeleton`
//! - YieldExpression dual-oracle composing real `continue53_yield_skeleton`
//! - Yield* dual-oracle composing real `continue53_yield_star_skeleton`
//! - AwaitExpression dual-oracle composing real `continue53_await_skeleton`
//! - ImportExpression dual-oracle composing real
//!   `continue53_import_dynamic_skeleton`
//! - ChainExpression / optional member dual-oracle composing real
//!   `continue53_optional_chain_skeleton`
//! - Composed sequence/update/yield/await/chain/import residual shells
//!
//! Intentionally does **not** re-wrap continue118 super/this/meta/template poles
//! (continue51 bases), continue117 class private/method/accessor/object poles
//! (continue50 bases), or continue93 sequence/update/yield/await poles
//! (continue25/26 bases). Composes real shipped pure helpers from continue53
//! sequence/update/yield/await/chain/import bases. Full engines remain product
//! dens. NO authority_rust / ts_deleted. dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue53_await_skeleton, continue53_import_dynamic_skeleton,
    continue53_optional_chain_skeleton, continue53_sequence_skeleton,
    continue53_update_postfix_skeleton, continue53_update_prefix_skeleton,
    continue53_yield_skeleton, continue53_yield_star_skeleton,
};

/// Dual-oracle residual: continue119 related AST type catalog.
pub const CONTINUE119_RELATED_TYPES: &[&str] = &[
    "SequenceExpression",
    "UpdateExpression",
    "YieldExpression",
    "AwaitExpression",
    "ChainExpression",
    "ImportExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_sequence_update_yield_await_chain_import_related_type(t: &str) -> bool {
    CONTINUE119_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue119_sequence_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue119_update_type(t: &str) -> bool {
    t == "UpdateExpression"
}

#[must_use]
pub fn is_continue119_yield_type(t: &str) -> bool {
    t == "YieldExpression"
}

#[must_use]
pub fn is_continue119_await_type(t: &str) -> bool {
    t == "AwaitExpression"
}

#[must_use]
pub fn is_continue119_chain_type(t: &str) -> bool {
    t == "ChainExpression"
}

#[must_use]
pub fn is_continue119_import_type(t: &str) -> bool {
    t == "ImportExpression"
}

#[must_use]
pub fn is_continue119_control_expr_plane_type(t: &str) -> bool {
    matches!(
        t,
        "SequenceExpression" | "UpdateExpression" | "YieldExpression" | "AwaitExpression"
    )
}

#[must_use]
pub fn is_continue119_chain_import_plane_type(t: &str) -> bool {
    matches!(t, "ChainExpression" | "ImportExpression")
}

#[must_use]
pub fn is_continue119_type(t: &str) -> bool {
    matches!(
        t,
        "SequenceExpression"
            | "UpdateExpression"
            | "YieldExpression"
            | "AwaitExpression"
            | "ChainExpression"
            | "ImportExpression"
    )
}

// ── SequenceExpression dual-oracle ──────────────────────────────────────────

/// Dual-oracle SequenceExpression skeleton composing real
/// [`continue53_sequence_skeleton`].
#[must_use]
pub fn continue119_sequence_skeleton(left: &str, right: &str) -> String {
    continue53_sequence_skeleton(left, right)
}

/// Dual-oracle SequenceExpression pretty alias.
#[must_use]
pub fn continue119_sequence_pretty(left: &str, right: &str) -> String {
    continue119_sequence_skeleton(left, right)
}

/// Dual-oracle SequenceExpression minify alias.
#[must_use]
pub fn continue119_sequence_minify(left: &str, right: &str) -> String {
    continue119_sequence_skeleton(left, right)
}

// ── UpdateExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle UpdateExpression prefix skeleton composing real
/// [`continue53_update_prefix_skeleton`].
#[must_use]
pub fn continue119_update_prefix_skeleton(op: &str, arg: &str) -> String {
    continue53_update_prefix_skeleton(op, arg)
}

/// Dual-oracle UpdateExpression prefix pretty alias.
#[must_use]
pub fn continue119_update_prefix_pretty(op: &str, arg: &str) -> String {
    continue119_update_prefix_skeleton(op, arg)
}

/// Dual-oracle UpdateExpression prefix minify alias.
#[must_use]
pub fn continue119_update_prefix_minify(op: &str, arg: &str) -> String {
    continue119_update_prefix_skeleton(op, arg)
}

/// Dual-oracle UpdateExpression postfix skeleton composing real
/// [`continue53_update_postfix_skeleton`].
#[must_use]
pub fn continue119_update_postfix_skeleton(arg: &str, op: &str) -> String {
    continue53_update_postfix_skeleton(arg, op)
}

/// Dual-oracle UpdateExpression postfix pretty alias.
#[must_use]
pub fn continue119_update_postfix_pretty(arg: &str, op: &str) -> String {
    continue119_update_postfix_skeleton(arg, op)
}

/// Dual-oracle UpdateExpression postfix minify alias.
#[must_use]
pub fn continue119_update_postfix_minify(arg: &str, op: &str) -> String {
    continue119_update_postfix_skeleton(arg, op)
}

/// Dual-oracle UpdateExpression unified pole.
#[must_use]
pub fn continue119_update_skeleton(arg: &str, op: &str, prefix: bool) -> String {
    if prefix {
        continue119_update_prefix_skeleton(op, arg)
    } else {
        continue119_update_postfix_skeleton(arg, op)
    }
}

// ── YieldExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle YieldExpression skeleton composing real
/// [`continue53_yield_skeleton`].
#[must_use]
pub fn continue119_yield_skeleton(arg: &str) -> String {
    continue53_yield_skeleton(arg)
}

/// Dual-oracle YieldExpression pretty alias.
#[must_use]
pub fn continue119_yield_pretty(arg: &str) -> String {
    continue119_yield_skeleton(arg)
}

/// Dual-oracle YieldExpression minify alias.
#[must_use]
pub fn continue119_yield_minify(arg: &str) -> String {
    continue119_yield_skeleton(arg)
}

/// Dual-oracle yield* skeleton composing real
/// [`continue53_yield_star_skeleton`].
#[must_use]
pub fn continue119_yield_star_skeleton(arg: &str) -> String {
    continue53_yield_star_skeleton(arg)
}

/// Dual-oracle yield* pretty alias.
#[must_use]
pub fn continue119_yield_star_pretty(arg: &str) -> String {
    continue119_yield_star_skeleton(arg)
}

/// Dual-oracle yield* minify alias.
#[must_use]
pub fn continue119_yield_star_minify(arg: &str) -> String {
    continue119_yield_star_skeleton(arg)
}

// ── AwaitExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle AwaitExpression skeleton composing real
/// [`continue53_await_skeleton`].
#[must_use]
pub fn continue119_await_skeleton(arg: &str) -> String {
    continue53_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression pretty alias.
#[must_use]
pub fn continue119_await_pretty(arg: &str) -> String {
    continue119_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression minify alias.
#[must_use]
pub fn continue119_await_minify(arg: &str) -> String {
    continue119_await_skeleton(arg)
}

// ── ImportExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle ImportExpression skeleton composing real
/// [`continue53_import_dynamic_skeleton`].
#[must_use]
pub fn continue119_import_dynamic_skeleton(spec: &str) -> String {
    continue53_import_dynamic_skeleton(spec)
}

/// Dual-oracle ImportExpression pretty alias.
#[must_use]
pub fn continue119_import_dynamic_pretty(spec: &str) -> String {
    continue119_import_dynamic_skeleton(spec)
}

/// Dual-oracle ImportExpression minify alias.
#[must_use]
pub fn continue119_import_dynamic_minify(spec: &str) -> String {
    continue119_import_dynamic_skeleton(spec)
}

// ── ChainExpression / optional member dual-oracle ───────────────────────────

/// Dual-oracle optional-chain skeleton composing real
/// [`continue53_optional_chain_skeleton`].
#[must_use]
pub fn continue119_optional_chain_skeleton(obj: &str, prop: &str) -> String {
    continue53_optional_chain_skeleton(obj, prop)
}

/// Dual-oracle optional-chain pretty alias.
#[must_use]
pub fn continue119_optional_chain_pretty(obj: &str, prop: &str) -> String {
    continue119_optional_chain_skeleton(obj, prop)
}

/// Dual-oracle optional-chain minify alias.
#[must_use]
pub fn continue119_optional_chain_minify(obj: &str, prop: &str) -> String {
    continue119_optional_chain_skeleton(obj, prop)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: sequence then update-prefix.
#[must_use]
pub fn continue119_sequence_then_update_prefix(
    left: &str,
    right: &str,
    op: &str,
    arg: &str,
) -> String {
    let seq = continue119_sequence_skeleton(left, right);
    let up = continue119_update_prefix_skeleton(op, arg);
    format!("{seq} {up}")
}

/// Dual-oracle residual: yield then await.
#[must_use]
pub fn continue119_yield_then_await(yarg: &str, aarg: &str) -> String {
    let y = continue119_yield_skeleton(yarg);
    let a = continue119_await_skeleton(aarg);
    format!("{y} {a}")
}

/// Dual-oracle residual: yield* then import().
#[must_use]
pub fn continue119_yield_star_then_import(yarg: &str, spec: &str) -> String {
    let y = continue119_yield_star_skeleton(yarg);
    let i = continue119_import_dynamic_skeleton(spec);
    format!("{y} {i}")
}

/// Dual-oracle residual: optional chain then sequence.
#[must_use]
pub fn continue119_chain_then_sequence(obj: &str, prop: &str, left: &str, right: &str) -> String {
    let c = continue119_optional_chain_skeleton(obj, prop);
    let s = continue119_sequence_skeleton(left, right);
    format!("{c} {s}")
}

/// Dual-oracle residual: update prefix/postfix pair.
#[must_use]
pub fn continue119_update_pair(arg: &str) -> String {
    let pre = continue119_update_prefix_skeleton("++", arg);
    let post = continue119_update_postfix_skeleton(arg, "--");
    format!("{pre} {post}")
}

/// Dual-oracle residual: empty-ish seed helpers.
#[must_use]
pub fn continue119_sequence_empty_pair() -> String {
    continue119_sequence_skeleton("", "")
}

/// Dual-oracle residual: bare yield seed.
#[must_use]
pub fn continue119_yield_empty() -> String {
    continue119_yield_skeleton("")
}

/// Dual-oracle residual: bare await seed.
#[must_use]
pub fn continue119_await_empty() -> String {
    continue119_await_skeleton("")
}

/// Dual-oracle residual: empty import() seed.
#[must_use]
pub fn continue119_import_empty() -> String {
    continue119_import_dynamic_skeleton("")
}

/// Dual-oracle residual: o?.x seed helper.
#[must_use]
pub fn continue119_o_optional_x() -> String {
    continue119_optional_chain_skeleton("o", "x")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue119_sep(pretty: bool) -> &'static str {
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
        continue53_await_skeleton, continue53_import_dynamic_skeleton,
        continue53_optional_chain_skeleton, continue53_sequence_skeleton,
        continue53_update_postfix_skeleton, continue53_update_prefix_skeleton,
        continue53_yield_skeleton, continue53_yield_star_skeleton,
    };

    #[test]
    fn continue119_type_catalog() {
        assert_eq!(CONTINUE119_RELATED_TYPES.len(), 6);
        assert!(is_sequence_update_yield_await_chain_import_related_type(
            "SequenceExpression"
        ));
        assert!(is_sequence_update_yield_await_chain_import_related_type(
            "UpdateExpression"
        ));
        assert!(is_sequence_update_yield_await_chain_import_related_type(
            "YieldExpression"
        ));
        assert!(is_sequence_update_yield_await_chain_import_related_type(
            "AwaitExpression"
        ));
        assert!(is_sequence_update_yield_await_chain_import_related_type(
            "ChainExpression"
        ));
        assert!(is_sequence_update_yield_await_chain_import_related_type(
            "ImportExpression"
        ));
        assert!(!is_sequence_update_yield_await_chain_import_related_type(
            "Super"
        ));
        assert!(!is_sequence_update_yield_await_chain_import_related_type(
            "ThisExpression"
        ));

        assert!(is_continue119_sequence_type("SequenceExpression"));
        assert!(!is_continue119_sequence_type("UpdateExpression"));
        assert!(is_continue119_update_type("UpdateExpression"));
        assert!(is_continue119_yield_type("YieldExpression"));
        assert!(is_continue119_await_type("AwaitExpression"));
        assert!(is_continue119_chain_type("ChainExpression"));
        assert!(is_continue119_import_type("ImportExpression"));
        assert!(is_continue119_control_expr_plane_type("SequenceExpression"));
        assert!(is_continue119_control_expr_plane_type("YieldExpression"));
        assert!(!is_continue119_control_expr_plane_type("ImportExpression"));
        assert!(is_continue119_chain_import_plane_type("ChainExpression"));
        assert!(is_continue119_chain_import_plane_type("ImportExpression"));
        assert!(!is_continue119_chain_import_plane_type("UpdateExpression"));
        assert!(is_continue119_type("ImportExpression"));
        assert!(!is_continue119_type("Super"));
        assert!(!is_continue119_type("Identifier"));
    }

    #[test]
    fn continue119_sequence_update_emit() {
        assert_eq!(continue119_sequence_skeleton("a", "b"), "a, b");
        assert_eq!(
            continue119_sequence_skeleton("a", "b"),
            continue53_sequence_skeleton("a", "b")
        );
        assert_eq!(
            continue119_sequence_pretty("x", "y"),
            continue119_sequence_minify("x", "y")
        );
        assert_eq!(continue119_sequence_empty_pair(), ", ");

        assert_eq!(continue119_update_prefix_skeleton("++", "i"), "++i");
        assert_eq!(
            continue119_update_prefix_skeleton("++", "i"),
            continue53_update_prefix_skeleton("++", "i")
        );
        assert_eq!(
            continue119_update_prefix_pretty("--", "n"),
            continue119_update_prefix_minify("--", "n")
        );

        assert_eq!(continue119_update_postfix_skeleton("i", "--"), "i--");
        assert_eq!(
            continue119_update_postfix_skeleton("i", "--"),
            continue53_update_postfix_skeleton("i", "--")
        );
        assert_eq!(
            continue119_update_postfix_pretty("n", "++"),
            continue119_update_postfix_minify("n", "++")
        );

        assert_eq!(continue119_update_skeleton("i", "++", true), "++i");
        assert_eq!(continue119_update_skeleton("i", "++", false), "i++");
        assert_eq!(continue119_update_pair("i"), "++i i--");
    }

    #[test]
    fn continue119_yield_await_chain_import_emit() {
        assert_eq!(continue119_yield_skeleton("x"), "yield x");
        assert_eq!(
            continue119_yield_skeleton("x"),
            continue53_yield_skeleton("x")
        );
        assert_eq!(
            continue119_yield_pretty("y"),
            continue119_yield_minify("y")
        );
        assert_eq!(continue119_yield_empty(), "yield ");

        assert_eq!(continue119_yield_star_skeleton("xs"), "yield* xs");
        assert_eq!(
            continue119_yield_star_skeleton("xs"),
            continue53_yield_star_skeleton("xs")
        );
        assert_eq!(
            continue119_yield_star_pretty("it"),
            continue119_yield_star_minify("it")
        );

        assert_eq!(continue119_await_skeleton("p"), "await p");
        assert_eq!(
            continue119_await_skeleton("p"),
            continue53_await_skeleton("p")
        );
        assert_eq!(
            continue119_await_pretty("q"),
            continue119_await_minify("q")
        );
        assert_eq!(continue119_await_empty(), "await ");

        assert_eq!(
            continue119_import_dynamic_skeleton("'./m'"),
            "import('./m')"
        );
        assert_eq!(
            continue119_import_dynamic_skeleton("'./m'"),
            continue53_import_dynamic_skeleton("'./m'")
        );
        assert_eq!(
            continue119_import_dynamic_pretty("s"),
            continue119_import_dynamic_minify("s")
        );
        assert_eq!(continue119_import_empty(), "import()");

        assert_eq!(continue119_optional_chain_skeleton("o", "x"), "o?.x");
        assert_eq!(
            continue119_optional_chain_skeleton("o", "x"),
            continue53_optional_chain_skeleton("o", "x")
        );
        assert_eq!(
            continue119_optional_chain_pretty("a", "b"),
            continue119_optional_chain_minify("a", "b")
        );
        assert_eq!(continue119_o_optional_x(), "o?.x");
    }

    #[test]
    fn continue119_composed_residual_shells() {
        assert_eq!(
            continue119_sequence_then_update_prefix("a", "b", "++", "i"),
            "a, b ++i"
        );
        assert_eq!(continue119_yield_then_await("x", "p"), "yield x await p");
        assert_eq!(
            continue119_yield_star_then_import("xs", "'./m'"),
            "yield* xs import('./m')"
        );
        assert_eq!(
            continue119_chain_then_sequence("o", "x", "a", "b"),
            "o?.x a, b"
        );
        assert_eq!(continue119_sep(true), " ");
        assert_eq!(continue119_sep(false), "");
        assert_ne!(continue119_sep(true), continue119_sep(false));
    }
}

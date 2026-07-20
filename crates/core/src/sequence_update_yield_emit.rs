//! Pure SequenceExpression + UpdateExpression + YieldExpression +
//! AwaitExpression dual-oracle emission — residual pure **continue93** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–92:
//! - SequenceExpression dual-oracle composing real
//!   `continue26_sequence_two_skeleton`
//! - UpdateExpression dual-oracle composing real
//!   `continue26_update_prefix_skeleton` /
//!   `continue26_update_postfix_skeleton`
//! - YieldExpression dual-oracle composing real `continue26_yield_skeleton`
//! - AwaitExpression dual-oracle composing real `continue26_await_skeleton`
//! - Residual one-expr TemplateLiteral / TaggedTemplate shells composing real
//!   `continue25_template_literal_one_expr_skeleton` /
//!   `continue25_tagged_template_one_expr_skeleton` (bridges continue92 no-expr
//!   poles and continue75 multi-quasi poles without re-wrapping either)
//! - Composed sequence + update + yield/await residual shells
//!
//! Intentionally does **not** re-wrap continue76 UpdateExpression pretty/minify
//! poles, continue78 SequenceExpression list sep, continue83 yield/meta/await
//! full dual-oracle, or continue92 no-expr template/array/object surfaces.
//! Composes real shipped pure helpers from continue25/26 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue25_tagged_template_one_expr_skeleton, continue25_template_literal_one_expr_skeleton,
    continue26_await_skeleton, continue26_sequence_two_skeleton,
    continue26_update_postfix_skeleton, continue26_update_prefix_skeleton,
    continue26_yield_skeleton,
};

/// Dual-oracle residual: continue93 related AST type catalog.
pub const CONTINUE93_RELATED_TYPES: &[&str] = &[
    "SequenceExpression",
    "UpdateExpression",
    "YieldExpression",
    "AwaitExpression",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_sequence_update_yield_related_type(t: &str) -> bool {
    CONTINUE93_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue93_sequence_expression_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue93_update_expression_type(t: &str) -> bool {
    t == "UpdateExpression"
}

#[must_use]
pub fn is_continue93_yield_expression_type(t: &str) -> bool {
    t == "YieldExpression"
}

#[must_use]
pub fn is_continue93_await_expression_type(t: &str) -> bool {
    t == "AwaitExpression"
}

// ── SequenceExpression dual-oracle ──────────────────────────────────────────

/// Dual-oracle SequenceExpression two-part skeleton composing real
/// [`continue26_sequence_two_skeleton`].
#[must_use]
pub fn continue93_sequence_expression_skeleton(left: &str, right: &str) -> String {
    continue26_sequence_two_skeleton(left, right)
}

/// Dual-oracle SequenceExpression pretty alias.
#[must_use]
pub fn sequence_expression_pretty(left: &str, right: &str) -> String {
    continue93_sequence_expression_skeleton(left, right)
}

/// Dual-oracle SequenceExpression minify alias (base is space-after-comma).
#[must_use]
pub fn sequence_expression_minify(left: &str, right: &str) -> String {
    continue93_sequence_expression_skeleton(left, right)
}

// ── UpdateExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle UpdateExpression prefix skeleton composing real
/// [`continue26_update_prefix_skeleton`].
#[must_use]
pub fn continue93_update_prefix_skeleton(op: &str, arg: &str) -> String {
    continue26_update_prefix_skeleton(op, arg)
}

/// Dual-oracle UpdateExpression postfix skeleton composing real
/// [`continue26_update_postfix_skeleton`].
#[must_use]
pub fn continue93_update_postfix_skeleton(arg: &str, op: &str) -> String {
    continue26_update_postfix_skeleton(arg, op)
}

/// Dual-oracle UpdateExpression unified skeleton.
#[must_use]
pub fn continue93_update_expression_skeleton(arg: &str, op: &str, prefix: bool) -> String {
    if prefix {
        continue93_update_prefix_skeleton(op, arg)
    } else {
        continue93_update_postfix_skeleton(arg, op)
    }
}

/// Dual-oracle UpdateExpression pretty alias.
#[must_use]
pub fn update_expression_pretty(arg: &str, op: &str, prefix: bool) -> String {
    continue93_update_expression_skeleton(arg, op, prefix)
}

/// Dual-oracle UpdateExpression minify alias.
#[must_use]
pub fn update_expression_minify(arg: &str, op: &str, prefix: bool) -> String {
    continue93_update_expression_skeleton(arg, op, prefix)
}

// ── YieldExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle YieldExpression skeleton composing real
/// [`continue26_yield_skeleton`].
#[must_use]
pub fn continue93_yield_expression_skeleton(delegate: bool, arg: Option<&str>) -> String {
    continue26_yield_skeleton(delegate, arg)
}

/// Dual-oracle bare `yield`.
#[must_use]
pub fn continue93_yield_bare() -> String {
    continue93_yield_expression_skeleton(false, None)
}

/// Dual-oracle `yield arg`.
#[must_use]
pub fn continue93_yield_arg(arg: &str) -> String {
    continue93_yield_expression_skeleton(false, Some(arg))
}

/// Dual-oracle `yield* arg`.
#[must_use]
pub fn continue93_yield_delegate(arg: &str) -> String {
    continue93_yield_expression_skeleton(true, Some(arg))
}

/// Dual-oracle YieldExpression pretty alias.
#[must_use]
pub fn yield_expression_pretty(delegate: bool, arg: Option<&str>) -> String {
    continue93_yield_expression_skeleton(delegate, arg)
}

/// Dual-oracle YieldExpression minify alias.
#[must_use]
pub fn yield_expression_minify(delegate: bool, arg: Option<&str>) -> String {
    continue93_yield_expression_skeleton(delegate, arg)
}

// ── AwaitExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle AwaitExpression skeleton composing real
/// [`continue26_await_skeleton`].
#[must_use]
pub fn continue93_await_expression_skeleton(arg: &str) -> String {
    continue26_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression pretty alias.
#[must_use]
pub fn await_expression_pretty(arg: &str) -> String {
    continue93_await_expression_skeleton(arg)
}

/// Dual-oracle AwaitExpression minify alias.
#[must_use]
pub fn await_expression_minify(arg: &str) -> String {
    continue93_await_expression_skeleton(arg)
}

// ── One-expr template residual shells (continue25 bridge) ───────────────────

/// Dual-oracle residual: TemplateLiteral one-expr shell composing real
/// [`continue25_template_literal_one_expr_skeleton`].
#[must_use]
pub fn continue93_template_literal_one_expr(
    cooked_head: &str,
    expr: &str,
    cooked_tail: &str,
) -> String {
    continue25_template_literal_one_expr_skeleton(cooked_head, expr, cooked_tail)
}

/// Dual-oracle residual: TaggedTemplate one-expr shell composing real
/// [`continue25_tagged_template_one_expr_skeleton`].
#[must_use]
pub fn continue93_tagged_template_one_expr(
    tag: &str,
    cooked_head: &str,
    expr: &str,
    cooked_tail: &str,
) -> String {
    continue25_tagged_template_one_expr_skeleton(tag, cooked_head, expr, cooked_tail)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: sequence of update then yield.
#[must_use]
pub fn continue93_update_then_yield(arg: &str, op: &str, prefix: bool) -> String {
    let u = continue93_update_expression_skeleton(arg, op, prefix);
    let y = continue93_yield_arg(arg);
    continue93_sequence_expression_skeleton(&u, &y)
}

/// Dual-oracle residual: await of a yield argument.
#[must_use]
pub fn continue93_await_yield(arg: &str) -> String {
    let y = continue93_yield_arg(arg);
    continue93_await_expression_skeleton(&y)
}

/// Dual-oracle residual: await of postfix update.
#[must_use]
pub fn continue93_await_update(arg: &str, op: &str) -> String {
    let u = continue93_update_postfix_skeleton(arg, op);
    continue93_await_expression_skeleton(&u)
}

/// Dual-oracle residual: sequence of await + bare yield.
#[must_use]
pub fn continue93_await_then_yield_bare(arg: &str) -> String {
    let a = continue93_await_expression_skeleton(arg);
    let y = continue93_yield_bare();
    continue93_sequence_expression_skeleton(&a, &y)
}

/// Dual-oracle residual: sequence separator pole (pretty vs tight).
#[must_use]
pub fn continue93_sequence_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Dual-oracle residual: join N sequence parts with sep pole.
#[must_use]
pub fn continue93_join_sequence(parts: &[&str], pretty: bool) -> String {
    let sep = continue93_sequence_sep(pretty);
    parts.join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue25_tagged_template_one_expr_skeleton, continue25_template_literal_one_expr_skeleton,
        continue26_await_skeleton, continue26_sequence_two_skeleton,
        continue26_update_postfix_skeleton, continue26_update_prefix_skeleton,
        continue26_yield_skeleton,
    };

    #[test]
    fn continue93_type_catalog() {
        assert_eq!(CONTINUE93_RELATED_TYPES.len(), 4);
        assert!(is_sequence_update_yield_related_type("SequenceExpression"));
        assert!(is_sequence_update_yield_related_type("UpdateExpression"));
        assert!(is_sequence_update_yield_related_type("YieldExpression"));
        assert!(is_sequence_update_yield_related_type("AwaitExpression"));
        assert!(!is_sequence_update_yield_related_type("TemplateLiteral"));
        assert!(!is_sequence_update_yield_related_type("AssignmentExpression"));
        assert!(!is_sequence_update_yield_related_type("BinaryExpression"));
        assert!(!is_sequence_update_yield_related_type("LogicalExpression"));
        assert!(is_continue93_sequence_expression_type("SequenceExpression"));
        assert!(is_continue93_update_expression_type("UpdateExpression"));
        assert!(is_continue93_yield_expression_type("YieldExpression"));
        assert!(is_continue93_await_expression_type("AwaitExpression"));
        assert!(!is_continue93_sequence_expression_type("UpdateExpression"));
        assert!(!is_continue93_await_expression_type("YieldExpression"));
    }

    #[test]
    fn continue93_sequence_update_dual_oracle() {
        assert_eq!(continue93_sequence_expression_skeleton("a", "b"), "a, b");
        assert_eq!(
            continue93_sequence_expression_skeleton("a", "b"),
            continue26_sequence_two_skeleton("a", "b")
        );
        assert_eq!(
            sequence_expression_pretty("x", "y"),
            sequence_expression_minify("x", "y")
        );
        assert_ne!(
            sequence_expression_pretty("a", "b"),
            sequence_expression_pretty("a", "c")
        );

        assert_eq!(continue93_update_prefix_skeleton("++", "i"), "++i");
        assert_eq!(
            continue93_update_prefix_skeleton("++", "i"),
            continue26_update_prefix_skeleton("++", "i")
        );
        assert_eq!(continue93_update_postfix_skeleton("i", "--"), "i--");
        assert_eq!(
            continue93_update_postfix_skeleton("i", "--"),
            continue26_update_postfix_skeleton("i", "--")
        );
        assert_eq!(
            continue93_update_expression_skeleton("n", "++", true),
            "++n"
        );
        assert_eq!(
            continue93_update_expression_skeleton("n", "++", false),
            "n++"
        );
        assert_eq!(
            update_expression_pretty("x", "--", true),
            update_expression_minify("x", "--", true)
        );
        assert_ne!(
            update_expression_pretty("x", "++", true),
            update_expression_pretty("x", "++", false)
        );
    }

    #[test]
    fn continue93_yield_await_template_dual_oracle() {
        assert_eq!(continue93_yield_bare(), "yield");
        assert_eq!(continue93_yield_arg("v"), "yield v");
        assert_eq!(continue93_yield_delegate("g"), "yield* g");
        assert_eq!(
            continue93_yield_expression_skeleton(false, Some("v")),
            continue26_yield_skeleton(false, Some("v"))
        );
        assert_eq!(
            yield_expression_pretty(true, Some("x")),
            yield_expression_minify(true, Some("x"))
        );
        assert_eq!(
            yield_expression_pretty(true, Some("x")),
            "yield* x"
        );

        assert_eq!(continue93_await_expression_skeleton("p"), "await p");
        assert_eq!(
            continue93_await_expression_skeleton("p"),
            continue26_await_skeleton("p")
        );
        assert_eq!(
            await_expression_pretty("q"),
            await_expression_minify("q")
        );
        assert_ne!(
            await_expression_pretty("a"),
            await_expression_pretty("b")
        );

        assert_eq!(
            continue93_template_literal_one_expr("pre", "x", "post"),
            "`pre${x}post`"
        );
        assert_eq!(
            continue93_template_literal_one_expr("pre", "x", "post"),
            continue25_template_literal_one_expr_skeleton("pre", "x", "post")
        );
        assert_eq!(
            continue93_tagged_template_one_expr("tag", "a", "e", "b"),
            "tag`a${e}b`"
        );
        assert_eq!(
            continue93_tagged_template_one_expr("tag", "a", "e", "b"),
            continue25_tagged_template_one_expr_skeleton("tag", "a", "e", "b")
        );

        assert_eq!(
            continue93_update_then_yield("i", "++", true),
            "++i, yield i"
        );
        assert_eq!(continue93_await_yield("v"), "await yield v");
        assert_eq!(continue93_await_update("i", "++"), "await i++");
        assert_eq!(
            continue93_await_then_yield_bare("p"),
            "await p, yield"
        );
        assert_eq!(continue93_sequence_sep(true), ", ");
        assert_eq!(continue93_sequence_sep(false), ",");
        assert_eq!(
            continue93_join_sequence(&["a", "b", "c"], true),
            "a, b, c"
        );
        assert_eq!(
            continue93_join_sequence(&["a", "b", "c"], false),
            "a,b,c"
        );
        assert_ne!(
            continue93_join_sequence(&["a", "b"], true),
            continue93_join_sequence(&["a", "b"], false)
        );
    }
}

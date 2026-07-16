//! Pure ThisExpression + Super + ConditionalExpression + LogicalExpression +
//! DoWhileStatement + ForIn/Of + Break/Continue/Labeled dual-oracle emission —
//! residual pure **continue94** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–93:
//! - ThisExpression dual-oracle composing real `continue27_this_skeleton`
//! - Super dual-oracle composing real `continue27_super_skeleton`
//! - ConditionalExpression dual-oracle composing real
//!   `continue27_conditional_skeleton`
//! - LogicalExpression dual-oracle composing real `continue27_logical_skeleton`
//! - DoWhileStatement dual-oracle composing real `continue27_do_while_skeleton`
//! - ForInStatement dual-oracle composing real `continue27_for_in_skeleton`
//! - ForOfStatement (+ await) dual-oracle composing real
//!   `continue27_for_of_skeleton`
//! - BreakStatement / ContinueStatement / LabeledStatement residual shells
//!   composing real continue27 break/continue/labeled skeletons
//! - Composed this/super + conditional/logical + loop residual shells
//!
//! Intentionally does **not** re-wrap continue76 LogicalExpression pretty/minify
//! poles, continue86 ThisExpression/Super token poles, continue80 ForIn/Of/
//! DoWhile full dual-oracle, continue74 Conditional/ForOf, or continue93
//! sequence/update/yield surfaces. Composes real shipped pure helpers from
//! continue27 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue27_break_skeleton, continue27_conditional_skeleton, continue27_continue_skeleton,
    continue27_do_while_skeleton, continue27_for_in_skeleton, continue27_for_of_skeleton,
    continue27_labeled_skeleton, continue27_logical_skeleton, continue27_super_skeleton,
    continue27_this_skeleton,
};

/// Dual-oracle residual: continue94 related AST type catalog.
pub const CONTINUE94_RELATED_TYPES: &[&str] = &[
    "ThisExpression",
    "Super",
    "ConditionalExpression",
    "LogicalExpression",
    "DoWhileStatement",
    "ForInStatement",
    "ForOfStatement",
    "BreakStatement",
    "ContinueStatement",
    "LabeledStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_this_conditional_logical_related_type(t: &str) -> bool {
    CONTINUE94_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue94_this_expression_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue94_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue94_conditional_expression_type(t: &str) -> bool {
    t == "ConditionalExpression"
}

#[must_use]
pub fn is_continue94_logical_expression_type(t: &str) -> bool {
    t == "LogicalExpression"
}

#[must_use]
pub fn is_continue94_do_while_type(t: &str) -> bool {
    t == "DoWhileStatement"
}

#[must_use]
pub fn is_continue94_for_in_type(t: &str) -> bool {
    t == "ForInStatement"
}

#[must_use]
pub fn is_continue94_for_of_type(t: &str) -> bool {
    t == "ForOfStatement"
}

#[must_use]
pub fn is_continue94_break_type(t: &str) -> bool {
    t == "BreakStatement"
}

#[must_use]
pub fn is_continue94_continue_type(t: &str) -> bool {
    t == "ContinueStatement"
}

#[must_use]
pub fn is_continue94_labeled_type(t: &str) -> bool {
    t == "LabeledStatement"
}

// ── ThisExpression / Super dual-oracle ──────────────────────────────────────

/// Dual-oracle ThisExpression skeleton composing real [`continue27_this_skeleton`].
#[must_use]
pub fn continue94_this_expression_skeleton() -> &'static str {
    continue27_this_skeleton()
}

/// Dual-oracle ThisExpression pretty alias.
#[must_use]
pub fn this_expression_pretty() -> &'static str {
    continue94_this_expression_skeleton()
}

/// Dual-oracle ThisExpression minify alias.
#[must_use]
pub fn this_expression_minify() -> &'static str {
    continue94_this_expression_skeleton()
}

/// Dual-oracle Super skeleton composing real [`continue27_super_skeleton`].
#[must_use]
pub fn continue94_super_skeleton() -> &'static str {
    continue27_super_skeleton()
}

/// Dual-oracle Super pretty alias.
#[must_use]
pub fn super_pretty() -> &'static str {
    continue94_super_skeleton()
}

/// Dual-oracle Super minify alias.
#[must_use]
pub fn super_minify() -> &'static str {
    continue94_super_skeleton()
}

// ── ConditionalExpression dual-oracle ───────────────────────────────────────

/// Dual-oracle ConditionalExpression skeleton composing real
/// [`continue27_conditional_skeleton`].
#[must_use]
pub fn continue94_conditional_expression_skeleton(
    test: &str,
    consequent: &str,
    alternate: &str,
) -> String {
    continue27_conditional_skeleton(test, consequent, alternate)
}

/// Dual-oracle ConditionalExpression pretty alias.
#[must_use]
pub fn conditional_expression_pretty(
    test: &str,
    consequent: &str,
    alternate: &str,
) -> String {
    continue94_conditional_expression_skeleton(test, consequent, alternate)
}

/// Dual-oracle ConditionalExpression minify alias (base is space-padded).
#[must_use]
pub fn conditional_expression_minify(
    test: &str,
    consequent: &str,
    alternate: &str,
) -> String {
    continue94_conditional_expression_skeleton(test, consequent, alternate)
}

/// Dual-oracle ConditionalExpression tight residual (no spaces around `?`/`:`).
#[must_use]
pub fn continue94_conditional_tight(test: &str, consequent: &str, alternate: &str) -> String {
    format!("{test}?{consequent}:{alternate}")
}

// ── LogicalExpression dual-oracle ───────────────────────────────────────────

/// Dual-oracle LogicalExpression skeleton composing real
/// [`continue27_logical_skeleton`].
#[must_use]
pub fn continue94_logical_expression_skeleton(left: &str, op: &str, right: &str) -> String {
    continue27_logical_skeleton(left, op, right)
}

/// Dual-oracle LogicalExpression pretty alias.
#[must_use]
pub fn logical_expression_pretty(left: &str, op: &str, right: &str) -> String {
    continue94_logical_expression_skeleton(left, op, right)
}

/// Dual-oracle LogicalExpression minify alias (base is space-padded).
#[must_use]
pub fn logical_expression_minify(left: &str, op: &str, right: &str) -> String {
    continue94_logical_expression_skeleton(left, op, right)
}

/// Dual-oracle LogicalExpression tight residual.
#[must_use]
pub fn continue94_logical_tight(left: &str, op: &str, right: &str) -> String {
    format!("{left}{op}{right}")
}

// ── DoWhile / ForIn / ForOf dual-oracle ──────────────────────────────────────

/// Dual-oracle DoWhileStatement skeleton composing real
/// [`continue27_do_while_skeleton`].
#[must_use]
pub fn continue94_do_while_statement_skeleton(body: &str, test: &str) -> String {
    continue27_do_while_skeleton(body, test)
}

/// Dual-oracle DoWhileStatement pretty alias.
#[must_use]
pub fn do_while_statement_pretty(body: &str, test: &str) -> String {
    continue94_do_while_statement_skeleton(body, test)
}

/// Dual-oracle DoWhileStatement minify alias.
#[must_use]
pub fn do_while_statement_minify(body: &str, test: &str) -> String {
    continue94_do_while_statement_skeleton(body, test)
}

/// Dual-oracle ForInStatement skeleton composing real [`continue27_for_in_skeleton`].
#[must_use]
pub fn continue94_for_in_statement_skeleton(left: &str, right: &str, body: &str) -> String {
    continue27_for_in_skeleton(left, right, body)
}

/// Dual-oracle ForInStatement pretty alias.
#[must_use]
pub fn for_in_statement_pretty(left: &str, right: &str, body: &str) -> String {
    continue94_for_in_statement_skeleton(left, right, body)
}

/// Dual-oracle ForInStatement minify alias.
#[must_use]
pub fn for_in_statement_minify(left: &str, right: &str, body: &str) -> String {
    continue94_for_in_statement_skeleton(left, right, body)
}

/// Dual-oracle ForOfStatement skeleton composing real [`continue27_for_of_skeleton`].
#[must_use]
pub fn continue94_for_of_statement_skeleton(
    left: &str,
    right: &str,
    body: &str,
    awaited: bool,
) -> String {
    continue27_for_of_skeleton(left, right, body, awaited)
}

/// Dual-oracle ForOfStatement pretty alias.
#[must_use]
pub fn for_of_statement_pretty(left: &str, right: &str, body: &str, awaited: bool) -> String {
    continue94_for_of_statement_skeleton(left, right, body, awaited)
}

/// Dual-oracle ForOfStatement minify alias.
#[must_use]
pub fn for_of_statement_minify(left: &str, right: &str, body: &str, awaited: bool) -> String {
    continue94_for_of_statement_skeleton(left, right, body, awaited)
}

// ── Break / Continue / Labeled residual shells ──────────────────────────────

/// Dual-oracle BreakStatement skeleton composing real [`continue27_break_skeleton`].
#[must_use]
pub fn continue94_break_statement_skeleton(label: Option<&str>) -> String {
    continue27_break_skeleton(label)
}

/// Dual-oracle bare `break`.
#[must_use]
pub fn continue94_break_bare() -> String {
    continue94_break_statement_skeleton(None)
}

/// Dual-oracle `break label`.
#[must_use]
pub fn continue94_break_labeled(label: &str) -> String {
    continue94_break_statement_skeleton(Some(label))
}

/// Dual-oracle ContinueStatement skeleton composing real
/// [`continue27_continue_skeleton`].
#[must_use]
pub fn continue94_continue_statement_skeleton(label: Option<&str>) -> String {
    continue27_continue_skeleton(label)
}

/// Dual-oracle bare `continue`.
#[must_use]
pub fn continue94_continue_bare() -> String {
    continue94_continue_statement_skeleton(None)
}

/// Dual-oracle `continue label`.
#[must_use]
pub fn continue94_continue_labeled(label: &str) -> String {
    continue94_continue_statement_skeleton(Some(label))
}

/// Dual-oracle LabeledStatement skeleton composing real
/// [`continue27_labeled_skeleton`].
#[must_use]
pub fn continue94_labeled_statement_skeleton(label: &str, body: &str) -> String {
    continue27_labeled_skeleton(label, body)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: conditional of this vs super.
#[must_use]
pub fn continue94_this_or_super(test: &str) -> String {
    continue94_conditional_expression_skeleton(
        test,
        continue94_this_expression_skeleton(),
        continue94_super_skeleton(),
    )
}

/// Dual-oracle residual: logical of this && super.
#[must_use]
pub fn continue94_this_and_super() -> String {
    continue94_logical_expression_skeleton(
        continue94_this_expression_skeleton(),
        "&&",
        continue94_super_skeleton(),
    )
}

/// Dual-oracle residual: do-while with this test.
#[must_use]
pub fn continue94_do_while_this(body: &str) -> String {
    continue94_do_while_statement_skeleton(body, continue94_this_expression_skeleton())
}

/// Dual-oracle residual: labeled for-of of this.
#[must_use]
pub fn continue94_labeled_for_of(
    label: &str,
    left: &str,
    right: &str,
    body: &str,
    awaited: bool,
) -> String {
    let loop_body = continue94_for_of_statement_skeleton(left, right, body, awaited);
    continue94_labeled_statement_skeleton(label, &loop_body)
}

/// Dual-oracle residual: for-in body with break.
#[must_use]
pub fn continue94_for_in_break(left: &str, right: &str) -> String {
    continue94_for_in_statement_skeleton(left, right, &continue94_break_bare())
}

/// Dual-oracle residual: for-of body with continue label.
#[must_use]
pub fn continue94_for_of_continue_labeled(
    left: &str,
    right: &str,
    cont_label: &str,
    awaited: bool,
) -> String {
    let cont = continue94_continue_labeled(cont_label);
    continue94_for_of_statement_skeleton(left, right, &cont, awaited)
}

/// Dual-oracle residual: logical sep pole (pretty vs tight).
#[must_use]
pub fn continue94_logical_sep(pretty: bool) -> &'static str {
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
        continue27_break_skeleton, continue27_conditional_skeleton, continue27_continue_skeleton,
        continue27_do_while_skeleton, continue27_for_in_skeleton, continue27_for_of_skeleton,
        continue27_labeled_skeleton, continue27_logical_skeleton, continue27_super_skeleton,
        continue27_this_skeleton,
    };

    #[test]
    fn continue94_type_catalog() {
        assert_eq!(CONTINUE94_RELATED_TYPES.len(), 10);
        assert!(is_this_conditional_logical_related_type("ThisExpression"));
        assert!(is_this_conditional_logical_related_type("Super"));
        assert!(is_this_conditional_logical_related_type("ConditionalExpression"));
        assert!(is_this_conditional_logical_related_type("LogicalExpression"));
        assert!(is_this_conditional_logical_related_type("DoWhileStatement"));
        assert!(is_this_conditional_logical_related_type("ForInStatement"));
        assert!(is_this_conditional_logical_related_type("ForOfStatement"));
        assert!(is_this_conditional_logical_related_type("BreakStatement"));
        assert!(is_this_conditional_logical_related_type("ContinueStatement"));
        assert!(is_this_conditional_logical_related_type("LabeledStatement"));
        assert!(!is_this_conditional_logical_related_type("SequenceExpression"));
        assert!(!is_this_conditional_logical_related_type("UpdateExpression"));
        assert!(!is_this_conditional_logical_related_type("YieldExpression"));
        assert!(!is_this_conditional_logical_related_type("BinaryExpression"));
        assert!(is_continue94_this_expression_type("ThisExpression"));
        assert!(is_continue94_super_type("Super"));
        assert!(is_continue94_conditional_expression_type("ConditionalExpression"));
        assert!(is_continue94_logical_expression_type("LogicalExpression"));
        assert!(is_continue94_do_while_type("DoWhileStatement"));
        assert!(is_continue94_for_in_type("ForInStatement"));
        assert!(is_continue94_for_of_type("ForOfStatement"));
        assert!(is_continue94_break_type("BreakStatement"));
        assert!(is_continue94_continue_type("ContinueStatement"));
        assert!(is_continue94_labeled_type("LabeledStatement"));
        assert!(!is_continue94_this_expression_type("Super"));
        assert!(!is_continue94_for_of_type("ForInStatement"));
    }

    #[test]
    fn continue94_this_super_conditional_logical_dual_oracle() {
        assert_eq!(continue94_this_expression_skeleton(), "this");
        assert_eq!(
            continue94_this_expression_skeleton(),
            continue27_this_skeleton()
        );
        assert_eq!(this_expression_pretty(), this_expression_minify());
        assert_eq!(this_expression_pretty(), "this");

        assert_eq!(continue94_super_skeleton(), "super");
        assert_eq!(continue94_super_skeleton(), continue27_super_skeleton());
        assert_eq!(super_pretty(), super_minify());
        assert_ne!(this_expression_pretty(), super_pretty());

        assert_eq!(
            continue94_conditional_expression_skeleton("a", "b", "c"),
            "a ? b : c"
        );
        assert_eq!(
            continue94_conditional_expression_skeleton("a", "b", "c"),
            continue27_conditional_skeleton("a", "b", "c")
        );
        assert_eq!(
            conditional_expression_pretty("x", "y", "z"),
            conditional_expression_minify("x", "y", "z")
        );
        assert_eq!(
            continue94_conditional_tight("a", "b", "c"),
            "a?b:c"
        );
        assert_ne!(
            continue94_conditional_expression_skeleton("a", "b", "c"),
            continue94_conditional_tight("a", "b", "c")
        );

        assert_eq!(
            continue94_logical_expression_skeleton("a", "&&", "b"),
            "a && b"
        );
        assert_eq!(
            continue94_logical_expression_skeleton("a", "||", "b"),
            continue27_logical_skeleton("a", "||", "b")
        );
        assert_eq!(
            logical_expression_pretty("p", "??", "q"),
            logical_expression_minify("p", "??", "q")
        );
        assert_eq!(continue94_logical_tight("a", "&&", "b"), "a&&b");
        assert_ne!(
            logical_expression_pretty("a", "&&", "b"),
            continue94_logical_tight("a", "&&", "b")
        );
    }

    #[test]
    fn continue94_loops_jumps_composed_dual_oracle() {
        assert_eq!(
            continue94_do_while_statement_skeleton("{}", "x"),
            "do {} while (x)"
        );
        assert_eq!(
            continue94_do_while_statement_skeleton("{}", "x"),
            continue27_do_while_skeleton("{}", "x")
        );
        assert_eq!(
            do_while_statement_pretty("{}", "t"),
            do_while_statement_minify("{}", "t")
        );

        assert_eq!(
            continue94_for_in_statement_skeleton("k", "obj", "{}"),
            "for (k in obj) {}"
        );
        assert_eq!(
            continue94_for_in_statement_skeleton("k", "obj", "{}"),
            continue27_for_in_skeleton("k", "obj", "{}")
        );
        assert_eq!(
            for_in_statement_pretty("k", "o", "{}"),
            for_in_statement_minify("k", "o", "{}")
        );

        assert_eq!(
            continue94_for_of_statement_skeleton("v", "arr", "{}", false),
            "for (v of arr) {}"
        );
        assert_eq!(
            continue94_for_of_statement_skeleton("v", "arr", "{}", true),
            "for await (v of arr) {}"
        );
        assert_eq!(
            continue94_for_of_statement_skeleton("v", "arr", "{}", true),
            continue27_for_of_skeleton("v", "arr", "{}", true)
        );
        assert_eq!(
            for_of_statement_pretty("v", "xs", "{}", false),
            for_of_statement_minify("v", "xs", "{}", false)
        );
        assert_ne!(
            for_of_statement_pretty("v", "xs", "{}", false),
            for_of_statement_pretty("v", "xs", "{}", true)
        );

        assert_eq!(continue94_break_bare(), "break");
        assert_eq!(continue94_break_labeled("outer"), "break outer");
        assert_eq!(
            continue94_break_statement_skeleton(Some("L")),
            continue27_break_skeleton(Some("L"))
        );
        assert_eq!(continue94_continue_bare(), "continue");
        assert_eq!(continue94_continue_labeled("outer"), "continue outer");
        assert_eq!(
            continue94_continue_statement_skeleton(None),
            continue27_continue_skeleton(None)
        );
        assert_eq!(
            continue94_labeled_statement_skeleton("L", "break"),
            "L: break"
        );
        assert_eq!(
            continue94_labeled_statement_skeleton("L", "break"),
            continue27_labeled_skeleton("L", "break")
        );

        assert_eq!(continue94_this_or_super("flag"), "flag ? this : super");
        assert_eq!(continue94_this_and_super(), "this && super");
        assert_eq!(
            continue94_do_while_this("{}"),
            "do {} while (this)"
        );
        assert_eq!(
            continue94_labeled_for_of("loop", "x", "xs", "{}", false),
            "loop: for (x of xs) {}"
        );
        assert_eq!(
            continue94_for_in_break("k", "o"),
            "for (k in o) break"
        );
        assert_eq!(
            continue94_for_of_continue_labeled("v", "xs", "loop", false),
            "for (v of xs) continue loop"
        );
        assert_eq!(continue94_logical_sep(true), " ");
        assert_eq!(continue94_logical_sep(false), "");
        assert_ne!(
            continue94_logical_sep(true),
            continue94_logical_sep(false)
        );
    }
}

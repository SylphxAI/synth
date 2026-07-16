//! Pure IfStatement + WhileStatement + DoWhileStatement +
//! ConditionalExpression + LabeledStatement + WithStatement dual-oracle
//! emission — residual pure **continue102** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–101:
//! - IfStatement dual-oracle composing real `continue35_if_skeleton`
//! - IfStatement else dual-oracle composing real `continue35_if_else_skeleton`
//! - WhileStatement dual-oracle composing real `continue35_while_skeleton`
//! - DoWhileStatement dual-oracle composing real `continue35_do_while_skeleton`
//! - ConditionalExpression dual-oracle composing real
//!   `continue35_conditional_skeleton`
//! - LabeledStatement dual-oracle composing real `continue35_labeled_skeleton`
//! - WithStatement dual-oracle composing real `continue35_with_skeleton`
//! - Composed if/while/do/conditional/labeled/with residual shells
//!
//! Intentionally does **not** re-wrap continue95 if/while/return/throw poles,
//! continue94 this/conditional/logical poles, continue80 while/do pretty-minify
//! poles, or continue101 for/var/assignment poles. Composes real shipped pure
//! helpers from continue35 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue35_conditional_skeleton, continue35_do_while_skeleton, continue35_if_else_skeleton,
    continue35_if_skeleton, continue35_labeled_skeleton, continue35_while_skeleton,
    continue35_with_skeleton,
};

/// Dual-oracle residual: continue102 related AST type catalog.
pub const CONTINUE102_RELATED_TYPES: &[&str] = &[
    "IfStatement",
    "WhileStatement",
    "DoWhileStatement",
    "ConditionalExpression",
    "LabeledStatement",
    "WithStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_if_while_do_conditional_labeled_with_related_type(t: &str) -> bool {
    CONTINUE102_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue102_if_type(t: &str) -> bool {
    t == "IfStatement"
}

#[must_use]
pub fn is_continue102_while_type(t: &str) -> bool {
    t == "WhileStatement"
}

#[must_use]
pub fn is_continue102_do_while_type(t: &str) -> bool {
    t == "DoWhileStatement"
}

#[must_use]
pub fn is_continue102_conditional_type(t: &str) -> bool {
    t == "ConditionalExpression"
}

#[must_use]
pub fn is_continue102_labeled_type(t: &str) -> bool {
    t == "LabeledStatement"
}

#[must_use]
pub fn is_continue102_with_type(t: &str) -> bool {
    t == "WithStatement"
}

#[must_use]
pub fn is_continue102_while_family_type(t: &str) -> bool {
    matches!(t, "WhileStatement" | "DoWhileStatement")
}

#[must_use]
pub fn is_continue102_control_flow_type(t: &str) -> bool {
    matches!(
        t,
        "IfStatement" | "WhileStatement" | "DoWhileStatement" | "LabeledStatement" | "WithStatement"
    )
}

// ── IfStatement dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle IfStatement skeleton composing real [`continue35_if_skeleton`].
#[must_use]
pub fn continue102_if_skeleton(test: &str, body: &str) -> String {
    continue35_if_skeleton(test, body)
}

/// Dual-oracle IfStatement pretty alias.
#[must_use]
pub fn continue102_if_pretty(test: &str, body: &str) -> String {
    continue102_if_skeleton(test, body)
}

/// Dual-oracle IfStatement minify alias.
#[must_use]
pub fn continue102_if_minify(test: &str, body: &str) -> String {
    continue102_if_skeleton(test, body)
}

/// Dual-oracle IfStatement else skeleton composing real
/// [`continue35_if_else_skeleton`].
#[must_use]
pub fn continue102_if_else_skeleton(test: &str, then_body: &str, else_body: &str) -> String {
    continue35_if_else_skeleton(test, then_body, else_body)
}

/// Dual-oracle IfStatement else pretty alias.
#[must_use]
pub fn continue102_if_else_pretty(test: &str, then_body: &str, else_body: &str) -> String {
    continue102_if_else_skeleton(test, then_body, else_body)
}

/// Dual-oracle IfStatement else minify alias.
#[must_use]
pub fn continue102_if_else_minify(test: &str, then_body: &str, else_body: &str) -> String {
    continue102_if_else_skeleton(test, then_body, else_body)
}

// ── WhileStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle WhileStatement skeleton composing real
/// [`continue35_while_skeleton`].
#[must_use]
pub fn continue102_while_skeleton(test: &str, body: &str) -> String {
    continue35_while_skeleton(test, body)
}

/// Dual-oracle WhileStatement pretty alias.
#[must_use]
pub fn continue102_while_pretty(test: &str, body: &str) -> String {
    continue102_while_skeleton(test, body)
}

/// Dual-oracle WhileStatement minify alias.
#[must_use]
pub fn continue102_while_minify(test: &str, body: &str) -> String {
    continue102_while_skeleton(test, body)
}

// ── DoWhileStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle DoWhileStatement skeleton composing real
/// [`continue35_do_while_skeleton`].
#[must_use]
pub fn continue102_do_while_skeleton(body: &str, test: &str) -> String {
    continue35_do_while_skeleton(body, test)
}

/// Dual-oracle DoWhileStatement pretty alias.
#[must_use]
pub fn continue102_do_while_pretty(body: &str, test: &str) -> String {
    continue102_do_while_skeleton(body, test)
}

/// Dual-oracle DoWhileStatement minify alias.
#[must_use]
pub fn continue102_do_while_minify(body: &str, test: &str) -> String {
    continue102_do_while_skeleton(body, test)
}

// ── ConditionalExpression dual-oracle ───────────────────────────────────────

/// Dual-oracle ConditionalExpression skeleton composing real
/// [`continue35_conditional_skeleton`].
#[must_use]
pub fn continue102_conditional_skeleton(test: &str, consequent: &str, alternate: &str) -> String {
    continue35_conditional_skeleton(test, consequent, alternate)
}

/// Dual-oracle ConditionalExpression pretty alias.
#[must_use]
pub fn continue102_conditional_pretty(test: &str, consequent: &str, alternate: &str) -> String {
    continue102_conditional_skeleton(test, consequent, alternate)
}

/// Dual-oracle ConditionalExpression minify alias.
#[must_use]
pub fn continue102_conditional_minify(test: &str, consequent: &str, alternate: &str) -> String {
    continue102_conditional_skeleton(test, consequent, alternate)
}

// ── LabeledStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle LabeledStatement skeleton composing real
/// [`continue35_labeled_skeleton`].
#[must_use]
pub fn continue102_labeled_skeleton(label: &str, body: &str) -> String {
    continue35_labeled_skeleton(label, body)
}

/// Dual-oracle LabeledStatement pretty alias.
#[must_use]
pub fn continue102_labeled_pretty(label: &str, body: &str) -> String {
    continue102_labeled_skeleton(label, body)
}

/// Dual-oracle LabeledStatement minify alias.
#[must_use]
pub fn continue102_labeled_minify(label: &str, body: &str) -> String {
    continue102_labeled_skeleton(label, body)
}

// ── WithStatement dual-oracle ───────────────────────────────────────────────

/// Dual-oracle WithStatement skeleton composing real
/// [`continue35_with_skeleton`].
#[must_use]
pub fn continue102_with_skeleton(object: &str, body: &str) -> String {
    continue35_with_skeleton(object, body)
}

/// Dual-oracle WithStatement pretty alias.
#[must_use]
pub fn continue102_with_pretty(object: &str, body: &str) -> String {
    continue102_with_skeleton(object, body)
}

/// Dual-oracle WithStatement minify alias.
#[must_use]
pub fn continue102_with_minify(object: &str, body: &str) -> String {
    continue102_with_skeleton(object, body)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: if + else arms with same test.
#[must_use]
pub fn continue102_if_then_else(test: &str, then_stmt: &str, else_stmt: &str) -> String {
    continue102_if_else_skeleton(test, then_stmt, else_stmt)
}

/// Dual-oracle residual: while body wrapping a conditional expression.
#[must_use]
pub fn continue102_while_with_conditional(
    loop_test: &str,
    cond_test: &str,
    consequent: &str,
    alternate: &str,
) -> String {
    let body = continue102_conditional_skeleton(cond_test, consequent, alternate);
    continue102_while_skeleton(loop_test, &body)
}

/// Dual-oracle residual: do-while with labeled body.
#[must_use]
pub fn continue102_do_while_labeled(label: &str, body: &str, test: &str) -> String {
    let labeled = continue102_labeled_skeleton(label, body);
    continue102_do_while_skeleton(&labeled, test)
}

/// Dual-oracle residual: with object then if body.
#[must_use]
pub fn continue102_with_if(object: &str, test: &str, body: &str) -> String {
    let if_stmt = continue102_if_skeleton(test, body);
    continue102_with_skeleton(object, &if_stmt)
}

/// Dual-oracle residual: labeled while loop.
#[must_use]
pub fn continue102_labeled_while(label: &str, test: &str, body: &str) -> String {
    let while_stmt = continue102_while_skeleton(test, body);
    continue102_labeled_skeleton(label, &while_stmt)
}

/// Dual-oracle residual: ternary-guarded if body string compose.
#[must_use]
pub fn continue102_if_guarded_by_conditional(
    cond_test: &str,
    consequent: &str,
    alternate: &str,
    if_body: &str,
) -> String {
    let test = continue102_conditional_skeleton(cond_test, consequent, alternate);
    continue102_if_skeleton(&test, if_body)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue102_stmt_sep(pretty: bool) -> &'static str {
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
        continue35_conditional_skeleton, continue35_do_while_skeleton, continue35_if_else_skeleton,
        continue35_if_skeleton, continue35_labeled_skeleton, continue35_while_skeleton,
        continue35_with_skeleton,
    };

    #[test]
    fn continue102_type_catalog() {
        assert_eq!(CONTINUE102_RELATED_TYPES.len(), 6);
        assert!(is_if_while_do_conditional_labeled_with_related_type(
            "IfStatement"
        ));
        assert!(is_if_while_do_conditional_labeled_with_related_type(
            "WhileStatement"
        ));
        assert!(is_if_while_do_conditional_labeled_with_related_type(
            "DoWhileStatement"
        ));
        assert!(is_if_while_do_conditional_labeled_with_related_type(
            "ConditionalExpression"
        ));
        assert!(is_if_while_do_conditional_labeled_with_related_type(
            "LabeledStatement"
        ));
        assert!(is_if_while_do_conditional_labeled_with_related_type(
            "WithStatement"
        ));
        assert!(!is_if_while_do_conditional_labeled_with_related_type(
            "ForStatement"
        ));
        assert!(!is_if_while_do_conditional_labeled_with_related_type(
            "SwitchStatement"
        ));

        assert!(is_continue102_if_type("IfStatement"));
        assert!(!is_continue102_if_type("WhileStatement"));
        assert!(is_continue102_while_type("WhileStatement"));
        assert!(is_continue102_do_while_type("DoWhileStatement"));
        assert!(is_continue102_conditional_type("ConditionalExpression"));
        assert!(is_continue102_labeled_type("LabeledStatement"));
        assert!(is_continue102_with_type("WithStatement"));
        assert!(is_continue102_while_family_type("WhileStatement"));
        assert!(is_continue102_while_family_type("DoWhileStatement"));
        assert!(!is_continue102_while_family_type("IfStatement"));
        assert!(is_continue102_control_flow_type("IfStatement"));
        assert!(is_continue102_control_flow_type("WithStatement"));
        assert!(!is_continue102_control_flow_type(
            "ConditionalExpression"
        ));
    }

    #[test]
    fn continue102_if_while_do_emit() {
        assert_eq!(
            continue102_if_skeleton("x", "y()"),
            "if (x) { y() }"
        );
        assert_eq!(
            continue102_if_skeleton("x", "y()"),
            continue35_if_skeleton("x", "y()")
        );
        assert_eq!(
            continue102_if_pretty("a", "b"),
            continue102_if_minify("a", "b")
        );

        assert_eq!(
            continue102_if_else_skeleton("x", "a", "b"),
            "if (x) { a } else { b }"
        );
        assert_eq!(
            continue102_if_else_skeleton("x", "a", "b"),
            continue35_if_else_skeleton("x", "a", "b")
        );
        assert_eq!(
            continue102_if_else_pretty("t", "u", "v"),
            continue102_if_else_minify("t", "u", "v")
        );

        assert_eq!(
            continue102_while_skeleton("i < 3", "i++"),
            "while (i < 3) { i++ }"
        );
        assert_eq!(
            continue102_while_skeleton("i < 3", "i++"),
            continue35_while_skeleton("i < 3", "i++")
        );
        assert_eq!(
            continue102_while_pretty("a", "b"),
            continue102_while_minify("a", "b")
        );

        assert_eq!(
            continue102_do_while_skeleton("work()", "ok"),
            "do { work() } while (ok);"
        );
        assert_eq!(
            continue102_do_while_skeleton("work()", "ok"),
            continue35_do_while_skeleton("work()", "ok")
        );
        assert_eq!(
            continue102_do_while_pretty("a", "b"),
            continue102_do_while_minify("a", "b")
        );
    }

    #[test]
    fn continue102_conditional_labeled_with_emit() {
        assert_eq!(
            continue102_conditional_skeleton("c", "a", "b"),
            "c ? a : b"
        );
        assert_eq!(
            continue102_conditional_skeleton("c", "a", "b"),
            continue35_conditional_skeleton("c", "a", "b")
        );
        assert_eq!(
            continue102_conditional_pretty("x", "y", "z"),
            continue102_conditional_minify("x", "y", "z")
        );

        assert_eq!(
            continue102_labeled_skeleton("loop", "break;"),
            "loop: break;"
        );
        assert_eq!(
            continue102_labeled_skeleton("loop", "break;"),
            continue35_labeled_skeleton("loop", "break;")
        );
        assert_eq!(
            continue102_labeled_pretty("L", "s"),
            continue102_labeled_minify("L", "s")
        );

        assert_eq!(
            continue102_with_skeleton("obj", "x = 1"),
            "with (obj) { x = 1 }"
        );
        assert_eq!(
            continue102_with_skeleton("obj", "x = 1"),
            continue35_with_skeleton("obj", "x = 1")
        );
        assert_eq!(
            continue102_with_pretty("o", "b"),
            continue102_with_minify("o", "b")
        );
    }

    #[test]
    fn continue102_composed_residual_shells() {
        assert_eq!(
            continue102_if_then_else("ready", "go()", "wait()"),
            "if (ready) { go() } else { wait() }"
        );
        assert_eq!(
            continue102_while_with_conditional("i < 2", "i", "a", "b"),
            "while (i < 2) { i ? a : b }"
        );
        assert_eq!(
            continue102_do_while_labeled("outer", "step()", "more"),
            "do { outer: step() } while (more);"
        );
        assert_eq!(
            continue102_with_if("ctx", "flag", "use()"),
            "with (ctx) { if (flag) { use() } }"
        );
        assert_eq!(
            continue102_labeled_while("outer", "n > 0", "n--"),
            "outer: while (n > 0) { n-- }"
        );
        assert_eq!(
            continue102_if_guarded_by_conditional("x", "1", "0", "run()"),
            "if (x ? 1 : 0) { run() }"
        );
        assert_eq!(continue102_stmt_sep(true), " ");
        assert_eq!(continue102_stmt_sep(false), "");
        assert_ne!(continue102_stmt_sep(true), continue102_stmt_sep(false));
    }
}

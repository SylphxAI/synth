//! Pure UnaryExpression + BinaryExpression + AwaitExpression + Identifier +
//! BlockStatement + BreakStatement + ContinueStatement + LabeledStatement +
//! ExpressionStatement dual-oracle emission — residual pure **continue98** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–97:
//! - UnaryExpression dual-oracle composing real `continue31_unary_skeleton`
//! - BinaryExpression dual-oracle composing real `continue31_binary_skeleton`
//! - AwaitExpression dual-oracle composing real `continue31_await_skeleton`
//! - Identifier dual-oracle composing real `continue31_identifier_skeleton`
//! - BlockStatement dual-oracle composing real `continue31_block_skeleton`
//! - BreakStatement dual-oracle composing real `continue31_break_skeleton`
//! - ContinueStatement dual-oracle composing real `continue31_continue_skeleton`
//! - LabeledStatement dual-oracle composing real `continue31_labeled_skeleton`
//! - ExpressionStatement dual-oracle composing real
//!   `continue31_expression_stmt_skeleton`
//! - Composed unary/binary/await/jump residual shells
//!
//! Intentionally does **not** re-wrap continue15 assignment/logical/update,
//! continue76–77 call/member poles, continue93 sequence/update/yield,
//! continue94 this/conditional/logical, continue95 if/while/return/throw,
//! continue96 class/function/import/export, or continue97
//! assignment/update/member/call poles. Composes real shipped pure helpers
//! from continue31 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue31_await_skeleton, continue31_binary_skeleton, continue31_block_skeleton,
    continue31_break_skeleton, continue31_continue_skeleton, continue31_expression_stmt_skeleton,
    continue31_identifier_skeleton, continue31_labeled_skeleton, continue31_unary_skeleton,
};

/// Dual-oracle residual: continue98 related AST type catalog.
pub const CONTINUE98_RELATED_TYPES: &[&str] = &[
    "UnaryExpression",
    "BinaryExpression",
    "AwaitExpression",
    "Identifier",
    "BlockStatement",
    "BreakStatement",
    "ContinueStatement",
    "LabeledStatement",
    "ExpressionStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_unary_binary_await_jump_related_type(t: &str) -> bool {
    CONTINUE98_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue98_unary_type(t: &str) -> bool {
    t == "UnaryExpression"
}

#[must_use]
pub fn is_continue98_binary_type(t: &str) -> bool {
    t == "BinaryExpression"
}

#[must_use]
pub fn is_continue98_await_type(t: &str) -> bool {
    t == "AwaitExpression"
}

#[must_use]
pub fn is_continue98_identifier_type(t: &str) -> bool {
    t == "Identifier"
}

#[must_use]
pub fn is_continue98_block_type(t: &str) -> bool {
    t == "BlockStatement"
}

#[must_use]
pub fn is_continue98_break_type(t: &str) -> bool {
    t == "BreakStatement"
}

#[must_use]
pub fn is_continue98_continue_type(t: &str) -> bool {
    t == "ContinueStatement"
}

#[must_use]
pub fn is_continue98_labeled_type(t: &str) -> bool {
    t == "LabeledStatement"
}

#[must_use]
pub fn is_continue98_expression_stmt_type(t: &str) -> bool {
    t == "ExpressionStatement"
}

#[must_use]
pub fn is_continue98_jump_type(t: &str) -> bool {
    matches!(
        t,
        "BreakStatement" | "ContinueStatement" | "LabeledStatement"
    )
}

// ── UnaryExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle UnaryExpression skeleton composing real
/// [`continue31_unary_skeleton`].
#[must_use]
pub fn continue98_unary_skeleton(op: &str, arg: &str, prefix: bool) -> String {
    continue31_unary_skeleton(op, arg, prefix)
}

/// Dual-oracle UnaryExpression prefix alias.
#[must_use]
pub fn continue98_unary_prefix(op: &str, arg: &str) -> String {
    continue98_unary_skeleton(op, arg, true)
}

/// Dual-oracle UnaryExpression postfix alias (rare; e.g. legacy).
#[must_use]
pub fn continue98_unary_postfix(arg: &str, op: &str) -> String {
    continue98_unary_skeleton(op, arg, false)
}

/// Dual-oracle UnaryExpression pretty alias.
#[must_use]
pub fn continue98_unary_pretty(op: &str, arg: &str, prefix: bool) -> String {
    continue98_unary_skeleton(op, arg, prefix)
}

/// Dual-oracle UnaryExpression minify alias.
#[must_use]
pub fn continue98_unary_minify(op: &str, arg: &str, prefix: bool) -> String {
    continue98_unary_skeleton(op, arg, prefix)
}

// ── BinaryExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle BinaryExpression skeleton composing real
/// [`continue31_binary_skeleton`].
#[must_use]
pub fn continue98_binary_skeleton(left: &str, op: &str, right: &str) -> String {
    continue31_binary_skeleton(left, op, right)
}

/// Dual-oracle BinaryExpression pretty alias.
#[must_use]
pub fn continue98_binary_pretty(left: &str, op: &str, right: &str) -> String {
    continue98_binary_skeleton(left, op, right)
}

/// Dual-oracle BinaryExpression minify alias.
#[must_use]
pub fn continue98_binary_minify(left: &str, op: &str, right: &str) -> String {
    continue98_binary_skeleton(left, op, right)
}

// ── AwaitExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle AwaitExpression skeleton composing real
/// [`continue31_await_skeleton`].
#[must_use]
pub fn continue98_await_skeleton(arg: &str) -> String {
    continue31_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression pretty alias.
#[must_use]
pub fn continue98_await_pretty(arg: &str) -> String {
    continue98_await_skeleton(arg)
}

/// Dual-oracle AwaitExpression minify alias.
#[must_use]
pub fn continue98_await_minify(arg: &str) -> String {
    continue98_await_skeleton(arg)
}

// ── Identifier dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle Identifier skeleton composing real
/// [`continue31_identifier_skeleton`].
#[must_use]
pub fn continue98_identifier_skeleton(name: &str) -> String {
    continue31_identifier_skeleton(name)
}

/// Dual-oracle Identifier pretty alias.
#[must_use]
pub fn continue98_identifier_pretty(name: &str) -> String {
    continue98_identifier_skeleton(name)
}

/// Dual-oracle Identifier minify alias.
#[must_use]
pub fn continue98_identifier_minify(name: &str) -> String {
    continue98_identifier_skeleton(name)
}

// ── BlockStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BlockStatement skeleton composing real
/// [`continue31_block_skeleton`].
#[must_use]
pub fn continue98_block_skeleton(body: &str) -> String {
    continue31_block_skeleton(body)
}

/// Dual-oracle BlockStatement pretty alias.
#[must_use]
pub fn continue98_block_pretty(body: &str) -> String {
    continue98_block_skeleton(body)
}

/// Dual-oracle BlockStatement minify alias.
#[must_use]
pub fn continue98_block_minify(body: &str) -> String {
    continue98_block_skeleton(body)
}

// ── BreakStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BreakStatement skeleton composing real
/// [`continue31_break_skeleton`].
#[must_use]
pub fn continue98_break_skeleton(label: Option<&str>) -> String {
    continue31_break_skeleton(label)
}

/// Dual-oracle bare break.
#[must_use]
pub fn continue98_break_bare() -> String {
    continue98_break_skeleton(None)
}

/// Dual-oracle labeled break.
#[must_use]
pub fn continue98_break_label(label: &str) -> String {
    continue98_break_skeleton(Some(label))
}

/// Dual-oracle BreakStatement pretty alias.
#[must_use]
pub fn continue98_break_pretty(label: Option<&str>) -> String {
    continue98_break_skeleton(label)
}

/// Dual-oracle BreakStatement minify alias.
#[must_use]
pub fn continue98_break_minify(label: Option<&str>) -> String {
    continue98_break_skeleton(label)
}

// ── ContinueStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle ContinueStatement skeleton composing real
/// [`continue31_continue_skeleton`].
#[must_use]
pub fn continue98_continue_skeleton(label: Option<&str>) -> String {
    continue31_continue_skeleton(label)
}

/// Dual-oracle bare continue.
#[must_use]
pub fn continue98_continue_bare() -> String {
    continue98_continue_skeleton(None)
}

/// Dual-oracle labeled continue.
#[must_use]
pub fn continue98_continue_label(label: &str) -> String {
    continue98_continue_skeleton(Some(label))
}

/// Dual-oracle ContinueStatement pretty alias.
#[must_use]
pub fn continue98_continue_pretty(label: Option<&str>) -> String {
    continue98_continue_skeleton(label)
}

/// Dual-oracle ContinueStatement minify alias.
#[must_use]
pub fn continue98_continue_minify(label: Option<&str>) -> String {
    continue98_continue_skeleton(label)
}

// ── LabeledStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle LabeledStatement skeleton composing real
/// [`continue31_labeled_skeleton`].
#[must_use]
pub fn continue98_labeled_skeleton(label: &str, body: &str) -> String {
    continue31_labeled_skeleton(label, body)
}

/// Dual-oracle LabeledStatement pretty alias.
#[must_use]
pub fn continue98_labeled_pretty(label: &str, body: &str) -> String {
    continue98_labeled_skeleton(label, body)
}

/// Dual-oracle LabeledStatement minify alias.
#[must_use]
pub fn continue98_labeled_minify(label: &str, body: &str) -> String {
    continue98_labeled_skeleton(label, body)
}

// ── ExpressionStatement dual-oracle ─────────────────────────────────────────

/// Dual-oracle ExpressionStatement skeleton composing real
/// [`continue31_expression_stmt_skeleton`].
#[must_use]
pub fn continue98_expression_stmt_skeleton(expr: &str) -> String {
    continue31_expression_stmt_skeleton(expr)
}

/// Dual-oracle ExpressionStatement pretty alias.
#[must_use]
pub fn continue98_expression_stmt_pretty(expr: &str) -> String {
    continue98_expression_stmt_skeleton(expr)
}

/// Dual-oracle ExpressionStatement minify alias.
#[must_use]
pub fn continue98_expression_stmt_minify(expr: &str) -> String {
    continue98_expression_stmt_skeleton(expr)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: logical not `!x`.
#[must_use]
pub fn continue98_not(arg: &str) -> String {
    continue98_unary_prefix("!", arg)
}

/// Dual-oracle residual: typeof `typeof x`.
#[must_use]
pub fn continue98_typeof(arg: &str) -> String {
    continue98_unary_prefix("typeof", arg)
}

/// Dual-oracle residual: void `void 0`.
#[must_use]
pub fn continue98_void(arg: &str) -> String {
    continue98_unary_prefix("void", arg)
}

/// Dual-oracle residual: delete `delete x`.
#[must_use]
pub fn continue98_delete(arg: &str) -> String {
    continue98_unary_prefix("delete", arg)
}

/// Dual-oracle residual: unary plus `+x`.
#[must_use]
pub fn continue98_unary_plus(arg: &str) -> String {
    continue98_unary_prefix("+", arg)
}

/// Dual-oracle residual: unary minus `-x`.
#[must_use]
pub fn continue98_unary_minus(arg: &str) -> String {
    continue98_unary_prefix("-", arg)
}

/// Dual-oracle residual: addition `a + b`.
#[must_use]
pub fn continue98_add(left: &str, right: &str) -> String {
    continue98_binary_skeleton(left, "+", right)
}

/// Dual-oracle residual: equality `a === b`.
#[must_use]
pub fn continue98_strict_eq(left: &str, right: &str) -> String {
    continue98_binary_skeleton(left, "===", right)
}

/// Dual-oracle residual: await identifier.
#[must_use]
pub fn continue98_await_ident(name: &str) -> String {
    let id = continue98_identifier_skeleton(name);
    continue98_await_skeleton(&id)
}

/// Dual-oracle residual: expression statement of await.
#[must_use]
pub fn continue98_await_stmt(arg: &str) -> String {
    let aw = continue98_await_skeleton(arg);
    continue98_expression_stmt_skeleton(&aw)
}

/// Dual-oracle residual: labeled block with body.
#[must_use]
pub fn continue98_labeled_block(label: &str, body: &str) -> String {
    let block = continue98_block_skeleton(body);
    continue98_labeled_skeleton(label, &block)
}

/// Dual-oracle residual: labeled break target `loop: break loop;`.
#[must_use]
pub fn continue98_labeled_break(label: &str) -> String {
    let br = continue98_break_label(label);
    continue98_labeled_skeleton(label, &br)
}

/// Dual-oracle residual: block containing bare break.
#[must_use]
pub fn continue98_block_break() -> String {
    continue98_block_skeleton(&continue98_break_bare())
}

/// Dual-oracle residual: block containing bare continue.
#[must_use]
pub fn continue98_block_continue() -> String {
    continue98_block_skeleton(&continue98_continue_bare())
}

/// Dual-oracle residual: binary inside expression statement.
#[must_use]
pub fn continue98_binary_stmt(left: &str, op: &str, right: &str) -> String {
    let bin = continue98_binary_skeleton(left, op, right);
    continue98_expression_stmt_skeleton(&bin)
}

/// Dual-oracle residual: unary not inside expression statement.
#[must_use]
pub fn continue98_not_stmt(arg: &str) -> String {
    let u = continue98_not(arg);
    continue98_expression_stmt_skeleton(&u)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue98_stmt_sep(pretty: bool) -> &'static str {
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
        continue31_await_skeleton, continue31_binary_skeleton, continue31_block_skeleton,
        continue31_break_skeleton, continue31_continue_skeleton, continue31_expression_stmt_skeleton,
        continue31_identifier_skeleton, continue31_labeled_skeleton, continue31_unary_skeleton,
    };

    #[test]
    fn continue98_type_catalog() {
        assert_eq!(CONTINUE98_RELATED_TYPES.len(), 9);
        assert!(is_unary_binary_await_jump_related_type("UnaryExpression"));
        assert!(is_unary_binary_await_jump_related_type("BinaryExpression"));
        assert!(is_unary_binary_await_jump_related_type("AwaitExpression"));
        assert!(is_unary_binary_await_jump_related_type("Identifier"));
        assert!(is_unary_binary_await_jump_related_type("BlockStatement"));
        assert!(is_unary_binary_await_jump_related_type("BreakStatement"));
        assert!(is_unary_binary_await_jump_related_type("ContinueStatement"));
        assert!(is_unary_binary_await_jump_related_type("LabeledStatement"));
        assert!(is_unary_binary_await_jump_related_type(
            "ExpressionStatement"
        ));
        assert!(!is_unary_binary_await_jump_related_type(
            "AssignmentExpression"
        ));
        assert!(!is_unary_binary_await_jump_related_type("IfStatement"));

        assert!(is_continue98_unary_type("UnaryExpression"));
        assert!(!is_continue98_unary_type("BinaryExpression"));
        assert!(is_continue98_binary_type("BinaryExpression"));
        assert!(is_continue98_await_type("AwaitExpression"));
        assert!(is_continue98_identifier_type("Identifier"));
        assert!(is_continue98_block_type("BlockStatement"));
        assert!(is_continue98_break_type("BreakStatement"));
        assert!(is_continue98_continue_type("ContinueStatement"));
        assert!(is_continue98_labeled_type("LabeledStatement"));
        assert!(is_continue98_expression_stmt_type("ExpressionStatement"));
        assert!(is_continue98_jump_type("BreakStatement"));
        assert!(is_continue98_jump_type("ContinueStatement"));
        assert!(is_continue98_jump_type("LabeledStatement"));
        assert!(!is_continue98_jump_type("UnaryExpression"));
    }

    #[test]
    fn continue98_unary_binary_await_ident_block_emit() {
        assert_eq!(continue98_unary_skeleton("!", "x", true), "!x");
        assert_eq!(
            continue98_unary_skeleton("!", "x", true),
            continue31_unary_skeleton("!", "x", true)
        );
        assert_eq!(continue98_unary_skeleton("typeof", "x", true), "typeof x");
        assert_eq!(continue98_unary_skeleton("void", "0", true), "void 0");
        assert_eq!(continue98_unary_skeleton("~", "n", true), "~n");
        assert_eq!(continue98_unary_skeleton("!", "x", false), "x!");
        assert_eq!(
            continue98_unary_pretty("!", "a", true),
            continue98_unary_minify("!", "a", true)
        );
        assert_eq!(continue98_unary_prefix("delete", "obj.x"), "delete obj.x");
        assert_eq!(continue98_unary_postfix("x", "!"), "x!");

        assert_eq!(continue98_binary_skeleton("a", "+", "b"), "a + b");
        assert_eq!(
            continue98_binary_skeleton("a", "+", "b"),
            continue31_binary_skeleton("a", "+", "b")
        );
        assert_eq!(continue98_binary_skeleton("a", "===", "b"), "a === b");
        assert_eq!(
            continue98_binary_pretty("x", "*", "2"),
            continue98_binary_minify("x", "*", "2")
        );

        assert_eq!(continue98_await_skeleton("p"), "await p");
        assert_eq!(
            continue98_await_skeleton("p"),
            continue31_await_skeleton("p")
        );
        assert_eq!(
            continue98_await_pretty("q"),
            continue98_await_minify("q")
        );

        assert_eq!(continue98_identifier_skeleton("foo"), "foo");
        assert_eq!(
            continue98_identifier_skeleton("foo"),
            continue31_identifier_skeleton("foo")
        );
        assert_eq!(
            continue98_identifier_pretty("bar"),
            continue98_identifier_minify("bar")
        );

        assert_eq!(continue98_block_skeleton("x;"), "{ x; }");
        assert_eq!(
            continue98_block_skeleton("x;"),
            continue31_block_skeleton("x;")
        );
        assert_eq!(
            continue98_block_pretty(""),
            continue98_block_minify("")
        );
    }

    #[test]
    fn continue98_jump_and_expression_stmt_emit() {
        assert_eq!(continue98_break_skeleton(None), "break;");
        assert_eq!(continue98_break_skeleton(Some("loop")), "break loop;");
        assert_eq!(
            continue98_break_skeleton(Some("loop")),
            continue31_break_skeleton(Some("loop"))
        );
        assert_eq!(continue98_break_bare(), "break;");
        assert_eq!(continue98_break_label("outer"), "break outer;");
        assert_eq!(
            continue98_break_pretty(None),
            continue98_break_minify(None)
        );

        assert_eq!(continue98_continue_skeleton(None), "continue;");
        assert_eq!(
            continue98_continue_skeleton(Some("loop")),
            "continue loop;"
        );
        assert_eq!(
            continue98_continue_skeleton(Some("loop")),
            continue31_continue_skeleton(Some("loop"))
        );
        assert_eq!(continue98_continue_bare(), "continue;");
        assert_eq!(continue98_continue_label("inner"), "continue inner;");
        assert_eq!(
            continue98_continue_pretty(None),
            continue98_continue_minify(None)
        );

        assert_eq!(
            continue98_labeled_skeleton("loop", "break;"),
            "loop: break;"
        );
        assert_eq!(
            continue98_labeled_skeleton("loop", "break;"),
            continue31_labeled_skeleton("loop", "break;")
        );
        assert_eq!(
            continue98_labeled_pretty("a", "b"),
            continue98_labeled_minify("a", "b")
        );

        assert_eq!(continue98_expression_stmt_skeleton("x"), "x;");
        assert_eq!(
            continue98_expression_stmt_skeleton("x"),
            continue31_expression_stmt_skeleton("x")
        );
        assert_eq!(
            continue98_expression_stmt_pretty("y()"),
            continue98_expression_stmt_minify("y()")
        );
    }

    #[test]
    fn continue98_composed_residual_shells() {
        assert_eq!(continue98_not("x"), "!x");
        assert_eq!(continue98_typeof("x"), "typeof x");
        assert_eq!(continue98_void("0"), "void 0");
        assert_eq!(continue98_delete("a.b"), "delete a.b");
        assert_eq!(continue98_unary_plus("n"), "+n");
        assert_eq!(continue98_unary_minus("n"), "-n");
        assert_eq!(continue98_add("1", "2"), "1 + 2");
        assert_eq!(continue98_strict_eq("a", "b"), "a === b");
        assert_eq!(continue98_await_ident("p"), "await p");
        assert_eq!(continue98_await_stmt("p"), "await p;");
        assert_eq!(
            continue98_labeled_block("loop", "break;"),
            "loop: { break; }"
        );
        assert_eq!(
            continue98_labeled_break("loop"),
            "loop: break loop;"
        );
        assert_eq!(continue98_block_break(), "{ break; }");
        assert_eq!(continue98_block_continue(), "{ continue; }");
        assert_eq!(continue98_binary_stmt("a", "+", "b"), "a + b;");
        assert_eq!(continue98_not_stmt("x"), "!x;");
        assert_eq!(continue98_stmt_sep(true), " ");
        assert_eq!(continue98_stmt_sep(false), "");
        assert_ne!(continue98_stmt_sep(true), continue98_stmt_sep(false));
    }
}

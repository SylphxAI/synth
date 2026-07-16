//! Pure WithStatement + LabeledStatement + DebuggerStatement +
//! ThrowStatement + ExpressionStatement + IfStatement dual-oracle emission
//! — residual pure **continue110** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–109:
//! - WithStatement dual-oracle composing real `continue43_with_skeleton`
//! - LabeledStatement dual-oracle composing real `continue43_labeled_skeleton`
//! - DebuggerStatement dual-oracle composing real
//!   `continue43_debugger_skeleton`
//! - ThrowStatement dual-oracle composing real `continue43_throw_skeleton`
//! - ExpressionStatement dual-oracle composing real
//!   `continue43_expression_stmt_skeleton`
//! - IfStatement dual-oracle composing real `continue43_if_skeleton` /
//!   `continue43_if_else_skeleton`
//! - Composed with/labeled/debugger/throw/expr/if residual shells
//!
//! Intentionally does **not** re-wrap continue109 new/conditional/this/super/
//! sequence/empty poles (continue42 bases), continue108 unary/binary/await/
//! arrow/spread/rest poles (continue41 bases), or continue40–41 bases.
//! Composes real shipped pure helpers from continue43 bases. Full engines
//! remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue43_debugger_skeleton, continue43_expression_stmt_skeleton,
    continue43_if_else_skeleton, continue43_if_skeleton, continue43_labeled_skeleton,
    continue43_throw_skeleton, continue43_with_skeleton,
};

/// Dual-oracle residual: continue110 related AST type catalog.
pub const CONTINUE110_RELATED_TYPES: &[&str] = &[
    "WithStatement",
    "LabeledStatement",
    "DebuggerStatement",
    "ThrowStatement",
    "ExpressionStatement",
    "IfStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_with_labeled_debugger_throw_expr_if_related_type(t: &str) -> bool {
    CONTINUE110_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue110_with_type(t: &str) -> bool {
    t == "WithStatement"
}

#[must_use]
pub fn is_continue110_labeled_type(t: &str) -> bool {
    t == "LabeledStatement"
}

#[must_use]
pub fn is_continue110_debugger_type(t: &str) -> bool {
    t == "DebuggerStatement"
}

#[must_use]
pub fn is_continue110_throw_type(t: &str) -> bool {
    t == "ThrowStatement"
}

#[must_use]
pub fn is_continue110_expression_stmt_type(t: &str) -> bool {
    t == "ExpressionStatement"
}

#[must_use]
pub fn is_continue110_if_type(t: &str) -> bool {
    t == "IfStatement"
}

#[must_use]
pub fn is_continue110_stmt_type(t: &str) -> bool {
    matches!(
        t,
        "WithStatement"
            | "LabeledStatement"
            | "DebuggerStatement"
            | "ThrowStatement"
            | "ExpressionStatement"
            | "IfStatement"
    )
}

// ── WithStatement dual-oracle ───────────────────────────────────────────────

/// Dual-oracle WithStatement skeleton composing real
/// [`continue43_with_skeleton`].
#[must_use]
pub fn continue110_with_skeleton(obj: &str, body: &str) -> String {
    continue43_with_skeleton(obj, body)
}

/// Dual-oracle WithStatement pretty alias.
#[must_use]
pub fn continue110_with_pretty(obj: &str, body: &str) -> String {
    continue110_with_skeleton(obj, body)
}

/// Dual-oracle WithStatement minify alias.
#[must_use]
pub fn continue110_with_minify(obj: &str, body: &str) -> String {
    continue110_with_skeleton(obj, body)
}

// ── LabeledStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle LabeledStatement skeleton composing real
/// [`continue43_labeled_skeleton`].
#[must_use]
pub fn continue110_labeled_skeleton(label: &str, body: &str) -> String {
    continue43_labeled_skeleton(label, body)
}

/// Dual-oracle LabeledStatement pretty alias.
#[must_use]
pub fn continue110_labeled_pretty(label: &str, body: &str) -> String {
    continue110_labeled_skeleton(label, body)
}

/// Dual-oracle LabeledStatement minify alias.
#[must_use]
pub fn continue110_labeled_minify(label: &str, body: &str) -> String {
    continue110_labeled_skeleton(label, body)
}

// ── DebuggerStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle DebuggerStatement skeleton composing real
/// [`continue43_debugger_skeleton`].
#[must_use]
pub fn continue110_debugger_skeleton() -> &'static str {
    continue43_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement pretty alias.
#[must_use]
pub fn continue110_debugger_pretty() -> &'static str {
    continue110_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement minify alias.
#[must_use]
pub fn continue110_debugger_minify() -> &'static str {
    continue110_debugger_skeleton()
}

// ── ThrowStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThrowStatement skeleton composing real
/// [`continue43_throw_skeleton`].
#[must_use]
pub fn continue110_throw_skeleton(arg: &str) -> String {
    continue43_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement pretty alias.
#[must_use]
pub fn continue110_throw_pretty(arg: &str) -> String {
    continue110_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement minify alias.
#[must_use]
pub fn continue110_throw_minify(arg: &str) -> String {
    continue110_throw_skeleton(arg)
}

// ── ExpressionStatement dual-oracle ─────────────────────────────────────────

/// Dual-oracle ExpressionStatement skeleton composing real
/// [`continue43_expression_stmt_skeleton`].
#[must_use]
pub fn continue110_expression_stmt_skeleton(expr: &str) -> String {
    continue43_expression_stmt_skeleton(expr)
}

/// Dual-oracle ExpressionStatement pretty alias.
#[must_use]
pub fn continue110_expression_stmt_pretty(expr: &str) -> String {
    continue110_expression_stmt_skeleton(expr)
}

/// Dual-oracle ExpressionStatement minify alias.
#[must_use]
pub fn continue110_expression_stmt_minify(expr: &str) -> String {
    continue110_expression_stmt_skeleton(expr)
}

// ── IfStatement dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle IfStatement (no else) skeleton composing real
/// [`continue43_if_skeleton`].
#[must_use]
pub fn continue110_if_skeleton(test: &str, cons: &str) -> String {
    continue43_if_skeleton(test, cons)
}

/// Dual-oracle IfStatement if-else skeleton composing real
/// [`continue43_if_else_skeleton`].
#[must_use]
pub fn continue110_if_else_skeleton(test: &str, cons: &str, alt: &str) -> String {
    continue43_if_else_skeleton(test, cons, alt)
}

/// Dual-oracle IfStatement pretty alias (no else).
#[must_use]
pub fn continue110_if_pretty(test: &str, cons: &str) -> String {
    continue110_if_skeleton(test, cons)
}

/// Dual-oracle IfStatement minify alias (no else).
#[must_use]
pub fn continue110_if_minify(test: &str, cons: &str) -> String {
    continue110_if_skeleton(test, cons)
}

/// Dual-oracle IfStatement if-else pretty alias.
#[must_use]
pub fn continue110_if_else_pretty(test: &str, cons: &str, alt: &str) -> String {
    continue110_if_else_skeleton(test, cons, alt)
}

/// Dual-oracle IfStatement if-else minify alias.
#[must_use]
pub fn continue110_if_else_minify(test: &str, cons: &str, alt: &str) -> String {
    continue110_if_else_skeleton(test, cons, alt)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: empty with body (`with (obj) {}`).
#[must_use]
pub fn continue110_with_empty(obj: &str) -> String {
    continue110_with_skeleton(obj, "{}")
}

/// Dual-oracle residual: labeled debugger (`label: debugger;`).
#[must_use]
pub fn continue110_labeled_debugger(label: &str) -> String {
    continue110_labeled_skeleton(label, continue110_debugger_skeleton())
}

/// Dual-oracle residual: labeled throw (`label: throw err;`).
#[must_use]
pub fn continue110_labeled_throw(label: &str, arg: &str) -> String {
    let body = continue110_throw_skeleton(arg);
    continue110_labeled_skeleton(label, &body)
}

/// Dual-oracle residual: expression statement of bare call seed.
#[must_use]
pub fn continue110_call_stmt(callee: &str) -> String {
    continue110_expression_stmt_skeleton(&format!("{callee}()"))
}

/// Dual-oracle residual: if with empty consequent block.
#[must_use]
pub fn continue110_if_empty(test: &str) -> String {
    continue110_if_skeleton(test, "{}")
}

/// Dual-oracle residual: if/else with empty blocks.
#[must_use]
pub fn continue110_if_else_empty(test: &str) -> String {
    continue110_if_else_skeleton(test, "{}", "{}")
}

/// Dual-oracle residual: if then throw.
#[must_use]
pub fn continue110_if_throw(test: &str, arg: &str) -> String {
    let thr = continue110_throw_skeleton(arg);
    continue110_if_skeleton(test, &thr)
}

/// Dual-oracle residual: if then debugger else expression-stmt.
#[must_use]
pub fn continue110_if_debugger_else_expr(test: &str, expr: &str) -> String {
    let alt = continue110_expression_stmt_skeleton(expr);
    continue110_if_else_skeleton(test, continue110_debugger_skeleton(), &alt)
}

/// Dual-oracle residual: with body is a labeled block seed.
#[must_use]
pub fn continue110_with_labeled(obj: &str, label: &str, body: &str) -> String {
    let labeled = continue110_labeled_skeleton(label, body);
    continue110_with_skeleton(obj, &labeled)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue110_stmt_sep(pretty: bool) -> &'static str {
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
        continue43_debugger_skeleton, continue43_expression_stmt_skeleton,
        continue43_if_else_skeleton, continue43_if_skeleton, continue43_labeled_skeleton,
        continue43_throw_skeleton, continue43_with_skeleton,
    };

    #[test]
    fn continue110_type_catalog() {
        assert_eq!(CONTINUE110_RELATED_TYPES.len(), 6);
        assert!(is_with_labeled_debugger_throw_expr_if_related_type(
            "WithStatement"
        ));
        assert!(is_with_labeled_debugger_throw_expr_if_related_type(
            "LabeledStatement"
        ));
        assert!(is_with_labeled_debugger_throw_expr_if_related_type(
            "DebuggerStatement"
        ));
        assert!(is_with_labeled_debugger_throw_expr_if_related_type(
            "ThrowStatement"
        ));
        assert!(is_with_labeled_debugger_throw_expr_if_related_type(
            "ExpressionStatement"
        ));
        assert!(is_with_labeled_debugger_throw_expr_if_related_type(
            "IfStatement"
        ));
        assert!(!is_with_labeled_debugger_throw_expr_if_related_type(
            "NewExpression"
        ));
        assert!(!is_with_labeled_debugger_throw_expr_if_related_type(
            "EmptyStatement"
        ));

        assert!(is_continue110_with_type("WithStatement"));
        assert!(!is_continue110_with_type("IfStatement"));
        assert!(is_continue110_labeled_type("LabeledStatement"));
        assert!(is_continue110_debugger_type("DebuggerStatement"));
        assert!(is_continue110_throw_type("ThrowStatement"));
        assert!(is_continue110_expression_stmt_type("ExpressionStatement"));
        assert!(is_continue110_if_type("IfStatement"));
        assert!(is_continue110_stmt_type("WithStatement"));
        assert!(is_continue110_stmt_type("IfStatement"));
        assert!(!is_continue110_stmt_type("NewExpression"));
    }

    #[test]
    fn continue110_with_labeled_debugger_emit() {
        assert_eq!(
            continue110_with_skeleton("obj", "{}"),
            "with (obj) {}"
        );
        assert_eq!(
            continue110_with_skeleton("obj", "{}"),
            continue43_with_skeleton("obj", "{}")
        );
        assert_eq!(
            continue110_with_pretty("o", "x;"),
            continue110_with_minify("o", "x;")
        );

        assert_eq!(
            continue110_labeled_skeleton("loop", "break;"),
            "loop: break;"
        );
        assert_eq!(
            continue110_labeled_skeleton("loop", "break;"),
            continue43_labeled_skeleton("loop", "break;")
        );
        assert_eq!(
            continue110_labeled_pretty("L", "x;"),
            continue110_labeled_minify("L", "x;")
        );

        assert_eq!(continue110_debugger_skeleton(), "debugger;");
        assert_eq!(
            continue110_debugger_skeleton(),
            continue43_debugger_skeleton()
        );
        assert_eq!(
            continue110_debugger_pretty(),
            continue110_debugger_minify()
        );
    }

    #[test]
    fn continue110_throw_expr_if_emit() {
        assert_eq!(continue110_throw_skeleton("err"), "throw err;");
        assert_eq!(
            continue110_throw_skeleton("err"),
            continue43_throw_skeleton("err")
        );
        assert_eq!(
            continue110_throw_pretty("e"),
            continue110_throw_minify("e")
        );

        assert_eq!(
            continue110_expression_stmt_skeleton("foo()"),
            "foo();"
        );
        assert_eq!(
            continue110_expression_stmt_skeleton("foo()"),
            continue43_expression_stmt_skeleton("foo()")
        );
        assert_eq!(
            continue110_expression_stmt_pretty("x"),
            continue110_expression_stmt_minify("x")
        );

        assert_eq!(continue110_if_skeleton("ok", "a;"), "if (ok) a;");
        assert_eq!(
            continue110_if_skeleton("ok", "a;"),
            continue43_if_skeleton("ok", "a;")
        );
        assert_eq!(
            continue110_if_pretty("t", "b;"),
            continue110_if_minify("t", "b;")
        );

        assert_eq!(
            continue110_if_else_skeleton("ok", "a;", "b;"),
            "if (ok) a; else b;"
        );
        assert_eq!(
            continue110_if_else_skeleton("ok", "a;", "b;"),
            continue43_if_else_skeleton("ok", "a;", "b;")
        );
        assert_eq!(
            continue110_if_else_pretty("t", "a;", "b;"),
            continue110_if_else_minify("t", "a;", "b;")
        );
    }

    #[test]
    fn continue110_composed_residual_shells() {
        assert_eq!(continue110_with_empty("obj"), "with (obj) {}");
        assert_eq!(
            continue110_labeled_debugger("dbg"),
            "dbg: debugger;"
        );
        assert_eq!(
            continue110_labeled_throw("fail", "err"),
            "fail: throw err;"
        );
        assert_eq!(continue110_call_stmt("foo"), "foo();");
        assert_eq!(continue110_if_empty("ok"), "if (ok) {}");
        assert_eq!(
            continue110_if_else_empty("ok"),
            "if (ok) {} else {}"
        );
        assert_eq!(
            continue110_if_throw("bad", "e"),
            "if (bad) throw e;"
        );
        assert_eq!(
            continue110_if_debugger_else_expr("flag", "x"),
            "if (flag) debugger; else x;"
        );
        assert_eq!(
            continue110_with_labeled("env", "inner", "x;"),
            "with (env) inner: x;"
        );
        assert_eq!(continue110_stmt_sep(true), " ");
        assert_eq!(continue110_stmt_sep(false), "");
        assert_ne!(continue110_stmt_sep(true), continue110_stmt_sep(false));
    }
}

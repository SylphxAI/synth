//! Pure IfStatement + WhileStatement + ReturnStatement + ThrowStatement +
//! TryStatement/CatchClause + SwitchStatement/SwitchCase + EmptyStatement +
//! DebuggerStatement dual-oracle emission — residual pure **continue95** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–94:
//! - IfStatement dual-oracle composing real `continue28_if_skeleton`
//! - WhileStatement dual-oracle composing real `continue28_while_skeleton`
//! - ReturnStatement dual-oracle composing real `continue28_return_skeleton`
//! - ThrowStatement dual-oracle composing real `continue28_throw_skeleton`
//! - TryStatement/CatchClause dual-oracle composing real
//!   `continue28_try_catch_skeleton`
//! - SwitchStatement/SwitchCase dual-oracle composing real
//!   `continue28_switch_case_skeleton`
//! - EmptyStatement dual-oracle composing real `continue28_empty_skeleton`
//! - DebuggerStatement dual-oracle composing real `continue28_debugger_skeleton`
//! - Composed if/while/return/throw/try/switch residual shells
//!
//! Intentionally does **not** re-wrap continue79 if/return pretty-minify poles,
//! continue80 while/switch full dual-oracle, continue81 try/throw surfaces, or
//! continue94 this/conditional/logical poles. Composes real shipped pure
//! helpers from continue28 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue28_debugger_skeleton, continue28_empty_skeleton, continue28_if_skeleton,
    continue28_return_skeleton, continue28_switch_case_skeleton, continue28_throw_skeleton,
    continue28_try_catch_skeleton, continue28_while_skeleton,
};

/// Dual-oracle residual: continue95 related AST type catalog.
pub const CONTINUE95_RELATED_TYPES: &[&str] = &[
    "IfStatement",
    "WhileStatement",
    "ReturnStatement",
    "ThrowStatement",
    "TryStatement",
    "CatchClause",
    "SwitchStatement",
    "SwitchCase",
    "EmptyStatement",
    "DebuggerStatement",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_if_while_return_throw_related_type(t: &str) -> bool {
    CONTINUE95_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue95_if_statement_type(t: &str) -> bool {
    t == "IfStatement"
}

#[must_use]
pub fn is_continue95_while_statement_type(t: &str) -> bool {
    t == "WhileStatement"
}

#[must_use]
pub fn is_continue95_return_statement_type(t: &str) -> bool {
    t == "ReturnStatement"
}

#[must_use]
pub fn is_continue95_throw_statement_type(t: &str) -> bool {
    t == "ThrowStatement"
}

#[must_use]
pub fn is_continue95_try_statement_type(t: &str) -> bool {
    t == "TryStatement"
}

#[must_use]
pub fn is_continue95_catch_clause_type(t: &str) -> bool {
    t == "CatchClause"
}

#[must_use]
pub fn is_continue95_switch_statement_type(t: &str) -> bool {
    t == "SwitchStatement"
}

#[must_use]
pub fn is_continue95_switch_case_type(t: &str) -> bool {
    t == "SwitchCase"
}

#[must_use]
pub fn is_continue95_empty_statement_type(t: &str) -> bool {
    t == "EmptyStatement"
}

#[must_use]
pub fn is_continue95_debugger_statement_type(t: &str) -> bool {
    t == "DebuggerStatement"
}

// ── IfStatement dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle IfStatement skeleton composing real [`continue28_if_skeleton`].
#[must_use]
pub fn continue95_if_statement_skeleton(
    test: &str,
    consequent: &str,
    alternate: Option<&str>,
) -> String {
    continue28_if_skeleton(test, consequent, alternate)
}

/// Dual-oracle IfStatement pretty alias (no else).
#[must_use]
pub fn continue95_if_pretty(test: &str, consequent: &str) -> String {
    continue95_if_statement_skeleton(test, consequent, None)
}

/// Dual-oracle IfStatement minify alias (no else; base is space-padded).
#[must_use]
pub fn continue95_if_minify(test: &str, consequent: &str) -> String {
    continue95_if_statement_skeleton(test, consequent, None)
}

/// Dual-oracle IfStatement with else pretty alias.
#[must_use]
pub fn continue95_if_else_pretty(test: &str, consequent: &str, alternate: &str) -> String {
    continue95_if_statement_skeleton(test, consequent, Some(alternate))
}

/// Dual-oracle IfStatement with else minify alias.
#[must_use]
pub fn continue95_if_else_minify(test: &str, consequent: &str, alternate: &str) -> String {
    continue95_if_statement_skeleton(test, consequent, Some(alternate))
}

// ── WhileStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle WhileStatement skeleton composing real
/// [`continue28_while_skeleton`].
#[must_use]
pub fn continue95_while_statement_skeleton(test: &str, body: &str) -> String {
    continue28_while_skeleton(test, body)
}

/// Dual-oracle WhileStatement pretty alias.
#[must_use]
pub fn continue95_while_pretty(test: &str, body: &str) -> String {
    continue95_while_statement_skeleton(test, body)
}

/// Dual-oracle WhileStatement minify alias.
#[must_use]
pub fn continue95_while_minify(test: &str, body: &str) -> String {
    continue95_while_statement_skeleton(test, body)
}

// ── ReturnStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ReturnStatement skeleton composing real
/// [`continue28_return_skeleton`].
#[must_use]
pub fn continue95_return_statement_skeleton(arg: Option<&str>) -> String {
    continue28_return_skeleton(arg)
}

/// Dual-oracle bare `return`.
#[must_use]
pub fn continue95_return_bare() -> String {
    continue95_return_statement_skeleton(None)
}

/// Dual-oracle `return arg`.
#[must_use]
pub fn continue95_return_arg(arg: &str) -> String {
    continue95_return_statement_skeleton(Some(arg))
}

/// Dual-oracle ReturnStatement pretty alias.
#[must_use]
pub fn continue95_return_pretty(arg: Option<&str>) -> String {
    continue95_return_statement_skeleton(arg)
}

/// Dual-oracle ReturnStatement minify alias.
#[must_use]
pub fn continue95_return_minify(arg: Option<&str>) -> String {
    continue95_return_statement_skeleton(arg)
}

// ── ThrowStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThrowStatement skeleton composing real
/// [`continue28_throw_skeleton`].
#[must_use]
pub fn continue95_throw_statement_skeleton(arg: &str) -> String {
    continue28_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement pretty alias.
#[must_use]
pub fn continue95_throw_pretty(arg: &str) -> String {
    continue95_throw_statement_skeleton(arg)
}

/// Dual-oracle ThrowStatement minify alias.
#[must_use]
pub fn continue95_throw_minify(arg: &str) -> String {
    continue95_throw_statement_skeleton(arg)
}

// ── Try / Catch dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle TryStatement/CatchClause skeleton composing real
/// [`continue28_try_catch_skeleton`].
#[must_use]
pub fn continue95_try_catch_skeleton(body: &str, param: &str, handler: &str) -> String {
    continue28_try_catch_skeleton(body, param, handler)
}

/// Dual-oracle TryStatement pretty alias.
#[must_use]
pub fn continue95_try_catch_pretty(body: &str, param: &str, handler: &str) -> String {
    continue95_try_catch_skeleton(body, param, handler)
}

/// Dual-oracle TryStatement minify alias.
#[must_use]
pub fn continue95_try_catch_minify(body: &str, param: &str, handler: &str) -> String {
    continue95_try_catch_skeleton(body, param, handler)
}

// ── Switch / Case dual-oracle ───────────────────────────────────────────────

/// Dual-oracle SwitchStatement/SwitchCase skeleton composing real
/// [`continue28_switch_case_skeleton`].
#[must_use]
pub fn continue95_switch_case_skeleton(disc: &str, case_test: &str, case_body: &str) -> String {
    continue28_switch_case_skeleton(disc, case_test, case_body)
}

/// Dual-oracle SwitchStatement pretty alias.
#[must_use]
pub fn continue95_switch_pretty(disc: &str, case_test: &str, case_body: &str) -> String {
    continue95_switch_case_skeleton(disc, case_test, case_body)
}

/// Dual-oracle SwitchStatement minify alias.
#[must_use]
pub fn continue95_switch_minify(disc: &str, case_test: &str, case_body: &str) -> String {
    continue95_switch_case_skeleton(disc, case_test, case_body)
}

// ── Empty / Debugger dual-oracle ────────────────────────────────────────────

/// Dual-oracle EmptyStatement skeleton composing real
/// [`continue28_empty_skeleton`].
#[must_use]
pub fn continue95_empty_statement_skeleton() -> &'static str {
    continue28_empty_skeleton()
}

/// Dual-oracle EmptyStatement pretty alias.
#[must_use]
pub fn continue95_empty_pretty() -> &'static str {
    continue95_empty_statement_skeleton()
}

/// Dual-oracle EmptyStatement minify alias.
#[must_use]
pub fn continue95_empty_minify() -> &'static str {
    continue95_empty_statement_skeleton()
}

/// Dual-oracle DebuggerStatement skeleton composing real
/// [`continue28_debugger_skeleton`].
#[must_use]
pub fn continue95_debugger_statement_skeleton() -> &'static str {
    continue28_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement pretty alias.
#[must_use]
pub fn continue95_debugger_pretty() -> &'static str {
    continue95_debugger_statement_skeleton()
}

/// Dual-oracle DebuggerStatement minify alias.
#[must_use]
pub fn continue95_debugger_minify() -> &'static str {
    continue95_debugger_statement_skeleton()
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: if with return in consequent.
#[must_use]
pub fn continue95_if_return(test: &str, arg: Option<&str>) -> String {
    let ret = continue95_return_statement_skeleton(arg);
    continue95_if_statement_skeleton(test, &format!("{{ {ret} }}"), None)
}

/// Dual-oracle residual: if/else return poles.
#[must_use]
pub fn continue95_if_else_return(test: &str, then_arg: &str, else_arg: &str) -> String {
    let then_body = format!("{{ {} }}", continue95_return_arg(then_arg));
    let else_body = format!("{{ {} }}", continue95_return_arg(else_arg));
    continue95_if_statement_skeleton(test, &then_body, Some(&else_body))
}

/// Dual-oracle residual: while body with empty statement.
#[must_use]
pub fn continue95_while_empty(test: &str) -> String {
    continue95_while_statement_skeleton(test, continue95_empty_statement_skeleton())
}

/// Dual-oracle residual: while body with debugger.
#[must_use]
pub fn continue95_while_debugger(test: &str) -> String {
    continue95_while_statement_skeleton(test, continue95_debugger_statement_skeleton())
}

/// Dual-oracle residual: try/catch that rethrows.
#[must_use]
pub fn continue95_try_catch_rethrow(body: &str, param: &str) -> String {
    let handler = format!("{{ {} }}", continue95_throw_statement_skeleton(param));
    continue95_try_catch_skeleton(body, param, &handler)
}

/// Dual-oracle residual: switch case that returns.
#[must_use]
pub fn continue95_switch_return(disc: &str, case_test: &str, ret_arg: &str) -> String {
    let body = continue95_return_arg(ret_arg);
    continue95_switch_case_skeleton(disc, case_test, &body)
}

/// Dual-oracle residual: if throw else return.
#[must_use]
pub fn continue95_if_throw_else_return(test: &str, err: &str, ok: &str) -> String {
    let then_body = format!("{{ {} }}", continue95_throw_statement_skeleton(err));
    let else_body = format!("{{ {} }}", continue95_return_arg(ok));
    continue95_if_statement_skeleton(test, &then_body, Some(&else_body))
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue95_stmt_sep(pretty: bool) -> &'static str {
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
        continue28_debugger_skeleton, continue28_empty_skeleton, continue28_if_skeleton,
        continue28_return_skeleton, continue28_switch_case_skeleton, continue28_throw_skeleton,
        continue28_try_catch_skeleton, continue28_while_skeleton,
    };

    #[test]
    fn continue95_type_catalog() {
        assert_eq!(CONTINUE95_RELATED_TYPES.len(), 10);
        assert!(is_if_while_return_throw_related_type("IfStatement"));
        assert!(is_if_while_return_throw_related_type("WhileStatement"));
        assert!(is_if_while_return_throw_related_type("ReturnStatement"));
        assert!(is_if_while_return_throw_related_type("ThrowStatement"));
        assert!(is_if_while_return_throw_related_type("TryStatement"));
        assert!(is_if_while_return_throw_related_type("CatchClause"));
        assert!(is_if_while_return_throw_related_type("SwitchStatement"));
        assert!(is_if_while_return_throw_related_type("SwitchCase"));
        assert!(is_if_while_return_throw_related_type("EmptyStatement"));
        assert!(is_if_while_return_throw_related_type("DebuggerStatement"));
        assert!(!is_if_while_return_throw_related_type("ThisExpression"));
        assert!(!is_if_while_return_throw_related_type("ForOfStatement"));
        assert!(!is_if_while_return_throw_related_type("LogicalExpression"));
        assert!(is_continue95_if_statement_type("IfStatement"));
        assert!(is_continue95_while_statement_type("WhileStatement"));
        assert!(is_continue95_return_statement_type("ReturnStatement"));
        assert!(is_continue95_throw_statement_type("ThrowStatement"));
        assert!(is_continue95_try_statement_type("TryStatement"));
        assert!(is_continue95_catch_clause_type("CatchClause"));
        assert!(is_continue95_switch_statement_type("SwitchStatement"));
        assert!(is_continue95_switch_case_type("SwitchCase"));
        assert!(is_continue95_empty_statement_type("EmptyStatement"));
        assert!(is_continue95_debugger_statement_type("DebuggerStatement"));
        assert!(!is_continue95_if_statement_type("WhileStatement"));
    }

    #[test]
    fn continue95_if_while_return_throw_emit() {
        assert_eq!(
            continue95_if_statement_skeleton("a", "{}", None),
            "if (a) {}"
        );
        assert_eq!(
            continue95_if_statement_skeleton("a", "{x}", Some("{y}")),
            "if (a) {x} else {y}"
        );
        assert_eq!(
            continue95_if_statement_skeleton("a", "{}", None),
            continue28_if_skeleton("a", "{}", None)
        );
        assert_eq!(
            continue95_if_pretty("t", "{}"),
            continue95_if_minify("t", "{}")
        );
        assert_eq!(
            continue95_if_else_pretty("t", "{a}", "{b}"),
            continue95_if_else_minify("t", "{a}", "{b}")
        );

        assert_eq!(continue95_while_statement_skeleton("c", "{}"), "while (c) {}");
        assert_eq!(
            continue95_while_statement_skeleton("c", "{}"),
            continue28_while_skeleton("c", "{}")
        );
        assert_eq!(
            continue95_while_pretty("c", "{}"),
            continue95_while_minify("c", "{}")
        );

        assert_eq!(continue95_return_bare(), "return");
        assert_eq!(continue95_return_arg("1"), "return 1");
        assert_eq!(
            continue95_return_statement_skeleton(Some("x")),
            continue28_return_skeleton(Some("x"))
        );
        assert_eq!(
            continue95_return_pretty(None),
            continue95_return_minify(None)
        );

        assert_eq!(continue95_throw_statement_skeleton("e"), "throw e");
        assert_eq!(
            continue95_throw_statement_skeleton("e"),
            continue28_throw_skeleton("e")
        );
        assert_eq!(
            continue95_throw_pretty("err"),
            continue95_throw_minify("err")
        );

        assert_eq!(
            continue95_try_catch_skeleton("{}", "e", "{}"),
            "try {} catch (e) {}"
        );
        assert_eq!(
            continue95_try_catch_skeleton("{}", "e", "{}"),
            continue28_try_catch_skeleton("{}", "e", "{}")
        );
        assert_eq!(
            continue95_try_catch_pretty("{}", "e", "{}"),
            continue95_try_catch_minify("{}", "e", "{}")
        );

        assert_eq!(
            continue95_switch_case_skeleton("x", "1", "break"),
            "switch (x) { case 1: break }"
        );
        assert_eq!(
            continue95_switch_case_skeleton("x", "1", "break"),
            continue28_switch_case_skeleton("x", "1", "break")
        );
        assert_eq!(
            continue95_switch_pretty("x", "0", "return"),
            continue95_switch_minify("x", "0", "return")
        );

        assert_eq!(continue95_empty_statement_skeleton(), ";");
        assert_eq!(
            continue95_empty_statement_skeleton(),
            continue28_empty_skeleton()
        );
        assert_eq!(continue95_empty_pretty(), continue95_empty_minify());

        assert_eq!(continue95_debugger_statement_skeleton(), "debugger");
        assert_eq!(
            continue95_debugger_statement_skeleton(),
            continue28_debugger_skeleton()
        );
        assert_eq!(continue95_debugger_pretty(), continue95_debugger_minify());
    }

    #[test]
    fn continue95_composed_residual_shells() {
        assert_eq!(
            continue95_if_return("ok", Some("1")),
            "if (ok) { return 1 }"
        );
        assert_eq!(
            continue95_if_return("done", None),
            "if (done) { return }"
        );
        assert_eq!(
            continue95_if_else_return("flag", "a", "b"),
            "if (flag) { return a } else { return b }"
        );
        assert_eq!(continue95_while_empty("run"), "while (run) ;");
        assert_eq!(
            continue95_while_debugger("run"),
            "while (run) debugger"
        );
        assert_eq!(
            continue95_try_catch_rethrow("{}", "e"),
            "try {} catch (e) { throw e }"
        );
        assert_eq!(
            continue95_switch_return("k", "1", "v"),
            "switch (k) { case 1: return v }"
        );
        assert_eq!(
            continue95_if_throw_else_return("bad", "err", "ok"),
            "if (bad) { throw err } else { return ok }"
        );
        assert_eq!(continue95_stmt_sep(true), " ");
        assert_eq!(continue95_stmt_sep(false), "");
        assert_ne!(continue95_stmt_sep(true), continue95_stmt_sep(false));
    }
}

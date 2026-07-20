//! Pure SwitchStatement + SwitchCase + TryStatement + CatchClause +
//! ThrowStatement + DebuggerStatement dual-oracle emission — residual pure
//! **continue99** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–98:
//! - SwitchStatement dual-oracle composing real `continue32_switch_skeleton`
//! - SwitchCase dual-oracle composing real `continue32_case_skeleton`
//! - TryStatement catch dual-oracle composing real
//!   `continue32_try_catch_skeleton`
//! - TryStatement finally dual-oracle composing real
//!   `continue32_try_finally_skeleton`
//! - ThrowStatement dual-oracle composing real `continue32_throw_skeleton`
//! - DebuggerStatement dual-oracle composing real
//!   `continue32_debugger_skeleton`
//! - Composed switch/try/throw/debugger residual shells
//!
//! Intentionally does **not** re-wrap continue28/95 if/while/return/throw poles,
//! continue80 switch/loop pretty-minify poles, continue81 try/throw/with
//! surfaces, or continue98 unary/binary/await/jump poles. Composes real shipped
//! pure helpers from continue32 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue32_case_skeleton, continue32_debugger_skeleton, continue32_switch_skeleton,
    continue32_throw_skeleton, continue32_try_catch_skeleton, continue32_try_finally_skeleton,
};

/// Dual-oracle residual: continue99 related AST type catalog.
pub const CONTINUE99_RELATED_TYPES: &[&str] = &[
    "SwitchStatement",
    "SwitchCase",
    "TryStatement",
    "CatchClause",
    "ThrowStatement",
    "DebuggerStatement",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_switch_try_throw_debugger_related_type(t: &str) -> bool {
    CONTINUE99_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue99_switch_type(t: &str) -> bool {
    t == "SwitchStatement"
}

#[must_use]
pub fn is_continue99_switch_case_type(t: &str) -> bool {
    t == "SwitchCase"
}

#[must_use]
pub fn is_continue99_try_type(t: &str) -> bool {
    t == "TryStatement"
}

#[must_use]
pub fn is_continue99_catch_type(t: &str) -> bool {
    t == "CatchClause"
}

#[must_use]
pub fn is_continue99_throw_type(t: &str) -> bool {
    t == "ThrowStatement"
}

#[must_use]
pub fn is_continue99_debugger_type(t: &str) -> bool {
    t == "DebuggerStatement"
}

#[must_use]
pub fn is_continue99_switch_family_type(t: &str) -> bool {
    matches!(t, "SwitchStatement" | "SwitchCase")
}

#[must_use]
pub fn is_continue99_try_family_type(t: &str) -> bool {
    matches!(t, "TryStatement" | "CatchClause")
}

// ── SwitchStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle SwitchStatement skeleton composing real
/// [`continue32_switch_skeleton`].
#[must_use]
pub fn continue99_switch_skeleton(disc: &str, body: &str) -> String {
    continue32_switch_skeleton(disc, body)
}

/// Dual-oracle SwitchStatement pretty alias.
#[must_use]
pub fn continue99_switch_pretty(disc: &str, body: &str) -> String {
    continue99_switch_skeleton(disc, body)
}

/// Dual-oracle SwitchStatement minify alias.
#[must_use]
pub fn continue99_switch_minify(disc: &str, body: &str) -> String {
    continue99_switch_skeleton(disc, body)
}

// ── SwitchCase dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle SwitchCase skeleton composing real [`continue32_case_skeleton`].
#[must_use]
pub fn continue99_case_skeleton(test: Option<&str>, body: &str) -> String {
    continue32_case_skeleton(test, body)
}

/// Dual-oracle SwitchCase with test.
#[must_use]
pub fn continue99_case(test: &str, body: &str) -> String {
    continue99_case_skeleton(Some(test), body)
}

/// Dual-oracle default case.
#[must_use]
pub fn continue99_default_case(body: &str) -> String {
    continue99_case_skeleton(None, body)
}

/// Dual-oracle SwitchCase pretty alias.
#[must_use]
pub fn continue99_case_pretty(test: Option<&str>, body: &str) -> String {
    continue99_case_skeleton(test, body)
}

/// Dual-oracle SwitchCase minify alias.
#[must_use]
pub fn continue99_case_minify(test: Option<&str>, body: &str) -> String {
    continue99_case_skeleton(test, body)
}

// ── TryStatement dual-oracle ────────────────────────────────────────────────

/// Dual-oracle try/catch skeleton composing real
/// [`continue32_try_catch_skeleton`].
#[must_use]
pub fn continue99_try_catch_skeleton(body: &str, param: &str, catch_body: &str) -> String {
    continue32_try_catch_skeleton(body, param, catch_body)
}

/// Dual-oracle try/catch pretty alias.
#[must_use]
pub fn continue99_try_catch_pretty(body: &str, param: &str, catch_body: &str) -> String {
    continue99_try_catch_skeleton(body, param, catch_body)
}

/// Dual-oracle try/catch minify alias.
#[must_use]
pub fn continue99_try_catch_minify(body: &str, param: &str, catch_body: &str) -> String {
    continue99_try_catch_skeleton(body, param, catch_body)
}

/// Dual-oracle try/finally skeleton composing real
/// [`continue32_try_finally_skeleton`].
#[must_use]
pub fn continue99_try_finally_skeleton(body: &str, fin: &str) -> String {
    continue32_try_finally_skeleton(body, fin)
}

/// Dual-oracle try/finally pretty alias.
#[must_use]
pub fn continue99_try_finally_pretty(body: &str, fin: &str) -> String {
    continue99_try_finally_skeleton(body, fin)
}

/// Dual-oracle try/finally minify alias.
#[must_use]
pub fn continue99_try_finally_minify(body: &str, fin: &str) -> String {
    continue99_try_finally_skeleton(body, fin)
}

// ── ThrowStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThrowStatement skeleton composing real
/// [`continue32_throw_skeleton`].
#[must_use]
pub fn continue99_throw_skeleton(arg: &str) -> String {
    continue32_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement pretty alias.
#[must_use]
pub fn continue99_throw_pretty(arg: &str) -> String {
    continue99_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement minify alias.
#[must_use]
pub fn continue99_throw_minify(arg: &str) -> String {
    continue99_throw_skeleton(arg)
}

// ── DebuggerStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle DebuggerStatement skeleton composing real
/// [`continue32_debugger_skeleton`].
#[must_use]
pub fn continue99_debugger_skeleton() -> &'static str {
    continue32_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement pretty alias.
#[must_use]
pub fn continue99_debugger_pretty() -> &'static str {
    continue99_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement minify alias.
#[must_use]
pub fn continue99_debugger_minify() -> &'static str {
    continue99_debugger_skeleton()
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: single-case switch body.
#[must_use]
pub fn continue99_switch_one_case(disc: &str, test: &str, body: &str) -> String {
    let case = continue99_case(test, body);
    continue99_switch_skeleton(disc, &case)
}

/// Dual-oracle residual: switch with case + default.
#[must_use]
pub fn continue99_switch_case_default(disc: &str, test: &str, case_body: &str, def_body: &str) -> String {
    let c = continue99_case(test, case_body);
    let d = continue99_default_case(def_body);
    let body = format!("{c} {d}");
    continue99_switch_skeleton(disc, &body)
}

/// Dual-oracle residual: try/catch then throw in catch.
#[must_use]
pub fn continue99_try_catch_rethrow(body: &str, param: &str) -> String {
    let thr = continue99_throw_skeleton(param);
    continue99_try_catch_skeleton(body, param, &thr)
}

/// Dual-oracle residual: try/finally with debugger in finally.
#[must_use]
pub fn continue99_try_finally_debugger(body: &str) -> String {
    continue99_try_finally_skeleton(body, continue99_debugger_skeleton())
}

/// Dual-oracle residual: throw string literal error.
#[must_use]
pub fn continue99_throw_string(msg: &str) -> String {
    continue99_throw_skeleton(&format!("\"{msg}\""))
}

/// Dual-oracle residual: throw new Error skeleton (arg already formed).
#[must_use]
pub fn continue99_throw_new_error(msg: &str) -> String {
    continue99_throw_skeleton(&format!("new Error(\"{msg}\")"))
}

/// Dual-oracle residual: bare debugger statement token.
#[must_use]
pub fn continue99_debugger() -> &'static str {
    continue99_debugger_skeleton()
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue99_stmt_sep(pretty: bool) -> &'static str {
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
        continue32_case_skeleton, continue32_debugger_skeleton, continue32_switch_skeleton,
        continue32_throw_skeleton, continue32_try_catch_skeleton, continue32_try_finally_skeleton,
    };

    #[test]
    fn continue99_type_catalog() {
        assert_eq!(CONTINUE99_RELATED_TYPES.len(), 6);
        assert!(is_switch_try_throw_debugger_related_type("SwitchStatement"));
        assert!(is_switch_try_throw_debugger_related_type("SwitchCase"));
        assert!(is_switch_try_throw_debugger_related_type("TryStatement"));
        assert!(is_switch_try_throw_debugger_related_type("CatchClause"));
        assert!(is_switch_try_throw_debugger_related_type("ThrowStatement"));
        assert!(is_switch_try_throw_debugger_related_type(
            "DebuggerStatement"
        ));
        assert!(!is_switch_try_throw_debugger_related_type("IfStatement"));
        assert!(!is_switch_try_throw_debugger_related_type(
            "UnaryExpression"
        ));

        assert!(is_continue99_switch_type("SwitchStatement"));
        assert!(!is_continue99_switch_type("SwitchCase"));
        assert!(is_continue99_switch_case_type("SwitchCase"));
        assert!(is_continue99_try_type("TryStatement"));
        assert!(is_continue99_catch_type("CatchClause"));
        assert!(is_continue99_throw_type("ThrowStatement"));
        assert!(is_continue99_debugger_type("DebuggerStatement"));
        assert!(is_continue99_switch_family_type("SwitchStatement"));
        assert!(is_continue99_switch_family_type("SwitchCase"));
        assert!(!is_continue99_switch_family_type("TryStatement"));
        assert!(is_continue99_try_family_type("TryStatement"));
        assert!(is_continue99_try_family_type("CatchClause"));
        assert!(!is_continue99_try_family_type("ThrowStatement"));
    }

    #[test]
    fn continue99_switch_case_emit() {
        assert_eq!(
            continue99_switch_skeleton("x", "case 1: break;"),
            "switch (x) { case 1: break; }"
        );
        assert_eq!(
            continue99_switch_skeleton("x", "case 1: break;"),
            continue32_switch_skeleton("x", "case 1: break;")
        );
        assert_eq!(
            continue99_switch_pretty("y", "default: ;"),
            continue99_switch_minify("y", "default: ;")
        );

        assert_eq!(continue99_case_skeleton(Some("1"), "break;"), "case 1: break;");
        assert_eq!(
            continue99_case_skeleton(Some("1"), "break;"),
            continue32_case_skeleton(Some("1"), "break;")
        );
        assert_eq!(continue99_case("0", "return;"), "case 0: return;");
        assert_eq!(continue99_default_case("break;"), "default: break;");
        assert_eq!(
            continue99_case_skeleton(None, "break;"),
            continue32_case_skeleton(None, "break;")
        );
        assert_eq!(
            continue99_case_pretty(Some("a"), "b"),
            continue99_case_minify(Some("a"), "b")
        );
    }

    #[test]
    fn continue99_try_throw_debugger_emit() {
        assert_eq!(
            continue99_try_catch_skeleton("a;", "e", "b;"),
            "try { a; } catch (e) { b; }"
        );
        assert_eq!(
            continue99_try_catch_skeleton("a;", "e", "b;"),
            continue32_try_catch_skeleton("a;", "e", "b;")
        );
        assert_eq!(
            continue99_try_catch_pretty("x;", "err", "y;"),
            continue99_try_catch_minify("x;", "err", "y;")
        );

        assert_eq!(
            continue99_try_finally_skeleton("a;", "b;"),
            "try { a; } finally { b; }"
        );
        assert_eq!(
            continue99_try_finally_skeleton("a;", "b;"),
            continue32_try_finally_skeleton("a;", "b;")
        );
        assert_eq!(
            continue99_try_finally_pretty("x;", "y;"),
            continue99_try_finally_minify("x;", "y;")
        );

        assert_eq!(continue99_throw_skeleton("e"), "throw e;");
        assert_eq!(
            continue99_throw_skeleton("e"),
            continue32_throw_skeleton("e")
        );
        assert_eq!(
            continue99_throw_pretty("err"),
            continue99_throw_minify("err")
        );

        assert_eq!(continue99_debugger_skeleton(), "debugger;");
        assert_eq!(
            continue99_debugger_skeleton(),
            continue32_debugger_skeleton()
        );
        assert_eq!(
            continue99_debugger_pretty(),
            continue99_debugger_minify()
        );
    }

    #[test]
    fn continue99_composed_residual_shells() {
        assert_eq!(
            continue99_switch_one_case("x", "1", "break;"),
            "switch (x) { case 1: break; }"
        );
        assert_eq!(
            continue99_switch_case_default("k", "\"a\"", "return 1;", "return 0;"),
            "switch (k) { case \"a\": return 1; default: return 0; }"
        );
        assert_eq!(
            continue99_try_catch_rethrow("risky();", "e"),
            "try { risky(); } catch (e) { throw e; }"
        );
        assert_eq!(
            continue99_try_finally_debugger("work();"),
            "try { work(); } finally { debugger; }"
        );
        assert_eq!(continue99_throw_string("boom"), "throw \"boom\";");
        assert_eq!(
            continue99_throw_new_error("fail"),
            "throw new Error(\"fail\");"
        );
        assert_eq!(continue99_debugger(), "debugger;");
        assert_eq!(continue99_stmt_sep(true), " ");
        assert_eq!(continue99_stmt_sep(false), "");
        assert_ne!(continue99_stmt_sep(true), continue99_stmt_sep(false));
    }
}

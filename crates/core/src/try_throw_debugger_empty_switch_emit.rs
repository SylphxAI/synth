//! Pure TryStatement + CatchClause + ThrowStatement + DebuggerStatement +
//! EmptyStatement + SwitchStatement dual-oracle emission — residual pure
//! **continue104** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–103:
//! - TryStatement catch dual-oracle composing real
//!   `continue37_try_catch_skeleton`
//! - TryStatement finally dual-oracle composing real
//!   `continue37_try_finally_skeleton`
//! - ThrowStatement dual-oracle composing real `continue37_throw_skeleton`
//! - DebuggerStatement dual-oracle composing real
//!   `continue37_debugger_skeleton`
//! - EmptyStatement dual-oracle composing real `continue37_empty_skeleton`
//! - SwitchStatement dual-oracle composing real `continue37_switch_skeleton`
//! - Composed try/throw/debugger/empty/switch residual shells
//!
//! Intentionally does **not** re-wrap continue99 switch/try/throw/debugger
//! poles (continue32 bases), continue32/28/81 surfaces, continue100 empty via
//! continue33, or continue103 class/return/this/super/meta poles. Composes real
//! shipped pure helpers from continue37 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue37_debugger_skeleton, continue37_empty_skeleton, continue37_switch_skeleton,
    continue37_throw_skeleton, continue37_try_catch_skeleton, continue37_try_finally_skeleton,
};

/// Dual-oracle residual: continue104 related AST type catalog.
pub const CONTINUE104_RELATED_TYPES: &[&str] = &[
    "TryStatement",
    "CatchClause",
    "ThrowStatement",
    "DebuggerStatement",
    "EmptyStatement",
    "SwitchStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_try_throw_debugger_empty_switch_related_type(t: &str) -> bool {
    CONTINUE104_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue104_try_type(t: &str) -> bool {
    t == "TryStatement"
}

#[must_use]
pub fn is_continue104_catch_type(t: &str) -> bool {
    t == "CatchClause"
}

#[must_use]
pub fn is_continue104_throw_type(t: &str) -> bool {
    t == "ThrowStatement"
}

#[must_use]
pub fn is_continue104_debugger_type(t: &str) -> bool {
    t == "DebuggerStatement"
}

#[must_use]
pub fn is_continue104_empty_type(t: &str) -> bool {
    t == "EmptyStatement"
}

#[must_use]
pub fn is_continue104_switch_type(t: &str) -> bool {
    t == "SwitchStatement"
}

#[must_use]
pub fn is_continue104_try_family_type(t: &str) -> bool {
    matches!(t, "TryStatement" | "CatchClause")
}

#[must_use]
pub fn is_continue104_control_type(t: &str) -> bool {
    matches!(
        t,
        "ThrowStatement" | "DebuggerStatement" | "EmptyStatement" | "SwitchStatement"
    )
}

// ── TryStatement catch dual-oracle ──────────────────────────────────────────

/// Dual-oracle TryStatement catch skeleton composing real
/// [`continue37_try_catch_skeleton`].
#[must_use]
pub fn continue104_try_catch_skeleton(
    try_body: &str,
    catch_param: &str,
    catch_body: &str,
) -> String {
    continue37_try_catch_skeleton(try_body, catch_param, catch_body)
}

/// Dual-oracle TryStatement catch pretty alias.
#[must_use]
pub fn continue104_try_catch_pretty(
    try_body: &str,
    catch_param: &str,
    catch_body: &str,
) -> String {
    continue104_try_catch_skeleton(try_body, catch_param, catch_body)
}

/// Dual-oracle TryStatement catch minify alias.
#[must_use]
pub fn continue104_try_catch_minify(
    try_body: &str,
    catch_param: &str,
    catch_body: &str,
) -> String {
    continue104_try_catch_skeleton(try_body, catch_param, catch_body)
}

// ── TryStatement finally dual-oracle ────────────────────────────────────────

/// Dual-oracle TryStatement finally skeleton composing real
/// [`continue37_try_finally_skeleton`].
#[must_use]
pub fn continue104_try_finally_skeleton(try_body: &str, finally_body: &str) -> String {
    continue37_try_finally_skeleton(try_body, finally_body)
}

/// Dual-oracle TryStatement finally pretty alias.
#[must_use]
pub fn continue104_try_finally_pretty(try_body: &str, finally_body: &str) -> String {
    continue104_try_finally_skeleton(try_body, finally_body)
}

/// Dual-oracle TryStatement finally minify alias.
#[must_use]
pub fn continue104_try_finally_minify(try_body: &str, finally_body: &str) -> String {
    continue104_try_finally_skeleton(try_body, finally_body)
}

// ── ThrowStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThrowStatement skeleton composing real
/// [`continue37_throw_skeleton`].
#[must_use]
pub fn continue104_throw_skeleton(arg: &str) -> String {
    continue37_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement pretty alias.
#[must_use]
pub fn continue104_throw_pretty(arg: &str) -> String {
    continue104_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement minify alias.
#[must_use]
pub fn continue104_throw_minify(arg: &str) -> String {
    continue104_throw_skeleton(arg)
}

// ── DebuggerStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle DebuggerStatement skeleton composing real
/// [`continue37_debugger_skeleton`].
#[must_use]
pub fn continue104_debugger_skeleton() -> String {
    continue37_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement pretty alias.
#[must_use]
pub fn continue104_debugger_pretty() -> String {
    continue104_debugger_skeleton()
}

/// Dual-oracle DebuggerStatement minify alias.
#[must_use]
pub fn continue104_debugger_minify() -> String {
    continue104_debugger_skeleton()
}

// ── EmptyStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle EmptyStatement skeleton composing real
/// [`continue37_empty_skeleton`].
#[must_use]
pub fn continue104_empty_skeleton() -> String {
    continue37_empty_skeleton()
}

/// Dual-oracle EmptyStatement pretty alias.
#[must_use]
pub fn continue104_empty_pretty() -> String {
    continue104_empty_skeleton()
}

/// Dual-oracle EmptyStatement minify alias.
#[must_use]
pub fn continue104_empty_minify() -> String {
    continue104_empty_skeleton()
}

// ── SwitchStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle SwitchStatement skeleton composing real
/// [`continue37_switch_skeleton`].
#[must_use]
pub fn continue104_switch_skeleton(disc: &str, body: &str) -> String {
    continue37_switch_skeleton(disc, body)
}

/// Dual-oracle SwitchStatement pretty alias.
#[must_use]
pub fn continue104_switch_pretty(disc: &str, body: &str) -> String {
    continue104_switch_skeleton(disc, body)
}

/// Dual-oracle SwitchStatement minify alias.
#[must_use]
pub fn continue104_switch_minify(disc: &str, body: &str) -> String {
    continue104_switch_skeleton(disc, body)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: try/catch that rethrows the catch param.
#[must_use]
pub fn continue104_try_catch_rethrow(try_body: &str, param: &str) -> String {
    let thr = continue104_throw_skeleton(param);
    continue104_try_catch_skeleton(try_body, param, &thr)
}

/// Dual-oracle residual: try/finally with debugger in finally.
#[must_use]
pub fn continue104_try_finally_debugger(try_body: &str) -> String {
    continue104_try_finally_skeleton(try_body, &continue104_debugger_skeleton())
}

/// Dual-oracle residual: try/finally with empty statement in finally.
#[must_use]
pub fn continue104_try_finally_empty(try_body: &str) -> String {
    continue104_try_finally_skeleton(try_body, &continue104_empty_skeleton())
}

/// Dual-oracle residual: switch whose body is a bare empty statement.
#[must_use]
pub fn continue104_switch_empty(disc: &str) -> String {
    continue104_switch_skeleton(disc, &continue104_empty_skeleton())
}

/// Dual-oracle residual: throw a string literal.
#[must_use]
pub fn continue104_throw_string(msg: &str) -> String {
    continue104_throw_skeleton(&format!("\"{msg}\""))
}

/// Dual-oracle residual: throw `new Error("msg")`.
#[must_use]
pub fn continue104_throw_new_error(msg: &str) -> String {
    continue104_throw_skeleton(&format!("new Error(\"{msg}\")"))
}

/// Dual-oracle residual: bare debugger statement.
#[must_use]
pub fn continue104_debugger() -> String {
    continue104_debugger_skeleton()
}

/// Dual-oracle residual: bare empty statement.
#[must_use]
pub fn continue104_empty() -> String {
    continue104_empty_skeleton()
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue104_stmt_sep(pretty: bool) -> &'static str {
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
        continue37_debugger_skeleton, continue37_empty_skeleton, continue37_switch_skeleton,
        continue37_throw_skeleton, continue37_try_catch_skeleton, continue37_try_finally_skeleton,
    };

    #[test]
    fn continue104_type_catalog() {
        assert_eq!(CONTINUE104_RELATED_TYPES.len(), 6);
        assert!(is_try_throw_debugger_empty_switch_related_type(
            "TryStatement"
        ));
        assert!(is_try_throw_debugger_empty_switch_related_type(
            "CatchClause"
        ));
        assert!(is_try_throw_debugger_empty_switch_related_type(
            "ThrowStatement"
        ));
        assert!(is_try_throw_debugger_empty_switch_related_type(
            "DebuggerStatement"
        ));
        assert!(is_try_throw_debugger_empty_switch_related_type(
            "EmptyStatement"
        ));
        assert!(is_try_throw_debugger_empty_switch_related_type(
            "SwitchStatement"
        ));
        assert!(!is_try_throw_debugger_empty_switch_related_type(
            "ClassDeclaration"
        ));
        assert!(!is_try_throw_debugger_empty_switch_related_type(
            "SwitchCase"
        ));

        assert!(is_continue104_try_type("TryStatement"));
        assert!(!is_continue104_try_type("CatchClause"));
        assert!(is_continue104_catch_type("CatchClause"));
        assert!(is_continue104_throw_type("ThrowStatement"));
        assert!(is_continue104_debugger_type("DebuggerStatement"));
        assert!(is_continue104_empty_type("EmptyStatement"));
        assert!(is_continue104_switch_type("SwitchStatement"));
        assert!(is_continue104_try_family_type("TryStatement"));
        assert!(is_continue104_try_family_type("CatchClause"));
        assert!(!is_continue104_try_family_type("ThrowStatement"));
        assert!(is_continue104_control_type("ThrowStatement"));
        assert!(is_continue104_control_type("DebuggerStatement"));
        assert!(is_continue104_control_type("EmptyStatement"));
        assert!(is_continue104_control_type("SwitchStatement"));
        assert!(!is_continue104_control_type("TryStatement"));
    }

    #[test]
    fn continue104_try_throw_emit() {
        assert_eq!(
            continue104_try_catch_skeleton("a();", "e", "handle(e);"),
            "try { a(); } catch (e) { handle(e); }"
        );
        assert_eq!(
            continue104_try_catch_skeleton("a();", "e", "handle(e);"),
            continue37_try_catch_skeleton("a();", "e", "handle(e);")
        );
        assert_eq!(
            continue104_try_catch_pretty("x;", "err", "y;"),
            continue104_try_catch_minify("x;", "err", "y;")
        );

        assert_eq!(
            continue104_try_finally_skeleton("a();", "cleanup();"),
            "try { a(); } finally { cleanup(); }"
        );
        assert_eq!(
            continue104_try_finally_skeleton("a();", "cleanup();"),
            continue37_try_finally_skeleton("a();", "cleanup();")
        );
        assert_eq!(
            continue104_try_finally_pretty("a;", "b;"),
            continue104_try_finally_minify("a;", "b;")
        );

        assert_eq!(continue104_throw_skeleton("err"), "throw err;");
        assert_eq!(
            continue104_throw_skeleton("err"),
            continue37_throw_skeleton("err")
        );
        assert_eq!(
            continue104_throw_pretty("x"),
            continue104_throw_minify("x")
        );
    }

    #[test]
    fn continue104_debugger_empty_switch_emit() {
        assert_eq!(continue104_debugger_skeleton(), "debugger;");
        assert_eq!(
            continue104_debugger_skeleton(),
            continue37_debugger_skeleton()
        );
        assert_eq!(
            continue104_debugger_pretty(),
            continue104_debugger_minify()
        );

        assert_eq!(continue104_empty_skeleton(), ";");
        assert_eq!(continue104_empty_skeleton(), continue37_empty_skeleton());
        assert_eq!(continue104_empty_pretty(), continue104_empty_minify());

        assert_eq!(
            continue104_switch_skeleton("x", "case 1: break;"),
            "switch (x) { case 1: break; }"
        );
        assert_eq!(
            continue104_switch_skeleton("x", "case 1: break;"),
            continue37_switch_skeleton("x", "case 1: break;")
        );
        assert_eq!(
            continue104_switch_pretty("y", "default: ;"),
            continue104_switch_minify("y", "default: ;")
        );
    }

    #[test]
    fn continue104_composed_residual_shells() {
        assert_eq!(
            continue104_try_catch_rethrow("a();", "e"),
            "try { a(); } catch (e) { throw e; }"
        );
        assert_eq!(
            continue104_try_finally_debugger("work();"),
            "try { work(); } finally { debugger; }"
        );
        assert_eq!(
            continue104_try_finally_empty("work();"),
            "try { work(); } finally { ; }"
        );
        assert_eq!(
            continue104_switch_empty("x"),
            "switch (x) { ; }"
        );
        assert_eq!(continue104_throw_string("boom"), "throw \"boom\";");
        assert_eq!(
            continue104_throw_new_error("fail"),
            "throw new Error(\"fail\");"
        );
        assert_eq!(continue104_debugger(), "debugger;");
        assert_eq!(continue104_empty(), ";");
        assert_eq!(continue104_stmt_sep(true), " ");
        assert_eq!(continue104_stmt_sep(false), "");
        assert_ne!(continue104_stmt_sep(true), continue104_stmt_sep(false));
    }
}

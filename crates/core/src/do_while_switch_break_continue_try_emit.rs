//! Pure DoWhileStatement + WhileStatement + SwitchStatement + BreakStatement +
//! ContinueStatement + TryStatement dual-oracle emission
//! — residual pure **continue112** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–111:
//! - DoWhileStatement dual-oracle composing real `continue45_do_while_skeleton`
//! - WhileStatement dual-oracle composing real `continue45_while_skeleton`
//! - SwitchStatement dual-oracle composing real `continue45_switch_skeleton`
//! - BreakStatement dual-oracle composing real `continue45_break_skeleton` /
//!   `continue45_break_label_skeleton`
//! - ContinueStatement dual-oracle composing real `continue45_continue_skeleton`
//! - TryStatement catch/finally dual-oracle composing real
//!   `continue45_try_catch_skeleton` / `continue45_try_finally_skeleton`
//! - Composed loop/switch/jump/try residual shells
//!
//! Intentionally does **not** re-wrap continue111 jsx element/fragment/attr
//! poles (continue44 bases), continue110 with/labeled/debugger/throw/expr/if
//! poles (continue43 bases), or continue40–43 bases.
//! Composes real shipped pure helpers from continue45 bases. Full engines
//! remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue45_break_label_skeleton, continue45_break_skeleton, continue45_continue_skeleton,
    continue45_do_while_skeleton, continue45_switch_skeleton, continue45_try_catch_skeleton,
    continue45_try_finally_skeleton, continue45_while_skeleton,
};

/// Dual-oracle residual: continue112 related AST type catalog.
pub const CONTINUE112_RELATED_TYPES: &[&str] = &[
    "DoWhileStatement",
    "WhileStatement",
    "SwitchStatement",
    "BreakStatement",
    "ContinueStatement",
    "TryStatement",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_do_while_switch_break_continue_try_related_type(t: &str) -> bool {
    CONTINUE112_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue112_do_while_type(t: &str) -> bool {
    t == "DoWhileStatement"
}

#[must_use]
pub fn is_continue112_while_type(t: &str) -> bool {
    t == "WhileStatement"
}

#[must_use]
pub fn is_continue112_switch_type(t: &str) -> bool {
    t == "SwitchStatement"
}

#[must_use]
pub fn is_continue112_break_type(t: &str) -> bool {
    t == "BreakStatement"
}

#[must_use]
pub fn is_continue112_continue_type(t: &str) -> bool {
    t == "ContinueStatement"
}

#[must_use]
pub fn is_continue112_try_type(t: &str) -> bool {
    t == "TryStatement"
}

#[must_use]
pub fn is_continue112_loop_type(t: &str) -> bool {
    matches!(t, "DoWhileStatement" | "WhileStatement")
}

#[must_use]
pub fn is_continue112_jump_type(t: &str) -> bool {
    matches!(t, "BreakStatement" | "ContinueStatement")
}

#[must_use]
pub fn is_continue112_type(t: &str) -> bool {
    matches!(
        t,
        "DoWhileStatement"
            | "WhileStatement"
            | "SwitchStatement"
            | "BreakStatement"
            | "ContinueStatement"
            | "TryStatement"
    )
}

// ── DoWhileStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle DoWhileStatement skeleton composing real
/// [`continue45_do_while_skeleton`].
#[must_use]
pub fn continue112_do_while_skeleton(body: &str, test: &str) -> String {
    continue45_do_while_skeleton(body, test)
}

/// Dual-oracle DoWhileStatement pretty alias.
#[must_use]
pub fn continue112_do_while_pretty(body: &str, test: &str) -> String {
    continue112_do_while_skeleton(body, test)
}

/// Dual-oracle DoWhileStatement minify alias.
#[must_use]
pub fn continue112_do_while_minify(body: &str, test: &str) -> String {
    continue112_do_while_skeleton(body, test)
}

// ── WhileStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle WhileStatement skeleton composing real
/// [`continue45_while_skeleton`].
#[must_use]
pub fn continue112_while_skeleton(test: &str, body: &str) -> String {
    continue45_while_skeleton(test, body)
}

/// Dual-oracle WhileStatement pretty alias.
#[must_use]
pub fn continue112_while_pretty(test: &str, body: &str) -> String {
    continue112_while_skeleton(test, body)
}

/// Dual-oracle WhileStatement minify alias.
#[must_use]
pub fn continue112_while_minify(test: &str, body: &str) -> String {
    continue112_while_skeleton(test, body)
}

// ── SwitchStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle SwitchStatement skeleton composing real
/// [`continue45_switch_skeleton`].
#[must_use]
pub fn continue112_switch_skeleton(disc: &str, body: &str) -> String {
    continue45_switch_skeleton(disc, body)
}

/// Dual-oracle SwitchStatement pretty alias.
#[must_use]
pub fn continue112_switch_pretty(disc: &str, body: &str) -> String {
    continue112_switch_skeleton(disc, body)
}

/// Dual-oracle SwitchStatement minify alias.
#[must_use]
pub fn continue112_switch_minify(disc: &str, body: &str) -> String {
    continue112_switch_skeleton(disc, body)
}

// ── BreakStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BreakStatement skeleton composing real
/// [`continue45_break_skeleton`].
#[must_use]
pub fn continue112_break_skeleton() -> &'static str {
    continue45_break_skeleton()
}

/// Dual-oracle labeled BreakStatement skeleton composing real
/// [`continue45_break_label_skeleton`].
#[must_use]
pub fn continue112_break_label_skeleton(label: &str) -> String {
    continue45_break_label_skeleton(label)
}

/// Dual-oracle BreakStatement pretty alias.
#[must_use]
pub fn continue112_break_pretty() -> &'static str {
    continue112_break_skeleton()
}

/// Dual-oracle BreakStatement minify alias.
#[must_use]
pub fn continue112_break_minify() -> &'static str {
    continue112_break_skeleton()
}

/// Dual-oracle labeled BreakStatement pretty alias.
#[must_use]
pub fn continue112_break_label_pretty(label: &str) -> String {
    continue112_break_label_skeleton(label)
}

/// Dual-oracle labeled BreakStatement minify alias.
#[must_use]
pub fn continue112_break_label_minify(label: &str) -> String {
    continue112_break_label_skeleton(label)
}

// ── ContinueStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle ContinueStatement skeleton composing real
/// [`continue45_continue_skeleton`].
#[must_use]
pub fn continue112_continue_skeleton() -> &'static str {
    continue45_continue_skeleton()
}

/// Dual-oracle ContinueStatement pretty alias.
#[must_use]
pub fn continue112_continue_pretty() -> &'static str {
    continue112_continue_skeleton()
}

/// Dual-oracle ContinueStatement minify alias.
#[must_use]
pub fn continue112_continue_minify() -> &'static str {
    continue112_continue_skeleton()
}

// ── TryStatement dual-oracle ────────────────────────────────────────────────

/// Dual-oracle TryStatement catch skeleton composing real
/// [`continue45_try_catch_skeleton`].
#[must_use]
pub fn continue112_try_catch_skeleton(block: &str, param: &str, handler: &str) -> String {
    continue45_try_catch_skeleton(block, param, handler)
}

/// Dual-oracle TryStatement finally skeleton composing real
/// [`continue45_try_finally_skeleton`].
#[must_use]
pub fn continue112_try_finally_skeleton(block: &str, finalizer: &str) -> String {
    continue45_try_finally_skeleton(block, finalizer)
}

/// Dual-oracle TryStatement catch pretty alias.
#[must_use]
pub fn continue112_try_catch_pretty(block: &str, param: &str, handler: &str) -> String {
    continue112_try_catch_skeleton(block, param, handler)
}

/// Dual-oracle TryStatement catch minify alias.
#[must_use]
pub fn continue112_try_catch_minify(block: &str, param: &str, handler: &str) -> String {
    continue112_try_catch_skeleton(block, param, handler)
}

/// Dual-oracle TryStatement finally pretty alias.
#[must_use]
pub fn continue112_try_finally_pretty(block: &str, finalizer: &str) -> String {
    continue112_try_finally_skeleton(block, finalizer)
}

/// Dual-oracle TryStatement finally minify alias.
#[must_use]
pub fn continue112_try_finally_minify(block: &str, finalizer: &str) -> String {
    continue112_try_finally_skeleton(block, finalizer)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: empty-body while (`while (t) {}`).
#[must_use]
pub fn continue112_while_empty(test: &str) -> String {
    continue112_while_skeleton(test, "{}")
}

/// Dual-oracle residual: empty-body do-while (`do {} while (t);`).
#[must_use]
pub fn continue112_do_while_empty(test: &str) -> String {
    continue112_do_while_skeleton("{}", test)
}

/// Dual-oracle residual: switch with empty body block.
#[must_use]
pub fn continue112_switch_empty(disc: &str) -> String {
    continue112_switch_skeleton(disc, "{}")
}

/// Dual-oracle residual: while body containing bare break.
#[must_use]
pub fn continue112_while_break(test: &str) -> String {
    let br = continue112_break_skeleton();
    continue112_while_skeleton(test, &format!("{{ {br} }}"))
}

/// Dual-oracle residual: while body containing bare continue.
#[must_use]
pub fn continue112_while_continue(test: &str) -> String {
    let c = continue112_continue_skeleton();
    continue112_while_skeleton(test, &format!("{{ {c} }}"))
}

/// Dual-oracle residual: labeled break inside while shell seed.
#[must_use]
pub fn continue112_while_break_label(test: &str, label: &str) -> String {
    let br = continue112_break_label_skeleton(label);
    continue112_while_skeleton(test, &format!("{{ {br} }}"))
}

/// Dual-oracle residual: try/catch with empty blocks.
#[must_use]
pub fn continue112_try_catch_empty(param: &str) -> String {
    continue112_try_catch_skeleton("{}", param, "{}")
}

/// Dual-oracle residual: try/finally with empty blocks.
#[must_use]
pub fn continue112_try_finally_empty() -> String {
    continue112_try_finally_skeleton("{}", "{}")
}

/// Dual-oracle residual: try/catch wrapping a throw-like handler body seed.
#[must_use]
pub fn continue112_try_catch_rethrow(block: &str, param: &str) -> String {
    let handler = format!("{{ throw {param}; }}");
    continue112_try_catch_skeleton(block, param, &handler)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue112_sep(pretty: bool) -> &'static str {
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
        continue45_break_label_skeleton, continue45_break_skeleton, continue45_continue_skeleton,
        continue45_do_while_skeleton, continue45_switch_skeleton, continue45_try_catch_skeleton,
        continue45_try_finally_skeleton, continue45_while_skeleton,
    };

    #[test]
    fn continue112_type_catalog() {
        assert_eq!(CONTINUE112_RELATED_TYPES.len(), 6);
        assert!(is_do_while_switch_break_continue_try_related_type(
            "DoWhileStatement"
        ));
        assert!(is_do_while_switch_break_continue_try_related_type(
            "WhileStatement"
        ));
        assert!(is_do_while_switch_break_continue_try_related_type(
            "SwitchStatement"
        ));
        assert!(is_do_while_switch_break_continue_try_related_type(
            "BreakStatement"
        ));
        assert!(is_do_while_switch_break_continue_try_related_type(
            "ContinueStatement"
        ));
        assert!(is_do_while_switch_break_continue_try_related_type(
            "TryStatement"
        ));
        assert!(!is_do_while_switch_break_continue_try_related_type(
            "JSXElement"
        ));
        assert!(!is_do_while_switch_break_continue_try_related_type(
            "WithStatement"
        ));

        assert!(is_continue112_do_while_type("DoWhileStatement"));
        assert!(!is_continue112_do_while_type("WhileStatement"));
        assert!(is_continue112_while_type("WhileStatement"));
        assert!(is_continue112_switch_type("SwitchStatement"));
        assert!(is_continue112_break_type("BreakStatement"));
        assert!(is_continue112_continue_type("ContinueStatement"));
        assert!(is_continue112_try_type("TryStatement"));
        assert!(is_continue112_loop_type("DoWhileStatement"));
        assert!(is_continue112_loop_type("WhileStatement"));
        assert!(!is_continue112_loop_type("SwitchStatement"));
        assert!(is_continue112_jump_type("BreakStatement"));
        assert!(is_continue112_jump_type("ContinueStatement"));
        assert!(!is_continue112_jump_type("TryStatement"));
        assert!(is_continue112_type("DoWhileStatement"));
        assert!(is_continue112_type("TryStatement"));
        assert!(!is_continue112_type("IfStatement"));
    }

    #[test]
    fn continue112_do_while_while_switch_emit() {
        assert_eq!(
            continue112_do_while_skeleton("{}", "x"),
            "do {} while (x);"
        );
        assert_eq!(
            continue112_do_while_skeleton("{}", "x"),
            continue45_do_while_skeleton("{}", "x")
        );
        assert_eq!(
            continue112_do_while_pretty("{ a; }", "ok"),
            continue112_do_while_minify("{ a; }", "ok")
        );

        assert_eq!(continue112_while_skeleton("x", "{}"), "while (x) {}");
        assert_eq!(
            continue112_while_skeleton("x", "{}"),
            continue45_while_skeleton("x", "{}")
        );
        assert_eq!(
            continue112_while_pretty("t", "{ b; }"),
            continue112_while_minify("t", "{ b; }")
        );

        assert_eq!(
            continue112_switch_skeleton("disc", "{ case 1: break; }"),
            "switch (disc) { case 1: break; }"
        );
        assert_eq!(
            continue112_switch_skeleton("d", "{}"),
            continue45_switch_skeleton("d", "{}")
        );
        assert_eq!(
            continue112_switch_pretty("x", "{}"),
            continue112_switch_minify("x", "{}")
        );
    }

    #[test]
    fn continue112_break_continue_try_emit() {
        assert_eq!(continue112_break_skeleton(), "break;");
        assert_eq!(continue112_break_skeleton(), continue45_break_skeleton());
        assert_eq!(continue112_break_pretty(), continue112_break_minify());

        assert_eq!(continue112_break_label_skeleton("L"), "break L;");
        assert_eq!(
            continue112_break_label_skeleton("L"),
            continue45_break_label_skeleton("L")
        );
        assert_eq!(
            continue112_break_label_pretty("outer"),
            continue112_break_label_minify("outer")
        );

        assert_eq!(continue112_continue_skeleton(), "continue;");
        assert_eq!(
            continue112_continue_skeleton(),
            continue45_continue_skeleton()
        );
        assert_eq!(
            continue112_continue_pretty(),
            continue112_continue_minify()
        );

        assert_eq!(
            continue112_try_catch_skeleton("{}", "e", "{}"),
            "try {} catch (e) {}"
        );
        assert_eq!(
            continue112_try_catch_skeleton("{}", "e", "{}"),
            continue45_try_catch_skeleton("{}", "e", "{}")
        );
        assert_eq!(
            continue112_try_catch_pretty("{ a; }", "err", "{ b; }"),
            continue112_try_catch_minify("{ a; }", "err", "{ b; }")
        );

        assert_eq!(
            continue112_try_finally_skeleton("{}", "{}"),
            "try {} finally {}"
        );
        assert_eq!(
            continue112_try_finally_skeleton("{}", "{}"),
            continue45_try_finally_skeleton("{}", "{}")
        );
        assert_eq!(
            continue112_try_finally_pretty("{ a; }", "{ b; }"),
            continue112_try_finally_minify("{ a; }", "{ b; }")
        );
    }

    #[test]
    fn continue112_composed_residual_shells() {
        assert_eq!(continue112_while_empty("t"), "while (t) {}");
        assert_eq!(continue112_do_while_empty("t"), "do {} while (t);");
        assert_eq!(continue112_switch_empty("x"), "switch (x) {}");
        assert_eq!(
            continue112_while_break("t"),
            "while (t) { break; }"
        );
        assert_eq!(
            continue112_while_continue("t"),
            "while (t) { continue; }"
        );
        assert_eq!(
            continue112_while_break_label("t", "L"),
            "while (t) { break L; }"
        );
        assert_eq!(continue112_try_catch_empty("e"), "try {} catch (e) {}");
        assert_eq!(continue112_try_finally_empty(), "try {} finally {}");
        assert_eq!(
            continue112_try_catch_rethrow("{ risky(); }", "e"),
            "try { risky(); } catch (e) { throw e; }"
        );
        assert_eq!(continue112_sep(true), " ");
        assert_eq!(continue112_sep(false), "");
        assert_ne!(continue112_sep(true), continue112_sep(false));
    }
}

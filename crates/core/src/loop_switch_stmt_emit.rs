//! Pure ForStatement + ForIn/Of + While + DoWhile + Switch/Case dual-oracle
//! emission — residual pure **continue80** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - ForStatement dual-oracle pretty/minify clause seps driving real
//!   `for_statement_skeleton` (continue18)
//! - ForInStatement / ForOfStatement dual-oracle (non-await; await-of stays
//!   continue74)
//! - WhileStatement dual-oracle open/close spacing
//! - DoWhileStatement dual-oracle body spacing + semi policy
//! - SwitchCase dual-oracle `case`/`default` colon spacing
//! - SwitchStatement dual-oracle pretty multi-line vs minify compact cases
//!
//! Intentionally does **not** re-wrap continue64–79 partition shells
//! (var/return/if continue79 stays separate; for-await continue74 stays
//! separate; template continue75 stays separate). Composes real shipped pure
//! helpers from `loop_switch_full_emit`.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::loop_switch_full_emit::{
    do_while_statement_skeleton, for_in_statement_skeleton, for_of_statement_skeleton,
    for_statement_skeleton, switch_case_skeleton, switch_statement_skeleton,
    while_statement_skeleton,
};

/// Dual-oracle residual: continue80 related AST type catalog.
pub const CONTINUE80_RELATED_TYPES: &[&str] = &[
    "ForStatement",
    "ForInStatement",
    "ForOfStatement",
    "WhileStatement",
    "DoWhileStatement",
    "SwitchStatement",
    "SwitchCase",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_loop_switch_stmt_related_type(t: &str) -> bool {
    CONTINUE80_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue80_for_type(t: &str) -> bool {
    t == "ForStatement"
}

#[must_use]
pub fn is_continue80_for_in_type(t: &str) -> bool {
    t == "ForInStatement"
}

#[must_use]
pub fn is_continue80_for_of_type(t: &str) -> bool {
    t == "ForOfStatement"
}

#[must_use]
pub fn is_continue80_while_type(t: &str) -> bool {
    t == "WhileStatement"
}

#[must_use]
pub fn is_continue80_do_while_type(t: &str) -> bool {
    t == "DoWhileStatement"
}

#[must_use]
pub fn is_continue80_switch_type(t: &str) -> bool {
    t == "SwitchStatement"
}

#[must_use]
pub fn is_continue80_switch_case_type(t: &str) -> bool {
    t == "SwitchCase"
}

// ── ForStatement dual-oracle ────────────────────────────────────────────────

/// Dual-oracle ForStatement skeleton composing real [`for_statement_skeleton`].
///
/// Pretty: `for (init; test; update) body`; minify: `for(init;test;update)body`.
#[must_use]
pub fn continue80_for_statement_skeleton(
    init: Option<&str>,
    test: Option<&str>,
    update: Option<&str>,
    body: &str,
    pretty: bool,
) -> String {
    for_statement_skeleton(init, test, update, body, pretty)
}

/// Convenience: pretty for.
#[must_use]
pub fn for_statement_pretty(
    init: Option<&str>,
    test: Option<&str>,
    update: Option<&str>,
    body: &str,
) -> String {
    continue80_for_statement_skeleton(init, test, update, body, true)
}

/// Convenience: minify for.
#[must_use]
pub fn for_statement_minify(
    init: Option<&str>,
    test: Option<&str>,
    update: Option<&str>,
    body: &str,
) -> String {
    continue80_for_statement_skeleton(init, test, update, body, false)
}

// ── ForIn / ForOf dual-oracle ───────────────────────────────────────────────

/// Dual-oracle ForInStatement skeleton composing real [`for_in_statement_skeleton`].
#[must_use]
pub fn continue80_for_in_statement_skeleton(
    left: &str,
    right: &str,
    body: &str,
    pretty: bool,
) -> String {
    for_in_statement_skeleton(left, right, body, pretty)
}

/// Convenience: pretty for-in.
#[must_use]
pub fn for_in_statement_pretty(left: &str, right: &str, body: &str) -> String {
    continue80_for_in_statement_skeleton(left, right, body, true)
}

/// Convenience: minify for-in.
#[must_use]
pub fn for_in_statement_minify(left: &str, right: &str, body: &str) -> String {
    continue80_for_in_statement_skeleton(left, right, body, false)
}

/// Dual-oracle ForOfStatement skeleton composing real [`for_of_statement_skeleton`].
///
/// Non-await only; `for await (… of …)` stays continue74.
#[must_use]
pub fn continue80_for_of_statement_skeleton(
    left: &str,
    right: &str,
    body: &str,
    pretty: bool,
) -> String {
    for_of_statement_skeleton(left, right, body, pretty)
}

/// Convenience: pretty for-of.
#[must_use]
pub fn for_of_statement_pretty(left: &str, right: &str, body: &str) -> String {
    continue80_for_of_statement_skeleton(left, right, body, true)
}

/// Convenience: minify for-of.
#[must_use]
pub fn for_of_statement_minify(left: &str, right: &str, body: &str) -> String {
    continue80_for_of_statement_skeleton(left, right, body, false)
}

// ── While / DoWhile dual-oracle ─────────────────────────────────────────────

/// Dual-oracle WhileStatement skeleton composing real [`while_statement_skeleton`].
#[must_use]
pub fn continue80_while_statement_skeleton(test: &str, body: &str, pretty: bool) -> String {
    while_statement_skeleton(test, body, pretty)
}

/// Convenience: pretty while.
#[must_use]
pub fn while_statement_pretty(test: &str, body: &str) -> String {
    continue80_while_statement_skeleton(test, body, true)
}

/// Convenience: minify while.
#[must_use]
pub fn while_statement_minify(test: &str, body: &str) -> String {
    continue80_while_statement_skeleton(test, body, false)
}

/// Dual-oracle DoWhileStatement skeleton composing real
/// [`do_while_statement_skeleton`].
#[must_use]
pub fn continue80_do_while_statement_skeleton(
    body: &str,
    test: &str,
    pretty: bool,
    semi: bool,
) -> String {
    do_while_statement_skeleton(body, test, pretty, semi)
}

/// Convenience: pretty do-while with semi.
#[must_use]
pub fn do_while_statement_pretty(body: &str, test: &str) -> String {
    continue80_do_while_statement_skeleton(body, test, true, true)
}

/// Convenience: minify do-while with semi.
#[must_use]
pub fn do_while_statement_minify(body: &str, test: &str) -> String {
    continue80_do_while_statement_skeleton(body, test, false, true)
}

// ── SwitchCase / SwitchStatement dual-oracle ────────────────────────────────

/// Dual-oracle SwitchCase skeleton composing real [`switch_case_skeleton`].
///
/// `test = Some` → `case test: …`; `None` → `default: …`.
#[must_use]
pub fn continue80_switch_case_skeleton(
    test: Option<&str>,
    consequent: &str,
    pretty: bool,
) -> String {
    switch_case_skeleton(test, consequent, pretty)
}

/// Convenience: pretty case.
#[must_use]
pub fn switch_case_pretty(test: &str, consequent: &str) -> String {
    continue80_switch_case_skeleton(Some(test), consequent, true)
}

/// Convenience: minify case.
#[must_use]
pub fn switch_case_minify(test: &str, consequent: &str) -> String {
    continue80_switch_case_skeleton(Some(test), consequent, false)
}

/// Convenience: pretty default case.
#[must_use]
pub fn switch_default_pretty(consequent: &str) -> String {
    continue80_switch_case_skeleton(None, consequent, true)
}

/// Convenience: minify default case.
#[must_use]
pub fn switch_default_minify(consequent: &str) -> String {
    continue80_switch_case_skeleton(None, consequent, false)
}

/// Dual-oracle SwitchStatement skeleton composing real
/// [`switch_statement_skeleton`].
#[must_use]
pub fn continue80_switch_statement_skeleton(
    discriminant: &str,
    cases: &[&str],
    pretty: bool,
) -> String {
    switch_statement_skeleton(discriminant, cases, pretty)
}

/// Convenience: pretty switch.
#[must_use]
pub fn switch_statement_pretty(discriminant: &str, cases: &[&str]) -> String {
    continue80_switch_statement_skeleton(discriminant, cases, true)
}

/// Convenience: minify switch.
#[must_use]
pub fn switch_statement_minify(discriminant: &str, cases: &[&str]) -> String {
    continue80_switch_statement_skeleton(discriminant, cases, false)
}

/// Dual-oracle residual: compose for + while chain (loop nest pure half).
#[must_use]
pub fn continue80_for_then_while(
    init: Option<&str>,
    test: Option<&str>,
    update: Option<&str>,
    while_test: &str,
    while_body: &str,
    pretty: bool,
) -> String {
    let inner = continue80_while_statement_skeleton(while_test, while_body, pretty);
    continue80_for_statement_skeleton(init, test, update, &inner, pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loop_switch_full_emit::{
        do_while_statement_skeleton, for_in_statement_skeleton, for_of_statement_skeleton,
        for_statement_skeleton, switch_case_skeleton, switch_statement_skeleton,
        while_statement_skeleton,
    };

    #[test]
    fn continue80_type_catalog() {
        assert_eq!(CONTINUE80_RELATED_TYPES.len(), 7);
        assert!(is_loop_switch_stmt_related_type("ForStatement"));
        assert!(is_loop_switch_stmt_related_type("ForInStatement"));
        assert!(is_loop_switch_stmt_related_type("ForOfStatement"));
        assert!(is_loop_switch_stmt_related_type("WhileStatement"));
        assert!(is_loop_switch_stmt_related_type("DoWhileStatement"));
        assert!(is_loop_switch_stmt_related_type("SwitchStatement"));
        assert!(is_loop_switch_stmt_related_type("SwitchCase"));
        assert!(!is_loop_switch_stmt_related_type("IfStatement"));
        assert!(!is_loop_switch_stmt_related_type("ForAwaitStatement"));
        assert!(!is_loop_switch_stmt_related_type("VariableDeclaration"));
        assert!(is_continue80_for_type("ForStatement"));
        assert!(is_continue80_for_in_type("ForInStatement"));
        assert!(is_continue80_for_of_type("ForOfStatement"));
        assert!(is_continue80_while_type("WhileStatement"));
        assert!(is_continue80_do_while_type("DoWhileStatement"));
        assert!(is_continue80_switch_type("SwitchStatement"));
        assert!(is_continue80_switch_case_type("SwitchCase"));
        assert!(!is_continue80_for_type("ForOfStatement"));
        assert!(!is_continue80_switch_case_type("SwitchStatement"));
    }

    #[test]
    fn continue80_for_statement_dual_oracle() {
        assert_eq!(
            for_statement_pretty(Some("let i=0"), Some("i<3"), Some("i++"), "{}"),
            "for (let i=0; i<3; i++) {}"
        );
        assert_eq!(
            for_statement_minify(Some("let i=0"), Some("i<3"), Some("i++"), "{}"),
            "for(let i=0;i<3;i++){}"
        );
        assert_eq!(
            continue80_for_statement_skeleton(None, None, None, "x()", true),
            for_statement_skeleton(None, None, None, "x()", true)
        );
        assert_eq!(
            continue80_for_statement_skeleton(
                Some("i=0"),
                Some("i<1"),
                Some("i++"),
                "x()",
                false
            ),
            for_statement_skeleton(Some("i=0"), Some("i<1"), Some("i++"), "x()", false)
        );
        assert_ne!(
            for_statement_pretty(Some("i=0"), Some("i<1"), Some("i++"), "{}"),
            for_statement_minify(Some("i=0"), Some("i<1"), Some("i++"), "{}")
        );
    }

    #[test]
    fn continue80_for_in_of_while_do_dual_oracle() {
        assert_eq!(
            for_in_statement_pretty("k", "obj", "{}"),
            "for (k in obj) {}"
        );
        assert_eq!(
            for_in_statement_minify("k", "obj", "{}"),
            "for(k in obj){}"
        );
        assert_eq!(
            continue80_for_in_statement_skeleton("a", "b", "c()", true),
            for_in_statement_skeleton("a", "b", "c()", true)
        );

        assert_eq!(
            for_of_statement_pretty("x", "xs", "{}"),
            "for (x of xs) {}"
        );
        assert_eq!(
            for_of_statement_minify("x", "xs", "{}"),
            "for(x of xs){}"
        );
        assert_eq!(
            continue80_for_of_statement_skeleton("v", "arr", "f()", false),
            for_of_statement_skeleton("v", "arr", "f()", false)
        );

        assert_eq!(while_statement_pretty("ok", "work()"), "while (ok) work()");
        assert_eq!(while_statement_minify("ok", "work()"), "while(ok)work()");
        assert_eq!(
            continue80_while_statement_skeleton("t", "b", true),
            while_statement_skeleton("t", "b", true)
        );

        assert_eq!(
            do_while_statement_pretty("{}", "x"),
            "do {} while (x);"
        );
        assert_eq!(
            do_while_statement_minify("{}", "x"),
            "do{}while(x);"
        );
        assert_eq!(
            continue80_do_while_statement_skeleton("{}", "y", true, false),
            do_while_statement_skeleton("{}", "y", true, false)
        );
        assert_ne!(
            while_statement_pretty("x", "y()"),
            while_statement_minify("x", "y()")
        );
    }

    #[test]
    fn continue80_switch_dual_oracle() {
        assert_eq!(switch_case_pretty("1", "break;"), "case 1: break;");
        assert_eq!(switch_case_minify("1", "break;"), "case 1:break;");
        assert_eq!(switch_default_pretty("return;"), "default: return;");
        assert_eq!(switch_default_minify("return;"), "default:return;");
        assert_eq!(
            continue80_switch_case_skeleton(Some("2"), "x();", true),
            switch_case_skeleton(Some("2"), "x();", true)
        );
        assert_eq!(
            continue80_switch_case_skeleton(None, "y();", false),
            switch_case_skeleton(None, "y();", false)
        );

        let case_p = switch_case_pretty("1", "break;");
        let def_p = switch_default_pretty("return;");
        let pretty = switch_statement_pretty("x", &[case_p.as_str(), def_p.as_str()]);
        assert_eq!(
            pretty,
            "switch (x) {\n  case 1: break;\n  default: return;\n}"
        );

        let case_m = switch_case_minify("1", "break;");
        let def_m = switch_default_minify("return;");
        assert_eq!(
            switch_statement_minify("x", &[case_m.as_str(), def_m.as_str()]),
            "switch(x){case 1:break;default:return;}"
        );
        assert_eq!(
            continue80_switch_statement_skeleton("d", &["case 0:z;"], false),
            switch_statement_skeleton("d", &["case 0:z;"], false)
        );
        assert_ne!(
            switch_statement_pretty("x", &["case 1: break;"]),
            switch_statement_minify("x", &["case 1:break;"])
        );

        assert_eq!(
            continue80_for_then_while(
                Some("i=0"),
                Some("i<1"),
                Some("i++"),
                "ready",
                "tick()",
                true
            ),
            "for (i=0; i<1; i++) while (ready) tick()"
        );
        assert_eq!(
            continue80_for_then_while(None, None, None, "ok", "x()", false),
            "for(;;)while(ok)x()"
        );
    }
}

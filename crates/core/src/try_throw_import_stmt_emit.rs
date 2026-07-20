//! Pure Try + Catch + Throw + Labeled + Break + Continue + With + Import
//! dual-oracle emission — residual pure **continue81** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - TryStatement dual-oracle pretty/minify catch/finally spacing driving real
//!   `try_statement_skeleton` (continue17)
//! - CatchClause dual-oracle open/param-close spacing
//! - ThrowStatement dual-oracle semi policy
//! - LabeledStatement dual-oracle colon spacing
//! - BreakStatement / ContinueStatement dual-oracle optional label + semi
//! - WithStatement dual-oracle open/close spacing
//! - ImportDeclaration dual-oracle pretty/minify full skeleton
//!
//! Intentionally does **not** re-wrap continue64–80 partition shells
//! (for/while/switch continue80 stays separate; var/return/if continue79 stays
//! separate; import-expression continue75 stays separate). Composes real shipped
//! pure helpers from `try_import_emit`.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::try_import_emit::{
    break_continue_skeleton, import_declaration_full_skeleton, labeled_statement_skeleton,
    throw_statement_skeleton, try_statement_skeleton, with_close, with_open,
};

/// Dual-oracle residual: continue81 related AST type catalog.
pub const CONTINUE81_RELATED_TYPES: &[&str] = &[
    "TryStatement",
    "CatchClause",
    "ThrowStatement",
    "LabeledStatement",
    "BreakStatement",
    "ContinueStatement",
    "WithStatement",
    "ImportDeclaration",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_try_throw_import_stmt_related_type(t: &str) -> bool {
    CONTINUE81_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue81_try_type(t: &str) -> bool {
    t == "TryStatement"
}

#[must_use]
pub fn is_continue81_catch_type(t: &str) -> bool {
    t == "CatchClause"
}

#[must_use]
pub fn is_continue81_throw_type(t: &str) -> bool {
    t == "ThrowStatement"
}

#[must_use]
pub fn is_continue81_labeled_type(t: &str) -> bool {
    t == "LabeledStatement"
}

#[must_use]
pub fn is_continue81_break_type(t: &str) -> bool {
    t == "BreakStatement"
}

#[must_use]
pub fn is_continue81_continue_type(t: &str) -> bool {
    t == "ContinueStatement"
}

#[must_use]
pub fn is_continue81_with_type(t: &str) -> bool {
    t == "WithStatement"
}

#[must_use]
pub fn is_continue81_import_declaration_type(t: &str) -> bool {
    t == "ImportDeclaration"
}

// ── TryStatement dual-oracle ────────────────────────────────────────────────

/// Dual-oracle TryStatement skeleton composing real [`try_statement_skeleton`].
#[must_use]
pub fn continue81_try_statement_skeleton(
    try_block: &str,
    catch_param: Option<&str>,
    catch_block: Option<&str>,
    finally_block: Option<&str>,
    pretty: bool,
) -> String {
    try_statement_skeleton(try_block, catch_param, catch_block, finally_block, pretty)
}

/// Convenience: pretty try/catch.
#[must_use]
pub fn try_catch_pretty(try_block: &str, catch_param: &str, catch_block: &str) -> String {
    continue81_try_statement_skeleton(
        try_block,
        Some(catch_param),
        Some(catch_block),
        None,
        true,
    )
}

/// Convenience: minify try/catch.
#[must_use]
pub fn try_catch_minify(try_block: &str, catch_param: &str, catch_block: &str) -> String {
    continue81_try_statement_skeleton(
        try_block,
        Some(catch_param),
        Some(catch_block),
        None,
        false,
    )
}

/// Convenience: pretty try/finally (no catch).
#[must_use]
pub fn try_finally_pretty(try_block: &str, finally_block: &str) -> String {
    continue81_try_statement_skeleton(try_block, None, None, Some(finally_block), true)
}

/// Convenience: minify try/finally.
#[must_use]
pub fn try_finally_minify(try_block: &str, finally_block: &str) -> String {
    continue81_try_statement_skeleton(try_block, None, None, Some(finally_block), false)
}

// ── ThrowStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThrowStatement skeleton composing real [`throw_statement_skeleton`].
#[must_use]
pub fn continue81_throw_statement_skeleton(expr: &str, semi: bool) -> String {
    throw_statement_skeleton(expr, semi)
}

/// Convenience: throw with semi.
#[must_use]
pub fn throw_statement_semi(expr: &str) -> String {
    continue81_throw_statement_skeleton(expr, true)
}

/// Convenience: throw without semi.
#[must_use]
pub fn throw_statement_bare(expr: &str) -> String {
    continue81_throw_statement_skeleton(expr, false)
}

// ── LabeledStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle LabeledStatement skeleton composing real [`labeled_statement_skeleton`].
#[must_use]
pub fn continue81_labeled_statement_skeleton(label: &str, body: &str, pretty: bool) -> String {
    labeled_statement_skeleton(label, body, pretty)
}

/// Convenience: pretty labeled.
#[must_use]
pub fn labeled_statement_pretty(label: &str, body: &str) -> String {
    continue81_labeled_statement_skeleton(label, body, true)
}

/// Convenience: minify labeled.
#[must_use]
pub fn labeled_statement_minify(label: &str, body: &str) -> String {
    continue81_labeled_statement_skeleton(label, body, false)
}

// ── Break / Continue dual-oracle ────────────────────────────────────────────

/// Dual-oracle BreakStatement skeleton.
#[must_use]
pub fn continue81_break_statement_skeleton(label: Option<&str>, semi: bool) -> String {
    break_continue_skeleton("break", label, semi)
}

/// Dual-oracle ContinueStatement skeleton.
#[must_use]
pub fn continue81_continue_statement_skeleton(label: Option<&str>, semi: bool) -> String {
    break_continue_skeleton("continue", label, semi)
}

/// Convenience: break;
#[must_use]
pub fn break_statement_semi() -> String {
    continue81_break_statement_skeleton(None, true)
}

/// Convenience: continue label;
#[must_use]
pub fn continue_statement_labeled_semi(label: &str) -> String {
    continue81_continue_statement_skeleton(Some(label), true)
}

// ── WithStatement dual-oracle ───────────────────────────────────────────────

/// Dual-oracle WithStatement skeleton: `with (obj) body` / `with(obj)body`.
#[must_use]
pub fn continue81_with_statement_skeleton(object: &str, body: &str, pretty: bool) -> String {
    let mut out = String::from(with_open(pretty));
    out.push_str(object);
    out.push_str(with_close(pretty));
    out.push_str(body);
    out
}

/// Convenience: pretty with.
#[must_use]
pub fn with_statement_pretty(object: &str, body: &str) -> String {
    continue81_with_statement_skeleton(object, body, true)
}

/// Convenience: minify with.
#[must_use]
pub fn with_statement_minify(object: &str, body: &str) -> String {
    continue81_with_statement_skeleton(object, body, false)
}

// ── ImportDeclaration dual-oracle ───────────────────────────────────────────

/// Dual-oracle ImportDeclaration skeleton composing real
/// [`import_declaration_full_skeleton`].
#[must_use]
pub fn continue81_import_declaration_skeleton(
    default_local: Option<&str>,
    namespace_local: Option<&str>,
    named: &[&str],
    source: &str,
    pretty: bool,
    semi: bool,
    single_quote: bool,
) -> String {
    import_declaration_full_skeleton(
        default_local,
        namespace_local,
        named,
        source,
        pretty,
        semi,
        single_quote,
    )
}

/// Convenience: pretty named import with semi.
#[must_use]
pub fn import_named_pretty(named: &[&str], source: &str) -> String {
    continue81_import_declaration_skeleton(None, None, named, source, true, true, false)
}

/// Convenience: minify named import with semi.
#[must_use]
pub fn import_named_minify(named: &[&str], source: &str) -> String {
    continue81_import_declaration_skeleton(None, None, named, source, false, true, false)
}

/// Convenience: side-effect import pretty.
#[must_use]
pub fn import_side_effect_pretty(source: &str) -> String {
    continue81_import_declaration_skeleton(None, None, &[], source, true, true, false)
}

/// Dual-oracle residual: compose try + throw chain (error path pure half).
#[must_use]
pub fn continue81_try_then_throw(
    try_block: &str,
    catch_param: &str,
    throw_expr: &str,
    pretty: bool,
) -> String {
    let catch_body = continue81_throw_statement_skeleton(throw_expr, true);
    continue81_try_statement_skeleton(
        try_block,
        Some(catch_param),
        Some(&catch_body),
        None,
        pretty,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::try_import_emit::{
        break_continue_skeleton, import_declaration_full_skeleton, labeled_statement_skeleton,
        throw_statement_skeleton, try_statement_skeleton,
    };

    #[test]
    fn continue81_type_catalog() {
        assert_eq!(CONTINUE81_RELATED_TYPES.len(), 8);
        assert!(is_try_throw_import_stmt_related_type("TryStatement"));
        assert!(is_try_throw_import_stmt_related_type("CatchClause"));
        assert!(is_try_throw_import_stmt_related_type("ThrowStatement"));
        assert!(is_try_throw_import_stmt_related_type("LabeledStatement"));
        assert!(is_try_throw_import_stmt_related_type("BreakStatement"));
        assert!(is_try_throw_import_stmt_related_type("ContinueStatement"));
        assert!(is_try_throw_import_stmt_related_type("WithStatement"));
        assert!(is_try_throw_import_stmt_related_type("ImportDeclaration"));
        assert!(!is_try_throw_import_stmt_related_type("ForStatement"));
        assert!(!is_try_throw_import_stmt_related_type("ImportExpression"));
        assert!(!is_try_throw_import_stmt_related_type("IfStatement"));
        assert!(is_continue81_try_type("TryStatement"));
        assert!(is_continue81_catch_type("CatchClause"));
        assert!(is_continue81_throw_type("ThrowStatement"));
        assert!(is_continue81_labeled_type("LabeledStatement"));
        assert!(is_continue81_break_type("BreakStatement"));
        assert!(is_continue81_continue_type("ContinueStatement"));
        assert!(is_continue81_with_type("WithStatement"));
        assert!(is_continue81_import_declaration_type("ImportDeclaration"));
        assert!(!is_continue81_try_type("CatchClause"));
        assert!(!is_continue81_import_declaration_type("ImportSpecifier"));
    }

    #[test]
    fn continue81_try_throw_labeled_dual_oracle() {
        assert_eq!(
            try_catch_pretty("{}", "e", "{}"),
            "try {} catch (e) {}"
        );
        assert_eq!(
            try_catch_minify("{}", "e", "{}"),
            "try {}catch(e){}"
        );
        assert_eq!(
            continue81_try_statement_skeleton("{}", Some("e"), Some("{}"), None, true),
            try_statement_skeleton("{}", Some("e"), Some("{}"), None, true)
        );
        assert_eq!(
            continue81_try_statement_skeleton("{}", Some("e"), Some("{}"), None, false),
            try_statement_skeleton("{}", Some("e"), Some("{}"), None, false)
        );
        assert_eq!(try_finally_pretty("{}", "{}"), "try {} finally {}");
        assert_eq!(try_finally_minify("{}", "{}"), "try {}finally{}");
        assert_ne!(
            try_catch_pretty("{}", "e", "{}"),
            try_catch_minify("{}", "e", "{}")
        );

        assert_eq!(throw_statement_semi("err"), "throw err;");
        assert_eq!(throw_statement_bare("err"), "throw err");
        assert_eq!(
            continue81_throw_statement_skeleton("x", true),
            throw_statement_skeleton("x", true)
        );

        assert_eq!(
            labeled_statement_pretty("loop", "break;"),
            "loop: break;"
        );
        assert_eq!(
            labeled_statement_minify("loop", "break;"),
            "loop:break;"
        );
        assert_eq!(
            continue81_labeled_statement_skeleton("L", "x()", true),
            labeled_statement_skeleton("L", "x()", true)
        );
        assert_ne!(
            labeled_statement_pretty("a", "b"),
            labeled_statement_minify("a", "b")
        );
    }

    #[test]
    fn continue81_break_with_import_dual_oracle() {
        assert_eq!(break_statement_semi(), "break;");
        assert_eq!(
            continue81_break_statement_skeleton(Some("loop"), true),
            "break loop;"
        );
        assert_eq!(
            continue_statement_labeled_semi("outer"),
            "continue outer;"
        );
        assert_eq!(
            continue81_continue_statement_skeleton(None, false),
            "continue"
        );
        assert_eq!(
            continue81_break_statement_skeleton(None, true),
            break_continue_skeleton("break", None, true)
        );

        assert_eq!(with_statement_pretty("obj", "{}"), "with (obj) {}");
        assert_eq!(with_statement_minify("obj", "{}"), "with(obj){}");
        assert_ne!(
            with_statement_pretty("x", "y()"),
            with_statement_minify("x", "y()")
        );

        assert_eq!(
            import_named_pretty(&["a", "b"], "mod"),
            import_declaration_full_skeleton(None, None, &["a", "b"], "mod", true, true, false)
        );
        assert_eq!(
            import_named_minify(&["a", "b"], "mod"),
            import_declaration_full_skeleton(None, None, &["a", "b"], "mod", false, true, false)
        );
        assert_eq!(
            continue81_import_declaration_skeleton(
                Some("Def"),
                None,
                &["x"],
                "pkg",
                true,
                true,
                false
            ),
            import_declaration_full_skeleton(Some("Def"), None, &["x"], "pkg", true, true, false)
        );
        assert_eq!(
            import_side_effect_pretty("side-effect"),
            "import \"side-effect\";"
        );
        assert_ne!(
            import_named_pretty(&["a"], "m"),
            import_named_minify(&["a"], "m")
        );

        assert_eq!(
            continue81_try_then_throw("{}", "e", "e", true),
            "try {} catch (e) throw e;"
        );
        assert_eq!(
            continue81_try_then_throw("{}", "e", "e", false),
            "try {}catch(e)throw e;"
        );
    }
}

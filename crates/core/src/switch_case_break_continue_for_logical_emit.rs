//! Pure SwitchCase + BreakStatement + ContinueStatement + ForInStatement +
//! ForOfStatement + LogicalExpression dual-oracle emission — residual pure
//! **continue106** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–105:
//! - SwitchCase dual-oracle composing real `continue39_switch_case_skeleton`
//!   (optional test → case/default)
//! - BreakStatement dual-oracle composing real `continue39_break_skeleton`
//! - ContinueStatement dual-oracle composing real
//!   `continue39_continue_skeleton`
//! - ForInStatement dual-oracle composing real `continue39_for_in_skeleton`
//! - ForOfStatement dual-oracle composing real `continue39_for_of_skeleton`
//! - LogicalExpression dual-oracle composing real
//!   `continue39_logical_skeleton`
//! - Composed switch-case/break/continue/for-in-of/logical residual shells
//!
//! Intentionally does **not** re-wrap continue98 unary/binary/await/jump poles,
//! continue99 switch/try poles, continue102 if/while/do/conditional/labeled/
//! with poles, continue105 switch-case via continue38 (required test), or
//! continue32–38 bases. Composes real shipped pure helpers from continue39
//! bases. Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue39_break_skeleton, continue39_continue_skeleton, continue39_for_in_skeleton,
    continue39_for_of_skeleton, continue39_logical_skeleton, continue39_switch_case_skeleton,
};

/// Dual-oracle residual: continue106 related AST type catalog.
pub const CONTINUE106_RELATED_TYPES: &[&str] = &[
    "SwitchCase",
    "BreakStatement",
    "ContinueStatement",
    "ForInStatement",
    "ForOfStatement",
    "LogicalExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_switch_case_break_continue_for_logical_related_type(t: &str) -> bool {
    CONTINUE106_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue106_switch_case_type(t: &str) -> bool {
    t == "SwitchCase"
}

#[must_use]
pub fn is_continue106_break_type(t: &str) -> bool {
    t == "BreakStatement"
}

#[must_use]
pub fn is_continue106_continue_type(t: &str) -> bool {
    t == "ContinueStatement"
}

#[must_use]
pub fn is_continue106_for_in_type(t: &str) -> bool {
    t == "ForInStatement"
}

#[must_use]
pub fn is_continue106_for_of_type(t: &str) -> bool {
    t == "ForOfStatement"
}

#[must_use]
pub fn is_continue106_logical_type(t: &str) -> bool {
    t == "LogicalExpression"
}

#[must_use]
pub fn is_continue106_jump_type(t: &str) -> bool {
    matches!(t, "BreakStatement" | "ContinueStatement")
}

#[must_use]
pub fn is_continue106_for_enum_type(t: &str) -> bool {
    matches!(t, "ForInStatement" | "ForOfStatement")
}

// ── SwitchCase dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle SwitchCase skeleton composing real
/// [`continue39_switch_case_skeleton`] (Some test → case, None → default).
#[must_use]
pub fn continue106_switch_case_skeleton(test: Option<&str>, body: &str) -> String {
    continue39_switch_case_skeleton(test, body)
}

/// Dual-oracle SwitchCase pretty alias.
#[must_use]
pub fn continue106_switch_case_pretty(test: Option<&str>, body: &str) -> String {
    continue106_switch_case_skeleton(test, body)
}

/// Dual-oracle SwitchCase minify alias.
#[must_use]
pub fn continue106_switch_case_minify(test: Option<&str>, body: &str) -> String {
    continue106_switch_case_skeleton(test, body)
}

// ── BreakStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BreakStatement skeleton composing real
/// [`continue39_break_skeleton`].
#[must_use]
pub fn continue106_break_skeleton(label: Option<&str>) -> String {
    continue39_break_skeleton(label)
}

/// Dual-oracle BreakStatement pretty alias.
#[must_use]
pub fn continue106_break_pretty(label: Option<&str>) -> String {
    continue106_break_skeleton(label)
}

/// Dual-oracle BreakStatement minify alias.
#[must_use]
pub fn continue106_break_minify(label: Option<&str>) -> String {
    continue106_break_skeleton(label)
}

// ── ContinueStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle ContinueStatement skeleton composing real
/// [`continue39_continue_skeleton`].
#[must_use]
pub fn continue106_continue_skeleton(label: Option<&str>) -> String {
    continue39_continue_skeleton(label)
}

/// Dual-oracle ContinueStatement pretty alias.
#[must_use]
pub fn continue106_continue_pretty(label: Option<&str>) -> String {
    continue106_continue_skeleton(label)
}

/// Dual-oracle ContinueStatement minify alias.
#[must_use]
pub fn continue106_continue_minify(label: Option<&str>) -> String {
    continue106_continue_skeleton(label)
}

// ── ForInStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ForInStatement skeleton composing real
/// [`continue39_for_in_skeleton`].
#[must_use]
pub fn continue106_for_in_skeleton(left: &str, right: &str, body: &str) -> String {
    continue39_for_in_skeleton(left, right, body)
}

/// Dual-oracle ForInStatement pretty alias.
#[must_use]
pub fn continue106_for_in_pretty(left: &str, right: &str, body: &str) -> String {
    continue106_for_in_skeleton(left, right, body)
}

/// Dual-oracle ForInStatement minify alias.
#[must_use]
pub fn continue106_for_in_minify(left: &str, right: &str, body: &str) -> String {
    continue106_for_in_skeleton(left, right, body)
}

// ── ForOfStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ForOfStatement skeleton composing real
/// [`continue39_for_of_skeleton`].
#[must_use]
pub fn continue106_for_of_skeleton(left: &str, right: &str, body: &str) -> String {
    continue39_for_of_skeleton(left, right, body)
}

/// Dual-oracle ForOfStatement pretty alias.
#[must_use]
pub fn continue106_for_of_pretty(left: &str, right: &str, body: &str) -> String {
    continue106_for_of_skeleton(left, right, body)
}

/// Dual-oracle ForOfStatement minify alias.
#[must_use]
pub fn continue106_for_of_minify(left: &str, right: &str, body: &str) -> String {
    continue106_for_of_skeleton(left, right, body)
}

// ── LogicalExpression dual-oracle ───────────────────────────────────────────

/// Dual-oracle LogicalExpression skeleton composing real
/// [`continue39_logical_skeleton`].
#[must_use]
pub fn continue106_logical_skeleton(left: &str, op: &str, right: &str) -> String {
    continue39_logical_skeleton(left, op, right)
}

/// Dual-oracle LogicalExpression pretty alias.
#[must_use]
pub fn continue106_logical_pretty(left: &str, op: &str, right: &str) -> String {
    continue106_logical_skeleton(left, op, right)
}

/// Dual-oracle LogicalExpression minify alias.
#[must_use]
pub fn continue106_logical_minify(left: &str, op: &str, right: &str) -> String {
    continue106_logical_skeleton(left, op, right)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: switch case whose body is bare break.
#[must_use]
pub fn continue106_case_break(test: &str) -> String {
    let br = continue106_break_skeleton(None);
    continue106_switch_case_skeleton(Some(test), &br)
}

/// Dual-oracle residual: switch default whose body is bare continue.
#[must_use]
pub fn continue106_default_continue() -> String {
    let c = continue106_continue_skeleton(None);
    continue106_switch_case_skeleton(None, &c)
}

/// Dual-oracle residual: labeled break.
#[must_use]
pub fn continue106_break_label(label: &str) -> String {
    continue106_break_skeleton(Some(label))
}

/// Dual-oracle residual: labeled continue.
#[must_use]
pub fn continue106_continue_label(label: &str) -> String {
    continue106_continue_skeleton(Some(label))
}

/// Dual-oracle residual: for-in whose body is bare break.
#[must_use]
pub fn continue106_for_in_break(left: &str, right: &str) -> String {
    let br = continue106_break_skeleton(None);
    continue106_for_in_skeleton(left, right, &br)
}

/// Dual-oracle residual: for-of whose body is bare continue.
#[must_use]
pub fn continue106_for_of_continue(left: &str, right: &str) -> String {
    let c = continue106_continue_skeleton(None);
    continue106_for_of_skeleton(left, right, &c)
}

/// Dual-oracle residual: logical AND.
#[must_use]
pub fn continue106_logical_and(left: &str, right: &str) -> String {
    continue106_logical_skeleton(left, "&&", right)
}

/// Dual-oracle residual: logical OR.
#[must_use]
pub fn continue106_logical_or(left: &str, right: &str) -> String {
    continue106_logical_skeleton(left, "||", right)
}

/// Dual-oracle residual: for-of body gated by logical AND.
#[must_use]
pub fn continue106_for_of_logical_guard(
    left: &str,
    right: &str,
    guard_left: &str,
    guard_right: &str,
    body: &str,
) -> String {
    let guard = continue106_logical_and(guard_left, guard_right);
    let stmt = format!("if ({guard}) {{ {body} }}");
    continue106_for_of_skeleton(left, right, &stmt)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue106_stmt_sep(pretty: bool) -> &'static str {
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
        continue39_break_skeleton, continue39_continue_skeleton, continue39_for_in_skeleton,
        continue39_for_of_skeleton, continue39_logical_skeleton, continue39_switch_case_skeleton,
    };

    #[test]
    fn continue106_type_catalog() {
        assert_eq!(CONTINUE106_RELATED_TYPES.len(), 6);
        assert!(is_switch_case_break_continue_for_logical_related_type(
            "SwitchCase"
        ));
        assert!(is_switch_case_break_continue_for_logical_related_type(
            "BreakStatement"
        ));
        assert!(is_switch_case_break_continue_for_logical_related_type(
            "ContinueStatement"
        ));
        assert!(is_switch_case_break_continue_for_logical_related_type(
            "ForInStatement"
        ));
        assert!(is_switch_case_break_continue_for_logical_related_type(
            "ForOfStatement"
        ));
        assert!(is_switch_case_break_continue_for_logical_related_type(
            "LogicalExpression"
        ));
        assert!(!is_switch_case_break_continue_for_logical_related_type(
            "TryStatement"
        ));
        assert!(!is_switch_case_break_continue_for_logical_related_type(
            "MethodDefinition"
        ));

        assert!(is_continue106_switch_case_type("SwitchCase"));
        assert!(!is_continue106_switch_case_type("SwitchStatement"));
        assert!(is_continue106_break_type("BreakStatement"));
        assert!(is_continue106_continue_type("ContinueStatement"));
        assert!(is_continue106_for_in_type("ForInStatement"));
        assert!(is_continue106_for_of_type("ForOfStatement"));
        assert!(is_continue106_logical_type("LogicalExpression"));
        assert!(is_continue106_jump_type("BreakStatement"));
        assert!(is_continue106_jump_type("ContinueStatement"));
        assert!(!is_continue106_jump_type("ForInStatement"));
        assert!(is_continue106_for_enum_type("ForInStatement"));
        assert!(is_continue106_for_enum_type("ForOfStatement"));
        assert!(!is_continue106_for_enum_type("LogicalExpression"));
    }

    #[test]
    fn continue106_switch_case_break_continue_emit() {
        assert_eq!(
            continue106_switch_case_skeleton(Some("1"), "break;"),
            "case 1: break;"
        );
        assert_eq!(
            continue106_switch_case_skeleton(None, "return;"),
            "default: return;"
        );
        assert_eq!(
            continue106_switch_case_skeleton(Some("1"), "break;"),
            continue39_switch_case_skeleton(Some("1"), "break;")
        );
        assert_eq!(
            continue106_switch_case_pretty(Some("x"), "return;"),
            continue106_switch_case_minify(Some("x"), "return;")
        );

        assert_eq!(continue106_break_skeleton(None), "break;");
        assert_eq!(continue106_break_skeleton(Some("outer")), "break outer;");
        assert_eq!(
            continue106_break_skeleton(Some("outer")),
            continue39_break_skeleton(Some("outer"))
        );
        assert_eq!(
            continue106_break_pretty(None),
            continue106_break_minify(None)
        );

        assert_eq!(continue106_continue_skeleton(None), "continue;");
        assert_eq!(
            continue106_continue_skeleton(Some("loop")),
            "continue loop;"
        );
        assert_eq!(
            continue106_continue_skeleton(Some("loop")),
            continue39_continue_skeleton(Some("loop"))
        );
        assert_eq!(
            continue106_continue_pretty(None),
            continue106_continue_minify(None)
        );
    }

    #[test]
    fn continue106_for_logical_emit() {
        assert_eq!(
            continue106_for_in_skeleton("const k", "obj", "use(k);"),
            "for (const k in obj) { use(k); }"
        );
        assert_eq!(
            continue106_for_in_skeleton("const k", "obj", "use(k);"),
            continue39_for_in_skeleton("const k", "obj", "use(k);")
        );
        assert_eq!(
            continue106_for_in_pretty("a", "b", "c;"),
            continue106_for_in_minify("a", "b", "c;")
        );

        assert_eq!(
            continue106_for_of_skeleton("const x", "arr", "use(x);"),
            "for (const x of arr) { use(x); }"
        );
        assert_eq!(
            continue106_for_of_skeleton("const x", "arr", "use(x);"),
            continue39_for_of_skeleton("const x", "arr", "use(x);")
        );
        assert_eq!(
            continue106_for_of_pretty("a", "b", "c;"),
            continue106_for_of_minify("a", "b", "c;")
        );

        assert_eq!(continue106_logical_skeleton("a", "&&", "b"), "a && b");
        assert_eq!(continue106_logical_skeleton("a", "||", "b"), "a || b");
        assert_eq!(
            continue106_logical_skeleton("a", "&&", "b"),
            continue39_logical_skeleton("a", "&&", "b")
        );
        assert_eq!(
            continue106_logical_pretty("x", "||", "y"),
            continue106_logical_minify("x", "||", "y")
        );
    }

    #[test]
    fn continue106_composed_residual_shells() {
        assert_eq!(continue106_case_break("1"), "case 1: break;");
        assert_eq!(continue106_default_continue(), "default: continue;");
        assert_eq!(continue106_break_label("outer"), "break outer;");
        assert_eq!(continue106_continue_label("loop"), "continue loop;");
        assert_eq!(
            continue106_for_in_break("const k", "obj"),
            "for (const k in obj) { break; }"
        );
        assert_eq!(
            continue106_for_of_continue("const x", "arr"),
            "for (const x of arr) { continue; }"
        );
        assert_eq!(continue106_logical_and("a", "b"), "a && b");
        assert_eq!(continue106_logical_or("a", "b"), "a || b");
        assert_eq!(
            continue106_for_of_logical_guard("const x", "xs", "x", "ready", "use(x);"),
            "for (const x of xs) { if (x && ready) { use(x); } }"
        );
        assert_eq!(continue106_stmt_sep(true), " ");
        assert_eq!(continue106_stmt_sep(false), "");
        assert_ne!(continue106_stmt_sep(true), continue106_stmt_sep(false));
    }
}

//! Pure JSXOpeningFragment + JSXClosingFragment + JSXSpreadChild + BigIntLiteral
//! + RegExpLiteral + Directive + PrivateName + ParenthesizedExpression dual-oracle
//! emission — residual pure **continue90** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–89:
//! - JSXOpeningFragment dual-oracle residual shell (`<>`)
//! - JSXClosingFragment dual-oracle residual shell (`</>`)
//! - JSXSpreadChild dual-oracle residual shell (`{...expr}`)
//! - BigIntLiteral dual-oracle composing real `continue21_bigint_literal_skeleton`
//! - RegExpLiteral dual-oracle composing real `continue21_regexp_literal_skeleton`
//! - Directive dual-oracle composing real `continue21_directive_literal_skeleton`
//! - PrivateName dual-oracle composing real `continue25_private_name_skeleton`
//! - ParenthesizedExpression dual-oracle composing real
//!   `continue25_parenthesized_expression_skeleton`
//! - Composed fragment open/close + spread-child residual shells
//!
//! Intentionally does **not** re-wrap continue88 JSXElement/JSXFragment full
//! shells (those stay separate). Opening/ClosingFragment were catalogued in
//! continue46 but never densed as emit skeletons; continue88 explicitly
//! excludes them. Composes real shipped pure helpers from continue21/25 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue21_bigint_literal_skeleton, continue21_directive_literal_skeleton,
    continue21_regexp_literal_skeleton, continue25_parenthesized_expression_skeleton,
    continue25_private_name_skeleton,
};

/// Dual-oracle residual: continue90 related AST type catalog.
pub const CONTINUE90_RELATED_TYPES: &[&str] = &[
    "JSXOpeningFragment",
    "JSXClosingFragment",
    "JSXSpreadChild",
    "BigIntLiteral",
    "RegExpLiteral",
    "Directive",
    "PrivateName",
    "ParenthesizedExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_jsx_fragment_literal_related_type(t: &str) -> bool {
    CONTINUE90_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue90_jsx_opening_fragment_type(t: &str) -> bool {
    t == "JSXOpeningFragment"
}

#[must_use]
pub fn is_continue90_jsx_closing_fragment_type(t: &str) -> bool {
    t == "JSXClosingFragment"
}

#[must_use]
pub fn is_continue90_jsx_spread_child_type(t: &str) -> bool {
    t == "JSXSpreadChild"
}

#[must_use]
pub fn is_continue90_bigint_literal_type(t: &str) -> bool {
    t == "BigIntLiteral"
}

#[must_use]
pub fn is_continue90_regexp_literal_type(t: &str) -> bool {
    t == "RegExpLiteral"
}

#[must_use]
pub fn is_continue90_directive_type(t: &str) -> bool {
    t == "Directive" || t == "DirectiveLiteral"
}

#[must_use]
pub fn is_continue90_private_name_type(t: &str) -> bool {
    t == "PrivateName" || t == "PrivateIdentifier"
}

#[must_use]
pub fn is_continue90_parenthesized_type(t: &str) -> bool {
    t == "ParenthesizedExpression"
}

// ── JSXOpeningFragment dual-oracle ──────────────────────────────────────────

/// Dual-oracle JSXOpeningFragment residual shell.
#[must_use]
pub fn continue90_jsx_opening_fragment_skeleton() -> &'static str {
    "<>"
}

/// Dual-oracle JSXOpeningFragment pretty alias.
#[must_use]
pub fn jsx_opening_fragment_pretty() -> &'static str {
    continue90_jsx_opening_fragment_skeleton()
}

/// Dual-oracle JSXOpeningFragment minify alias.
#[must_use]
pub fn jsx_opening_fragment_minify() -> &'static str {
    continue90_jsx_opening_fragment_skeleton()
}

// ── JSXClosingFragment dual-oracle ──────────────────────────────────────────

/// Dual-oracle JSXClosingFragment residual shell.
#[must_use]
pub fn continue90_jsx_closing_fragment_skeleton() -> &'static str {
    "</>"
}

/// Dual-oracle JSXClosingFragment pretty alias.
#[must_use]
pub fn jsx_closing_fragment_pretty() -> &'static str {
    continue90_jsx_closing_fragment_skeleton()
}

/// Dual-oracle JSXClosingFragment minify alias.
#[must_use]
pub fn jsx_closing_fragment_minify() -> &'static str {
    continue90_jsx_closing_fragment_skeleton()
}

// ── JSXSpreadChild dual-oracle ──────────────────────────────────────────────

/// Dual-oracle JSXSpreadChild residual shell (`{...expr}` as child node).
///
/// Distinct from JSXSpreadAttribute (attribute position) densed in continue88.
#[must_use]
pub fn continue90_jsx_spread_child_skeleton(expr: &str) -> String {
    format!("{{...{expr}}}")
}

/// Dual-oracle JSXSpreadChild pretty alias.
#[must_use]
pub fn jsx_spread_child_pretty(expr: &str) -> String {
    continue90_jsx_spread_child_skeleton(expr)
}

/// Dual-oracle JSXSpreadChild minify alias.
#[must_use]
pub fn jsx_spread_child_minify(expr: &str) -> String {
    continue90_jsx_spread_child_skeleton(expr)
}

// ── BigIntLiteral dual-oracle ───────────────────────────────────────────────

/// Dual-oracle BigIntLiteral skeleton composing real
/// [`continue21_bigint_literal_skeleton`].
#[must_use]
pub fn continue90_bigint_literal_skeleton(value: &str) -> String {
    continue21_bigint_literal_skeleton(value)
}

/// Dual-oracle BigIntLiteral pretty alias.
#[must_use]
pub fn bigint_literal_pretty(value: &str) -> String {
    continue90_bigint_literal_skeleton(value)
}

/// Dual-oracle BigIntLiteral minify alias.
#[must_use]
pub fn bigint_literal_minify(value: &str) -> String {
    continue90_bigint_literal_skeleton(value)
}

// ── RegExpLiteral dual-oracle ───────────────────────────────────────────────

/// Dual-oracle RegExpLiteral skeleton composing real
/// [`continue21_regexp_literal_skeleton`].
#[must_use]
pub fn continue90_regexp_literal_skeleton(pattern: &str, flags: &str) -> String {
    continue21_regexp_literal_skeleton(pattern, flags)
}

/// Dual-oracle RegExpLiteral pretty alias.
#[must_use]
pub fn regexp_literal_pretty(pattern: &str, flags: &str) -> String {
    continue90_regexp_literal_skeleton(pattern, flags)
}

/// Dual-oracle RegExpLiteral minify alias.
#[must_use]
pub fn regexp_literal_minify(pattern: &str, flags: &str) -> String {
    continue90_regexp_literal_skeleton(pattern, flags)
}

// ── Directive dual-oracle ───────────────────────────────────────────────────

/// Dual-oracle Directive skeleton composing real
/// [`continue21_directive_literal_skeleton`].
#[must_use]
pub fn continue90_directive_skeleton(value: &str, pretty: bool) -> String {
    continue21_directive_literal_skeleton(value, pretty)
}

/// Dual-oracle Directive pretty alias.
#[must_use]
pub fn directive_pretty(value: &str) -> String {
    continue90_directive_skeleton(value, true)
}

/// Dual-oracle Directive minify alias.
#[must_use]
pub fn directive_minify(value: &str) -> String {
    continue90_directive_skeleton(value, false)
}

// ── PrivateName dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle PrivateName skeleton composing real
/// [`continue25_private_name_skeleton`].
#[must_use]
pub fn continue90_private_name_skeleton(name: &str) -> String {
    continue25_private_name_skeleton(name)
}

/// Dual-oracle PrivateName pretty alias.
#[must_use]
pub fn private_name_pretty(name: &str) -> String {
    continue90_private_name_skeleton(name)
}

/// Dual-oracle PrivateName minify alias.
#[must_use]
pub fn private_name_minify(name: &str) -> String {
    continue90_private_name_skeleton(name)
}

// ── ParenthesizedExpression dual-oracle ─────────────────────────────────────

/// Dual-oracle ParenthesizedExpression skeleton composing real
/// [`continue25_parenthesized_expression_skeleton`].
#[must_use]
pub fn continue90_parenthesized_expression_skeleton(inner: &str) -> String {
    continue25_parenthesized_expression_skeleton(inner)
}

/// Dual-oracle ParenthesizedExpression pretty alias.
#[must_use]
pub fn parenthesized_expression_pretty(inner: &str) -> String {
    continue90_parenthesized_expression_skeleton(inner)
}

/// Dual-oracle ParenthesizedExpression minify alias.
#[must_use]
pub fn parenthesized_expression_minify(inner: &str) -> String {
    continue90_parenthesized_expression_skeleton(inner)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: assemble full fragment from open + children + close.
///
/// Pretty: children joined by newline between open/close lines when multi-line
/// requested; minify: open+children+close concatenated.
#[must_use]
pub fn continue90_jsx_fragment_from_poles(children: &str, pretty: bool) -> String {
    let open = continue90_jsx_opening_fragment_skeleton();
    let close = continue90_jsx_closing_fragment_skeleton();
    if children.is_empty() {
        return format!("{open}{close}");
    }
    if pretty && children.contains('\n') {
        format!("{open}\n{children}\n{close}")
    } else {
        format!("{open}{children}{close}")
    }
}

/// Dual-oracle residual: fragment whose sole child is a spread child.
#[must_use]
pub fn continue90_jsx_fragment_with_spread_child(expr: &str, pretty: bool) -> String {
    let child = continue90_jsx_spread_child_skeleton(expr);
    continue90_jsx_fragment_from_poles(&child, pretty)
}

/// Dual-oracle residual: parenthesized bigint literal (`(1n)`).
#[must_use]
pub fn continue90_parenthesized_bigint(value: &str) -> String {
    continue90_parenthesized_expression_skeleton(&continue90_bigint_literal_skeleton(value))
}

/// Dual-oracle residual: parenthesized regexp (`(/foo/g)`).
#[must_use]
pub fn continue90_parenthesized_regexp(pattern: &str, flags: &str) -> String {
    continue90_parenthesized_expression_skeleton(&continue90_regexp_literal_skeleton(
        pattern, flags,
    ))
}

/// Dual-oracle residual: private name inside parentheses (`(#field)`).
#[must_use]
pub fn continue90_parenthesized_private_name(name: &str) -> String {
    continue90_parenthesized_expression_skeleton(&continue90_private_name_skeleton(name))
}

/// Dual-oracle residual: child-list separator for fragment interior.
#[must_use]
pub fn continue90_jsx_child_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n"
    } else {
        ""
    }
}

/// Dual-oracle residual: join JSX children (elements/text/spread).
#[must_use]
pub fn continue90_join_jsx_children(children: &[&str], pretty: bool) -> String {
    let sep = continue90_jsx_child_sep(pretty);
    children.join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue21_bigint_literal_skeleton, continue21_directive_literal_skeleton,
        continue21_regexp_literal_skeleton, continue25_parenthesized_expression_skeleton,
        continue25_private_name_skeleton,
    };

    #[test]
    fn continue90_type_catalog() {
        assert_eq!(CONTINUE90_RELATED_TYPES.len(), 8);
        assert!(is_jsx_fragment_literal_related_type("JSXOpeningFragment"));
        assert!(is_jsx_fragment_literal_related_type("JSXClosingFragment"));
        assert!(is_jsx_fragment_literal_related_type("JSXSpreadChild"));
        assert!(is_jsx_fragment_literal_related_type("BigIntLiteral"));
        assert!(is_jsx_fragment_literal_related_type("RegExpLiteral"));
        assert!(is_jsx_fragment_literal_related_type("Directive"));
        assert!(is_jsx_fragment_literal_related_type("PrivateName"));
        assert!(is_jsx_fragment_literal_related_type("ParenthesizedExpression"));
        assert!(!is_jsx_fragment_literal_related_type("JSXElement"));
        assert!(!is_jsx_fragment_literal_related_type("JSXFragment"));
        assert!(!is_jsx_fragment_literal_related_type("JSXSpreadAttribute"));
        assert!(!is_jsx_fragment_literal_related_type("Decorator"));
        assert!(is_continue90_jsx_opening_fragment_type("JSXOpeningFragment"));
        assert!(is_continue90_jsx_closing_fragment_type("JSXClosingFragment"));
        assert!(is_continue90_jsx_spread_child_type("JSXSpreadChild"));
        assert!(is_continue90_bigint_literal_type("BigIntLiteral"));
        assert!(is_continue90_regexp_literal_type("RegExpLiteral"));
        assert!(is_continue90_directive_type("Directive"));
        assert!(is_continue90_directive_type("DirectiveLiteral"));
        assert!(is_continue90_private_name_type("PrivateName"));
        assert!(is_continue90_private_name_type("PrivateIdentifier"));
        assert!(is_continue90_parenthesized_type("ParenthesizedExpression"));
        assert!(!is_continue90_jsx_opening_fragment_type("JSXClosingFragment"));
        assert!(!is_continue90_bigint_literal_type("NumericLiteral"));
    }

    #[test]
    fn continue90_jsx_fragment_poles_spread_dual_oracle() {
        assert_eq!(continue90_jsx_opening_fragment_skeleton(), "<>");
        assert_eq!(jsx_opening_fragment_pretty(), "<>");
        assert_eq!(jsx_opening_fragment_minify(), "<>");
        assert_eq!(continue90_jsx_closing_fragment_skeleton(), "</>");
        assert_eq!(jsx_closing_fragment_pretty(), "</>");
        assert_ne!(
            continue90_jsx_opening_fragment_skeleton(),
            continue90_jsx_closing_fragment_skeleton()
        );

        assert_eq!(
            continue90_jsx_spread_child_skeleton("items"),
            "{...items}"
        );
        assert_eq!(jsx_spread_child_pretty("rest"), "{...rest}");
        assert_eq!(jsx_spread_child_minify("rest"), "{...rest}");
        assert_ne!(
            jsx_spread_child_pretty("a"),
            jsx_spread_child_pretty("b")
        );

        assert_eq!(continue90_jsx_fragment_from_poles("", true), "<></>");
        assert_eq!(continue90_jsx_fragment_from_poles("x", false), "<>x</>");
        assert_eq!(
            continue90_jsx_fragment_from_poles("a\nb", true),
            "<>\na\nb\n</>"
        );
        assert_eq!(
            continue90_jsx_fragment_with_spread_child("props", false),
            "<>{...props}</>"
        );
        assert_ne!(
            continue90_jsx_fragment_with_spread_child("a", false),
            continue90_jsx_fragment_with_spread_child("b", false)
        );

        assert_eq!(continue90_jsx_child_sep(true), "\n");
        assert_eq!(continue90_jsx_child_sep(false), "");
        assert_eq!(
            continue90_join_jsx_children(&["a", "{...b}"], true),
            "a\n{...b}"
        );
        assert_eq!(
            continue90_join_jsx_children(&["a", "{...b}"], false),
            "a{...b}"
        );
    }

    #[test]
    fn continue90_literal_private_paren_compose_dual_oracle() {
        assert_eq!(continue90_bigint_literal_skeleton("1"), "1n");
        assert_eq!(
            continue90_bigint_literal_skeleton("1"),
            continue21_bigint_literal_skeleton("1")
        );
        assert_eq!(continue90_bigint_literal_skeleton("2n"), "2n");
        assert_eq!(bigint_literal_pretty("42"), "42n");
        assert_eq!(bigint_literal_minify("42"), "42n");
        assert_ne!(
            bigint_literal_pretty("1"),
            bigint_literal_pretty("2")
        );

        assert_eq!(
            continue90_regexp_literal_skeleton("foo", "gi"),
            "/foo/gi"
        );
        assert_eq!(
            continue90_regexp_literal_skeleton("foo", "gi"),
            continue21_regexp_literal_skeleton("foo", "gi")
        );
        assert_eq!(regexp_literal_pretty("a", ""), "/a/");
        assert_ne!(
            regexp_literal_pretty("a", "g"),
            regexp_literal_pretty("a", "i")
        );

        assert_eq!(
            continue90_directive_skeleton("use strict", true),
            "\"use strict\";"
        );
        assert_eq!(
            continue90_directive_skeleton("use strict", true),
            continue21_directive_literal_skeleton("use strict", true)
        );
        assert_eq!(directive_pretty("use strict"), directive_minify("use strict"));
        assert_ne!(directive_pretty("a"), directive_pretty("b"));

        assert_eq!(continue90_private_name_skeleton("field"), "#field");
        assert_eq!(
            continue90_private_name_skeleton("field"),
            continue25_private_name_skeleton("field")
        );
        assert_eq!(continue90_private_name_skeleton("#already"), "#already");
        assert_eq!(private_name_pretty("x"), "#x");
        assert_ne!(private_name_pretty("x"), private_name_pretty("y"));

        assert_eq!(
            continue90_parenthesized_expression_skeleton("a + b"),
            "(a + b)"
        );
        assert_eq!(
            continue90_parenthesized_expression_skeleton("a + b"),
            continue25_parenthesized_expression_skeleton("a + b")
        );
        assert_eq!(parenthesized_expression_pretty("1"), "(1)");
        assert_eq!(parenthesized_expression_minify("1"), "(1)");
        assert_ne!(
            parenthesized_expression_pretty("a"),
            parenthesized_expression_pretty("b")
        );

        assert_eq!(continue90_parenthesized_bigint("7"), "(7n)");
        assert_eq!(continue90_parenthesized_regexp("x", "g"), "(/x/g)");
        assert_eq!(continue90_parenthesized_private_name("id"), "(#id)");
        assert_ne!(
            continue90_parenthesized_bigint("1"),
            continue90_parenthesized_private_name("1")
        );
    }
}

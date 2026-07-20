//! Pure JSXElement + JSXFragment + JSXAttribute + JSXIdentifier +
//! JSXText + JSXExpressionContainer dual-oracle emission
//! — residual pure **continue111** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–110:
//! - JSXElement dual-oracle composing real `continue44_jsx_element_skeleton` /
//!   `continue44_jsx_element_self_closing_skeleton`
//! - JSXFragment dual-oracle composing real `continue44_jsx_fragment_skeleton`
//! - JSXAttribute dual-oracle composing real `continue44_jsx_attribute_skeleton`
//! - JSXIdentifier dual-oracle composing real `continue44_jsx_identifier_skeleton`
//! - JSXText dual-oracle composing real `continue44_jsx_text_skeleton`
//! - JSXExpressionContainer dual-oracle composing real
//!   `continue44_jsx_expression_container_skeleton`
//! - Composed jsx element/fragment/attribute/text/expr residual shells
//!
//! Intentionally does **not** re-wrap continue110 with/labeled/debugger/throw/
//! expr/if poles (continue43 bases), continue109 new/conditional/this/super/
//! sequence/empty poles (continue42 bases), or continue40–43 bases.
//! Composes real shipped pure helpers from continue44 bases. Full engines
//! remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue44_jsx_attribute_skeleton, continue44_jsx_element_self_closing_skeleton,
    continue44_jsx_element_skeleton, continue44_jsx_expression_container_skeleton,
    continue44_jsx_fragment_skeleton, continue44_jsx_identifier_skeleton,
    continue44_jsx_text_skeleton,
};

/// Dual-oracle residual: continue111 related AST type catalog.
pub const CONTINUE111_RELATED_TYPES: &[&str] = &[
    "JSXElement",
    "JSXFragment",
    "JSXAttribute",
    "JSXIdentifier",
    "JSXText",
    "JSXExpressionContainer",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_jsx_element_fragment_attribute_related_type(t: &str) -> bool {
    CONTINUE111_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue111_jsx_element_type(t: &str) -> bool {
    t == "JSXElement"
}

#[must_use]
pub fn is_continue111_jsx_fragment_type(t: &str) -> bool {
    t == "JSXFragment"
}

#[must_use]
pub fn is_continue111_jsx_attribute_type(t: &str) -> bool {
    t == "JSXAttribute"
}

#[must_use]
pub fn is_continue111_jsx_identifier_type(t: &str) -> bool {
    t == "JSXIdentifier"
}

#[must_use]
pub fn is_continue111_jsx_text_type(t: &str) -> bool {
    t == "JSXText"
}

#[must_use]
pub fn is_continue111_jsx_expression_container_type(t: &str) -> bool {
    t == "JSXExpressionContainer"
}

#[must_use]
pub fn is_continue111_jsx_type(t: &str) -> bool {
    matches!(
        t,
        "JSXElement"
            | "JSXFragment"
            | "JSXAttribute"
            | "JSXIdentifier"
            | "JSXText"
            | "JSXExpressionContainer"
    )
}

// ── JSXElement dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle JSXElement (with children) skeleton composing real
/// [`continue44_jsx_element_skeleton`].
#[must_use]
pub fn continue111_jsx_element_skeleton(name: &str, children: &str) -> String {
    continue44_jsx_element_skeleton(name, children)
}

/// Dual-oracle JSXElement self-closing skeleton composing real
/// [`continue44_jsx_element_self_closing_skeleton`].
#[must_use]
pub fn continue111_jsx_element_self_closing_skeleton(name: &str) -> String {
    continue44_jsx_element_self_closing_skeleton(name)
}

/// Dual-oracle JSXElement pretty alias.
#[must_use]
pub fn continue111_jsx_element_pretty(name: &str, children: &str) -> String {
    continue111_jsx_element_skeleton(name, children)
}

/// Dual-oracle JSXElement minify alias.
#[must_use]
pub fn continue111_jsx_element_minify(name: &str, children: &str) -> String {
    continue111_jsx_element_skeleton(name, children)
}

/// Dual-oracle JSXElement self-closing pretty alias.
#[must_use]
pub fn continue111_jsx_self_closing_pretty(name: &str) -> String {
    continue111_jsx_element_self_closing_skeleton(name)
}

/// Dual-oracle JSXElement self-closing minify alias.
#[must_use]
pub fn continue111_jsx_self_closing_minify(name: &str) -> String {
    continue111_jsx_element_self_closing_skeleton(name)
}

// ── JSXFragment dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle JSXFragment skeleton composing real
/// [`continue44_jsx_fragment_skeleton`].
#[must_use]
pub fn continue111_jsx_fragment_skeleton(children: &str) -> String {
    continue44_jsx_fragment_skeleton(children)
}

/// Dual-oracle JSXFragment pretty alias.
#[must_use]
pub fn continue111_jsx_fragment_pretty(children: &str) -> String {
    continue111_jsx_fragment_skeleton(children)
}

/// Dual-oracle JSXFragment minify alias.
#[must_use]
pub fn continue111_jsx_fragment_minify(children: &str) -> String {
    continue111_jsx_fragment_skeleton(children)
}

// ── JSXAttribute dual-oracle ────────────────────────────────────────────────

/// Dual-oracle JSXAttribute skeleton composing real
/// [`continue44_jsx_attribute_skeleton`].
#[must_use]
pub fn continue111_jsx_attribute_skeleton(name: &str, value: &str) -> String {
    continue44_jsx_attribute_skeleton(name, value)
}

/// Dual-oracle JSXAttribute pretty alias.
#[must_use]
pub fn continue111_jsx_attribute_pretty(name: &str, value: &str) -> String {
    continue111_jsx_attribute_skeleton(name, value)
}

/// Dual-oracle JSXAttribute minify alias.
#[must_use]
pub fn continue111_jsx_attribute_minify(name: &str, value: &str) -> String {
    continue111_jsx_attribute_skeleton(name, value)
}

// ── JSXIdentifier dual-oracle ───────────────────────────────────────────────

/// Dual-oracle JSXIdentifier skeleton composing real
/// [`continue44_jsx_identifier_skeleton`].
#[must_use]
pub fn continue111_jsx_identifier_skeleton(name: &str) -> String {
    continue44_jsx_identifier_skeleton(name)
}

/// Dual-oracle JSXIdentifier pretty alias.
#[must_use]
pub fn continue111_jsx_identifier_pretty(name: &str) -> String {
    continue111_jsx_identifier_skeleton(name)
}

/// Dual-oracle JSXIdentifier minify alias.
#[must_use]
pub fn continue111_jsx_identifier_minify(name: &str) -> String {
    continue111_jsx_identifier_skeleton(name)
}

// ── JSXText dual-oracle ─────────────────────────────────────────────────────

/// Dual-oracle JSXText skeleton composing real
/// [`continue44_jsx_text_skeleton`].
#[must_use]
pub fn continue111_jsx_text_skeleton(text: &str) -> String {
    continue44_jsx_text_skeleton(text)
}

/// Dual-oracle JSXText pretty alias.
#[must_use]
pub fn continue111_jsx_text_pretty(text: &str) -> String {
    continue111_jsx_text_skeleton(text)
}

/// Dual-oracle JSXText minify alias.
#[must_use]
pub fn continue111_jsx_text_minify(text: &str) -> String {
    continue111_jsx_text_skeleton(text)
}

// ── JSXExpressionContainer dual-oracle ──────────────────────────────────────

/// Dual-oracle JSXExpressionContainer skeleton composing real
/// [`continue44_jsx_expression_container_skeleton`].
#[must_use]
pub fn continue111_jsx_expression_container_skeleton(expr: &str) -> String {
    continue44_jsx_expression_container_skeleton(expr)
}

/// Dual-oracle JSXExpressionContainer pretty alias.
#[must_use]
pub fn continue111_jsx_expression_container_pretty(expr: &str) -> String {
    continue111_jsx_expression_container_skeleton(expr)
}

/// Dual-oracle JSXExpressionContainer minify alias.
#[must_use]
pub fn continue111_jsx_expression_container_minify(expr: &str) -> String {
    continue111_jsx_expression_container_skeleton(expr)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: empty children element (`<div></div>`).
#[must_use]
pub fn continue111_jsx_element_empty(name: &str) -> String {
    continue111_jsx_element_skeleton(name, "")
}

/// Dual-oracle residual: empty fragment (`<></>`).
#[must_use]
pub fn continue111_jsx_fragment_empty() -> String {
    continue111_jsx_fragment_skeleton("")
}

/// Dual-oracle residual: element with text child.
#[must_use]
pub fn continue111_jsx_element_text(name: &str, text: &str) -> String {
    let body = continue111_jsx_text_skeleton(text);
    continue111_jsx_element_skeleton(name, &body)
}

/// Dual-oracle residual: element with expression-container child.
#[must_use]
pub fn continue111_jsx_element_expr(name: &str, expr: &str) -> String {
    let body = continue111_jsx_expression_container_skeleton(expr);
    continue111_jsx_element_skeleton(name, &body)
}

/// Dual-oracle residual: fragment wrapping text + expression.
#[must_use]
pub fn continue111_jsx_fragment_text_expr(text: &str, expr: &str) -> String {
    let t = continue111_jsx_text_skeleton(text);
    let e = continue111_jsx_expression_container_skeleton(expr);
    continue111_jsx_fragment_skeleton(&format!("{t}{e}"))
}

/// Dual-oracle residual: self-closing with attribute shell seed
/// (`<img className="x" />` composition via attr + name; skeleton is
/// self-closing name only — attr string is dual-oracle of attribute pole).
#[must_use]
pub fn continue111_jsx_attr_then_self_closing(attr_name: &str, attr_value: &str, el: &str) -> String {
    let attr = continue111_jsx_attribute_skeleton(attr_name, attr_value);
    let open = continue111_jsx_element_self_closing_skeleton(el);
    // Honest residual shell: attribute catalog + self-closing catalog composed.
    format!("/*{attr}*/{open}")
}

/// Dual-oracle residual: identifier names element.
#[must_use]
pub fn continue111_jsx_named_element(ident: &str, children: &str) -> String {
    let name = continue111_jsx_identifier_skeleton(ident);
    continue111_jsx_element_skeleton(&name, children)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue111_jsx_sep(pretty: bool) -> &'static str {
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
        continue44_jsx_attribute_skeleton, continue44_jsx_element_self_closing_skeleton,
        continue44_jsx_element_skeleton, continue44_jsx_expression_container_skeleton,
        continue44_jsx_fragment_skeleton, continue44_jsx_identifier_skeleton,
        continue44_jsx_text_skeleton,
    };

    #[test]
    fn continue111_type_catalog() {
        assert_eq!(CONTINUE111_RELATED_TYPES.len(), 6);
        assert!(is_jsx_element_fragment_attribute_related_type("JSXElement"));
        assert!(is_jsx_element_fragment_attribute_related_type("JSXFragment"));
        assert!(is_jsx_element_fragment_attribute_related_type("JSXAttribute"));
        assert!(is_jsx_element_fragment_attribute_related_type(
            "JSXIdentifier"
        ));
        assert!(is_jsx_element_fragment_attribute_related_type("JSXText"));
        assert!(is_jsx_element_fragment_attribute_related_type(
            "JSXExpressionContainer"
        ));
        assert!(!is_jsx_element_fragment_attribute_related_type(
            "WithStatement"
        ));
        assert!(!is_jsx_element_fragment_attribute_related_type(
            "NewExpression"
        ));

        assert!(is_continue111_jsx_element_type("JSXElement"));
        assert!(!is_continue111_jsx_element_type("JSXFragment"));
        assert!(is_continue111_jsx_fragment_type("JSXFragment"));
        assert!(is_continue111_jsx_attribute_type("JSXAttribute"));
        assert!(is_continue111_jsx_identifier_type("JSXIdentifier"));
        assert!(is_continue111_jsx_text_type("JSXText"));
        assert!(is_continue111_jsx_expression_container_type(
            "JSXExpressionContainer"
        ));
        assert!(is_continue111_jsx_type("JSXElement"));
        assert!(is_continue111_jsx_type("JSXExpressionContainer"));
        assert!(!is_continue111_jsx_type("IfStatement"));
    }

    #[test]
    fn continue111_jsx_element_fragment_emit() {
        assert_eq!(
            continue111_jsx_element_skeleton("div", "hi"),
            "<div>hi</div>"
        );
        assert_eq!(
            continue111_jsx_element_skeleton("div", "hi"),
            continue44_jsx_element_skeleton("div", "hi")
        );
        assert_eq!(
            continue111_jsx_element_pretty("span", "x"),
            continue111_jsx_element_minify("span", "x")
        );

        assert_eq!(
            continue111_jsx_element_self_closing_skeleton("img"),
            "<img />"
        );
        assert_eq!(
            continue111_jsx_element_self_closing_skeleton("img"),
            continue44_jsx_element_self_closing_skeleton("img")
        );
        assert_eq!(
            continue111_jsx_self_closing_pretty("br"),
            continue111_jsx_self_closing_minify("br")
        );

        assert_eq!(continue111_jsx_fragment_skeleton("x"), "<>x</>");
        assert_eq!(
            continue111_jsx_fragment_skeleton("x"),
            continue44_jsx_fragment_skeleton("x")
        );
        assert_eq!(
            continue111_jsx_fragment_pretty("a"),
            continue111_jsx_fragment_minify("a")
        );
    }

    #[test]
    fn continue111_jsx_attribute_text_expr_ident_emit() {
        assert_eq!(
            continue111_jsx_attribute_skeleton("className", "btn"),
            "className=\"btn\""
        );
        assert_eq!(
            continue111_jsx_attribute_skeleton("className", "btn"),
            continue44_jsx_attribute_skeleton("className", "btn")
        );
        assert_eq!(
            continue111_jsx_attribute_pretty("id", "a"),
            continue111_jsx_attribute_minify("id", "a")
        );

        assert_eq!(continue111_jsx_identifier_skeleton("Button"), "Button");
        assert_eq!(
            continue111_jsx_identifier_skeleton("Button"),
            continue44_jsx_identifier_skeleton("Button")
        );
        assert_eq!(
            continue111_jsx_identifier_pretty("A"),
            continue111_jsx_identifier_minify("A")
        );

        assert_eq!(continue111_jsx_text_skeleton("hello"), "hello");
        assert_eq!(
            continue111_jsx_text_skeleton("hello"),
            continue44_jsx_text_skeleton("hello")
        );
        assert_eq!(
            continue111_jsx_text_pretty("t"),
            continue111_jsx_text_minify("t")
        );

        assert_eq!(
            continue111_jsx_expression_container_skeleton("count"),
            "{count}"
        );
        assert_eq!(
            continue111_jsx_expression_container_skeleton("count"),
            continue44_jsx_expression_container_skeleton("count")
        );
        assert_eq!(
            continue111_jsx_expression_container_pretty("x"),
            continue111_jsx_expression_container_minify("x")
        );
    }

    #[test]
    fn continue111_composed_residual_shells() {
        assert_eq!(continue111_jsx_element_empty("div"), "<div></div>");
        assert_eq!(continue111_jsx_fragment_empty(), "<></>");
        assert_eq!(
            continue111_jsx_element_text("p", "hi"),
            "<p>hi</p>"
        );
        assert_eq!(
            continue111_jsx_element_expr("span", "n"),
            "<span>{n}</span>"
        );
        assert_eq!(
            continue111_jsx_fragment_text_expr("hi", "x"),
            "<>hi{x}</>"
        );
        assert_eq!(
            continue111_jsx_attr_then_self_closing("className", "x", "img"),
            "/*className=\"x\"*/<img />"
        );
        assert_eq!(
            continue111_jsx_named_element("Button", "ok"),
            "<Button>ok</Button>"
        );
        assert_eq!(continue111_jsx_sep(true), " ");
        assert_eq!(continue111_jsx_sep(false), "");
        assert_ne!(continue111_jsx_sep(true), continue111_jsx_sep(false));
    }
}

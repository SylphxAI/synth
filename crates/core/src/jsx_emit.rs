//! Pure JSXElement + JSXFragment + JSXAttribute + JSXIdentifier + JSXText +
//! JSXExpressionContainer + JSXMemberExpression + JSXNamespacedName +
//! JSXSpreadAttribute + JSXOpeningElement + JSXClosingElement +
//! JSXEmptyExpression dual-oracle emission — residual pure **continue88** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–87:
//! - JSXElement self-closing + children dual-oracle composing real
//!   `continue44_jsx_element_self_closing_skeleton` /
//!   `continue44_jsx_element_skeleton`
//! - JSXFragment dual-oracle composing real
//!   `continue44_jsx_fragment_skeleton`
//! - JSXAttribute dual-oracle composing real
//!   `continue44_jsx_attribute_skeleton`
//! - JSXIdentifier / JSXText dual-oracle composing real
//!   `continue44_jsx_identifier_skeleton` / `continue44_jsx_text_skeleton`
//! - JSXExpressionContainer dual-oracle composing real
//!   `continue44_jsx_expression_container_skeleton`
//! - JSXMemberExpression / JSXNamespacedName dual-oracle composing real
//!   `continue46_jsx_member_expression_skeleton` /
//!   `continue46_jsx_namespaced_name_skeleton`
//! - JSXSpreadAttribute dual-oracle composing real
//!   `continue46_jsx_spread_attribute_skeleton`
//! - JSXOpeningElement / JSXClosingElement dual-oracle composing real
//!   `continue46_jsx_opening_element_skeleton` /
//!   `continue46_jsx_closing_element_skeleton`
//! - JSXEmptyExpression dual-oracle composing real
//!   `continue46_jsx_empty_expression_skeleton`
//! - Composed attribute list + attributed element residual shells
//!
//! Intentionally does **not** re-wrap continue64–87 partition shells
//! (import/export specifier continue87 stays separate; function/class/this
//! continue86 stays separate). Composes real shipped pure helpers from
//! continue44/46 JSX bases in literal_widen.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue44_jsx_attribute_skeleton, continue44_jsx_element_self_closing_skeleton,
    continue44_jsx_element_skeleton, continue44_jsx_expression_container_skeleton,
    continue44_jsx_fragment_skeleton, continue44_jsx_identifier_skeleton,
    continue44_jsx_text_skeleton, continue46_jsx_closing_element_skeleton,
    continue46_jsx_empty_expression_skeleton, continue46_jsx_member_expression_skeleton,
    continue46_jsx_namespaced_name_skeleton, continue46_jsx_opening_element_skeleton,
    continue46_jsx_spread_attribute_skeleton,
};

/// Dual-oracle residual: continue88 related AST type catalog.
pub const CONTINUE88_RELATED_TYPES: &[&str] = &[
    "JSXElement",
    "JSXFragment",
    "JSXAttribute",
    "JSXIdentifier",
    "JSXText",
    "JSXExpressionContainer",
    "JSXMemberExpression",
    "JSXNamespacedName",
    "JSXSpreadAttribute",
    "JSXOpeningElement",
    "JSXClosingElement",
    "JSXEmptyExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_jsx_related_type(t: &str) -> bool {
    CONTINUE88_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue88_jsx_element_type(t: &str) -> bool {
    t == "JSXElement"
}

#[must_use]
pub fn is_continue88_jsx_fragment_type(t: &str) -> bool {
    t == "JSXFragment"
}

#[must_use]
pub fn is_continue88_jsx_attribute_type(t: &str) -> bool {
    t == "JSXAttribute"
}

#[must_use]
pub fn is_continue88_jsx_identifier_type(t: &str) -> bool {
    t == "JSXIdentifier"
}

#[must_use]
pub fn is_continue88_jsx_text_type(t: &str) -> bool {
    t == "JSXText"
}

#[must_use]
pub fn is_continue88_jsx_expression_container_type(t: &str) -> bool {
    t == "JSXExpressionContainer"
}

#[must_use]
pub fn is_continue88_jsx_member_expression_type(t: &str) -> bool {
    t == "JSXMemberExpression"
}

#[must_use]
pub fn is_continue88_jsx_namespaced_name_type(t: &str) -> bool {
    t == "JSXNamespacedName"
}

#[must_use]
pub fn is_continue88_jsx_spread_attribute_type(t: &str) -> bool {
    t == "JSXSpreadAttribute"
}

#[must_use]
pub fn is_continue88_jsx_opening_element_type(t: &str) -> bool {
    t == "JSXOpeningElement"
}

#[must_use]
pub fn is_continue88_jsx_closing_element_type(t: &str) -> bool {
    t == "JSXClosingElement"
}

#[must_use]
pub fn is_continue88_jsx_empty_expression_type(t: &str) -> bool {
    t == "JSXEmptyExpression"
}

// ── JSXElement dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle JSXElement self-closing skeleton composing real
/// [`continue44_jsx_element_self_closing_skeleton`].
#[must_use]
pub fn continue88_jsx_element_self_closing_skeleton(name: &str) -> String {
    continue44_jsx_element_self_closing_skeleton(name)
}

/// Dual-oracle JSXElement with children skeleton composing real
/// [`continue44_jsx_element_skeleton`].
#[must_use]
pub fn continue88_jsx_element_skeleton(name: &str, children: &str) -> String {
    continue44_jsx_element_skeleton(name, children)
}

/// Convenience self-closing element.
#[must_use]
pub fn jsx_element_self_closing(name: &str) -> String {
    continue88_jsx_element_self_closing_skeleton(name)
}

/// Convenience element with children.
#[must_use]
pub fn jsx_element(name: &str, children: &str) -> String {
    continue88_jsx_element_skeleton(name, children)
}

// ── JSXFragment dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle JSXFragment skeleton composing real
/// [`continue44_jsx_fragment_skeleton`].
#[must_use]
pub fn continue88_jsx_fragment_skeleton(children: &str) -> String {
    continue44_jsx_fragment_skeleton(children)
}

/// Convenience alias.
#[must_use]
pub fn jsx_fragment(children: &str) -> String {
    continue88_jsx_fragment_skeleton(children)
}

// ── JSXAttribute dual-oracle ────────────────────────────────────────────────

/// Dual-oracle JSXAttribute skeleton composing real
/// [`continue44_jsx_attribute_skeleton`].
#[must_use]
pub fn continue88_jsx_attribute_skeleton(name: &str, value: &str) -> String {
    continue44_jsx_attribute_skeleton(name, value)
}

/// Convenience alias.
#[must_use]
pub fn jsx_attribute(name: &str, value: &str) -> String {
    continue88_jsx_attribute_skeleton(name, value)
}

// ── JSXIdentifier / JSXText dual-oracle ─────────────────────────────────────

/// Dual-oracle JSXIdentifier skeleton composing real
/// [`continue44_jsx_identifier_skeleton`].
#[must_use]
pub fn continue88_jsx_identifier_skeleton(name: &str) -> String {
    continue44_jsx_identifier_skeleton(name)
}

/// Dual-oracle JSXText skeleton composing real
/// [`continue44_jsx_text_skeleton`].
#[must_use]
pub fn continue88_jsx_text_skeleton(text: &str) -> String {
    continue44_jsx_text_skeleton(text)
}

// ── JSXExpressionContainer dual-oracle ──────────────────────────────────────

/// Dual-oracle JSXExpressionContainer skeleton composing real
/// [`continue44_jsx_expression_container_skeleton`].
#[must_use]
pub fn continue88_jsx_expression_container_skeleton(expr: &str) -> String {
    continue44_jsx_expression_container_skeleton(expr)
}

/// Convenience alias.
#[must_use]
pub fn jsx_expression_container(expr: &str) -> String {
    continue88_jsx_expression_container_skeleton(expr)
}

// ── JSXMemberExpression / JSXNamespacedName dual-oracle ─────────────────────

/// Dual-oracle JSXMemberExpression skeleton composing real
/// [`continue46_jsx_member_expression_skeleton`].
#[must_use]
pub fn continue88_jsx_member_expression_skeleton(object: &str, property: &str) -> String {
    continue46_jsx_member_expression_skeleton(object, property)
}

/// Dual-oracle JSXNamespacedName skeleton composing real
/// [`continue46_jsx_namespaced_name_skeleton`].
#[must_use]
pub fn continue88_jsx_namespaced_name_skeleton(ns: &str, name: &str) -> String {
    continue46_jsx_namespaced_name_skeleton(ns, name)
}

// ── JSXSpreadAttribute dual-oracle ──────────────────────────────────────────

/// Dual-oracle JSXSpreadAttribute skeleton composing real
/// [`continue46_jsx_spread_attribute_skeleton`].
#[must_use]
pub fn continue88_jsx_spread_attribute_skeleton(expr: &str) -> String {
    continue46_jsx_spread_attribute_skeleton(expr)
}

/// Convenience alias.
#[must_use]
pub fn jsx_spread_attribute(expr: &str) -> String {
    continue88_jsx_spread_attribute_skeleton(expr)
}

// ── JSXOpeningElement / JSXClosingElement dual-oracle ───────────────────────

/// Dual-oracle JSXOpeningElement skeleton composing real
/// [`continue46_jsx_opening_element_skeleton`].
#[must_use]
pub fn continue88_jsx_opening_element_skeleton(name: &str) -> String {
    continue46_jsx_opening_element_skeleton(name)
}

/// Dual-oracle JSXClosingElement skeleton composing real
/// [`continue46_jsx_closing_element_skeleton`].
#[must_use]
pub fn continue88_jsx_closing_element_skeleton(name: &str) -> String {
    continue46_jsx_closing_element_skeleton(name)
}

// ── JSXEmptyExpression dual-oracle ──────────────────────────────────────────

/// Dual-oracle JSXEmptyExpression skeleton composing real
/// [`continue46_jsx_empty_expression_skeleton`].
#[must_use]
pub fn continue88_jsx_empty_expression_skeleton() -> String {
    continue46_jsx_empty_expression_skeleton()
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: attribute list separator poles (pretty space vs minify none).
#[must_use]
pub fn continue88_jsx_attr_sep(pretty: bool) -> &'static str {
    if pretty {
        " "
    } else {
        " "
    }
}

/// Dual-oracle residual: join JSX attributes with separator.
#[must_use]
pub fn continue88_jsx_join_attrs(attrs: &[&str], pretty: bool) -> String {
    let sep = continue88_jsx_attr_sep(pretty);
    attrs.join(sep)
}

/// Dual-oracle residual: self-closing element with attributes.
/// `<Name attr1 attr2 />` (pretty/minify share space sep in current printer poles).
#[must_use]
pub fn continue88_jsx_element_with_attrs_self_closing(name: &str, attrs: &[&str]) -> String {
    if attrs.is_empty() {
        return continue88_jsx_element_self_closing_skeleton(name);
    }
    let joined = continue88_jsx_join_attrs(attrs, true);
    format!("<{name} {joined} />")
}

/// Dual-oracle residual: element with attributes + children.
#[must_use]
pub fn continue88_jsx_element_with_attrs(name: &str, attrs: &[&str], children: &str) -> String {
    if attrs.is_empty() {
        return continue88_jsx_element_skeleton(name, children);
    }
    let joined = continue88_jsx_join_attrs(attrs, true);
    format!("<{name} {joined}>{children}</{name}>")
}

/// Dual-oracle residual: opening+children+closing compose via open/close helpers.
#[must_use]
pub fn continue88_jsx_element_from_open_close(name: &str, children: &str) -> String {
    format!(
        "{}{}{}",
        continue88_jsx_opening_element_skeleton(name),
        children,
        continue88_jsx_closing_element_skeleton(name)
    )
}

/// Dual-oracle residual: member-named self-closing (`<Foo.Bar />`).
#[must_use]
pub fn continue88_jsx_member_element_self_closing(object: &str, property: &str) -> String {
    let name = continue88_jsx_member_expression_skeleton(object, property);
    continue88_jsx_element_self_closing_skeleton(&name)
}

/// Dual-oracle residual: namespaced self-closing (`<svg:path />`).
#[must_use]
pub fn continue88_jsx_namespaced_element_self_closing(ns: &str, name: &str) -> String {
    let tag = continue88_jsx_namespaced_name_skeleton(ns, name);
    continue88_jsx_element_self_closing_skeleton(&tag)
}

/// Dual-oracle residual: fragment wrapping a single element child.
#[must_use]
pub fn continue88_jsx_fragment_wrap_element(name: &str, children: &str) -> String {
    let inner = continue88_jsx_element_skeleton(name, children);
    continue88_jsx_fragment_skeleton(&inner)
}

/// Dual-oracle residual: attribute with expression container value.
#[must_use]
pub fn continue88_jsx_attribute_expr(name: &str, expr: &str) -> String {
    format!(
        "{name}={}",
        continue88_jsx_expression_container_skeleton(expr)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue44_jsx_attribute_skeleton, continue44_jsx_element_self_closing_skeleton,
        continue44_jsx_element_skeleton, continue44_jsx_expression_container_skeleton,
        continue44_jsx_fragment_skeleton, continue44_jsx_identifier_skeleton,
        continue44_jsx_text_skeleton, continue46_jsx_closing_element_skeleton,
        continue46_jsx_empty_expression_skeleton, continue46_jsx_member_expression_skeleton,
        continue46_jsx_namespaced_name_skeleton, continue46_jsx_opening_element_skeleton,
        continue46_jsx_spread_attribute_skeleton,
    };

    #[test]
    fn continue88_type_catalog() {
        assert_eq!(CONTINUE88_RELATED_TYPES.len(), 12);
        assert!(is_jsx_related_type("JSXElement"));
        assert!(is_jsx_related_type("JSXFragment"));
        assert!(is_jsx_related_type("JSXAttribute"));
        assert!(is_jsx_related_type("JSXIdentifier"));
        assert!(is_jsx_related_type("JSXText"));
        assert!(is_jsx_related_type("JSXExpressionContainer"));
        assert!(is_jsx_related_type("JSXMemberExpression"));
        assert!(is_jsx_related_type("JSXNamespacedName"));
        assert!(is_jsx_related_type("JSXSpreadAttribute"));
        assert!(is_jsx_related_type("JSXOpeningElement"));
        assert!(is_jsx_related_type("JSXClosingElement"));
        assert!(is_jsx_related_type("JSXEmptyExpression"));
        assert!(!is_jsx_related_type("ImportSpecifier"));
        assert!(!is_jsx_related_type("FunctionDeclaration"));
        assert!(!is_jsx_related_type("ExportSpecifier"));
        assert!(!is_jsx_related_type("JSXOpeningFragment"));
        assert!(is_continue88_jsx_element_type("JSXElement"));
        assert!(is_continue88_jsx_fragment_type("JSXFragment"));
        assert!(is_continue88_jsx_attribute_type("JSXAttribute"));
        assert!(is_continue88_jsx_identifier_type("JSXIdentifier"));
        assert!(is_continue88_jsx_text_type("JSXText"));
        assert!(is_continue88_jsx_expression_container_type(
            "JSXExpressionContainer"
        ));
        assert!(is_continue88_jsx_member_expression_type(
            "JSXMemberExpression"
        ));
        assert!(is_continue88_jsx_namespaced_name_type("JSXNamespacedName"));
        assert!(is_continue88_jsx_spread_attribute_type("JSXSpreadAttribute"));
        assert!(is_continue88_jsx_opening_element_type("JSXOpeningElement"));
        assert!(is_continue88_jsx_closing_element_type("JSXClosingElement"));
        assert!(is_continue88_jsx_empty_expression_type(
            "JSXEmptyExpression"
        ));
        assert!(!is_continue88_jsx_element_type("JSXFragment"));
        assert!(!is_continue88_jsx_fragment_type("JSXElement"));
    }

    #[test]
    fn continue88_element_fragment_attribute_dual_oracle() {
        assert_eq!(jsx_element_self_closing("div"), "<div />");
        assert_eq!(
            continue88_jsx_element_self_closing_skeleton("img"),
            continue44_jsx_element_self_closing_skeleton("img")
        );
        assert_eq!(jsx_element("span", "hi"), "<span>hi</span>");
        assert_eq!(
            continue88_jsx_element_skeleton("p", "x"),
            continue44_jsx_element_skeleton("p", "x")
        );
        assert_ne!(
            jsx_element_self_closing("a"),
            jsx_element("a", "")
        );

        assert_eq!(jsx_fragment("child"), "<>child</>");
        assert_eq!(
            continue88_jsx_fragment_skeleton("x"),
            continue44_jsx_fragment_skeleton("x")
        );
        assert_eq!(jsx_fragment(""), "<></>");

        assert_eq!(jsx_attribute("className", "btn"), "className=\"btn\"");
        assert_eq!(
            continue88_jsx_attribute_skeleton("id", "a"),
            continue44_jsx_attribute_skeleton("id", "a")
        );
        assert_ne!(
            jsx_attribute("a", "1"),
            jsx_attribute("a", "2")
        );

        assert_eq!(
            continue88_jsx_identifier_skeleton("Foo"),
            continue44_jsx_identifier_skeleton("Foo")
        );
        assert_eq!(continue88_jsx_identifier_skeleton("Foo"), "Foo");
        assert_eq!(
            continue88_jsx_text_skeleton("hello"),
            continue44_jsx_text_skeleton("hello")
        );
        assert_eq!(continue88_jsx_text_skeleton("hello"), "hello");

        assert_eq!(jsx_expression_container("x + 1"), "{x + 1}");
        assert_eq!(
            continue88_jsx_expression_container_skeleton("n"),
            continue44_jsx_expression_container_skeleton("n")
        );
        assert_ne!(
            jsx_expression_container("a"),
            jsx_expression_container("b")
        );
    }

    #[test]
    fn continue88_member_open_close_compose_dual_oracle() {
        assert_eq!(
            continue88_jsx_member_expression_skeleton("Foo", "Bar"),
            continue46_jsx_member_expression_skeleton("Foo", "Bar")
        );
        assert_eq!(
            continue88_jsx_member_expression_skeleton("Foo", "Bar"),
            "Foo.Bar"
        );
        assert_eq!(
            continue88_jsx_namespaced_name_skeleton("svg", "path"),
            continue46_jsx_namespaced_name_skeleton("svg", "path")
        );
        assert_eq!(
            continue88_jsx_namespaced_name_skeleton("svg", "path"),
            "svg:path"
        );
        assert_ne!(
            continue88_jsx_member_expression_skeleton("A", "B"),
            continue88_jsx_namespaced_name_skeleton("A", "B")
        );

        assert_eq!(jsx_spread_attribute("props"), "{...props}");
        assert_eq!(
            continue88_jsx_spread_attribute_skeleton("rest"),
            continue46_jsx_spread_attribute_skeleton("rest")
        );

        assert_eq!(
            continue88_jsx_opening_element_skeleton("div"),
            continue46_jsx_opening_element_skeleton("div")
        );
        assert_eq!(continue88_jsx_opening_element_skeleton("div"), "<div>");
        assert_eq!(
            continue88_jsx_closing_element_skeleton("div"),
            continue46_jsx_closing_element_skeleton("div")
        );
        assert_eq!(continue88_jsx_closing_element_skeleton("div"), "</div>");
        assert_ne!(
            continue88_jsx_opening_element_skeleton("x"),
            continue88_jsx_closing_element_skeleton("x")
        );

        assert_eq!(
            continue88_jsx_empty_expression_skeleton(),
            continue46_jsx_empty_expression_skeleton()
        );
        assert_eq!(continue88_jsx_empty_expression_skeleton(), "{}");

        assert_eq!(
            continue88_jsx_element_from_open_close("div", "hi"),
            "<div>hi</div>"
        );
        assert_eq!(
            continue88_jsx_element_from_open_close("div", "hi"),
            continue88_jsx_element_skeleton("div", "hi")
        );

        assert_eq!(
            continue88_jsx_member_element_self_closing("Foo", "Bar"),
            "<Foo.Bar />"
        );
        assert_eq!(
            continue88_jsx_namespaced_element_self_closing("svg", "path"),
            "<svg:path />"
        );
        assert_ne!(
            continue88_jsx_member_element_self_closing("a", "b"),
            continue88_jsx_namespaced_element_self_closing("a", "b")
        );

        let with_attrs = continue88_jsx_element_with_attrs_self_closing(
            "button",
            &["type=\"button\"", "disabled=\"true\""],
        );
        assert_eq!(with_attrs, "<button type=\"button\" disabled=\"true\" />");
        assert_eq!(
            continue88_jsx_element_with_attrs_self_closing("br", &[]),
            "<br />"
        );

        let el = continue88_jsx_element_with_attrs(
            "div",
            &["className=\"box\""],
            "body",
        );
        assert_eq!(el, "<div className=\"box\">body</div>");

        let frag = continue88_jsx_fragment_wrap_element("span", "x");
        assert_eq!(frag, "<><span>x</span></>");

        let attr_expr = continue88_jsx_attribute_expr("onClick", "handler");
        assert_eq!(attr_expr, "onClick={handler}");
        assert_ne!(attr_expr, jsx_attribute("onClick", "handler"));
    }
}

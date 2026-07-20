//! Pure TemplateLiteral + TaggedTemplateExpression + TemplateElement +
//! ArrayExpression + ObjectExpression + ArrayPattern + Property dual-oracle
//! emission — residual pure **continue92** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–91:
//! - TemplateLiteral dual-oracle composing real
//!   `continue23_empty_template_literal_skeleton` /
//!   `continue23_template_literal_no_expr_skeleton`
//! - TaggedTemplateExpression dual-oracle composing real
//!   `continue23_tagged_template_skeleton`
//! - TemplateElement dual-oracle composing real
//!   `continue23_template_element_cooked`
//! - ArrayExpression dual-oracle composing real
//!   `continue24_empty_array_skeleton` /
//!   `continue24_array_string_literals_skeleton`
//! - ObjectExpression dual-oracle composing real
//!   `continue24_empty_object_skeleton` /
//!   `continue24_object_prop_skeleton`
//! - ArrayPattern dual-oracle composing real
//!   `continue24_empty_array_pattern_skeleton`
//! - Property dual-oracle composing real `continue24_object_prop_skeleton`
//! - Composed template + container residual shells
//!
//! Intentionally does **not** re-wrap continue75 full multi-quasi template
//! interpolation / continue77–78 non-empty array-object pretty/minify poles
//! (those stay separate) or continue91 primitive literals. Composes real
//! shipped pure helpers from continue23/24 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue23_empty_template_literal_skeleton, continue23_tagged_template_skeleton,
    continue23_template_element_cooked, continue23_template_literal_no_expr_skeleton,
    continue24_array_string_literals_skeleton, continue24_empty_array_pattern_skeleton,
    continue24_empty_array_skeleton, continue24_empty_object_skeleton,
    continue24_object_prop_skeleton,
};

/// Dual-oracle residual: continue92 related AST type catalog.
pub const CONTINUE92_RELATED_TYPES: &[&str] = &[
    "TemplateLiteral",
    "TaggedTemplateExpression",
    "TemplateElement",
    "ArrayExpression",
    "ObjectExpression",
    "ArrayPattern",
    "Property",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_template_array_object_related_type(t: &str) -> bool {
    CONTINUE92_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue92_template_literal_type(t: &str) -> bool {
    t == "TemplateLiteral"
}

#[must_use]
pub fn is_continue92_tagged_template_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

#[must_use]
pub fn is_continue92_template_element_type(t: &str) -> bool {
    t == "TemplateElement"
}

#[must_use]
pub fn is_continue92_array_expression_type(t: &str) -> bool {
    t == "ArrayExpression"
}

#[must_use]
pub fn is_continue92_object_expression_type(t: &str) -> bool {
    t == "ObjectExpression"
}

#[must_use]
pub fn is_continue92_array_pattern_type(t: &str) -> bool {
    t == "ArrayPattern"
}

#[must_use]
pub fn is_continue92_property_type(t: &str) -> bool {
    t == "Property" || t == "ObjectProperty"
}

// ── TemplateLiteral dual-oracle ─────────────────────────────────────────────

/// Dual-oracle empty TemplateLiteral skeleton composing real
/// [`continue23_empty_template_literal_skeleton`].
#[must_use]
pub fn continue92_empty_template_literal_skeleton() -> &'static str {
    continue23_empty_template_literal_skeleton()
}

/// Dual-oracle TemplateLiteral no-expr skeleton composing real
/// [`continue23_template_literal_no_expr_skeleton`].
#[must_use]
pub fn continue92_template_literal_skeleton(cooked: &str) -> String {
    continue23_template_literal_no_expr_skeleton(cooked)
}

/// Dual-oracle TemplateLiteral pretty alias.
#[must_use]
pub fn template_literal_pretty(cooked: &str) -> String {
    continue92_template_literal_skeleton(cooked)
}

/// Dual-oracle TemplateLiteral minify alias.
#[must_use]
pub fn template_literal_minify(cooked: &str) -> String {
    continue92_template_literal_skeleton(cooked)
}

// ── TaggedTemplateExpression dual-oracle ────────────────────────────────────

/// Dual-oracle TaggedTemplate skeleton composing real
/// [`continue23_tagged_template_skeleton`].
#[must_use]
pub fn continue92_tagged_template_skeleton(tag: &str) -> String {
    continue23_tagged_template_skeleton(tag)
}

/// Dual-oracle TaggedTemplate pretty alias.
#[must_use]
pub fn tagged_template_pretty(tag: &str) -> String {
    continue92_tagged_template_skeleton(tag)
}

/// Dual-oracle TaggedTemplate minify alias.
#[must_use]
pub fn tagged_template_minify(tag: &str) -> String {
    continue92_tagged_template_skeleton(tag)
}

// ── TemplateElement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle TemplateElement cooked fragment composing real
/// [`continue23_template_element_cooked`].
#[must_use]
pub fn continue92_template_element_skeleton(cooked: &str) -> String {
    continue23_template_element_cooked(cooked)
}

/// Dual-oracle TemplateElement pretty alias.
#[must_use]
pub fn template_element_pretty(cooked: &str) -> String {
    continue92_template_element_skeleton(cooked)
}

/// Dual-oracle TemplateElement minify alias.
#[must_use]
pub fn template_element_minify(cooked: &str) -> String {
    continue92_template_element_skeleton(cooked)
}

// ── ArrayExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle empty ArrayExpression skeleton composing real
/// [`continue24_empty_array_skeleton`].
#[must_use]
pub fn continue92_empty_array_skeleton() -> &'static str {
    continue24_empty_array_skeleton()
}

/// Dual-oracle ArrayExpression of string literals composing real
/// [`continue24_array_string_literals_skeleton`].
#[must_use]
pub fn continue92_array_expression_skeleton(elems: &[&str]) -> String {
    continue24_array_string_literals_skeleton(elems)
}

/// Dual-oracle ArrayExpression pretty alias (empty when no elems).
#[must_use]
pub fn array_expression_pretty(elems: &[&str]) -> String {
    if elems.is_empty() {
        continue92_empty_array_skeleton().to_string()
    } else {
        continue92_array_expression_skeleton(elems)
    }
}

/// Dual-oracle ArrayExpression minify alias.
#[must_use]
pub fn array_expression_minify(elems: &[&str]) -> String {
    array_expression_pretty(elems)
}

// ── ObjectExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle empty ObjectExpression skeleton composing real
/// [`continue24_empty_object_skeleton`].
#[must_use]
pub fn continue92_empty_object_skeleton() -> &'static str {
    continue24_empty_object_skeleton()
}

/// Dual-oracle ObjectExpression single-prop skeleton composing real
/// [`continue24_object_prop_skeleton`].
#[must_use]
pub fn continue92_object_expression_skeleton(key: &str, value_raw: &str) -> String {
    continue24_object_prop_skeleton(key, value_raw)
}

/// Dual-oracle ObjectExpression pretty alias.
#[must_use]
pub fn object_expression_pretty(key: &str, value_raw: &str) -> String {
    continue92_object_expression_skeleton(key, value_raw)
}

/// Dual-oracle ObjectExpression minify alias.
#[must_use]
pub fn object_expression_minify(key: &str, value_raw: &str) -> String {
    continue92_object_expression_skeleton(key, value_raw)
}

// ── ArrayPattern dual-oracle ────────────────────────────────────────────────

/// Dual-oracle empty ArrayPattern skeleton composing real
/// [`continue24_empty_array_pattern_skeleton`].
#[must_use]
pub fn continue92_empty_array_pattern_skeleton() -> &'static str {
    continue24_empty_array_pattern_skeleton()
}

/// Dual-oracle ArrayPattern pretty alias.
#[must_use]
pub fn array_pattern_pretty() -> &'static str {
    continue92_empty_array_pattern_skeleton()
}

/// Dual-oracle ArrayPattern minify alias.
#[must_use]
pub fn array_pattern_minify() -> &'static str {
    continue92_empty_array_pattern_skeleton()
}

// ── Property dual-oracle ────────────────────────────────────────────────────

/// Dual-oracle Property skeleton composing real
/// [`continue24_object_prop_skeleton`] (property interior = object shell).
#[must_use]
pub fn continue92_property_skeleton(key: &str, value_raw: &str) -> String {
    continue24_object_prop_skeleton(key, value_raw)
}

/// Dual-oracle Property pretty alias.
#[must_use]
pub fn property_pretty(key: &str, value_raw: &str) -> String {
    continue92_property_skeleton(key, value_raw)
}

/// Dual-oracle Property minify alias.
#[must_use]
pub fn property_minify(key: &str, value_raw: &str) -> String {
    continue92_property_skeleton(key, value_raw)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: tagged template with cooked quasi body.
#[must_use]
pub fn continue92_tagged_template_cooked(tag: &str, cooked: &str) -> String {
    format!("{tag}{}", continue92_template_literal_skeleton(cooked))
}

/// Dual-oracle residual: array of template-literal-as-element shells.
/// Elements are already emitted tokens (not re-quoted).
#[must_use]
pub fn continue92_join_container_elems(parts: &[&str], pretty: bool) -> String {
    let sep = if pretty { ", " } else { "," };
    parts.join(sep)
}

/// Dual-oracle residual: empty array + empty object pair shell.
#[must_use]
pub fn continue92_empty_containers_shell(pretty: bool) -> String {
    let a = continue92_empty_array_skeleton();
    let o = continue92_empty_object_skeleton();
    let sep = if pretty { ", " } else { "," };
    format!("{a}{sep}{o}")
}

/// Dual-oracle residual: object prop holding empty array pattern value.
#[must_use]
pub fn continue92_object_prop_empty_array(key: &str) -> String {
    continue92_object_expression_skeleton(key, continue92_empty_array_skeleton())
}

/// Dual-oracle residual: template + empty array compose shell.
#[must_use]
pub fn continue92_template_then_empty_array(cooked: &str) -> String {
    format!(
        "{};{}",
        continue92_template_literal_skeleton(cooked),
        continue92_empty_array_skeleton()
    )
}

/// Dual-oracle residual: pretty vs minify container separator pole.
#[must_use]
pub fn continue92_container_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue23_empty_template_literal_skeleton, continue23_tagged_template_skeleton,
        continue23_template_element_cooked, continue23_template_literal_no_expr_skeleton,
        continue24_array_string_literals_skeleton, continue24_empty_array_pattern_skeleton,
        continue24_empty_array_skeleton, continue24_empty_object_skeleton,
        continue24_object_prop_skeleton,
    };

    #[test]
    fn continue92_type_catalog() {
        assert_eq!(CONTINUE92_RELATED_TYPES.len(), 7);
        assert!(is_template_array_object_related_type("TemplateLiteral"));
        assert!(is_template_array_object_related_type("TaggedTemplateExpression"));
        assert!(is_template_array_object_related_type("TemplateElement"));
        assert!(is_template_array_object_related_type("ArrayExpression"));
        assert!(is_template_array_object_related_type("ObjectExpression"));
        assert!(is_template_array_object_related_type("ArrayPattern"));
        assert!(is_template_array_object_related_type("Property"));
        assert!(!is_template_array_object_related_type("StringLiteral"));
        assert!(!is_template_array_object_related_type("NullLiteral"));
        assert!(!is_template_array_object_related_type("JSXElement"));
        assert!(!is_template_array_object_related_type("ObjectPattern"));
        assert!(is_continue92_template_literal_type("TemplateLiteral"));
        assert!(is_continue92_tagged_template_type("TaggedTemplateExpression"));
        assert!(is_continue92_template_element_type("TemplateElement"));
        assert!(is_continue92_array_expression_type("ArrayExpression"));
        assert!(is_continue92_object_expression_type("ObjectExpression"));
        assert!(is_continue92_array_pattern_type("ArrayPattern"));
        assert!(is_continue92_property_type("Property"));
        assert!(is_continue92_property_type("ObjectProperty"));
        assert!(!is_continue92_property_type("ClassProperty"));
        assert!(!is_continue92_template_literal_type("TaggedTemplateExpression"));
    }

    #[test]
    fn continue92_template_dual_oracle() {
        assert_eq!(continue92_empty_template_literal_skeleton(), "``");
        assert_eq!(
            continue92_empty_template_literal_skeleton(),
            continue23_empty_template_literal_skeleton()
        );
        assert_eq!(continue92_template_literal_skeleton("hi"), "`hi`");
        assert_eq!(
            continue92_template_literal_skeleton("hi"),
            continue23_template_literal_no_expr_skeleton("hi")
        );
        assert_eq!(template_literal_pretty("x"), template_literal_minify("x"));
        assert_eq!(template_literal_pretty("x"), "`x`");
        assert_ne!(
            template_literal_pretty("a"),
            template_literal_pretty("b")
        );

        assert_eq!(continue92_tagged_template_skeleton("tag"), "tag``");
        assert_eq!(
            continue92_tagged_template_skeleton("String.raw"),
            continue23_tagged_template_skeleton("String.raw")
        );
        assert_eq!(tagged_template_pretty("t"), tagged_template_minify("t"));
        assert_eq!(tagged_template_pretty("t"), "t``");

        assert_eq!(continue92_template_element_skeleton("cooked"), "cooked");
        assert_eq!(
            continue92_template_element_skeleton("cooked"),
            continue23_template_element_cooked("cooked")
        );
        assert_eq!(template_element_pretty("q"), template_element_minify("q"));
        assert_eq!(
            continue92_tagged_template_cooked("css", "body{}"),
            "css`body{}`"
        );
    }

    #[test]
    fn continue92_array_object_pattern_dual_oracle() {
        assert_eq!(continue92_empty_array_skeleton(), "[]");
        assert_eq!(
            continue92_empty_array_skeleton(),
            continue24_empty_array_skeleton()
        );
        assert_eq!(continue92_empty_object_skeleton(), "{}");
        assert_eq!(
            continue92_empty_object_skeleton(),
            continue24_empty_object_skeleton()
        );
        assert_eq!(continue92_empty_array_pattern_skeleton(), "[]");
        assert_eq!(
            continue92_empty_array_pattern_skeleton(),
            continue24_empty_array_pattern_skeleton()
        );
        assert_eq!(array_pattern_pretty(), array_pattern_minify());

        assert_eq!(
            continue92_array_expression_skeleton(&["a", "b"]),
            "[\"a\",\"b\"]"
        );
        assert_eq!(
            continue92_array_expression_skeleton(&["a", "b"]),
            continue24_array_string_literals_skeleton(&["a", "b"])
        );
        assert_eq!(array_expression_pretty(&[]), "[]");
        assert_eq!(array_expression_minify(&["x"]), "[\"x\"]");
        assert_ne!(
            array_expression_pretty(&["a"]),
            array_expression_pretty(&["b"])
        );

        assert_eq!(continue92_object_expression_skeleton("k", "1"), "{k:1}");
        assert_eq!(
            continue92_object_expression_skeleton("k", "1"),
            continue24_object_prop_skeleton("k", "1")
        );
        assert_eq!(
            object_expression_pretty("a", "0"),
            object_expression_minify("a", "0")
        );
        assert_eq!(continue92_property_skeleton("p", "true"), "{p:true}");
        assert_eq!(
            property_pretty("p", "true"),
            property_minify("p", "true")
        );
        assert_ne!(
            property_pretty("a", "1"),
            property_pretty("a", "2")
        );

        assert_eq!(
            continue92_join_container_elems(&["[]", "{}"], true),
            "[], {}"
        );
        assert_eq!(
            continue92_join_container_elems(&["[]", "{}"], false),
            "[],{}"
        );
        assert_eq!(continue92_empty_containers_shell(false), "[],{}");
        assert_eq!(continue92_empty_containers_shell(true), "[], {}");
        assert_ne!(
            continue92_empty_containers_shell(true),
            continue92_empty_containers_shell(false)
        );
        assert_eq!(continue92_object_prop_empty_array("items"), "{items:[]}");
        assert_eq!(
            continue92_template_then_empty_array("ok"),
            "`ok`;[]"
        );
        assert_eq!(continue92_container_sep(true), ", ");
        assert_eq!(continue92_container_sep(false), ",");
    }
}

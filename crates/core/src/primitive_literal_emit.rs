//! Pure NullLiteral + BooleanLiteral + NumericLiteral + StringLiteral +
//! ClassProperty dual-oracle emission — residual pure **continue91** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–90:
//! - NullLiteral dual-oracle composing real `continue22_null_literal_skeleton`
//! - BooleanLiteral dual-oracle composing real
//!   `continue22_boolean_literal_skeleton`
//! - NumericLiteral dual-oracle composing real
//!   `continue22_numeric_literal_skeleton`
//! - StringLiteral dual-oracle composing real
//!   `continue22_string_literal_skeleton`
//! - ClassProperty dual-oracle composing real
//!   `continue33_class_property_skeleton`
//! - Composed null/bool/num/string + class-field residual shells
//!
//! Intentionally does **not** re-wrap continue90 BigInt/RegExp/Directive/
//! PrivateName/Parenthesized (those stay separate) or continue85 generic
//! Literal/string_literal_token poles. Composes real shipped pure helpers
//! from continue22/33 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue22_boolean_literal_skeleton, continue22_null_literal_skeleton,
    continue22_numeric_literal_skeleton, continue22_string_literal_skeleton,
    continue33_class_property_skeleton,
};

/// Dual-oracle residual: continue91 related AST type catalog.
pub const CONTINUE91_RELATED_TYPES: &[&str] = &[
    "NullLiteral",
    "BooleanLiteral",
    "NumericLiteral",
    "StringLiteral",
    "ClassProperty",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_primitive_literal_related_type(t: &str) -> bool {
    CONTINUE91_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue91_null_literal_type(t: &str) -> bool {
    t == "NullLiteral"
}

#[must_use]
pub fn is_continue91_boolean_literal_type(t: &str) -> bool {
    t == "BooleanLiteral"
}

#[must_use]
pub fn is_continue91_numeric_literal_type(t: &str) -> bool {
    t == "NumericLiteral"
}

#[must_use]
pub fn is_continue91_string_literal_type(t: &str) -> bool {
    t == "StringLiteral"
}

#[must_use]
pub fn is_continue91_class_property_type(t: &str) -> bool {
    t == "ClassProperty" || t == "ClassPropertyDefinition"
}

// ── NullLiteral dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle NullLiteral skeleton composing real
/// [`continue22_null_literal_skeleton`].
#[must_use]
pub fn continue91_null_literal_skeleton() -> &'static str {
    continue22_null_literal_skeleton()
}

/// Dual-oracle NullLiteral pretty alias.
#[must_use]
pub fn null_literal_pretty() -> &'static str {
    continue91_null_literal_skeleton()
}

/// Dual-oracle NullLiteral minify alias.
#[must_use]
pub fn null_literal_minify() -> &'static str {
    continue91_null_literal_skeleton()
}

// ── BooleanLiteral dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BooleanLiteral skeleton composing real
/// [`continue22_boolean_literal_skeleton`].
#[must_use]
pub fn continue91_boolean_literal_skeleton(value: bool) -> &'static str {
    continue22_boolean_literal_skeleton(value)
}

/// Dual-oracle BooleanLiteral pretty alias.
#[must_use]
pub fn boolean_literal_pretty(value: bool) -> &'static str {
    continue91_boolean_literal_skeleton(value)
}

/// Dual-oracle BooleanLiteral minify alias.
#[must_use]
pub fn boolean_literal_minify(value: bool) -> &'static str {
    continue91_boolean_literal_skeleton(value)
}

// ── NumericLiteral dual-oracle ──────────────────────────────────────────────

/// Dual-oracle NumericLiteral skeleton composing real
/// [`continue22_numeric_literal_skeleton`].
#[must_use]
pub fn continue91_numeric_literal_skeleton(raw: &str) -> String {
    continue22_numeric_literal_skeleton(raw)
}

/// Dual-oracle NumericLiteral pretty alias.
#[must_use]
pub fn numeric_literal_pretty(raw: &str) -> String {
    continue91_numeric_literal_skeleton(raw)
}

/// Dual-oracle NumericLiteral minify alias.
#[must_use]
pub fn numeric_literal_minify(raw: &str) -> String {
    continue91_numeric_literal_skeleton(raw)
}

// ── StringLiteral dual-oracle ───────────────────────────────────────────────

/// Dual-oracle StringLiteral skeleton composing real
/// [`continue22_string_literal_skeleton`].
#[must_use]
pub fn continue91_string_literal_skeleton(value: &str, double_quote: bool) -> String {
    continue22_string_literal_skeleton(value, double_quote)
}

/// Dual-oracle StringLiteral pretty alias (single-quoted, format-style).
#[must_use]
pub fn string_literal_pretty(value: &str) -> String {
    continue91_string_literal_skeleton(value, false)
}

/// Dual-oracle StringLiteral minify alias (double-quoted).
#[must_use]
pub fn string_literal_minify(value: &str) -> String {
    continue91_string_literal_skeleton(value, true)
}

// ── ClassProperty dual-oracle ───────────────────────────────────────────────

/// Dual-oracle ClassProperty skeleton composing real
/// [`continue33_class_property_skeleton`].
#[must_use]
pub fn continue91_class_property_skeleton(name: &str, value: Option<&str>) -> String {
    continue33_class_property_skeleton(name, value)
}

/// Dual-oracle ClassProperty pretty alias (with initializer).
#[must_use]
pub fn class_property_pretty(name: &str, value: &str) -> String {
    continue91_class_property_skeleton(name, Some(value))
}

/// Dual-oracle ClassProperty minify alias (with initializer).
#[must_use]
pub fn class_property_minify(name: &str, value: &str) -> String {
    continue91_class_property_skeleton(name, Some(value))
}

/// Dual-oracle ClassProperty bare field (no initializer).
#[must_use]
pub fn class_property_bare(name: &str) -> String {
    continue91_class_property_skeleton(name, None)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: class field initialized to null (`name = null;`).
#[must_use]
pub fn continue91_class_property_null(name: &str) -> String {
    continue91_class_property_skeleton(name, Some(continue91_null_literal_skeleton()))
}

/// Dual-oracle residual: class field initialized to boolean.
#[must_use]
pub fn continue91_class_property_bool(name: &str, value: bool) -> String {
    continue91_class_property_skeleton(name, Some(continue91_boolean_literal_skeleton(value)))
}

/// Dual-oracle residual: class field initialized to numeric raw.
#[must_use]
pub fn continue91_class_property_numeric(name: &str, raw: &str) -> String {
    let n = continue91_numeric_literal_skeleton(raw);
    continue91_class_property_skeleton(name, Some(&n))
}

/// Dual-oracle residual: class field initialized to double-quoted string.
#[must_use]
pub fn continue91_class_property_string(name: &str, value: &str) -> String {
    let s = continue91_string_literal_skeleton(value, true);
    continue91_class_property_skeleton(name, Some(&s))
}

/// Dual-oracle residual: pretty vs minify string quote style separator pole.
#[must_use]
pub fn continue91_string_quote_double(pretty: bool) -> bool {
    // pretty → single quote (false); minify → double quote (true)
    !pretty
}

/// Dual-oracle residual: join primitive literal tokens (array-like).
#[must_use]
pub fn continue91_join_primitives(parts: &[&str], pretty: bool) -> String {
    let sep = if pretty { ", " } else { "," };
    parts.join(sep)
}

/// Dual-oracle residual: primitive array shell `[null, true, 1, "a"]`.
#[must_use]
pub fn continue91_primitive_array_shell(pretty: bool) -> String {
    let null = continue91_null_literal_skeleton();
    let t = continue91_boolean_literal_skeleton(true);
    let n = continue91_numeric_literal_skeleton("1");
    let s = continue91_string_literal_skeleton("a", continue91_string_quote_double(pretty));
    let inner = continue91_join_primitives(&[null, t, n.as_str(), s.as_str()], pretty);
    format!("[{inner}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue22_boolean_literal_skeleton, continue22_null_literal_skeleton,
        continue22_numeric_literal_skeleton, continue22_string_literal_skeleton,
        continue33_class_property_skeleton,
    };

    #[test]
    fn continue91_type_catalog() {
        assert_eq!(CONTINUE91_RELATED_TYPES.len(), 5);
        assert!(is_primitive_literal_related_type("NullLiteral"));
        assert!(is_primitive_literal_related_type("BooleanLiteral"));
        assert!(is_primitive_literal_related_type("NumericLiteral"));
        assert!(is_primitive_literal_related_type("StringLiteral"));
        assert!(is_primitive_literal_related_type("ClassProperty"));
        assert!(!is_primitive_literal_related_type("BigIntLiteral"));
        assert!(!is_primitive_literal_related_type("RegExpLiteral"));
        assert!(!is_primitive_literal_related_type("Literal"));
        assert!(!is_primitive_literal_related_type("Directive"));
        assert!(!is_primitive_literal_related_type("JSXElement"));
        assert!(is_continue91_null_literal_type("NullLiteral"));
        assert!(is_continue91_boolean_literal_type("BooleanLiteral"));
        assert!(is_continue91_numeric_literal_type("NumericLiteral"));
        assert!(is_continue91_string_literal_type("StringLiteral"));
        assert!(is_continue91_class_property_type("ClassProperty"));
        assert!(is_continue91_class_property_type("ClassPropertyDefinition"));
        assert!(!is_continue91_null_literal_type("BooleanLiteral"));
        assert!(!is_continue91_string_literal_type("NumericLiteral"));
    }

    #[test]
    fn continue91_null_bool_num_string_dual_oracle() {
        assert_eq!(continue91_null_literal_skeleton(), "null");
        assert_eq!(
            continue91_null_literal_skeleton(),
            continue22_null_literal_skeleton()
        );
        assert_eq!(null_literal_pretty(), null_literal_minify());
        assert_eq!(null_literal_pretty(), "null");

        assert_eq!(continue91_boolean_literal_skeleton(true), "true");
        assert_eq!(continue91_boolean_literal_skeleton(false), "false");
        assert_eq!(
            continue91_boolean_literal_skeleton(true),
            continue22_boolean_literal_skeleton(true)
        );
        assert_eq!(
            continue91_boolean_literal_skeleton(false),
            continue22_boolean_literal_skeleton(false)
        );
        assert_eq!(boolean_literal_pretty(true), boolean_literal_minify(true));
        assert_ne!(
            boolean_literal_pretty(true),
            boolean_literal_pretty(false)
        );

        assert_eq!(continue91_numeric_literal_skeleton("42"), "42");
        assert_eq!(
            continue91_numeric_literal_skeleton(" 3.14 "),
            continue22_numeric_literal_skeleton(" 3.14 ")
        );
        assert_eq!(continue91_numeric_literal_skeleton(" 3.14 "), "3.14");
        assert_eq!(numeric_literal_pretty("0x2a"), "0x2a");
        assert_eq!(numeric_literal_minify("0x2a"), "0x2a");
        assert_ne!(
            numeric_literal_pretty("1"),
            numeric_literal_pretty("2")
        );

        assert_eq!(
            continue91_string_literal_skeleton("hi", true),
            "\"hi\""
        );
        assert_eq!(
            continue91_string_literal_skeleton("hi", false),
            "'hi'"
        );
        assert_eq!(
            continue91_string_literal_skeleton("x", true),
            continue22_string_literal_skeleton("x", true)
        );
        assert_eq!(string_literal_pretty("hi"), "'hi'");
        assert_eq!(string_literal_minify("hi"), "\"hi\"");
        assert_ne!(string_literal_pretty("hi"), string_literal_minify("hi"));
        assert!(!continue91_string_quote_double(true));
        assert!(continue91_string_quote_double(false));
    }

    #[test]
    fn continue91_class_property_compose_dual_oracle() {
        assert_eq!(
            continue91_class_property_skeleton("x", Some("1")),
            "x = 1;"
        );
        assert_eq!(
            continue91_class_property_skeleton("x", Some("1")),
            continue33_class_property_skeleton("x", Some("1"))
        );
        assert_eq!(continue91_class_property_skeleton("y", None), "y;");
        assert_eq!(
            continue91_class_property_skeleton("y", None),
            continue33_class_property_skeleton("y", None)
        );
        assert_eq!(class_property_pretty("a", "0"), "a = 0;");
        assert_eq!(class_property_minify("a", "0"), "a = 0;");
        assert_eq!(class_property_bare("z"), "z;");
        assert_ne!(class_property_pretty("a", "1"), class_property_pretty("a", "2"));

        assert_eq!(continue91_class_property_null("n"), "n = null;");
        assert_eq!(continue91_class_property_bool("f", false), "f = false;");
        assert_eq!(continue91_class_property_bool("t", true), "t = true;");
        assert_eq!(continue91_class_property_numeric("c", "42"), "c = 42;");
        assert_eq!(
            continue91_class_property_string("s", "ok"),
            "s = \"ok\";"
        );
        assert_ne!(
            continue91_class_property_null("x"),
            continue91_class_property_bool("x", true)
        );

        assert_eq!(
            continue91_join_primitives(&["null", "true"], true),
            "null, true"
        );
        assert_eq!(
            continue91_join_primitives(&["null", "true"], false),
            "null,true"
        );
        assert_eq!(
            continue91_primitive_array_shell(false),
            "[null,true,1,\"a\"]"
        );
        assert_eq!(
            continue91_primitive_array_shell(true),
            "[null, true, 1, 'a']"
        );
        assert_ne!(
            continue91_primitive_array_shell(true),
            continue91_primitive_array_shell(false)
        );
    }
}

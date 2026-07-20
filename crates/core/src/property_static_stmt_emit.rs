//! Pure PropertyDefinition + StaticBlock + ClassExpression + ClassBody +
//! PrivateIdentifier + AccessorProperty dual-oracle emission — residual pure
//! **continue82** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - PropertyDefinition dual-oracle pretty/minify static/private/semi composing
//!   real `property_definition_skeleton` (continue20 base)
//! - StaticBlock dual-oracle empty/non-empty pretty/minify
//! - ClassExpression dual-oracle named/anon body interior
//! - ClassBody dual-oracle via class expression body
//! - PrivateIdentifier dual-oracle key token
//! - AccessorProperty dual-oracle static/value/semi
//! - Computed property key dual-oracle brackets
//! - Method-like field dual-oracle async/static/kind prefixes
//!
//! Intentionally does **not** re-wrap continue64–81 partition shells
//! (try/throw/import continue81 stays separate; for/while/switch continue80
//! stays separate; class-extends continue71 / export-default continue72 stay
//! separate). Composes real shipped pure helpers from `property_static_emit`.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::property_static_emit::{
    accessor_property_skeleton, class_expression_skeleton, class_method_like_field_skeleton,
    computed_property_key, property_definition_skeleton, property_key_token, static_block_skeleton,
    static_prefix,
};

/// Dual-oracle residual: continue82 related AST type catalog.
pub const CONTINUE82_RELATED_TYPES: &[&str] = &[
    "PropertyDefinition",
    "StaticBlock",
    "ClassExpression",
    "ClassBody",
    "PrivateIdentifier",
    "AccessorProperty",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_property_static_stmt_related_type(t: &str) -> bool {
    CONTINUE82_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue82_property_definition_type(t: &str) -> bool {
    t == "PropertyDefinition"
}

#[must_use]
pub fn is_continue82_static_block_type(t: &str) -> bool {
    t == "StaticBlock"
}

#[must_use]
pub fn is_continue82_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue82_class_body_type(t: &str) -> bool {
    t == "ClassBody"
}

#[must_use]
pub fn is_continue82_private_identifier_type(t: &str) -> bool {
    t == "PrivateIdentifier"
}

#[must_use]
pub fn is_continue82_accessor_property_type(t: &str) -> bool {
    t == "AccessorProperty"
}

// ── PropertyDefinition dual-oracle ──────────────────────────────────────────

/// Dual-oracle PropertyDefinition skeleton composing real
/// [`property_definition_skeleton`].
#[must_use]
pub fn continue82_property_definition_skeleton(
    key: &str,
    value: Option<&str>,
    is_static: bool,
    is_private: bool,
    pretty: bool,
    semi: bool,
) -> String {
    property_definition_skeleton(key, value, is_static, is_private, pretty, semi)
}

/// Convenience: pretty public field with value + semi.
#[must_use]
pub fn property_definition_pretty(key: &str, value: &str) -> String {
    continue82_property_definition_skeleton(key, Some(value), false, false, true, true)
}

/// Convenience: minify public field with value + semi.
#[must_use]
pub fn property_definition_minify(key: &str, value: &str) -> String {
    continue82_property_definition_skeleton(key, Some(value), false, false, false, true)
}

/// Convenience: pretty static field.
#[must_use]
pub fn property_definition_static_pretty(key: &str, value: &str) -> String {
    continue82_property_definition_skeleton(key, Some(value), true, false, true, true)
}

/// Convenience: private bare field with semi.
#[must_use]
pub fn property_definition_private_semi(key: &str) -> String {
    continue82_property_definition_skeleton(key, None, false, true, true, true)
}

// ── StaticBlock dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle StaticBlock skeleton composing real [`static_block_skeleton`].
#[must_use]
pub fn continue82_static_block_skeleton(body_stmts: &[&str], pretty: bool) -> String {
    static_block_skeleton(body_stmts, pretty)
}

/// Convenience: pretty empty static block.
#[must_use]
pub fn static_block_empty_pretty() -> String {
    continue82_static_block_skeleton(&[], true)
}

/// Convenience: minify empty static block.
#[must_use]
pub fn static_block_empty_minify() -> String {
    continue82_static_block_skeleton(&[], false)
}

/// Convenience: pretty static block with body stmts.
#[must_use]
pub fn static_block_pretty(body_stmts: &[&str]) -> String {
    continue82_static_block_skeleton(body_stmts, true)
}

/// Convenience: minify static block with body stmts.
#[must_use]
pub fn static_block_minify(body_stmts: &[&str]) -> String {
    continue82_static_block_skeleton(body_stmts, false)
}

// ── ClassExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ClassExpression skeleton composing real
/// [`class_expression_skeleton`].
#[must_use]
pub fn continue82_class_expression_skeleton(
    name: Option<&str>,
    methods: &[&str],
    pretty: bool,
) -> String {
    class_expression_skeleton(name, methods, pretty)
}

/// Convenience: pretty anonymous class expression.
#[must_use]
pub fn class_expression_anon_pretty(methods: &[&str]) -> String {
    continue82_class_expression_skeleton(None, methods, true)
}

/// Convenience: pretty named class expression.
#[must_use]
pub fn class_expression_named_pretty(name: &str, methods: &[&str]) -> String {
    continue82_class_expression_skeleton(Some(name), methods, true)
}

/// Convenience: minify named class expression.
#[must_use]
pub fn class_expression_named_minify(name: &str, methods: &[&str]) -> String {
    continue82_class_expression_skeleton(Some(name), methods, false)
}

// ── PrivateIdentifier / static prefix dual-oracle ───────────────────────────

/// Dual-oracle private vs public key token composing real [`property_key_token`].
#[must_use]
pub fn continue82_property_key_token(key: &str, is_private: bool) -> String {
    property_key_token(key, is_private)
}

/// Dual-oracle static prefix composing real [`static_prefix`].
#[must_use]
pub fn continue82_static_prefix(is_static: bool) -> &'static str {
    static_prefix(is_static)
}

// ── AccessorProperty dual-oracle ────────────────────────────────────────────

/// Dual-oracle AccessorProperty skeleton composing real
/// [`accessor_property_skeleton`].
#[must_use]
pub fn continue82_accessor_property_skeleton(
    key: &str,
    value: Option<&str>,
    is_static: bool,
    pretty: bool,
    semi: bool,
) -> String {
    accessor_property_skeleton(key, value, is_static, pretty, semi)
}

/// Convenience: pretty accessor with value + semi.
#[must_use]
pub fn accessor_property_pretty(key: &str, value: &str) -> String {
    continue82_accessor_property_skeleton(key, Some(value), false, true, true)
}

/// Convenience: minify accessor with value + semi.
#[must_use]
pub fn accessor_property_minify(key: &str, value: &str) -> String {
    continue82_accessor_property_skeleton(key, Some(value), false, false, true)
}

/// Convenience: pretty static accessor bare.
#[must_use]
pub fn accessor_property_static_pretty(key: &str) -> String {
    continue82_accessor_property_skeleton(key, None, true, true, true)
}

// ── Computed key + method-like field dual-oracle ────────────────────────────

/// Dual-oracle computed property key composing real [`computed_property_key`].
#[must_use]
pub fn continue82_computed_property_key(expr: &str) -> String {
    computed_property_key(expr)
}

/// Dual-oracle method-like field skeleton composing real
/// [`class_method_like_field_skeleton`].
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn continue82_class_method_like_field_skeleton(
    key: &str,
    params: &str,
    body: &str,
    kind: &str,
    is_static: bool,
    is_async: bool,
    is_generator: bool,
    pretty: bool,
) -> String {
    class_method_like_field_skeleton(
        key, params, body, kind, is_static, is_async, is_generator, pretty,
    )
}

/// Convenience: pretty static async method-like field.
#[must_use]
pub fn class_method_like_static_async_pretty(key: &str, params: &str, body: &str) -> String {
    continue82_class_method_like_field_skeleton(
        key, params, body, "method", true, true, false, true,
    )
}

/// Dual-oracle residual: compose class expression holding a property definition.
#[must_use]
pub fn continue82_class_with_field(
    name: &str,
    field_key: &str,
    field_value: &str,
    pretty: bool,
) -> String {
    let field = continue82_property_definition_skeleton(
        field_key,
        Some(field_value),
        false,
        false,
        pretty,
        true,
    );
    continue82_class_expression_skeleton(Some(name), &[field.as_str()], pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property_static_emit::{
        accessor_property_skeleton, class_expression_skeleton, property_definition_skeleton,
        static_block_skeleton,
    };

    #[test]
    fn continue82_type_catalog() {
        assert_eq!(CONTINUE82_RELATED_TYPES.len(), 6);
        assert!(is_property_static_stmt_related_type("PropertyDefinition"));
        assert!(is_property_static_stmt_related_type("StaticBlock"));
        assert!(is_property_static_stmt_related_type("ClassExpression"));
        assert!(is_property_static_stmt_related_type("ClassBody"));
        assert!(is_property_static_stmt_related_type("PrivateIdentifier"));
        assert!(is_property_static_stmt_related_type("AccessorProperty"));
        assert!(!is_property_static_stmt_related_type("ClassDeclaration"));
        assert!(!is_property_static_stmt_related_type("TryStatement"));
        assert!(!is_property_static_stmt_related_type("MethodDefinition"));
        assert!(is_continue82_property_definition_type("PropertyDefinition"));
        assert!(is_continue82_static_block_type("StaticBlock"));
        assert!(is_continue82_class_expression_type("ClassExpression"));
        assert!(is_continue82_class_body_type("ClassBody"));
        assert!(is_continue82_private_identifier_type("PrivateIdentifier"));
        assert!(is_continue82_accessor_property_type("AccessorProperty"));
        assert!(!is_continue82_property_definition_type("StaticBlock"));
        assert!(!is_continue82_class_expression_type("ClassDeclaration"));
    }

    #[test]
    fn continue82_property_static_block_dual_oracle() {
        assert_eq!(property_definition_pretty("count", "0"), "count = 0;");
        assert_eq!(property_definition_minify("count", "0"), "count=0;");
        assert_eq!(
            property_definition_static_pretty("n", "1"),
            "static n = 1;"
        );
        assert_eq!(property_definition_private_semi("id"), "#id;");
        assert_eq!(
            continue82_property_definition_skeleton("x", Some("1"), true, false, false, true),
            property_definition_skeleton("x", Some("1"), true, false, false, true)
        );
        assert_ne!(
            property_definition_pretty("a", "1"),
            property_definition_minify("a", "1")
        );

        assert_eq!(static_block_empty_pretty(), "static {}");
        assert_eq!(static_block_empty_minify(), "static{}");
        assert_eq!(
            static_block_pretty(&["this.x=1;"]),
            "static { this.x=1; }"
        );
        assert_eq!(
            static_block_minify(&["this.x=1;"]),
            "static{this.x=1;}"
        );
        assert_eq!(
            continue82_static_block_skeleton(&["a;", "b;"], true),
            static_block_skeleton(&["a;", "b;"], true)
        );
        assert_ne!(static_block_empty_pretty(), static_block_empty_minify());

        assert_eq!(continue82_static_prefix(true), "static ");
        assert_eq!(continue82_static_prefix(false), "");
        assert_eq!(continue82_property_key_token("x", false), "x");
        assert_eq!(continue82_property_key_token("x", true), "#x");
        assert_eq!(continue82_property_key_token("#x", true), "#x");
    }

    #[test]
    fn continue82_class_accessor_method_dual_oracle() {
        let anon = class_expression_anon_pretty(&[]);
        assert!(anon.starts_with("class"));
        assert!(anon.contains('{'));
        let named = class_expression_named_pretty("Foo", &["x = 1;"]);
        assert!(named.contains("Foo"));
        assert!(named.contains("x = 1;"));
        assert_eq!(
            continue82_class_expression_skeleton(Some("Bar"), &[], true),
            class_expression_skeleton(Some("Bar"), &[], true)
        );
        let mini = class_expression_named_minify("Z", &["a=1;"]);
        assert!(mini.contains("Z"));
        assert!(mini.contains("a=1;"));

        assert_eq!(accessor_property_pretty("value", "1"), "accessor value = 1;");
        assert_eq!(accessor_property_minify("value", "1"), "accessor value=1;");
        assert_eq!(
            accessor_property_static_pretty("flag"),
            "accessor static flag;"
        );
        assert_eq!(
            continue82_accessor_property_skeleton("v", Some("2"), false, true, true),
            accessor_property_skeleton("v", Some("2"), false, true, true)
        );
        assert_ne!(
            accessor_property_pretty("a", "1"),
            accessor_property_minify("a", "1")
        );

        assert_eq!(continue82_computed_property_key("k"), "[k]");
        assert_eq!(continue82_computed_property_key("a+b"), "[a+b]");

        let m = class_method_like_static_async_pretty("run", "", "{}");
        assert!(m.starts_with("static async "));
        assert!(m.contains("run()"));
        assert!(m.contains("{}"));

        let composed = continue82_class_with_field("C", "n", "0", true);
        assert!(composed.contains("class"));
        assert!(composed.contains("C"));
        assert!(composed.contains("n = 0;"));
        let composed_mini = continue82_class_with_field("C", "n", "0", false);
        assert!(composed_mini.contains("n=0;"));
        assert_ne!(composed, composed_mini);
    }
}

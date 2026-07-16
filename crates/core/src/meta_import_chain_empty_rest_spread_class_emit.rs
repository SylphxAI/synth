//! Pure MetaProperty + ImportExpression + ChainExpression + EmptyStatement +
//! RestElement + SpreadElement + ClassProperty dual-oracle emission — residual
//! pure **continue100** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–99:
//! - MetaProperty dual-oracle composing real `continue33_meta_property_skeleton`
//! - ImportExpression dual-oracle composing real
//!   `continue33_import_expression_skeleton`
//! - ChainExpression dual-oracle composing real
//!   `continue33_chain_expression_skeleton`
//! - EmptyStatement dual-oracle composing real
//!   `continue33_empty_statement_skeleton`
//! - RestElement dual-oracle composing real `continue33_rest_element_skeleton`
//! - SpreadElement dual-oracle composing real
//!   `continue33_spread_element_skeleton`
//! - ClassProperty dual-oracle composing real
//!   `continue33_class_property_skeleton`
//! - Composed meta/import/chain/empty/rest/spread/class residual shells
//!
//! Intentionally does **not** re-wrap continue20/83 yield/meta/import poles,
//! continue73 optional-chain poles, continue84 pattern rest/spread poles,
//! continue91 primitive/class-property poles, or continue99
//! switch/try/throw/debugger poles. Composes real shipped pure helpers from
//! continue33 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue33_chain_expression_skeleton, continue33_class_property_skeleton,
    continue33_empty_statement_skeleton, continue33_import_expression_skeleton,
    continue33_meta_property_skeleton, continue33_rest_element_skeleton,
    continue33_spread_element_skeleton,
};

/// Dual-oracle residual: continue100 related AST type catalog.
pub const CONTINUE100_RELATED_TYPES: &[&str] = &[
    "MetaProperty",
    "ImportExpression",
    "ChainExpression",
    "EmptyStatement",
    "RestElement",
    "SpreadElement",
    "ClassProperty",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_meta_import_chain_empty_rest_spread_class_related_type(t: &str) -> bool {
    CONTINUE100_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue100_meta_property_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue100_import_expression_type(t: &str) -> bool {
    t == "ImportExpression"
}

#[must_use]
pub fn is_continue100_chain_expression_type(t: &str) -> bool {
    t == "ChainExpression"
}

#[must_use]
pub fn is_continue100_empty_statement_type(t: &str) -> bool {
    t == "EmptyStatement"
}

#[must_use]
pub fn is_continue100_rest_element_type(t: &str) -> bool {
    t == "RestElement"
}

#[must_use]
pub fn is_continue100_spread_element_type(t: &str) -> bool {
    t == "SpreadElement"
}

#[must_use]
pub fn is_continue100_class_property_type(t: &str) -> bool {
    t == "ClassProperty"
}

#[must_use]
pub fn is_continue100_rest_or_spread_type(t: &str) -> bool {
    matches!(t, "RestElement" | "SpreadElement")
}

#[must_use]
pub fn is_continue100_meta_import_family_type(t: &str) -> bool {
    matches!(t, "MetaProperty" | "ImportExpression" | "ChainExpression")
}

// ── MetaProperty dual-oracle ────────────────────────────────────────────────

/// Dual-oracle MetaProperty skeleton composing real
/// [`continue33_meta_property_skeleton`].
#[must_use]
pub fn continue100_meta_property_skeleton(meta: &str, property: &str) -> String {
    continue33_meta_property_skeleton(meta, property)
}

/// Dual-oracle MetaProperty pretty alias.
#[must_use]
pub fn continue100_meta_property_pretty(meta: &str, property: &str) -> String {
    continue100_meta_property_skeleton(meta, property)
}

/// Dual-oracle MetaProperty minify alias.
#[must_use]
pub fn continue100_meta_property_minify(meta: &str, property: &str) -> String {
    continue100_meta_property_skeleton(meta, property)
}

/// Dual-oracle residual: `import.meta`.
#[must_use]
pub fn continue100_import_meta() -> String {
    continue100_meta_property_skeleton("import", "meta")
}

/// Dual-oracle residual: `new.target`.
#[must_use]
pub fn continue100_new_target() -> String {
    continue100_meta_property_skeleton("new", "target")
}

// ── ImportExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle ImportExpression skeleton composing real
/// [`continue33_import_expression_skeleton`].
#[must_use]
pub fn continue100_import_expression_skeleton(source: &str) -> String {
    continue33_import_expression_skeleton(source)
}

/// Dual-oracle ImportExpression pretty alias.
#[must_use]
pub fn continue100_import_expression_pretty(source: &str) -> String {
    continue100_import_expression_skeleton(source)
}

/// Dual-oracle ImportExpression minify alias.
#[must_use]
pub fn continue100_import_expression_minify(source: &str) -> String {
    continue100_import_expression_skeleton(source)
}

/// Dual-oracle residual: `import("mod")` string-source shell.
#[must_use]
pub fn continue100_import_string(source: &str) -> String {
    continue100_import_expression_skeleton(&format!("\"{source}\""))
}

// ── ChainExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ChainExpression skeleton composing real
/// [`continue33_chain_expression_skeleton`].
#[must_use]
pub fn continue100_chain_expression_skeleton(expr: &str) -> String {
    continue33_chain_expression_skeleton(expr)
}

/// Dual-oracle ChainExpression pretty alias.
#[must_use]
pub fn continue100_chain_expression_pretty(expr: &str) -> String {
    continue100_chain_expression_skeleton(expr)
}

/// Dual-oracle ChainExpression minify alias.
#[must_use]
pub fn continue100_chain_expression_minify(expr: &str) -> String {
    continue100_chain_expression_skeleton(expr)
}

// ── EmptyStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle EmptyStatement skeleton composing real
/// [`continue33_empty_statement_skeleton`].
#[must_use]
pub fn continue100_empty_statement_skeleton() -> &'static str {
    continue33_empty_statement_skeleton()
}

/// Dual-oracle EmptyStatement pretty alias.
#[must_use]
pub fn continue100_empty_statement_pretty() -> &'static str {
    continue100_empty_statement_skeleton()
}

/// Dual-oracle EmptyStatement minify alias.
#[must_use]
pub fn continue100_empty_statement_minify() -> &'static str {
    continue100_empty_statement_skeleton()
}

// ── RestElement dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle RestElement skeleton composing real
/// [`continue33_rest_element_skeleton`].
#[must_use]
pub fn continue100_rest_element_skeleton(arg: &str) -> String {
    continue33_rest_element_skeleton(arg)
}

/// Dual-oracle RestElement pretty alias.
#[must_use]
pub fn continue100_rest_element_pretty(arg: &str) -> String {
    continue100_rest_element_skeleton(arg)
}

/// Dual-oracle RestElement minify alias.
#[must_use]
pub fn continue100_rest_element_minify(arg: &str) -> String {
    continue100_rest_element_skeleton(arg)
}

// ── SpreadElement dual-oracle ───────────────────────────────────────────────

/// Dual-oracle SpreadElement skeleton composing real
/// [`continue33_spread_element_skeleton`].
#[must_use]
pub fn continue100_spread_element_skeleton(arg: &str) -> String {
    continue33_spread_element_skeleton(arg)
}

/// Dual-oracle SpreadElement pretty alias.
#[must_use]
pub fn continue100_spread_element_pretty(arg: &str) -> String {
    continue100_spread_element_skeleton(arg)
}

/// Dual-oracle SpreadElement minify alias.
#[must_use]
pub fn continue100_spread_element_minify(arg: &str) -> String {
    continue100_spread_element_skeleton(arg)
}

// ── ClassProperty dual-oracle ───────────────────────────────────────────────

/// Dual-oracle ClassProperty skeleton composing real
/// [`continue33_class_property_skeleton`].
#[must_use]
pub fn continue100_class_property_skeleton(name: &str, value: Option<&str>) -> String {
    continue33_class_property_skeleton(name, value)
}

/// Dual-oracle ClassProperty with initializer.
#[must_use]
pub fn continue100_class_property(name: &str, value: &str) -> String {
    continue100_class_property_skeleton(name, Some(value))
}

/// Dual-oracle bare ClassProperty (no initializer).
#[must_use]
pub fn continue100_class_property_bare(name: &str) -> String {
    continue100_class_property_skeleton(name, None)
}

/// Dual-oracle ClassProperty pretty alias.
#[must_use]
pub fn continue100_class_property_pretty(name: &str, value: Option<&str>) -> String {
    continue100_class_property_skeleton(name, value)
}

/// Dual-oracle ClassProperty minify alias.
#[must_use]
pub fn continue100_class_property_minify(name: &str, value: Option<&str>) -> String {
    continue100_class_property_skeleton(name, value)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: import.meta.url shell.
#[must_use]
pub fn continue100_import_meta_url() -> String {
    format!("{}.url", continue100_import_meta())
}

/// Dual-oracle residual: dynamic import of import.meta.url-like source.
#[must_use]
pub fn continue100_import_meta_then_import() -> String {
    let meta = continue100_import_meta();
    continue100_import_expression_skeleton(&meta)
}

/// Dual-oracle residual: chain-wrapped optional member expression.
#[must_use]
pub fn continue100_chain_optional_member(object: &str, prop: &str) -> String {
    continue100_chain_expression_skeleton(&format!("{object}?.{prop}"))
}

/// Dual-oracle residual: rest then empty statement shell.
#[must_use]
pub fn continue100_rest_then_empty(arg: &str) -> String {
    format!(
        "{}{}",
        continue100_rest_element_skeleton(arg),
        continue100_empty_statement_skeleton()
    )
}

/// Dual-oracle residual: array-like spread pair shell.
#[must_use]
pub fn continue100_spread_pair(a: &str, b: &str) -> String {
    format!(
        "[{}, {}]",
        continue100_spread_element_skeleton(a),
        continue100_spread_element_skeleton(b)
    )
}

/// Dual-oracle residual: class field with import.meta initializer.
#[must_use]
pub fn continue100_class_field_import_meta(name: &str) -> String {
    let meta = continue100_import_meta();
    continue100_class_property(name, &meta)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue100_stmt_sep(pretty: bool) -> &'static str {
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
        continue33_chain_expression_skeleton, continue33_class_property_skeleton,
        continue33_empty_statement_skeleton, continue33_import_expression_skeleton,
        continue33_meta_property_skeleton, continue33_rest_element_skeleton,
        continue33_spread_element_skeleton,
    };

    #[test]
    fn continue100_type_catalog() {
        assert_eq!(CONTINUE100_RELATED_TYPES.len(), 7);
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "MetaProperty"
        ));
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "ImportExpression"
        ));
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "ChainExpression"
        ));
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "EmptyStatement"
        ));
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "RestElement"
        ));
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "SpreadElement"
        ));
        assert!(is_meta_import_chain_empty_rest_spread_class_related_type(
            "ClassProperty"
        ));
        assert!(!is_meta_import_chain_empty_rest_spread_class_related_type(
            "SwitchStatement"
        ));
        assert!(!is_meta_import_chain_empty_rest_spread_class_related_type(
            "UnaryExpression"
        ));

        assert!(is_continue100_meta_property_type("MetaProperty"));
        assert!(!is_continue100_meta_property_type("ImportExpression"));
        assert!(is_continue100_import_expression_type("ImportExpression"));
        assert!(is_continue100_chain_expression_type("ChainExpression"));
        assert!(is_continue100_empty_statement_type("EmptyStatement"));
        assert!(is_continue100_rest_element_type("RestElement"));
        assert!(is_continue100_spread_element_type("SpreadElement"));
        assert!(is_continue100_class_property_type("ClassProperty"));
        assert!(is_continue100_rest_or_spread_type("RestElement"));
        assert!(is_continue100_rest_or_spread_type("SpreadElement"));
        assert!(!is_continue100_rest_or_spread_type("ClassProperty"));
        assert!(is_continue100_meta_import_family_type("MetaProperty"));
        assert!(is_continue100_meta_import_family_type("ImportExpression"));
        assert!(is_continue100_meta_import_family_type("ChainExpression"));
        assert!(!is_continue100_meta_import_family_type("EmptyStatement"));
    }

    #[test]
    fn continue100_meta_import_chain_emit() {
        assert_eq!(
            continue100_meta_property_skeleton("import", "meta"),
            "import.meta"
        );
        assert_eq!(
            continue100_meta_property_skeleton("import", "meta"),
            continue33_meta_property_skeleton("import", "meta")
        );
        assert_eq!(
            continue100_meta_property_pretty("new", "target"),
            continue100_meta_property_minify("new", "target")
        );
        assert_eq!(continue100_import_meta(), "import.meta");
        assert_eq!(continue100_new_target(), "new.target");

        assert_eq!(
            continue100_import_expression_skeleton("\"x\""),
            "import(\"x\")"
        );
        assert_eq!(
            continue100_import_expression_skeleton("\"x\""),
            continue33_import_expression_skeleton("\"x\"")
        );
        assert_eq!(
            continue100_import_expression_pretty("src"),
            continue100_import_expression_minify("src")
        );
        assert_eq!(continue100_import_string("mod"), "import(\"mod\")");

        assert_eq!(
            continue100_chain_expression_skeleton("a?.b"),
            "a?.b"
        );
        assert_eq!(
            continue100_chain_expression_skeleton("a?.b"),
            continue33_chain_expression_skeleton("a?.b")
        );
        assert_eq!(
            continue100_chain_expression_pretty("x"),
            continue100_chain_expression_minify("x")
        );
    }

    #[test]
    fn continue100_empty_rest_spread_class_emit() {
        assert_eq!(continue100_empty_statement_skeleton(), ";");
        assert_eq!(
            continue100_empty_statement_skeleton(),
            continue33_empty_statement_skeleton()
        );
        assert_eq!(
            continue100_empty_statement_pretty(),
            continue100_empty_statement_minify()
        );

        assert_eq!(continue100_rest_element_skeleton("args"), "...args");
        assert_eq!(
            continue100_rest_element_skeleton("args"),
            continue33_rest_element_skeleton("args")
        );
        assert_eq!(
            continue100_rest_element_pretty("r"),
            continue100_rest_element_minify("r")
        );

        assert_eq!(continue100_spread_element_skeleton("xs"), "...xs");
        assert_eq!(
            continue100_spread_element_skeleton("xs"),
            continue33_spread_element_skeleton("xs")
        );
        assert_eq!(
            continue100_spread_element_pretty("s"),
            continue100_spread_element_minify("s")
        );

        assert_eq!(
            continue100_class_property_skeleton("x", Some("1")),
            "x = 1;"
        );
        assert_eq!(
            continue100_class_property_skeleton("x", Some("1")),
            continue33_class_property_skeleton("x", Some("1"))
        );
        assert_eq!(continue100_class_property("y", "2"), "y = 2;");
        assert_eq!(continue100_class_property_bare("z"), "z;");
        assert_eq!(
            continue100_class_property_skeleton("z", None),
            continue33_class_property_skeleton("z", None)
        );
        assert_eq!(
            continue100_class_property_pretty("a", Some("b")),
            continue100_class_property_minify("a", Some("b"))
        );
    }

    #[test]
    fn continue100_composed_residual_shells() {
        assert_eq!(continue100_import_meta_url(), "import.meta.url");
        assert_eq!(
            continue100_import_meta_then_import(),
            "import(import.meta)"
        );
        assert_eq!(
            continue100_chain_optional_member("obj", "prop"),
            "obj?.prop"
        );
        assert_eq!(continue100_rest_then_empty("rest"), "...rest;");
        assert_eq!(
            continue100_spread_pair("a", "b"),
            "[...a, ...b]"
        );
        assert_eq!(
            continue100_class_field_import_meta("url"),
            "url = import.meta;"
        );
        assert_eq!(continue100_stmt_sep(true), " ");
        assert_eq!(continue100_stmt_sep(false), "");
        assert_ne!(continue100_stmt_sep(true), continue100_stmt_sep(false));
    }
}

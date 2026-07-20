//! Pure ClassDeclaration + ClassExpression + ReturnStatement +
//! ThisExpression + Super + MetaProperty dual-oracle emission — residual pure
//! **continue103** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–102:
//! - ClassDeclaration dual-oracle composing real
//!   `continue36_class_declaration_skeleton`
//! - ClassExpression dual-oracle composing real
//!   `continue36_class_expression_skeleton`
//! - ReturnStatement dual-oracle composing real `continue36_return_skeleton`
//! - ThisExpression dual-oracle composing real `continue36_this_skeleton`
//! - Super dual-oracle composing real `continue36_super_skeleton`
//! - MetaProperty dual-oracle composing real
//!   `continue36_meta_property_skeleton`
//! - Composed class/return/this/super/meta residual shells
//!
//! Intentionally does **not** re-wrap continue96 class/function/import/export
//! poles, continue100 meta/import/chain poles, continue29/36 base surfaces via
//! continue71–95, or continue102 if/while/do/conditional/labeled/with poles.
//! Composes real shipped pure helpers from continue36 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue36_class_declaration_skeleton, continue36_class_expression_skeleton,
    continue36_meta_property_skeleton, continue36_return_skeleton, continue36_super_skeleton,
    continue36_this_skeleton,
};

/// Dual-oracle residual: continue103 related AST type catalog.
pub const CONTINUE103_RELATED_TYPES: &[&str] = &[
    "ClassDeclaration",
    "ClassExpression",
    "ReturnStatement",
    "ThisExpression",
    "Super",
    "MetaProperty",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_class_return_this_super_meta_related_type(t: &str) -> bool {
    CONTINUE103_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue103_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue103_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue103_return_type(t: &str) -> bool {
    t == "ReturnStatement"
}

#[must_use]
pub fn is_continue103_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue103_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue103_meta_property_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue103_class_family_type(t: &str) -> bool {
    matches!(t, "ClassDeclaration" | "ClassExpression")
}

#[must_use]
pub fn is_continue103_receiver_type(t: &str) -> bool {
    matches!(t, "ThisExpression" | "Super" | "MetaProperty")
}

// ── ClassDeclaration dual-oracle ────────────────────────────────────────────

/// Dual-oracle ClassDeclaration skeleton composing real
/// [`continue36_class_declaration_skeleton`].
#[must_use]
pub fn continue103_class_declaration_skeleton(name: &str, body: &str) -> String {
    continue36_class_declaration_skeleton(name, body)
}

/// Dual-oracle ClassDeclaration pretty alias.
#[must_use]
pub fn continue103_class_declaration_pretty(name: &str, body: &str) -> String {
    continue103_class_declaration_skeleton(name, body)
}

/// Dual-oracle ClassDeclaration minify alias.
#[must_use]
pub fn continue103_class_declaration_minify(name: &str, body: &str) -> String {
    continue103_class_declaration_skeleton(name, body)
}

// ── ClassExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ClassExpression skeleton composing real
/// [`continue36_class_expression_skeleton`].
#[must_use]
pub fn continue103_class_expression_skeleton(name: Option<&str>, body: &str) -> String {
    continue36_class_expression_skeleton(name, body)
}

/// Dual-oracle named ClassExpression.
#[must_use]
pub fn continue103_class_expression_named(name: &str, body: &str) -> String {
    continue103_class_expression_skeleton(Some(name), body)
}

/// Dual-oracle anonymous ClassExpression.
#[must_use]
pub fn continue103_class_expression_anon(body: &str) -> String {
    continue103_class_expression_skeleton(None, body)
}

/// Dual-oracle ClassExpression pretty alias.
#[must_use]
pub fn continue103_class_expression_pretty(name: Option<&str>, body: &str) -> String {
    continue103_class_expression_skeleton(name, body)
}

/// Dual-oracle ClassExpression minify alias.
#[must_use]
pub fn continue103_class_expression_minify(name: Option<&str>, body: &str) -> String {
    continue103_class_expression_skeleton(name, body)
}

// ── ReturnStatement dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ReturnStatement skeleton composing real
/// [`continue36_return_skeleton`].
#[must_use]
pub fn continue103_return_skeleton(arg: Option<&str>) -> String {
    continue36_return_skeleton(arg)
}

/// Dual-oracle ReturnStatement with argument.
#[must_use]
pub fn continue103_return_value(arg: &str) -> String {
    continue103_return_skeleton(Some(arg))
}

/// Dual-oracle bare ReturnStatement.
#[must_use]
pub fn continue103_return_bare() -> String {
    continue103_return_skeleton(None)
}

/// Dual-oracle ReturnStatement pretty alias.
#[must_use]
pub fn continue103_return_pretty(arg: Option<&str>) -> String {
    continue103_return_skeleton(arg)
}

/// Dual-oracle ReturnStatement minify alias.
#[must_use]
pub fn continue103_return_minify(arg: Option<&str>) -> String {
    continue103_return_skeleton(arg)
}

// ── ThisExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThisExpression skeleton composing real
/// [`continue36_this_skeleton`].
#[must_use]
pub fn continue103_this_skeleton() -> String {
    continue36_this_skeleton()
}

/// Dual-oracle ThisExpression pretty alias.
#[must_use]
pub fn continue103_this_pretty() -> String {
    continue103_this_skeleton()
}

/// Dual-oracle ThisExpression minify alias.
#[must_use]
pub fn continue103_this_minify() -> String {
    continue103_this_skeleton()
}

// ── Super dual-oracle ───────────────────────────────────────────────────────

/// Dual-oracle Super skeleton composing real [`continue36_super_skeleton`].
#[must_use]
pub fn continue103_super_skeleton() -> String {
    continue36_super_skeleton()
}

/// Dual-oracle Super pretty alias.
#[must_use]
pub fn continue103_super_pretty() -> String {
    continue103_super_skeleton()
}

/// Dual-oracle Super minify alias.
#[must_use]
pub fn continue103_super_minify() -> String {
    continue103_super_skeleton()
}

// ── MetaProperty dual-oracle ────────────────────────────────────────────────

/// Dual-oracle MetaProperty skeleton composing real
/// [`continue36_meta_property_skeleton`].
#[must_use]
pub fn continue103_meta_property_skeleton(meta: &str, property: &str) -> String {
    continue36_meta_property_skeleton(meta, property)
}

/// Dual-oracle MetaProperty pretty alias.
#[must_use]
pub fn continue103_meta_property_pretty(meta: &str, property: &str) -> String {
    continue103_meta_property_skeleton(meta, property)
}

/// Dual-oracle MetaProperty minify alias.
#[must_use]
pub fn continue103_meta_property_minify(meta: &str, property: &str) -> String {
    continue103_meta_property_skeleton(meta, property)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: class declaration whose body is a bare return.
#[must_use]
pub fn continue103_class_with_return(name: &str, ret: Option<&str>) -> String {
    let body = format!("{{ {} }}", continue103_return_skeleton(ret));
    continue103_class_declaration_skeleton(name, &body)
}

/// Dual-oracle residual: return this.
#[must_use]
pub fn continue103_return_this() -> String {
    continue103_return_value(&continue103_this_skeleton())
}

/// Dual-oracle residual: return super.
#[must_use]
pub fn continue103_return_super() -> String {
    continue103_return_value(&continue103_super_skeleton())
}

/// Dual-oracle residual: return import.meta / new.target style meta property.
#[must_use]
pub fn continue103_return_meta(meta: &str, property: &str) -> String {
    let m = continue103_meta_property_skeleton(meta, property);
    continue103_return_value(&m)
}

/// Dual-oracle residual: named class expression with empty body braces.
#[must_use]
pub fn continue103_class_expression_empty(name: &str) -> String {
    continue103_class_expression_named(name, "{}")
}

/// Dual-oracle residual: anonymous class expression returning this.
#[must_use]
pub fn continue103_class_expression_return_this() -> String {
    let body = format!("{{ {} }}", continue103_return_this());
    continue103_class_expression_anon(&body)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue103_stmt_sep(pretty: bool) -> &'static str {
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
        continue36_class_declaration_skeleton, continue36_class_expression_skeleton,
        continue36_meta_property_skeleton, continue36_return_skeleton, continue36_super_skeleton,
        continue36_this_skeleton,
    };

    #[test]
    fn continue103_type_catalog() {
        assert_eq!(CONTINUE103_RELATED_TYPES.len(), 6);
        assert!(is_class_return_this_super_meta_related_type(
            "ClassDeclaration"
        ));
        assert!(is_class_return_this_super_meta_related_type(
            "ClassExpression"
        ));
        assert!(is_class_return_this_super_meta_related_type(
            "ReturnStatement"
        ));
        assert!(is_class_return_this_super_meta_related_type(
            "ThisExpression"
        ));
        assert!(is_class_return_this_super_meta_related_type("Super"));
        assert!(is_class_return_this_super_meta_related_type("MetaProperty"));
        assert!(!is_class_return_this_super_meta_related_type(
            "FunctionDeclaration"
        ));
        assert!(!is_class_return_this_super_meta_related_type(
            "ImportExpression"
        ));

        assert!(is_continue103_class_declaration_type("ClassDeclaration"));
        assert!(!is_continue103_class_declaration_type("ClassExpression"));
        assert!(is_continue103_class_expression_type("ClassExpression"));
        assert!(is_continue103_return_type("ReturnStatement"));
        assert!(is_continue103_this_type("ThisExpression"));
        assert!(is_continue103_super_type("Super"));
        assert!(is_continue103_meta_property_type("MetaProperty"));
        assert!(is_continue103_class_family_type("ClassDeclaration"));
        assert!(is_continue103_class_family_type("ClassExpression"));
        assert!(!is_continue103_class_family_type("ReturnStatement"));
        assert!(is_continue103_receiver_type("ThisExpression"));
        assert!(is_continue103_receiver_type("Super"));
        assert!(is_continue103_receiver_type("MetaProperty"));
        assert!(!is_continue103_receiver_type("ClassDeclaration"));
    }

    #[test]
    fn continue103_class_return_emit() {
        assert_eq!(
            continue103_class_declaration_skeleton("Foo", "{ }"),
            "class Foo { }"
        );
        assert_eq!(
            continue103_class_declaration_skeleton("Foo", "{ }"),
            continue36_class_declaration_skeleton("Foo", "{ }")
        );
        assert_eq!(
            continue103_class_declaration_pretty("A", "{}"),
            continue103_class_declaration_minify("A", "{}")
        );

        assert_eq!(
            continue103_class_expression_named("Bar", "{}"),
            "class Bar {}"
        );
        assert_eq!(
            continue103_class_expression_named("Bar", "{}"),
            continue36_class_expression_skeleton(Some("Bar"), "{}")
        );
        assert_eq!(continue103_class_expression_anon("{}"), "class {}");
        assert_eq!(
            continue103_class_expression_anon("{}"),
            continue36_class_expression_skeleton(None, "{}")
        );
        assert_eq!(
            continue103_class_expression_pretty(Some("X"), "{}"),
            continue103_class_expression_minify(Some("X"), "{}")
        );

        assert_eq!(continue103_return_value("x"), "return x;");
        assert_eq!(
            continue103_return_value("x"),
            continue36_return_skeleton(Some("x"))
        );
        assert_eq!(continue103_return_bare(), "return;");
        assert_eq!(
            continue103_return_bare(),
            continue36_return_skeleton(None)
        );
        assert_eq!(
            continue103_return_pretty(Some("y")),
            continue103_return_minify(Some("y"))
        );
        assert_eq!(
            continue103_return_skeleton(Some("")),
            "return;"
        );
    }

    #[test]
    fn continue103_this_super_meta_emit() {
        assert_eq!(continue103_this_skeleton(), "this");
        assert_eq!(continue103_this_skeleton(), continue36_this_skeleton());
        assert_eq!(continue103_this_pretty(), continue103_this_minify());

        assert_eq!(continue103_super_skeleton(), "super");
        assert_eq!(continue103_super_skeleton(), continue36_super_skeleton());
        assert_eq!(continue103_super_pretty(), continue103_super_minify());

        assert_eq!(
            continue103_meta_property_skeleton("import", "meta"),
            "import.meta"
        );
        assert_eq!(
            continue103_meta_property_skeleton("import", "meta"),
            continue36_meta_property_skeleton("import", "meta")
        );
        assert_eq!(
            continue103_meta_property_skeleton("new", "target"),
            "new.target"
        );
        assert_eq!(
            continue103_meta_property_pretty("a", "b"),
            continue103_meta_property_minify("a", "b")
        );
    }

    #[test]
    fn continue103_composed_residual_shells() {
        assert_eq!(
            continue103_class_with_return("C", Some("1")),
            "class C { return 1; }"
        );
        assert_eq!(
            continue103_class_with_return("C", None),
            "class C { return; }"
        );
        assert_eq!(continue103_return_this(), "return this;");
        assert_eq!(continue103_return_super(), "return super;");
        assert_eq!(
            continue103_return_meta("import", "meta"),
            "return import.meta;"
        );
        assert_eq!(
            continue103_class_expression_empty("X"),
            "class X {}"
        );
        assert_eq!(
            continue103_class_expression_return_this(),
            "class { return this; }"
        );
        assert_eq!(continue103_stmt_sep(true), " ");
        assert_eq!(continue103_stmt_sep(false), "");
        assert_ne!(continue103_stmt_sep(true), continue103_stmt_sep(false));
    }
}

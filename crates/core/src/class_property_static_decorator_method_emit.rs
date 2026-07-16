//! Pure PropertyDefinition + StaticBlock + Decorator + ClassBody +
//! MethodDefinition + ExportDefaultSpecifier dual-oracle emission — residual
//! pure **continue116** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–115:
//! - PropertyDefinition dual-oracle composing real
//!   `continue49_property_definition_skeleton`
//! - Static property dual-oracle composing real
//!   `continue49_static_property_skeleton`
//! - StaticBlock dual-oracle composing real
//!   `continue49_static_block_skeleton`
//! - Decorator dual-oracle composing real `continue49_decorator_skeleton`
//! - ClassBody dual-oracle composing real `continue49_class_body_skeleton`
//! - MethodDefinition dual-oracle composing real
//!   `continue49_method_definition_skeleton`
//! - ExportDefaultSpecifier dual-oracle composing real
//!   `continue49_export_default_specifier_skeleton`
//! - Composed class-property/static/decorator/method residual shells
//!
//! Intentionally does **not** re-wrap continue115 optional/import/await/yield/
//! private poles (continue48 bases), continue114 class/import/export/new/this/
//! super/meta poles (continue47 bases), or continue113 for/throw/label/empty
//! poles (continue46 bases). Composes real shipped pure helpers from continue49
//! class property/static/decorator/method bases. Full engines remain product dens.
//! NO authority_rust / ts_deleted. dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue49_class_body_skeleton, continue49_decorator_skeleton,
    continue49_export_default_specifier_skeleton, continue49_method_definition_skeleton,
    continue49_property_definition_skeleton, continue49_static_block_skeleton,
    continue49_static_property_skeleton,
};

/// Dual-oracle residual: continue116 related AST type catalog.
pub const CONTINUE116_RELATED_TYPES: &[&str] = &[
    "PropertyDefinition",
    "StaticBlock",
    "Decorator",
    "ClassBody",
    "MethodDefinition",
    "ExportDefaultSpecifier",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_class_property_static_decorator_method_related_type(t: &str) -> bool {
    CONTINUE116_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue116_property_definition_type(t: &str) -> bool {
    t == "PropertyDefinition"
}

#[must_use]
pub fn is_continue116_static_block_type(t: &str) -> bool {
    t == "StaticBlock"
}

#[must_use]
pub fn is_continue116_decorator_type(t: &str) -> bool {
    t == "Decorator"
}

#[must_use]
pub fn is_continue116_class_body_type(t: &str) -> bool {
    t == "ClassBody"
}

#[must_use]
pub fn is_continue116_method_definition_type(t: &str) -> bool {
    t == "MethodDefinition"
}

#[must_use]
pub fn is_continue116_export_default_specifier_type(t: &str) -> bool {
    t == "ExportDefaultSpecifier"
}

#[must_use]
pub fn is_continue116_class_member_type(t: &str) -> bool {
    matches!(
        t,
        "PropertyDefinition" | "StaticBlock" | "MethodDefinition"
    )
}

#[must_use]
pub fn is_continue116_type(t: &str) -> bool {
    matches!(
        t,
        "PropertyDefinition"
            | "StaticBlock"
            | "Decorator"
            | "ClassBody"
            | "MethodDefinition"
            | "ExportDefaultSpecifier"
    )
}

// ── PropertyDefinition dual-oracle ──────────────────────────────────────────

/// Dual-oracle PropertyDefinition skeleton composing real
/// [`continue49_property_definition_skeleton`].
#[must_use]
pub fn continue116_property_definition_skeleton(key: &str, value: &str) -> String {
    continue49_property_definition_skeleton(key, value)
}

/// Dual-oracle PropertyDefinition pretty alias.
#[must_use]
pub fn continue116_property_definition_pretty(key: &str, value: &str) -> String {
    continue116_property_definition_skeleton(key, value)
}

/// Dual-oracle PropertyDefinition minify alias.
#[must_use]
pub fn continue116_property_definition_minify(key: &str, value: &str) -> String {
    continue116_property_definition_skeleton(key, value)
}

// ── Static property dual-oracle ─────────────────────────────────────────────

/// Dual-oracle static PropertyDefinition skeleton composing real
/// [`continue49_static_property_skeleton`].
#[must_use]
pub fn continue116_static_property_skeleton(key: &str, value: &str) -> String {
    continue49_static_property_skeleton(key, value)
}

/// Dual-oracle static property pretty alias.
#[must_use]
pub fn continue116_static_property_pretty(key: &str, value: &str) -> String {
    continue116_static_property_skeleton(key, value)
}

/// Dual-oracle static property minify alias.
#[must_use]
pub fn continue116_static_property_minify(key: &str, value: &str) -> String {
    continue116_static_property_skeleton(key, value)
}

// ── StaticBlock dual-oracle ─────────────────────────────────────────────────

/// Dual-oracle StaticBlock skeleton composing real
/// [`continue49_static_block_skeleton`].
#[must_use]
pub fn continue116_static_block_skeleton(body: &str) -> String {
    continue49_static_block_skeleton(body)
}

/// Dual-oracle StaticBlock pretty alias.
#[must_use]
pub fn continue116_static_block_pretty(body: &str) -> String {
    continue116_static_block_skeleton(body)
}

/// Dual-oracle StaticBlock minify alias.
#[must_use]
pub fn continue116_static_block_minify(body: &str) -> String {
    continue116_static_block_skeleton(body)
}

// ── Decorator dual-oracle ───────────────────────────────────────────────────

/// Dual-oracle Decorator skeleton composing real
/// [`continue49_decorator_skeleton`].
#[must_use]
pub fn continue116_decorator_skeleton(expr: &str) -> String {
    continue49_decorator_skeleton(expr)
}

/// Dual-oracle Decorator pretty alias.
#[must_use]
pub fn continue116_decorator_pretty(expr: &str) -> String {
    continue116_decorator_skeleton(expr)
}

/// Dual-oracle Decorator minify alias.
#[must_use]
pub fn continue116_decorator_minify(expr: &str) -> String {
    continue116_decorator_skeleton(expr)
}

// ── ClassBody dual-oracle ───────────────────────────────────────────────────

/// Dual-oracle ClassBody skeleton composing real
/// [`continue49_class_body_skeleton`].
#[must_use]
pub fn continue116_class_body_skeleton(members: &str) -> String {
    continue49_class_body_skeleton(members)
}

/// Dual-oracle ClassBody pretty alias.
#[must_use]
pub fn continue116_class_body_pretty(members: &str) -> String {
    continue116_class_body_skeleton(members)
}

/// Dual-oracle ClassBody minify alias.
#[must_use]
pub fn continue116_class_body_minify(members: &str) -> String {
    continue116_class_body_skeleton(members)
}

// ── MethodDefinition dual-oracle ────────────────────────────────────────────

/// Dual-oracle MethodDefinition skeleton composing real
/// [`continue49_method_definition_skeleton`].
#[must_use]
pub fn continue116_method_definition_skeleton(name: &str, params: &str, body: &str) -> String {
    continue49_method_definition_skeleton(name, params, body)
}

/// Dual-oracle MethodDefinition pretty alias.
#[must_use]
pub fn continue116_method_definition_pretty(name: &str, params: &str, body: &str) -> String {
    continue116_method_definition_skeleton(name, params, body)
}

/// Dual-oracle MethodDefinition minify alias.
#[must_use]
pub fn continue116_method_definition_minify(name: &str, params: &str, body: &str) -> String {
    continue116_method_definition_skeleton(name, params, body)
}

// ── ExportDefaultSpecifier dual-oracle ──────────────────────────────────────

/// Dual-oracle ExportDefaultSpecifier skeleton composing real
/// [`continue49_export_default_specifier_skeleton`].
#[must_use]
pub fn continue116_export_default_specifier_skeleton(local: &str) -> String {
    continue49_export_default_specifier_skeleton(local)
}

/// Dual-oracle ExportDefaultSpecifier pretty alias.
#[must_use]
pub fn continue116_export_default_specifier_pretty(local: &str) -> String {
    continue116_export_default_specifier_skeleton(local)
}

/// Dual-oracle ExportDefaultSpecifier minify alias.
#[must_use]
pub fn continue116_export_default_specifier_minify(local: &str) -> String {
    continue116_export_default_specifier_skeleton(local)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: class body with a single property definition.
#[must_use]
pub fn continue116_class_body_with_property(key: &str, value: &str) -> String {
    let prop = continue116_property_definition_skeleton(key, value);
    continue116_class_body_skeleton(&prop)
}

/// Dual-oracle residual: class body with a static property.
#[must_use]
pub fn continue116_class_body_with_static_property(key: &str, value: &str) -> String {
    let prop = continue116_static_property_skeleton(key, value);
    continue116_class_body_skeleton(&prop)
}

/// Dual-oracle residual: class body with a method definition.
#[must_use]
pub fn continue116_class_body_with_method(name: &str, params: &str, body: &str) -> String {
    let method = continue116_method_definition_skeleton(name, params, body);
    continue116_class_body_skeleton(&method)
}

/// Dual-oracle residual: class body with a static block.
#[must_use]
pub fn continue116_class_body_with_static_block(body: &str) -> String {
    let block = continue116_static_block_skeleton(body);
    continue116_class_body_skeleton(&block)
}

/// Dual-oracle residual: decorator then method seed (whitespace-joined).
#[must_use]
pub fn continue116_decorated_method(expr: &str, name: &str, params: &str, body: &str) -> String {
    let dec = continue116_decorator_skeleton(expr);
    let method = continue116_method_definition_skeleton(name, params, body);
    format!("{dec} {method}")
}

/// Dual-oracle residual: decorator then property seed (whitespace-joined).
#[must_use]
pub fn continue116_decorated_property(expr: &str, key: &str, value: &str) -> String {
    let dec = continue116_decorator_skeleton(expr);
    let prop = continue116_property_definition_skeleton(key, value);
    format!("{dec} {prop}")
}

/// Dual-oracle residual: class body with property + method members.
#[must_use]
pub fn continue116_class_body_property_and_method(
    key: &str,
    value: &str,
    name: &str,
    params: &str,
    body: &str,
) -> String {
    let prop = continue116_property_definition_skeleton(key, value);
    let method = continue116_method_definition_skeleton(name, params, body);
    let members = format!("{prop} {method}");
    continue116_class_body_skeleton(&members)
}

/// Dual-oracle residual: class body with static property + static block.
#[must_use]
pub fn continue116_class_body_static_property_and_block(
    key: &str,
    value: &str,
    block_body: &str,
) -> String {
    let prop = continue116_static_property_skeleton(key, value);
    let block = continue116_static_block_skeleton(block_body);
    let members = format!("{prop} {block}");
    continue116_class_body_skeleton(&members)
}

/// Dual-oracle residual: export default of a class-local name seed.
#[must_use]
pub fn continue116_export_default_class_local(local: &str) -> String {
    continue116_export_default_specifier_skeleton(local)
}

/// Dual-oracle residual: empty static block seed.
#[must_use]
pub fn continue116_static_block_empty() -> String {
    continue116_static_block_skeleton("")
}

/// Dual-oracle residual: empty class body seed.
#[must_use]
pub fn continue116_class_body_empty() -> String {
    continue116_class_body_skeleton("")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue116_sep(pretty: bool) -> &'static str {
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
        continue49_class_body_skeleton, continue49_decorator_skeleton,
        continue49_export_default_specifier_skeleton, continue49_method_definition_skeleton,
        continue49_property_definition_skeleton, continue49_static_block_skeleton,
        continue49_static_property_skeleton,
    };

    #[test]
    fn continue116_type_catalog() {
        assert_eq!(CONTINUE116_RELATED_TYPES.len(), 6);
        assert!(is_class_property_static_decorator_method_related_type(
            "PropertyDefinition"
        ));
        assert!(is_class_property_static_decorator_method_related_type(
            "StaticBlock"
        ));
        assert!(is_class_property_static_decorator_method_related_type(
            "Decorator"
        ));
        assert!(is_class_property_static_decorator_method_related_type(
            "ClassBody"
        ));
        assert!(is_class_property_static_decorator_method_related_type(
            "MethodDefinition"
        ));
        assert!(is_class_property_static_decorator_method_related_type(
            "ExportDefaultSpecifier"
        ));
        assert!(!is_class_property_static_decorator_method_related_type(
            "ClassDeclaration"
        ));
        assert!(!is_class_property_static_decorator_method_related_type(
            "OptionalMemberExpression"
        ));

        assert!(is_continue116_property_definition_type(
            "PropertyDefinition"
        ));
        assert!(!is_continue116_property_definition_type("StaticBlock"));
        assert!(is_continue116_static_block_type("StaticBlock"));
        assert!(is_continue116_decorator_type("Decorator"));
        assert!(is_continue116_class_body_type("ClassBody"));
        assert!(is_continue116_method_definition_type("MethodDefinition"));
        assert!(is_continue116_export_default_specifier_type(
            "ExportDefaultSpecifier"
        ));
        assert!(is_continue116_class_member_type("PropertyDefinition"));
        assert!(is_continue116_class_member_type("StaticBlock"));
        assert!(is_continue116_class_member_type("MethodDefinition"));
        assert!(!is_continue116_class_member_type("Decorator"));
        assert!(is_continue116_type("PropertyDefinition"));
        assert!(is_continue116_type("ExportDefaultSpecifier"));
        assert!(!is_continue116_type("ClassDeclaration"));
        assert!(!is_continue116_type("AwaitExpression"));
    }

    #[test]
    fn continue116_property_static_decorator_emit() {
        assert_eq!(
            continue116_property_definition_skeleton("x", "1"),
            "x = 1;"
        );
        assert_eq!(
            continue116_property_definition_skeleton("x", "1"),
            continue49_property_definition_skeleton("x", "1")
        );
        assert_eq!(
            continue116_property_definition_pretty("a", "b"),
            continue116_property_definition_minify("a", "b")
        );

        assert_eq!(
            continue116_static_property_skeleton("y", "2"),
            "static y = 2;"
        );
        assert_eq!(
            continue116_static_property_skeleton("y", "2"),
            continue49_static_property_skeleton("y", "2")
        );
        assert_eq!(
            continue116_static_property_pretty("k", "v"),
            continue116_static_property_minify("k", "v")
        );

        assert_eq!(
            continue116_static_block_skeleton("init();"),
            "static { init(); }"
        );
        assert_eq!(
            continue116_static_block_skeleton("init();"),
            continue49_static_block_skeleton("init();")
        );
        assert_eq!(
            continue116_static_block_pretty("x();"),
            continue116_static_block_minify("x();")
        );

        assert_eq!(continue116_decorator_skeleton("sealed"), "@sealed");
        assert_eq!(
            continue116_decorator_skeleton("sealed"),
            continue49_decorator_skeleton("sealed")
        );
        assert_eq!(
            continue116_decorator_pretty("obs"),
            continue116_decorator_minify("obs")
        );
    }

    #[test]
    fn continue116_class_body_method_export_emit() {
        assert_eq!(
            continue116_class_body_skeleton("m() {}"),
            "{ m() {} }"
        );
        assert_eq!(
            continue116_class_body_skeleton("m() {}"),
            continue49_class_body_skeleton("m() {}")
        );
        assert_eq!(
            continue116_class_body_pretty("x"),
            continue116_class_body_minify("x")
        );

        assert_eq!(
            continue116_method_definition_skeleton("m", "a", "return a;"),
            "m(a) { return a; }"
        );
        assert_eq!(
            continue116_method_definition_skeleton("m", "a", "return a;"),
            continue49_method_definition_skeleton("m", "a", "return a;")
        );
        assert_eq!(
            continue116_method_definition_pretty("run", "", "return 1;"),
            continue116_method_definition_minify("run", "", "return 1;")
        );

        assert_eq!(
            continue116_export_default_specifier_skeleton("Foo"),
            "export default Foo;"
        );
        assert_eq!(
            continue116_export_default_specifier_skeleton("Foo"),
            continue49_export_default_specifier_skeleton("Foo")
        );
        assert_eq!(
            continue116_export_default_specifier_pretty("Bar"),
            continue116_export_default_specifier_minify("Bar")
        );
    }

    #[test]
    fn continue116_composed_residual_shells() {
        assert_eq!(
            continue116_class_body_with_property("x", "1"),
            "{ x = 1; }"
        );
        assert_eq!(
            continue116_class_body_with_static_property("y", "2"),
            "{ static y = 2; }"
        );
        assert_eq!(
            continue116_class_body_with_method("m", "a", "return a;"),
            "{ m(a) { return a; } }"
        );
        assert_eq!(
            continue116_class_body_with_static_block("init();"),
            "{ static { init(); } }"
        );
        assert_eq!(
            continue116_decorated_method("sealed", "run", "", "return 1;"),
            "@sealed run() { return 1; }"
        );
        assert_eq!(
            continue116_decorated_property("obs", "x", "0"),
            "@obs x = 0;"
        );
        assert_eq!(
            continue116_class_body_property_and_method("x", "1", "m", "", "return this.x;"),
            "{ x = 1; m() { return this.x; } }"
        );
        assert_eq!(
            continue116_class_body_static_property_and_block("y", "2", "ready();"),
            "{ static y = 2; static { ready(); } }"
        );
        assert_eq!(
            continue116_export_default_class_local("Widget"),
            "export default Widget;"
        );
        assert_eq!(continue116_static_block_empty(), "static {  }");
        assert_eq!(continue116_class_body_empty(), "{  }");
        assert_eq!(continue116_sep(true), " ");
        assert_eq!(continue116_sep(false), "");
        assert_ne!(continue116_sep(true), continue116_sep(false));
    }
}

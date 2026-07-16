//! Pure ClassDeclaration + ClassExpression + ImportDeclaration +
//! ExportNamedDeclaration + ExportDefaultDeclaration + ExportAllDeclaration +
//! NewExpression + ThisExpression + Super + MetaProperty dual-oracle emission
//! — residual pure **continue114** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–113:
//! - ClassDeclaration dual-oracle composing real
//!   `continue47_class_declaration_skeleton`
//! - ClassExpression dual-oracle composing real
//!   `continue47_class_expression_skeleton`
//! - ImportDeclaration dual-oracle composing real `continue47_import_skeleton`
//! - ExportNamedDeclaration dual-oracle composing real
//!   `continue47_export_named_skeleton`
//! - ExportDefaultDeclaration dual-oracle composing real
//!   `continue47_export_default_skeleton`
//! - ExportAllDeclaration dual-oracle composing real
//!   `continue47_export_all_skeleton`
//! - NewExpression dual-oracle composing real `continue47_new_skeleton`
//! - ThisExpression dual-oracle composing real `continue47_this_skeleton`
//! - Super dual-oracle composing real `continue47_super_skeleton`
//! - MetaProperty dual-oracle composing real
//!   `continue47_meta_property_skeleton`
//! - Composed class/import/export/new/this/super/meta residual shells
//!
//! Intentionally does **not** re-wrap continue113 for/throw/label/empty poles
//! (continue46 bases), continue112 do-while/switch/break/continue/try poles
//! (continue45 bases), or continue111 jsx element/fragment/attr poles
//! (continue44 bases). Composes real shipped pure helpers from continue47
//! class/import/export bases. Full engines remain product dens. NO
//! authority_rust / ts_deleted. dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue47_class_declaration_skeleton, continue47_class_expression_skeleton,
    continue47_export_all_skeleton, continue47_export_default_skeleton,
    continue47_export_named_skeleton, continue47_import_skeleton,
    continue47_meta_property_skeleton, continue47_new_skeleton, continue47_super_skeleton,
    continue47_this_skeleton,
};

/// Dual-oracle residual: continue114 related AST type catalog.
pub const CONTINUE114_RELATED_TYPES: &[&str] = &[
    "ClassDeclaration",
    "ClassExpression",
    "ImportDeclaration",
    "ExportNamedDeclaration",
    "ExportDefaultDeclaration",
    "ExportAllDeclaration",
    "NewExpression",
    "ThisExpression",
    "Super",
    "MetaProperty",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_class_import_export_new_this_super_meta_related_type(t: &str) -> bool {
    CONTINUE114_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue114_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue114_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue114_import_type(t: &str) -> bool {
    t == "ImportDeclaration"
}

#[must_use]
pub fn is_continue114_export_named_type(t: &str) -> bool {
    t == "ExportNamedDeclaration"
}

#[must_use]
pub fn is_continue114_export_default_type(t: &str) -> bool {
    t == "ExportDefaultDeclaration"
}

#[must_use]
pub fn is_continue114_export_all_type(t: &str) -> bool {
    t == "ExportAllDeclaration"
}

#[must_use]
pub fn is_continue114_new_type(t: &str) -> bool {
    t == "NewExpression"
}

#[must_use]
pub fn is_continue114_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue114_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue114_meta_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue114_class_type(t: &str) -> bool {
    matches!(t, "ClassDeclaration" | "ClassExpression")
}

#[must_use]
pub fn is_continue114_export_type(t: &str) -> bool {
    matches!(
        t,
        "ExportNamedDeclaration" | "ExportDefaultDeclaration" | "ExportAllDeclaration"
    )
}

#[must_use]
pub fn is_continue114_type(t: &str) -> bool {
    matches!(
        t,
        "ClassDeclaration"
            | "ClassExpression"
            | "ImportDeclaration"
            | "ExportNamedDeclaration"
            | "ExportDefaultDeclaration"
            | "ExportAllDeclaration"
            | "NewExpression"
            | "ThisExpression"
            | "Super"
            | "MetaProperty"
    )
}

// ── ClassDeclaration dual-oracle ────────────────────────────────────────────

/// Dual-oracle ClassDeclaration skeleton composing real
/// [`continue47_class_declaration_skeleton`].
#[must_use]
pub fn continue114_class_declaration_skeleton(name: &str, body: &str) -> String {
    continue47_class_declaration_skeleton(name, body)
}

/// Dual-oracle ClassDeclaration pretty alias.
#[must_use]
pub fn continue114_class_declaration_pretty(name: &str, body: &str) -> String {
    continue114_class_declaration_skeleton(name, body)
}

/// Dual-oracle ClassDeclaration minify alias.
#[must_use]
pub fn continue114_class_declaration_minify(name: &str, body: &str) -> String {
    continue114_class_declaration_skeleton(name, body)
}

// ── ClassExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ClassExpression skeleton composing real
/// [`continue47_class_expression_skeleton`].
#[must_use]
pub fn continue114_class_expression_skeleton(name: Option<&str>, body: &str) -> String {
    continue47_class_expression_skeleton(name, body)
}

/// Dual-oracle ClassExpression pretty alias.
#[must_use]
pub fn continue114_class_expression_pretty(name: Option<&str>, body: &str) -> String {
    continue114_class_expression_skeleton(name, body)
}

/// Dual-oracle ClassExpression minify alias.
#[must_use]
pub fn continue114_class_expression_minify(name: Option<&str>, body: &str) -> String {
    continue114_class_expression_skeleton(name, body)
}

// ── ImportDeclaration dual-oracle ───────────────────────────────────────────

/// Dual-oracle ImportDeclaration skeleton composing real
/// [`continue47_import_skeleton`].
#[must_use]
pub fn continue114_import_skeleton(spec: &str, source: &str) -> String {
    continue47_import_skeleton(spec, source)
}

/// Dual-oracle ImportDeclaration pretty alias.
#[must_use]
pub fn continue114_import_pretty(spec: &str, source: &str) -> String {
    continue114_import_skeleton(spec, source)
}

/// Dual-oracle ImportDeclaration minify alias.
#[must_use]
pub fn continue114_import_minify(spec: &str, source: &str) -> String {
    continue114_import_skeleton(spec, source)
}

// ── ExportNamedDeclaration dual-oracle ──────────────────────────────────────

/// Dual-oracle ExportNamedDeclaration skeleton composing real
/// [`continue47_export_named_skeleton`].
#[must_use]
pub fn continue114_export_named_skeleton(body: &str) -> String {
    continue47_export_named_skeleton(body)
}

/// Dual-oracle ExportNamedDeclaration pretty alias.
#[must_use]
pub fn continue114_export_named_pretty(body: &str) -> String {
    continue114_export_named_skeleton(body)
}

/// Dual-oracle ExportNamedDeclaration minify alias.
#[must_use]
pub fn continue114_export_named_minify(body: &str) -> String {
    continue114_export_named_skeleton(body)
}

// ── ExportDefaultDeclaration dual-oracle ────────────────────────────────────

/// Dual-oracle ExportDefaultDeclaration skeleton composing real
/// [`continue47_export_default_skeleton`].
#[must_use]
pub fn continue114_export_default_skeleton(body: &str) -> String {
    continue47_export_default_skeleton(body)
}

/// Dual-oracle ExportDefaultDeclaration pretty alias.
#[must_use]
pub fn continue114_export_default_pretty(body: &str) -> String {
    continue114_export_default_skeleton(body)
}

/// Dual-oracle ExportDefaultDeclaration minify alias.
#[must_use]
pub fn continue114_export_default_minify(body: &str) -> String {
    continue114_export_default_skeleton(body)
}

// ── ExportAllDeclaration dual-oracle ────────────────────────────────────────

/// Dual-oracle ExportAllDeclaration skeleton composing real
/// [`continue47_export_all_skeleton`].
#[must_use]
pub fn continue114_export_all_skeleton(source: &str) -> String {
    continue47_export_all_skeleton(source)
}

/// Dual-oracle ExportAllDeclaration pretty alias.
#[must_use]
pub fn continue114_export_all_pretty(source: &str) -> String {
    continue114_export_all_skeleton(source)
}

/// Dual-oracle ExportAllDeclaration minify alias.
#[must_use]
pub fn continue114_export_all_minify(source: &str) -> String {
    continue114_export_all_skeleton(source)
}

// ── NewExpression dual-oracle ───────────────────────────────────────────────

/// Dual-oracle NewExpression skeleton composing real
/// [`continue47_new_skeleton`].
#[must_use]
pub fn continue114_new_skeleton(callee: &str, args: &str) -> String {
    continue47_new_skeleton(callee, args)
}

/// Dual-oracle NewExpression pretty alias.
#[must_use]
pub fn continue114_new_pretty(callee: &str, args: &str) -> String {
    continue114_new_skeleton(callee, args)
}

/// Dual-oracle NewExpression minify alias.
#[must_use]
pub fn continue114_new_minify(callee: &str, args: &str) -> String {
    continue114_new_skeleton(callee, args)
}

// ── ThisExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThisExpression skeleton composing real
/// [`continue47_this_skeleton`].
#[must_use]
pub fn continue114_this_skeleton() -> &'static str {
    continue47_this_skeleton()
}

/// Dual-oracle ThisExpression pretty alias.
#[must_use]
pub fn continue114_this_pretty() -> &'static str {
    continue114_this_skeleton()
}

/// Dual-oracle ThisExpression minify alias.
#[must_use]
pub fn continue114_this_minify() -> &'static str {
    continue114_this_skeleton()
}

// ── Super dual-oracle ───────────────────────────────────────────────────────

/// Dual-oracle Super skeleton composing real [`continue47_super_skeleton`].
#[must_use]
pub fn continue114_super_skeleton() -> &'static str {
    continue47_super_skeleton()
}

/// Dual-oracle Super pretty alias.
#[must_use]
pub fn continue114_super_pretty() -> &'static str {
    continue114_super_skeleton()
}

/// Dual-oracle Super minify alias.
#[must_use]
pub fn continue114_super_minify() -> &'static str {
    continue114_super_skeleton()
}

// ── MetaProperty dual-oracle ────────────────────────────────────────────────

/// Dual-oracle MetaProperty skeleton composing real
/// [`continue47_meta_property_skeleton`].
#[must_use]
pub fn continue114_meta_property_skeleton(meta: &str, property: &str) -> String {
    continue47_meta_property_skeleton(meta, property)
}

/// Dual-oracle MetaProperty pretty alias.
#[must_use]
pub fn continue114_meta_property_pretty(meta: &str, property: &str) -> String {
    continue114_meta_property_skeleton(meta, property)
}

/// Dual-oracle MetaProperty minify alias.
#[must_use]
pub fn continue114_meta_property_minify(meta: &str, property: &str) -> String {
    continue114_meta_property_skeleton(meta, property)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: empty class declaration body.
#[must_use]
pub fn continue114_class_empty(name: &str) -> String {
    continue114_class_declaration_skeleton(name, "{}")
}

/// Dual-oracle residual: anonymous empty class expression.
#[must_use]
pub fn continue114_class_expression_empty() -> String {
    continue114_class_expression_skeleton(None, "{}")
}

/// Dual-oracle residual: export default class declaration seed.
#[must_use]
pub fn continue114_export_default_class(name: &str, body: &str) -> String {
    let class = continue114_class_declaration_skeleton(name, body);
    continue114_export_default_skeleton(&class)
}

/// Dual-oracle residual: export named class declaration seed.
#[must_use]
pub fn continue114_export_named_class(name: &str, body: &str) -> String {
    let class = continue114_class_declaration_skeleton(name, body);
    continue114_export_named_skeleton(&class)
}

/// Dual-oracle residual: import then new composition seed.
#[must_use]
pub fn continue114_import_then_new(spec: &str, source: &str, callee: &str, args: &str) -> String {
    format!(
        "{} {}",
        continue114_import_skeleton(spec, source),
        continue114_new_skeleton(callee, args)
    )
}

/// Dual-oracle residual: export all then import composition seed.
#[must_use]
pub fn continue114_export_all_then_import(all_src: &str, spec: &str, import_src: &str) -> String {
    format!(
        "{} {}",
        continue114_export_all_skeleton(all_src),
        continue114_import_skeleton(spec, import_src)
    )
}

/// Dual-oracle residual: `new` of `this` constructor call seed (`new this()`).
#[must_use]
pub fn continue114_new_this() -> String {
    continue114_new_skeleton(continue114_this_skeleton(), "")
}

/// Dual-oracle residual: `new` of `super` constructor call seed (`new super()`).
#[must_use]
pub fn continue114_new_super() -> String {
    continue114_new_skeleton(continue114_super_skeleton(), "")
}

/// Dual-oracle residual: import.meta seed.
#[must_use]
pub fn continue114_import_meta() -> String {
    continue114_meta_property_skeleton("import", "meta")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue114_sep(pretty: bool) -> &'static str {
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
        continue47_class_declaration_skeleton, continue47_class_expression_skeleton,
        continue47_export_all_skeleton, continue47_export_default_skeleton,
        continue47_export_named_skeleton, continue47_import_skeleton,
        continue47_meta_property_skeleton, continue47_new_skeleton, continue47_super_skeleton,
        continue47_this_skeleton,
    };

    #[test]
    fn continue114_type_catalog() {
        assert_eq!(CONTINUE114_RELATED_TYPES.len(), 10);
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ClassDeclaration"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ClassExpression"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ImportDeclaration"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ExportNamedDeclaration"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ExportDefaultDeclaration"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ExportAllDeclaration"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "NewExpression"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "ThisExpression"
        ));
        assert!(is_class_import_export_new_this_super_meta_related_type("Super"));
        assert!(is_class_import_export_new_this_super_meta_related_type(
            "MetaProperty"
        ));
        assert!(!is_class_import_export_new_this_super_meta_related_type(
            "ForStatement"
        ));
        assert!(!is_class_import_export_new_this_super_meta_related_type(
            "JSXElement"
        ));

        assert!(is_continue114_class_declaration_type("ClassDeclaration"));
        assert!(!is_continue114_class_declaration_type("ClassExpression"));
        assert!(is_continue114_class_expression_type("ClassExpression"));
        assert!(is_continue114_import_type("ImportDeclaration"));
        assert!(is_continue114_export_named_type("ExportNamedDeclaration"));
        assert!(is_continue114_export_default_type(
            "ExportDefaultDeclaration"
        ));
        assert!(is_continue114_export_all_type("ExportAllDeclaration"));
        assert!(is_continue114_new_type("NewExpression"));
        assert!(is_continue114_this_type("ThisExpression"));
        assert!(is_continue114_super_type("Super"));
        assert!(is_continue114_meta_type("MetaProperty"));
        assert!(is_continue114_class_type("ClassDeclaration"));
        assert!(is_continue114_class_type("ClassExpression"));
        assert!(!is_continue114_class_type("ImportDeclaration"));
        assert!(is_continue114_export_type("ExportNamedDeclaration"));
        assert!(is_continue114_export_type("ExportDefaultDeclaration"));
        assert!(is_continue114_export_type("ExportAllDeclaration"));
        assert!(!is_continue114_export_type("ImportDeclaration"));
        assert!(is_continue114_type("ClassDeclaration"));
        assert!(is_continue114_type("MetaProperty"));
        assert!(!is_continue114_type("ForStatement"));
        assert!(!is_continue114_type("DoWhileStatement"));
    }

    #[test]
    fn continue114_class_import_export_emit() {
        assert_eq!(
            continue114_class_declaration_skeleton("Foo", "{}"),
            "class Foo {}"
        );
        assert_eq!(
            continue114_class_declaration_skeleton("Foo", "{}"),
            continue47_class_declaration_skeleton("Foo", "{}")
        );
        assert_eq!(
            continue114_class_declaration_pretty("A", "{ m() {} }"),
            continue114_class_declaration_minify("A", "{ m() {} }")
        );

        assert_eq!(
            continue114_class_expression_skeleton(None, "{}"),
            "class {}"
        );
        assert_eq!(
            continue114_class_expression_skeleton(Some("Bar"), "{}"),
            "class Bar {}"
        );
        assert_eq!(
            continue114_class_expression_skeleton(None, "{}"),
            continue47_class_expression_skeleton(None, "{}")
        );
        assert_eq!(
            continue114_class_expression_pretty(Some("C"), "{}"),
            continue114_class_expression_minify(Some("C"), "{}")
        );

        assert_eq!(
            continue114_import_skeleton("{ x }", "./mod.js"),
            "import { x } from \"./mod.js\";"
        );
        assert_eq!(
            continue114_import_skeleton("{ x }", "./mod.js"),
            continue47_import_skeleton("{ x }", "./mod.js")
        );
        assert_eq!(
            continue114_import_pretty("x", "y"),
            continue114_import_minify("x", "y")
        );

        assert_eq!(
            continue114_export_named_skeleton("const a = 1;"),
            "export const a = 1;"
        );
        assert_eq!(
            continue114_export_named_skeleton("const a = 1;"),
            continue47_export_named_skeleton("const a = 1;")
        );
        assert_eq!(
            continue114_export_named_pretty("let b;"),
            continue114_export_named_minify("let b;")
        );

        assert_eq!(
            continue114_export_default_skeleton("Foo;"),
            "export default Foo;"
        );
        assert_eq!(
            continue114_export_default_skeleton("Foo;"),
            continue47_export_default_skeleton("Foo;")
        );
        assert_eq!(
            continue114_export_default_pretty("x;"),
            continue114_export_default_minify("x;")
        );

        assert_eq!(
            continue114_export_all_skeleton("./all.js"),
            "export * from \"./all.js\";"
        );
        assert_eq!(
            continue114_export_all_skeleton("./all.js"),
            continue47_export_all_skeleton("./all.js")
        );
        assert_eq!(
            continue114_export_all_pretty("./z"),
            continue114_export_all_minify("./z")
        );
    }

    #[test]
    fn continue114_new_this_super_meta_emit() {
        assert_eq!(continue114_new_skeleton("Map", ""), "new Map()");
        assert_eq!(continue114_new_skeleton("Foo", "1, 2"), "new Foo(1, 2)");
        assert_eq!(
            continue114_new_skeleton("Map", ""),
            continue47_new_skeleton("Map", "")
        );
        assert_eq!(
            continue114_new_pretty("X", "a"),
            continue114_new_minify("X", "a")
        );

        assert_eq!(continue114_this_skeleton(), "this");
        assert_eq!(continue114_this_skeleton(), continue47_this_skeleton());
        assert_eq!(continue114_this_pretty(), continue114_this_minify());

        assert_eq!(continue114_super_skeleton(), "super");
        assert_eq!(continue114_super_skeleton(), continue47_super_skeleton());
        assert_eq!(continue114_super_pretty(), continue114_super_minify());

        assert_eq!(
            continue114_meta_property_skeleton("import", "meta"),
            "import.meta"
        );
        assert_eq!(
            continue114_meta_property_skeleton("import", "meta"),
            continue47_meta_property_skeleton("import", "meta")
        );
        assert_eq!(
            continue114_meta_property_pretty("import", "meta"),
            continue114_meta_property_minify("import", "meta")
        );
    }

    #[test]
    fn continue114_composed_residual_shells() {
        assert_eq!(continue114_class_empty("Foo"), "class Foo {}");
        assert_eq!(continue114_class_expression_empty(), "class {}");
        assert_eq!(
            continue114_export_default_class("Foo", "{}"),
            "export default class Foo {}"
        );
        assert_eq!(
            continue114_export_named_class("Bar", "{}"),
            "export class Bar {}"
        );
        assert_eq!(
            continue114_import_then_new("{ Map }", "mod", "Map", ""),
            "import { Map } from \"mod\"; new Map()"
        );
        assert_eq!(
            continue114_export_all_then_import("./a", "{ b }", "./c"),
            "export * from \"./a\"; import { b } from \"./c\";"
        );
        assert_eq!(continue114_new_this(), "new this()");
        assert_eq!(continue114_new_super(), "new super()");
        assert_eq!(continue114_import_meta(), "import.meta");
        assert_eq!(continue114_sep(true), " ");
        assert_eq!(continue114_sep(false), "");
        assert_ne!(continue114_sep(true), continue114_sep(false));
    }
}

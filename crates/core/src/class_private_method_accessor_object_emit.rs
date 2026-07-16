//! Pure ClassPrivateProperty + ClassPrivateMethod + ClassMethod +
//! ClassAccessorProperty + TSParameterProperty + ObjectMethod dual-oracle
//! emission — residual pure **continue117** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–116:
//! - ClassPrivateProperty dual-oracle composing real
//!   `continue50_private_property_skeleton`
//! - ClassPrivateMethod dual-oracle composing real
//!   `continue50_private_method_skeleton`
//! - ClassMethod getter dual-oracle composing real `continue50_getter_skeleton`
//! - ClassMethod setter dual-oracle composing real `continue50_setter_skeleton`
//! - ObjectMethod dual-oracle composing real
//!   `continue50_object_method_skeleton`
//! - TSParameterProperty dual-oracle composing real
//!   `continue50_ts_param_property_skeleton`
//! - Composed class-private/method/accessor/object residual shells
//!
//! Intentionally does **not** re-wrap continue116 class property/static/
//! decorator/method poles (continue49 bases), continue115 optional/import/
//! await/yield/private poles (continue48 bases), or continue114 class/import/
//! export/new/this/super/meta poles (continue47 bases). Composes real shipped
//! pure helpers from continue50 class private/method/accessor bases. Full
//! engines remain product dens. NO authority_rust / ts_deleted. dens ≠ flip;
//! PreferRust OFF.

use crate::literal_widen_emit::{
    continue50_getter_skeleton, continue50_object_method_skeleton,
    continue50_private_method_skeleton, continue50_private_property_skeleton,
    continue50_setter_skeleton, continue50_ts_param_property_skeleton,
};

/// Dual-oracle residual: continue117 related AST type catalog.
pub const CONTINUE117_RELATED_TYPES: &[&str] = &[
    "ClassPrivateProperty",
    "ClassPrivateMethod",
    "ClassMethod",
    "ClassAccessorProperty",
    "TSParameterProperty",
    "ObjectMethod",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_class_private_method_accessor_object_related_type(t: &str) -> bool {
    CONTINUE117_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue117_private_property_type(t: &str) -> bool {
    t == "ClassPrivateProperty"
}

#[must_use]
pub fn is_continue117_private_method_type(t: &str) -> bool {
    t == "ClassPrivateMethod"
}

#[must_use]
pub fn is_continue117_class_method_type(t: &str) -> bool {
    t == "ClassMethod"
}

#[must_use]
pub fn is_continue117_accessor_type(t: &str) -> bool {
    t == "ClassAccessorProperty"
}

#[must_use]
pub fn is_continue117_ts_param_property_type(t: &str) -> bool {
    t == "TSParameterProperty"
}

#[must_use]
pub fn is_continue117_object_method_type(t: &str) -> bool {
    t == "ObjectMethod"
}

#[must_use]
pub fn is_continue117_private_member_type(t: &str) -> bool {
    matches!(t, "ClassPrivateProperty" | "ClassPrivateMethod")
}

#[must_use]
pub fn is_continue117_type(t: &str) -> bool {
    matches!(
        t,
        "ClassPrivateProperty"
            | "ClassPrivateMethod"
            | "ClassMethod"
            | "ClassAccessorProperty"
            | "TSParameterProperty"
            | "ObjectMethod"
    )
}

// ── ClassPrivateProperty dual-oracle ────────────────────────────────────────

/// Dual-oracle ClassPrivateProperty skeleton composing real
/// [`continue50_private_property_skeleton`].
#[must_use]
pub fn continue117_private_property_skeleton(name: &str, value: &str) -> String {
    continue50_private_property_skeleton(name, value)
}

/// Dual-oracle ClassPrivateProperty pretty alias.
#[must_use]
pub fn continue117_private_property_pretty(name: &str, value: &str) -> String {
    continue117_private_property_skeleton(name, value)
}

/// Dual-oracle ClassPrivateProperty minify alias.
#[must_use]
pub fn continue117_private_property_minify(name: &str, value: &str) -> String {
    continue117_private_property_skeleton(name, value)
}

// ── ClassPrivateMethod dual-oracle ──────────────────────────────────────────

/// Dual-oracle ClassPrivateMethod skeleton composing real
/// [`continue50_private_method_skeleton`].
#[must_use]
pub fn continue117_private_method_skeleton(name: &str, params: &str, body: &str) -> String {
    continue50_private_method_skeleton(name, params, body)
}

/// Dual-oracle ClassPrivateMethod pretty alias.
#[must_use]
pub fn continue117_private_method_pretty(name: &str, params: &str, body: &str) -> String {
    continue117_private_method_skeleton(name, params, body)
}

/// Dual-oracle ClassPrivateMethod minify alias.
#[must_use]
pub fn continue117_private_method_minify(name: &str, params: &str, body: &str) -> String {
    continue117_private_method_skeleton(name, params, body)
}

// ── ClassMethod getter dual-oracle ──────────────────────────────────────────

/// Dual-oracle ClassMethod getter skeleton composing real
/// [`continue50_getter_skeleton`].
#[must_use]
pub fn continue117_getter_skeleton(name: &str, body: &str) -> String {
    continue50_getter_skeleton(name, body)
}

/// Dual-oracle getter pretty alias.
#[must_use]
pub fn continue117_getter_pretty(name: &str, body: &str) -> String {
    continue117_getter_skeleton(name, body)
}

/// Dual-oracle getter minify alias.
#[must_use]
pub fn continue117_getter_minify(name: &str, body: &str) -> String {
    continue117_getter_skeleton(name, body)
}

// ── ClassMethod setter dual-oracle ──────────────────────────────────────────

/// Dual-oracle ClassMethod setter skeleton composing real
/// [`continue50_setter_skeleton`].
#[must_use]
pub fn continue117_setter_skeleton(name: &str, param: &str, body: &str) -> String {
    continue50_setter_skeleton(name, param, body)
}

/// Dual-oracle setter pretty alias.
#[must_use]
pub fn continue117_setter_pretty(name: &str, param: &str, body: &str) -> String {
    continue117_setter_skeleton(name, param, body)
}

/// Dual-oracle setter minify alias.
#[must_use]
pub fn continue117_setter_minify(name: &str, param: &str, body: &str) -> String {
    continue117_setter_skeleton(name, param, body)
}

// ── ObjectMethod dual-oracle ────────────────────────────────────────────────

/// Dual-oracle ObjectMethod skeleton composing real
/// [`continue50_object_method_skeleton`].
#[must_use]
pub fn continue117_object_method_skeleton(name: &str, params: &str, body: &str) -> String {
    continue50_object_method_skeleton(name, params, body)
}

/// Dual-oracle ObjectMethod pretty alias.
#[must_use]
pub fn continue117_object_method_pretty(name: &str, params: &str, body: &str) -> String {
    continue117_object_method_skeleton(name, params, body)
}

/// Dual-oracle ObjectMethod minify alias.
#[must_use]
pub fn continue117_object_method_minify(name: &str, params: &str, body: &str) -> String {
    continue117_object_method_skeleton(name, params, body)
}

// ── TSParameterProperty dual-oracle ─────────────────────────────────────────

/// Dual-oracle TSParameterProperty skeleton composing real
/// [`continue50_ts_param_property_skeleton`].
#[must_use]
pub fn continue117_ts_param_property_skeleton(modif: &str, name: &str) -> String {
    continue50_ts_param_property_skeleton(modif, name)
}

/// Dual-oracle TSParameterProperty pretty alias.
#[must_use]
pub fn continue117_ts_param_property_pretty(modif: &str, name: &str) -> String {
    continue117_ts_param_property_skeleton(modif, name)
}

/// Dual-oracle TSParameterProperty minify alias.
#[must_use]
pub fn continue117_ts_param_property_minify(modif: &str, name: &str) -> String {
    continue117_ts_param_property_skeleton(modif, name)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: private property + private method seed (whitespace-joined).
#[must_use]
pub fn continue117_private_property_and_method(
    prop: &str,
    value: &str,
    method: &str,
    params: &str,
    body: &str,
) -> String {
    let p = continue117_private_property_skeleton(prop, value);
    let m = continue117_private_method_skeleton(method, params, body);
    format!("{p} {m}")
}

/// Dual-oracle residual: getter + setter pair seed (whitespace-joined).
#[must_use]
pub fn continue117_accessor_pair(name: &str, get_body: &str, set_param: &str, set_body: &str) -> String {
    let g = continue117_getter_skeleton(name, get_body);
    let s = continue117_setter_skeleton(name, set_param, set_body);
    format!("{g} {s}")
}

/// Dual-oracle residual: private field with matching getter.
#[must_use]
pub fn continue117_private_field_with_getter(
    field: &str,
    value: &str,
    getter: &str,
    get_body: &str,
) -> String {
    let p = continue117_private_property_skeleton(field, value);
    let g = continue117_getter_skeleton(getter, get_body);
    format!("{p} {g}")
}

/// Dual-oracle residual: private field with matching setter.
#[must_use]
pub fn continue117_private_field_with_setter(
    field: &str,
    value: &str,
    setter: &str,
    set_param: &str,
    set_body: &str,
) -> String {
    let p = continue117_private_property_skeleton(field, value);
    let s = continue117_setter_skeleton(setter, set_param, set_body);
    format!("{p} {s}")
}

/// Dual-oracle residual: object method map seed (two methods, whitespace-joined).
#[must_use]
pub fn continue117_object_method_pair(
    a_name: &str,
    a_params: &str,
    a_body: &str,
    b_name: &str,
    b_params: &str,
    b_body: &str,
) -> String {
    let a = continue117_object_method_skeleton(a_name, a_params, a_body);
    let b = continue117_object_method_skeleton(b_name, b_params, b_body);
    format!("{a} {b}")
}

/// Dual-oracle residual: constructor parameter property pair seed.
#[must_use]
pub fn continue117_ts_param_property_pair(
    mod_a: &str,
    name_a: &str,
    mod_b: &str,
    name_b: &str,
) -> String {
    let a = continue117_ts_param_property_skeleton(mod_a, name_a);
    let b = continue117_ts_param_property_skeleton(mod_b, name_b);
    format!("{a}, {b}")
}

/// Dual-oracle residual: private method calling through object method style shell.
#[must_use]
pub fn continue117_private_method_then_object_method(
    priv_name: &str,
    priv_params: &str,
    priv_body: &str,
    obj_name: &str,
    obj_params: &str,
    obj_body: &str,
) -> String {
    let p = continue117_private_method_skeleton(priv_name, priv_params, priv_body);
    let o = continue117_object_method_skeleton(obj_name, obj_params, obj_body);
    format!("{p} {o}")
}

/// Dual-oracle residual: empty private method body seed.
#[must_use]
pub fn continue117_private_method_empty(name: &str) -> String {
    continue117_private_method_skeleton(name, "", "")
}

/// Dual-oracle residual: empty object method body seed.
#[must_use]
pub fn continue117_object_method_empty(name: &str) -> String {
    continue117_object_method_skeleton(name, "", "")
}

/// Dual-oracle residual: private readonly-style parameter property seed.
#[must_use]
pub fn continue117_ts_param_private_readonly(name: &str) -> String {
    continue117_ts_param_property_skeleton("private readonly", name)
}

/// Dual-oracle residual: public parameter property seed.
#[must_use]
pub fn continue117_ts_param_public(name: &str) -> String {
    continue117_ts_param_property_skeleton("public", name)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue117_sep(pretty: bool) -> &'static str {
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
        continue50_getter_skeleton, continue50_object_method_skeleton,
        continue50_private_method_skeleton, continue50_private_property_skeleton,
        continue50_setter_skeleton, continue50_ts_param_property_skeleton,
    };

    #[test]
    fn continue117_type_catalog() {
        assert_eq!(CONTINUE117_RELATED_TYPES.len(), 6);
        assert!(is_class_private_method_accessor_object_related_type(
            "ClassPrivateProperty"
        ));
        assert!(is_class_private_method_accessor_object_related_type(
            "ClassPrivateMethod"
        ));
        assert!(is_class_private_method_accessor_object_related_type(
            "ClassMethod"
        ));
        assert!(is_class_private_method_accessor_object_related_type(
            "ClassAccessorProperty"
        ));
        assert!(is_class_private_method_accessor_object_related_type(
            "TSParameterProperty"
        ));
        assert!(is_class_private_method_accessor_object_related_type(
            "ObjectMethod"
        ));
        assert!(!is_class_private_method_accessor_object_related_type(
            "PropertyDefinition"
        ));
        assert!(!is_class_private_method_accessor_object_related_type(
            "Decorator"
        ));

        assert!(is_continue117_private_property_type("ClassPrivateProperty"));
        assert!(!is_continue117_private_property_type("ClassPrivateMethod"));
        assert!(is_continue117_private_method_type("ClassPrivateMethod"));
        assert!(is_continue117_class_method_type("ClassMethod"));
        assert!(is_continue117_accessor_type("ClassAccessorProperty"));
        assert!(is_continue117_ts_param_property_type("TSParameterProperty"));
        assert!(is_continue117_object_method_type("ObjectMethod"));
        assert!(is_continue117_private_member_type("ClassPrivateProperty"));
        assert!(is_continue117_private_member_type("ClassPrivateMethod"));
        assert!(!is_continue117_private_member_type("ObjectMethod"));
        assert!(is_continue117_type("ClassPrivateProperty"));
        assert!(is_continue117_type("ObjectMethod"));
        assert!(!is_continue117_type("PropertyDefinition"));
        assert!(!is_continue117_type("StaticBlock"));
    }

    #[test]
    fn continue117_private_property_method_emit() {
        assert_eq!(
            continue117_private_property_skeleton("x", "1"),
            "#x = 1;"
        );
        assert_eq!(
            continue117_private_property_skeleton("x", "1"),
            continue50_private_property_skeleton("x", "1")
        );
        assert_eq!(
            continue117_private_property_pretty("a", "b"),
            continue117_private_property_minify("a", "b")
        );

        assert_eq!(
            continue117_private_method_skeleton("m", "a", "return a;"),
            "#m(a) { return a; }"
        );
        assert_eq!(
            continue117_private_method_skeleton("m", "a", "return a;"),
            continue50_private_method_skeleton("m", "a", "return a;")
        );
        assert_eq!(
            continue117_private_method_pretty("run", "", "return 1;"),
            continue117_private_method_minify("run", "", "return 1;")
        );
    }

    #[test]
    fn continue117_getter_setter_object_ts_param_emit() {
        assert_eq!(
            continue117_getter_skeleton("v", "return this.#v;"),
            "get v() { return this.#v; }"
        );
        assert_eq!(
            continue117_getter_skeleton("v", "return this.#v;"),
            continue50_getter_skeleton("v", "return this.#v;")
        );
        assert_eq!(
            continue117_getter_pretty("x", "return 1;"),
            continue117_getter_minify("x", "return 1;")
        );

        assert_eq!(
            continue117_setter_skeleton("v", "n", "this.#v = n;"),
            "set v(n) { this.#v = n; }"
        );
        assert_eq!(
            continue117_setter_skeleton("v", "n", "this.#v = n;"),
            continue50_setter_skeleton("v", "n", "this.#v = n;")
        );
        assert_eq!(
            continue117_setter_pretty("x", "v", "this.x = v;"),
            continue117_setter_minify("x", "v", "this.x = v;")
        );

        assert_eq!(
            continue117_object_method_skeleton("run", "", "return 1;"),
            "run() { return 1; }"
        );
        assert_eq!(
            continue117_object_method_skeleton("run", "", "return 1;"),
            continue50_object_method_skeleton("run", "", "return 1;")
        );
        assert_eq!(
            continue117_object_method_pretty("m", "a", "return a;"),
            continue117_object_method_minify("m", "a", "return a;")
        );

        assert_eq!(
            continue117_ts_param_property_skeleton("private", "id"),
            "private id"
        );
        assert_eq!(
            continue117_ts_param_property_skeleton("private", "id"),
            continue50_ts_param_property_skeleton("private", "id")
        );
        assert_eq!(
            continue117_ts_param_property_pretty("public", "name"),
            continue117_ts_param_property_minify("public", "name")
        );
    }

    #[test]
    fn continue117_composed_residual_shells() {
        assert_eq!(
            continue117_private_property_and_method("x", "1", "m", "a", "return a;"),
            "#x = 1; #m(a) { return a; }"
        );
        assert_eq!(
            continue117_accessor_pair("v", "return this.#v;", "n", "this.#v = n;"),
            "get v() { return this.#v; } set v(n) { this.#v = n; }"
        );
        assert_eq!(
            continue117_private_field_with_getter("x", "0", "x", "return this.#x;"),
            "#x = 0; get x() { return this.#x; }"
        );
        assert_eq!(
            continue117_private_field_with_setter("x", "0", "x", "n", "this.#x = n;"),
            "#x = 0; set x(n) { this.#x = n; }"
        );
        assert_eq!(
            continue117_object_method_pair("a", "", "return 1;", "b", "x", "return x;"),
            "a() { return 1; } b(x) { return x; }"
        );
        assert_eq!(
            continue117_ts_param_property_pair("private", "id", "public", "name"),
            "private id, public name"
        );
        assert_eq!(
            continue117_private_method_then_object_method(
                "m",
                "",
                "return 1;",
                "run",
                "a",
                "return a;"
            ),
            "#m() { return 1; } run(a) { return a; }"
        );
        assert_eq!(continue117_private_method_empty("m"), "#m() {  }");
        assert_eq!(continue117_object_method_empty("run"), "run() {  }");
        assert_eq!(
            continue117_ts_param_private_readonly("id"),
            "private readonly id"
        );
        assert_eq!(continue117_ts_param_public("name"), "public name");
        assert_eq!(continue117_sep(true), " ");
        assert_eq!(continue117_sep(false), "");
        assert_ne!(continue117_sep(true), continue117_sep(false));
    }
}

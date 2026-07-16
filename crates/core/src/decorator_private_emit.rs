//! Pure Decorator + ClassPrivateProperty + ClassPrivateMethod + ClassMethod +
//! ClassAccessorProperty + TSParameterProperty + ExportDefaultSpecifier dual-oracle
//! emission — residual pure **continue89** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–88:
//! - Decorator dual-oracle composing real `continue49_decorator_skeleton`
//! - ClassPrivateProperty dual-oracle composing real
//!   `continue50_private_property_skeleton`
//! - ClassPrivateMethod dual-oracle composing real
//!   `continue50_private_method_skeleton`
//! - ClassMethod get/set dual-oracle composing real
//!   `continue50_getter_skeleton` / `continue50_setter_skeleton`
//! - ClassAccessorProperty dual-oracle residual shell (`accessor name = value;`)
//! - TSParameterProperty dual-oracle composing real
//!   `continue50_ts_param_property_skeleton`
//! - ExportDefaultSpecifier dual-oracle composing real
//!   `continue49_export_default_specifier_skeleton`
//! - Composed decorated private property / method residual shells
//!
//! Intentionally does **not** re-wrap continue64–88 partition shells
//! (PropertyDefinition/StaticBlock/PrivateIdentifier continue82 stays separate;
//! MethodDefinition/ObjectMethod continue72/74 stay separate; JSX continue88
//! stays separate). Composes real shipped pure helpers from continue49/50 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue49_decorator_skeleton, continue49_export_default_specifier_skeleton,
    continue50_getter_skeleton, continue50_private_method_skeleton,
    continue50_private_property_skeleton, continue50_setter_skeleton,
    continue50_ts_param_property_skeleton,
};

/// Dual-oracle residual: continue89 related AST type catalog.
pub const CONTINUE89_RELATED_TYPES: &[&str] = &[
    "Decorator",
    "ClassPrivateProperty",
    "ClassPrivateMethod",
    "ClassMethod",
    "ClassAccessorProperty",
    "TSParameterProperty",
    "ExportDefaultSpecifier",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_decorator_private_related_type(t: &str) -> bool {
    CONTINUE89_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue89_decorator_type(t: &str) -> bool {
    t == "Decorator"
}

#[must_use]
pub fn is_continue89_class_private_property_type(t: &str) -> bool {
    t == "ClassPrivateProperty"
}

#[must_use]
pub fn is_continue89_class_private_method_type(t: &str) -> bool {
    t == "ClassPrivateMethod"
}

#[must_use]
pub fn is_continue89_class_method_type(t: &str) -> bool {
    t == "ClassMethod"
}

#[must_use]
pub fn is_continue89_class_accessor_property_type(t: &str) -> bool {
    t == "ClassAccessorProperty"
}

#[must_use]
pub fn is_continue89_ts_parameter_property_type(t: &str) -> bool {
    t == "TSParameterProperty"
}

#[must_use]
pub fn is_continue89_export_default_specifier_type(t: &str) -> bool {
    t == "ExportDefaultSpecifier"
}

// ── Decorator dual-oracle ───────────────────────────────────────────────────

/// Dual-oracle Decorator skeleton composing real [`continue49_decorator_skeleton`].
#[must_use]
pub fn continue89_decorator_skeleton(expr: &str) -> String {
    continue49_decorator_skeleton(expr)
}

/// Dual-oracle Decorator pretty alias.
#[must_use]
pub fn decorator_pretty(expr: &str) -> String {
    continue89_decorator_skeleton(expr)
}

/// Dual-oracle Decorator minify alias (same surface — decorator has no sep poles).
#[must_use]
pub fn decorator_minify(expr: &str) -> String {
    continue89_decorator_skeleton(expr)
}

// ── ClassPrivateProperty dual-oracle ────────────────────────────────────────

/// Dual-oracle ClassPrivateProperty skeleton composing real
/// [`continue50_private_property_skeleton`].
#[must_use]
pub fn continue89_class_private_property_skeleton(name: &str, value: &str) -> String {
    continue50_private_property_skeleton(name, value)
}

/// Dual-oracle ClassPrivateProperty pretty alias.
#[must_use]
pub fn class_private_property_pretty(name: &str, value: &str) -> String {
    continue89_class_private_property_skeleton(name, value)
}

/// Dual-oracle ClassPrivateProperty minify alias.
#[must_use]
pub fn class_private_property_minify(name: &str, value: &str) -> String {
    continue89_class_private_property_skeleton(name, value)
}

// ── ClassPrivateMethod dual-oracle ──────────────────────────────────────────

/// Dual-oracle ClassPrivateMethod skeleton composing real
/// [`continue50_private_method_skeleton`].
#[must_use]
pub fn continue89_class_private_method_skeleton(name: &str, params: &str, body: &str) -> String {
    continue50_private_method_skeleton(name, params, body)
}

/// Dual-oracle ClassPrivateMethod pretty alias.
#[must_use]
pub fn class_private_method_pretty(name: &str, params: &str, body: &str) -> String {
    continue89_class_private_method_skeleton(name, params, body)
}

/// Dual-oracle ClassPrivateMethod minify alias.
#[must_use]
pub fn class_private_method_minify(name: &str, params: &str, body: &str) -> String {
    continue89_class_private_method_skeleton(name, params, body)
}

// ── ClassMethod get/set dual-oracle ─────────────────────────────────────────

/// Dual-oracle ClassMethod getter skeleton composing real
/// [`continue50_getter_skeleton`].
#[must_use]
pub fn continue89_class_method_getter_skeleton(name: &str, body: &str) -> String {
    continue50_getter_skeleton(name, body)
}

/// Dual-oracle ClassMethod setter skeleton composing real
/// [`continue50_setter_skeleton`].
#[must_use]
pub fn continue89_class_method_setter_skeleton(name: &str, param: &str, body: &str) -> String {
    continue50_setter_skeleton(name, param, body)
}

/// Dual-oracle ClassMethod getter pretty alias.
#[must_use]
pub fn class_method_getter_pretty(name: &str, body: &str) -> String {
    continue89_class_method_getter_skeleton(name, body)
}

/// Dual-oracle ClassMethod setter pretty alias.
#[must_use]
pub fn class_method_setter_pretty(name: &str, param: &str, body: &str) -> String {
    continue89_class_method_setter_skeleton(name, param, body)
}

// ── ClassAccessorProperty dual-oracle ───────────────────────────────────────

/// Dual-oracle ClassAccessorProperty residual shell.
///
/// Base continue50 only typed the catalog; emit is `accessor {name} = {value};`.
#[must_use]
pub fn continue89_class_accessor_property_skeleton(name: &str, value: &str) -> String {
    format!("accessor {name} = {value};")
}

/// Dual-oracle ClassAccessorProperty pretty alias.
#[must_use]
pub fn class_accessor_property_pretty(name: &str, value: &str) -> String {
    continue89_class_accessor_property_skeleton(name, value)
}

/// Dual-oracle ClassAccessorProperty minify alias.
#[must_use]
pub fn class_accessor_property_minify(name: &str, value: &str) -> String {
    continue89_class_accessor_property_skeleton(name, value)
}

// ── TSParameterProperty dual-oracle ─────────────────────────────────────────

/// Dual-oracle TSParameterProperty skeleton composing real
/// [`continue50_ts_param_property_skeleton`].
#[must_use]
pub fn continue89_ts_parameter_property_skeleton(modif: &str, name: &str) -> String {
    continue50_ts_param_property_skeleton(modif, name)
}

/// Dual-oracle TSParameterProperty pretty alias.
#[must_use]
pub fn ts_parameter_property_pretty(modif: &str, name: &str) -> String {
    continue89_ts_parameter_property_skeleton(modif, name)
}

/// Dual-oracle TSParameterProperty minify alias.
#[must_use]
pub fn ts_parameter_property_minify(modif: &str, name: &str) -> String {
    continue89_ts_parameter_property_skeleton(modif, name)
}

// ── ExportDefaultSpecifier dual-oracle ──────────────────────────────────────

/// Dual-oracle ExportDefaultSpecifier skeleton composing real
/// [`continue49_export_default_specifier_skeleton`].
#[must_use]
pub fn continue89_export_default_specifier_skeleton(local: &str) -> String {
    continue49_export_default_specifier_skeleton(local)
}

/// Dual-oracle ExportDefaultSpecifier pretty alias.
#[must_use]
pub fn export_default_specifier_pretty(local: &str) -> String {
    continue89_export_default_specifier_skeleton(local)
}

/// Dual-oracle ExportDefaultSpecifier minify alias.
#[must_use]
pub fn export_default_specifier_minify(local: &str) -> String {
    continue89_export_default_specifier_skeleton(local)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: decorator stack join (pretty `\n`, minify empty).
#[must_use]
pub fn continue89_decorator_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n"
    } else {
        ""
    }
}

/// Dual-oracle residual: join decorator skeletons.
#[must_use]
pub fn continue89_join_decorators(decorators: &[&str], pretty: bool) -> String {
    let sep = continue89_decorator_sep(pretty);
    decorators.join(sep)
}

/// Dual-oracle residual: one or more decorators applied to a private property.
///
/// Pretty: each decorator on its own line, then the property.
/// Minify: decorators concatenated then property.
#[must_use]
pub fn continue89_decorated_private_property(
    decorators: &[&str],
    name: &str,
    value: &str,
    pretty: bool,
) -> String {
    let prop = continue89_class_private_property_skeleton(name, value);
    if decorators.is_empty() {
        return prop;
    }
    let decs: Vec<String> = decorators
        .iter()
        .map(|d| continue89_decorator_skeleton(d))
        .collect();
    let joined = continue89_join_decorators(
        &decs.iter().map(String::as_str).collect::<Vec<_>>(),
        pretty,
    );
    if pretty {
        format!("{joined}\n{prop}")
    } else {
        format!("{joined}{prop}")
    }
}

/// Dual-oracle residual: decorator applied to a private method.
#[must_use]
pub fn continue89_decorated_private_method(
    decorator: &str,
    name: &str,
    params: &str,
    body: &str,
    pretty: bool,
) -> String {
    let dec = continue89_decorator_skeleton(decorator);
    let method = continue89_class_private_method_skeleton(name, params, body);
    if pretty {
        format!("{dec}\n{method}")
    } else {
        format!("{dec}{method}")
    }
}

/// Dual-oracle residual: private field + getter + setter residual shell.
#[must_use]
pub fn continue89_private_field_with_accessors(
    field: &str,
    value: &str,
    accessor: &str,
    pretty: bool,
) -> String {
    let prop = continue89_class_private_property_skeleton(field, value);
    let get = continue89_class_method_getter_skeleton(accessor, &format!("return this.#{field};"));
    let set = continue89_class_method_setter_skeleton(
        accessor,
        "v",
        &format!("this.#{field} = v;"),
    );
    let sep = if pretty { "\n" } else { "" };
    format!("{prop}{sep}{get}{sep}{set}")
}

/// Dual-oracle residual: constructor param property list fragment.
#[must_use]
pub fn continue89_ts_param_list(params: &[(&str, &str)], pretty: bool) -> String {
    let sep = if pretty { ", " } else { "," };
    params
        .iter()
        .map(|(m, n)| continue89_ts_parameter_property_skeleton(m, n))
        .collect::<Vec<_>>()
        .join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue49_decorator_skeleton, continue49_export_default_specifier_skeleton,
        continue50_getter_skeleton, continue50_private_method_skeleton,
        continue50_private_property_skeleton, continue50_setter_skeleton,
        continue50_ts_param_property_skeleton,
    };

    #[test]
    fn continue89_type_catalog() {
        assert_eq!(CONTINUE89_RELATED_TYPES.len(), 7);
        assert!(is_decorator_private_related_type("Decorator"));
        assert!(is_decorator_private_related_type("ClassPrivateProperty"));
        assert!(is_decorator_private_related_type("ClassPrivateMethod"));
        assert!(is_decorator_private_related_type("ClassMethod"));
        assert!(is_decorator_private_related_type("ClassAccessorProperty"));
        assert!(is_decorator_private_related_type("TSParameterProperty"));
        assert!(is_decorator_private_related_type("ExportDefaultSpecifier"));
        assert!(!is_decorator_private_related_type("JSXElement"));
        assert!(!is_decorator_private_related_type("PropertyDefinition"));
        assert!(!is_decorator_private_related_type("ObjectMethod"));
        assert!(!is_decorator_private_related_type("ImportSpecifier"));
        assert!(is_continue89_decorator_type("Decorator"));
        assert!(is_continue89_class_private_property_type(
            "ClassPrivateProperty"
        ));
        assert!(is_continue89_class_private_method_type("ClassPrivateMethod"));
        assert!(is_continue89_class_method_type("ClassMethod"));
        assert!(is_continue89_class_accessor_property_type(
            "ClassAccessorProperty"
        ));
        assert!(is_continue89_ts_parameter_property_type(
            "TSParameterProperty"
        ));
        assert!(is_continue89_export_default_specifier_type(
            "ExportDefaultSpecifier"
        ));
        assert!(!is_continue89_decorator_type("ClassMethod"));
        assert!(!is_continue89_class_method_type("Decorator"));
    }

    #[test]
    fn continue89_decorator_private_property_method_dual_oracle() {
        assert_eq!(decorator_pretty("sealed"), "@sealed");
        assert_eq!(
            continue89_decorator_skeleton("sealed"),
            continue49_decorator_skeleton("sealed")
        );
        assert_eq!(decorator_minify("obs"), decorator_pretty("obs"));
        assert_ne!(
            continue89_decorator_skeleton("a"),
            continue89_decorator_skeleton("b")
        );

        assert_eq!(
            class_private_property_pretty("x", "1"),
            "#x = 1;"
        );
        assert_eq!(
            continue89_class_private_property_skeleton("x", "1"),
            continue50_private_property_skeleton("x", "1")
        );
        assert_ne!(
            class_private_property_pretty("x", "1"),
            class_private_property_pretty("y", "1")
        );

        assert_eq!(
            class_private_method_pretty("m", "a", "return a;"),
            "#m(a) { return a; }"
        );
        assert_eq!(
            continue89_class_private_method_skeleton("m", "a", "return a;"),
            continue50_private_method_skeleton("m", "a", "return a;")
        );
        assert_ne!(
            class_private_method_pretty("m", "", "return 1;"),
            class_private_property_pretty("m", "1")
        );
    }

    #[test]
    fn continue89_getter_setter_accessor_export_compose_dual_oracle() {
        assert_eq!(
            continue89_class_method_getter_skeleton("v", "return this.#v;"),
            continue50_getter_skeleton("v", "return this.#v;")
        );
        assert_eq!(
            class_method_getter_pretty("v", "return this.#v;"),
            "get v() { return this.#v; }"
        );
        assert_eq!(
            continue89_class_method_setter_skeleton("v", "n", "this.#v = n;"),
            continue50_setter_skeleton("v", "n", "this.#v = n;")
        );
        assert_eq!(
            class_method_setter_pretty("v", "n", "this.#v = n;"),
            "set v(n) { this.#v = n; }"
        );
        assert_ne!(
            class_method_getter_pretty("v", "return 1;"),
            class_method_setter_pretty("v", "n", "this.#v = n;")
        );

        assert_eq!(
            continue89_class_accessor_property_skeleton("count", "0"),
            "accessor count = 0;"
        );
        assert_eq!(
            class_accessor_property_pretty("count", "0"),
            class_accessor_property_minify("count", "0")
        );
        assert_ne!(
            class_accessor_property_pretty("a", "1"),
            class_private_property_pretty("a", "1")
        );

        assert_eq!(
            continue89_ts_parameter_property_skeleton("private", "id"),
            continue50_ts_param_property_skeleton("private", "id")
        );
        assert_eq!(
            ts_parameter_property_pretty("readonly", "name"),
            "readonly name"
        );
        assert_ne!(
            ts_parameter_property_pretty("private", "id"),
            ts_parameter_property_pretty("public", "id")
        );

        assert_eq!(
            continue89_export_default_specifier_skeleton("Foo"),
            continue49_export_default_specifier_skeleton("Foo")
        );
        assert_eq!(
            export_default_specifier_pretty("Foo"),
            "export default Foo;"
        );
        assert_ne!(
            export_default_specifier_pretty("Foo"),
            export_default_specifier_pretty("Bar")
        );

        assert_eq!(continue89_decorator_sep(true), "\n");
        assert_eq!(continue89_decorator_sep(false), "");
        assert_eq!(
            continue89_join_decorators(&["@a", "@b"], true),
            "@a\n@b"
        );
        assert_eq!(
            continue89_join_decorators(&["@a", "@b"], false),
            "@a@b"
        );

        let dec_prop = continue89_decorated_private_property(&["obs"], "x", "1", true);
        assert_eq!(dec_prop, "@obs\n#x = 1;");
        let dec_prop_min = continue89_decorated_private_property(&["obs"], "x", "1", false);
        assert_eq!(dec_prop_min, "@obs#x = 1;");
        assert_ne!(dec_prop, dec_prop_min);
        assert_eq!(
            continue89_decorated_private_property(&[], "x", "1", true),
            "#x = 1;"
        );

        let dec_method =
            continue89_decorated_private_method("bound", "run", "", "return 1;", true);
        assert_eq!(dec_method, "@bound\n#run() { return 1; }");
        assert_eq!(
            continue89_decorated_private_method("bound", "run", "", "return 1;", false),
            "@bound#run() { return 1; }"
        );

        let bundle = continue89_private_field_with_accessors("v", "0", "value", true);
        assert!(bundle.contains("#v = 0;"));
        assert!(bundle.contains("get value()"));
        assert!(bundle.contains("set value(v)"));
        assert!(bundle.contains('\n'));
        let bundle_min = continue89_private_field_with_accessors("v", "0", "value", false);
        assert!(!bundle_min.contains('\n'));
        assert_ne!(bundle, bundle_min);

        assert_eq!(
            continue89_ts_param_list(&[("private", "id"), ("readonly", "name")], true),
            "private id, readonly name"
        );
        assert_eq!(
            continue89_ts_param_list(&[("private", "id"), ("readonly", "name")], false),
            "private id,readonly name"
        );
        assert_ne!(
            continue89_ts_param_list(&[("private", "id")], true),
            continue89_ts_param_list(&[("public", "id")], true)
        );
    }
}

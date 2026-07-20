//! Pure PropertyDefinition / StaticBlock / ClassExpression residual —
//! residual pure continue20 for tooling/format-minify-lint fragment.
//! Complements continue13 export/class + continue18 private identifier.
//! Full engines remain product residual. intentional ts_only×3 retained.
//! NO authority_rust / ts_deleted.

use crate::class_emit_kind::{class_body_close, class_body_open, class_keyword};
use crate::export_class_emit::{class_body_interior, class_id_fragment};
use crate::format_emit::{async_prefix, generator_suffix, method_kind_prefix};
use crate::loop_switch_full_emit::private_identifier_token;

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_property_static_related_type(t: &str) -> bool {
    matches!(
        t,
        "PropertyDefinition"
            | "StaticBlock"
            | "ClassExpression"
            | "ClassBody"
            | "PrivateIdentifier"
            | "AccessorProperty"
    )
}

/// Whether node type is PropertyDefinition.
#[must_use]
pub fn is_property_definition_type(t: &str) -> bool {
    t == "PropertyDefinition"
}

/// Whether node type is StaticBlock.
#[must_use]
pub fn is_static_block_type(t: &str) -> bool {
    t == "StaticBlock"
}

/// Whether node type is ClassExpression (vs ClassDeclaration).
#[must_use]
pub fn is_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

/// Whether node type is AccessorProperty.
#[must_use]
pub fn is_accessor_property_type(t: &str) -> bool {
    t == "AccessorProperty"
}

/// `static ` prefix when field/method is static.
#[must_use]
pub fn static_prefix(is_static: bool) -> &'static str {
    if is_static {
        "static "
    } else {
        ""
    }
}

/// `#` private field key vs public key fragment.
#[must_use]
pub fn property_key_token(key: &str, is_private: bool) -> String {
    if is_private {
        private_identifier_token(key.trim_start_matches('#'))
    } else {
        key.to_owned()
    }
}

/// PropertyDefinition skeleton: `[static ][#]key[= value][;]`.
/// `value` is pre-rendered; `None` means bare field declaration.
#[must_use]
pub fn property_definition_skeleton(
    key: &str,
    value: Option<&str>,
    is_static: bool,
    is_private: bool,
    pretty: bool,
    semi: bool,
) -> String {
    let mut out = String::new();
    out.push_str(static_prefix(is_static));
    out.push_str(&property_key_token(key, is_private));
    if let Some(v) = value {
        if pretty {
            out.push_str(" = ");
        } else {
            out.push('=');
        }
        out.push_str(v);
    }
    if semi {
        out.push(';');
    }
    out
}

/// StaticBlock skeleton: `static { body }` / minify `static{body}`.
#[must_use]
pub fn static_block_skeleton(body_stmts: &[&str], pretty: bool) -> String {
    if body_stmts.is_empty() {
        return if pretty {
            "static {}".to_string()
        } else {
            "static{}".to_string()
        };
    }
    if pretty {
        format!("static {{ {} }}", body_stmts.join(" "))
    } else {
        format!("static{{{}}}", body_stmts.join(""))
    }
}

/// ClassExpression skeleton: `class [Name]{methods}` / pretty spaced body (no `export`).
#[must_use]
pub fn class_expression_skeleton(name: Option<&str>, methods: &[&str], pretty: bool) -> String {
    // class_id_fragment returns `"Name "` or `""`.
    let id = class_id_fragment(name);
    let mut out = String::from(class_keyword());
    out.push(' ');
    out.push_str(&id);
    out.push_str(class_body_open());
    out.push_str(&class_body_interior(methods, pretty));
    out.push_str(class_body_close());
    out
}

/// Method-like PropertyDefinition with kind/async/generator prefixes.
/// Complements MethodDefinition residual for field-initialized methods.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn class_method_like_field_skeleton(
    key: &str,
    params: &str,
    body: &str,
    kind: &str,
    is_static: bool,
    is_async: bool,
    is_generator: bool,
    pretty: bool,
) -> String {
    let mut out = String::new();
    out.push_str(static_prefix(is_static));
    out.push_str(async_prefix(is_async));
    out.push_str(method_kind_prefix(kind));
    out.push_str(generator_suffix(is_generator));
    out.push_str(key);
    if pretty {
        out.push_str(&format!("({params}) "));
    } else {
        out.push_str(&format!("({params})"));
    }
    out.push_str(body);
    out
}

/// Accessor property skeleton (`accessor [static ]key [= value]`).
#[must_use]
pub fn accessor_property_skeleton(
    key: &str,
    value: Option<&str>,
    is_static: bool,
    pretty: bool,
    semi: bool,
) -> String {
    let mut out = String::from("accessor ");
    out.push_str(static_prefix(is_static));
    out.push_str(key);
    if let Some(v) = value {
        if pretty {
            out.push_str(" = ");
        } else {
            out.push('=');
        }
        out.push_str(v);
    }
    if semi {
        out.push(';');
    }
    out
}

/// Computed property key brackets: `[expr]`.
#[must_use]
pub fn computed_property_key(expr: &str) -> String {
    format!("[{expr}]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_guards() {
        assert!(is_property_static_related_type("PropertyDefinition"));
        assert!(is_property_static_related_type("StaticBlock"));
        assert!(is_property_static_related_type("ClassExpression"));
        assert!(is_property_static_related_type("PrivateIdentifier"));
        assert!(is_property_definition_type("PropertyDefinition"));
        assert!(is_static_block_type("StaticBlock"));
        assert!(is_class_expression_type("ClassExpression"));
        assert!(is_accessor_property_type("AccessorProperty"));
        assert!(!is_property_static_related_type("ClassDeclaration"));
    }

    #[test]
    fn property_static_emit_dual_oracle() {
        assert_eq!(static_prefix(true), "static ");
        assert_eq!(static_prefix(false), "");
        assert_eq!(property_key_token("x", false), "x");
        assert_eq!(property_key_token("x", true), "#x");
        assert_eq!(property_key_token("#x", true), "#x");

        assert_eq!(
            property_definition_skeleton("count", Some("0"), false, false, true, true),
            "count = 0;"
        );
        assert_eq!(
            property_definition_skeleton("count", Some("0"), true, false, false, true),
            "static count=0;"
        );
        assert_eq!(
            property_definition_skeleton("id", None, false, true, true, true),
            "#id;"
        );

        assert_eq!(static_block_skeleton(&[], true), "static {}");
        assert_eq!(static_block_skeleton(&[], false), "static{}");
        assert_eq!(
            static_block_skeleton(&["this.x=1;"], false),
            "static{this.x=1;}"
        );
        assert_eq!(
            static_block_skeleton(&["a;", "b;"], true),
            "static { a; b; }"
        );

        let anon = class_expression_skeleton(None, &[], true);
        assert!(anon.starts_with("class"));
        assert!(anon.contains('{'));
        let named = class_expression_skeleton(Some("Foo"), &["x = 1;"], true);
        assert!(named.contains("Foo"));
        assert!(named.contains("x = 1;"));

        assert_eq!(computed_property_key("k"), "[k]");
        assert_eq!(
            accessor_property_skeleton("value", Some("1"), false, true, true),
            "accessor value = 1;"
        );

        let m = class_method_like_field_skeleton(
            "run",
            "",
            "{}",
            "method",
            true,
            true,
            false,
            true,
        );
        assert!(m.starts_with("static async "));
        assert!(m.contains("run()"));
    }
}

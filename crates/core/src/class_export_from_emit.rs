//! Pure Class-extends heritage + Export-from dual-oracle emission —
//! residual pure **continue71** for tooling/format-minify-lint fragment.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - ClassDeclaration/Expression with `extends Super` heritage
//! - ExportNamedDeclaration re-export: `export { a, b as c } from "mod"`
//! - ExportNamespaceSpecifier: `export * as ns from "mod"`
//! - ExportAllDeclaration dual-oracle (pretty quote vs minify double)
//! - Super computed member: `super[expr]`
//!
//! Intentionally does **not** re-wrap continue64–70 partition shells.
//! Composes real shipped pure helpers (`named_specifier_braces`,
//! `string_literal_token`, `class_keyword_space`, `class_body_skeleton`,
//! `export_keyword`, `from_keyword`).
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::export_class_emit::{
    class_body_skeleton, class_id_fragment, class_keyword_space, export_keyword,
};
use crate::ident_literal_emit::{named_specifier_braces, string_literal_token};
use crate::node_fallback::from_keyword;

/// Dual-oracle residual: continue71 related AST type catalog.
pub const CONTINUE71_RELATED_TYPES: &[&str] = &[
    "ClassDeclaration",
    "ClassExpression",
    "ClassBody",
    "ExportNamedDeclaration",
    "ExportAllDeclaration",
    "ExportNamespaceSpecifier",
    "ExportSpecifier",
    "Super",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_class_export_from_related_type(t: &str) -> bool {
    CONTINUE71_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue71_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue71_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue71_export_named_type(t: &str) -> bool {
    t == "ExportNamedDeclaration"
}

#[must_use]
pub fn is_continue71_export_all_type(t: &str) -> bool {
    t == "ExportAllDeclaration"
}

#[must_use]
pub fn is_continue71_export_namespace_specifier_type(t: &str) -> bool {
    t == "ExportNamespaceSpecifier"
}

#[must_use]
pub fn is_continue71_export_specifier_type(t: &str) -> bool {
    t == "ExportSpecifier"
}

#[must_use]
pub fn is_continue71_super_type(t: &str) -> bool {
    t == "Super"
}

/// `extends` glue before super id: pretty ` extends `, minify denser `extends`
/// when immediately following a name (caller owns name trailing space).
///
/// Dual-oracle residual: pretty keeps readable spacing; minify collapses
/// surrounding spaces when the preceding token already ends with space
/// (`class Foo ` → `class Foo extendsBar` is wrong) so minify still uses a
/// single leading space: ` extends` is not used — instead minify uses
/// `extends ` after the name trailing space from `class_id_fragment`.
#[must_use]
pub fn class_extends_token(pretty: bool) -> &'static str {
    if pretty {
        "extends "
    } else {
        "extends "
    }
}

/// Whether heritage (`extends Super`) should be emitted.
#[must_use]
pub fn wants_class_extends(super_name: Option<&str>) -> bool {
    matches!(super_name, Some(s) if !s.is_empty())
}

/// Class heritage fragment: `extends Super` or empty.
#[must_use]
pub fn class_extends_fragment(super_name: Option<&str>, pretty: bool) -> String {
    match super_name.filter(|s| !s.is_empty()) {
        Some(s) => format!("{}{s} ", class_extends_token(pretty)),
        None => String::new(),
    }
}

/// Dual-oracle ClassDeclaration/Expression skeleton with optional extends.
///
/// Pretty: `class Foo extends Bar {\nmethod\n}` via [`class_body_skeleton`].
/// Minify: denser method join, always double-space-free body interior.
///
/// Drives real shipped: [`class_keyword_space`], [`class_id_fragment`],
/// [`class_body_skeleton`].
#[must_use]
pub fn class_extends_skeleton(
    name: Option<&str>,
    super_name: Option<&str>,
    methods: &[&str],
    pretty: bool,
) -> String {
    let mut out = String::with_capacity(
        24 + name.map_or(0, str::len)
            + super_name.map_or(0, str::len)
            + methods.iter().map(|m| m.len()).sum::<usize>(),
    );
    out.push_str(class_keyword_space());
    out.push_str(&class_id_fragment(name));
    out.push_str(&class_extends_fragment(super_name, pretty));
    out.push_str(&class_body_skeleton(methods, pretty));
    out
}

/// Dual-oracle module source literal for export-from / import-from residual.
/// Pretty honors `single_quote`; minify always double-quotes (compressor).
///
/// Drives real shipped: [`string_literal_token`].
#[must_use]
pub fn export_from_source_token(source: &str, pretty: bool, single_quote: bool) -> String {
    string_literal_token(source, single_quote && pretty, pretty)
}

/// Dual-oracle ExportNamedDeclaration re-export:
/// `export { a, b as c } from "mod"` (+ optional semi).
///
/// Drives real shipped: [`export_keyword`], [`named_specifier_braces`],
/// [`from_keyword`], [`string_literal_token`].
#[must_use]
pub fn export_named_from_skeleton(
    specifiers: &[&str],
    source: &str,
    pretty: bool,
    semi: bool,
    single_quote: bool,
) -> String {
    let braces = named_specifier_braces(specifiers, pretty);
    let src = export_from_source_token(source, pretty, single_quote);
    let mut out = String::with_capacity(16 + braces.len() + src.len() + 8);
    out.push_str(export_keyword());
    out.push_str(&braces);
    // from_keyword(minify) returns bare `from` — keep single spaces around it
    // so tokens do not glue (`}from"` is invalid). Pretty uses ` from `.
    if pretty {
        out.push_str(from_keyword(true));
    } else {
        out.push(' ');
        out.push_str(from_keyword(false));
        out.push(' ');
    }
    out.push_str(&src);
    if semi {
        out.push(';');
    }
    out
}

/// Dual-oracle ExportNamespaceSpecifier re-export:
/// `export * as ns from "mod"` (+ optional semi).
#[must_use]
pub fn export_namespace_from_skeleton(
    namespace_local: &str,
    source: &str,
    pretty: bool,
    semi: bool,
    single_quote: bool,
) -> String {
    let src = export_from_source_token(source, pretty, single_quote);
    let mut out = String::with_capacity(24 + namespace_local.len() + src.len());
    out.push_str(export_keyword());
    out.push_str("* as ");
    out.push_str(namespace_local);
    if pretty {
        out.push_str(from_keyword(true));
    } else {
        out.push(' ');
        out.push_str(from_keyword(false));
        out.push(' ');
    }
    out.push_str(&src);
    if semi {
        out.push(';');
    }
    out
}

/// Dual-oracle ExportAllDeclaration: `export * from "mod"` (+ optional semi).
/// Widens the single-form continue47 fixed-quote skeleton with pretty/minify
/// quote policy + semi option — not a shell re-wrap of continue64 poles.
#[must_use]
pub fn export_all_from_skeleton(
    source: &str,
    pretty: bool,
    semi: bool,
    single_quote: bool,
) -> String {
    let src = export_from_source_token(source, pretty, single_quote);
    let mut out = String::with_capacity(16 + src.len());
    out.push_str(export_keyword());
    out.push('*');
    if pretty {
        out.push_str(from_keyword(true));
    } else {
        out.push(' ');
        out.push_str(from_keyword(false));
        out.push(' ');
    }
    out.push_str(&src);
    if semi {
        out.push(';');
    }
    out
}

/// Dual-oracle Super static member: `super.name`.
#[must_use]
pub fn super_static_member_skeleton(name: &str) -> String {
    format!("super.{name}")
}

/// Dual-oracle Super computed member: `super[expr]`.
/// New residual vs prior `super.foo` / `super()` shells.
#[must_use]
pub fn super_computed_member_skeleton(expr: &str) -> String {
    format!("super[{expr}]")
}

/// Dual-path Super member (computed vs static).
#[must_use]
pub fn super_member_skeleton(property: &str, computed: bool) -> String {
    if computed {
        super_computed_member_skeleton(property)
    } else {
        super_static_member_skeleton(property)
    }
}

/// Dual-oracle Super bare call: `super(args)` (args already joined).
#[must_use]
pub fn super_call_skeleton(args: &str) -> String {
    format!("super({args})")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export_class_emit::{
        class_body_skeleton, class_declaration_skeleton, class_id_fragment, class_keyword_space,
        export_keyword,
    };
    use crate::ident_literal_emit::{
        named_specifier_braces, named_specifier_list_interior, specifier_alias_fragment,
        string_literal_token,
    };
    use crate::node_fallback::from_keyword;

    #[test]
    fn continue71_type_catalog() {
        assert_eq!(CONTINUE71_RELATED_TYPES.len(), 8);
        assert!(is_class_export_from_related_type("ClassDeclaration"));
        assert!(is_class_export_from_related_type("ExportNamedDeclaration"));
        assert!(is_class_export_from_related_type("ExportAllDeclaration"));
        assert!(is_class_export_from_related_type("ExportNamespaceSpecifier"));
        assert!(is_class_export_from_related_type("Super"));
        assert!(!is_class_export_from_related_type("ImportDeclaration"));
        assert!(!is_class_export_from_related_type("BinaryExpression"));
        assert!(is_continue71_class_declaration_type("ClassDeclaration"));
        assert!(is_continue71_class_expression_type("ClassExpression"));
        assert!(is_continue71_export_named_type("ExportNamedDeclaration"));
        assert!(is_continue71_export_all_type("ExportAllDeclaration"));
        assert!(is_continue71_export_namespace_specifier_type(
            "ExportNamespaceSpecifier"
        ));
        assert!(is_continue71_export_specifier_type("ExportSpecifier"));
        assert!(is_continue71_super_type("Super"));
        assert!(!is_continue71_super_type("ThisExpression"));
    }

    #[test]
    fn continue71_class_extends_dual_oracle_drives_shipped() {
        // Real shipped class helpers remain authority for head/body.
        assert_eq!(class_keyword_space(), "class ");
        assert_eq!(class_id_fragment(Some("Foo")), "Foo ");
        assert_eq!(class_body_skeleton(&[], true), "{}");
        assert_eq!(
            class_declaration_skeleton(Some("A"), &[], true),
            "class A {}"
        );

        assert!(wants_class_extends(Some("Bar")));
        assert!(!wants_class_extends(Some("")));
        assert!(!wants_class_extends(None));
        assert_eq!(class_extends_fragment(None, true), "");
        assert_eq!(class_extends_fragment(Some("Bar"), true), "extends Bar ");
        assert_eq!(class_extends_fragment(Some("Bar"), false), "extends Bar ");

        // Pretty: empty body
        assert_eq!(
            class_extends_skeleton(Some("Foo"), Some("Bar"), &[], true),
            "class Foo extends Bar {}"
        );
        // Minify: empty body (same for empty methods; separator residual shows with methods)
        assert_eq!(
            class_extends_skeleton(Some("Foo"), Some("Bar"), &[], false),
            "class Foo extends Bar {}"
        );
        // No heritage falls back to plain class shape
        assert_eq!(
            class_extends_skeleton(Some("Solo"), None, &[], true),
            "class Solo {}"
        );
        // Methods: dual-oracle body join via class_body_skeleton
        assert_eq!(
            class_extends_skeleton(Some("C"), Some("B"), &["m() {}"], false),
            "class C extends B {m() {}}"
        );
        let pretty_multi =
            class_extends_skeleton(Some("C"), Some("B"), &["a() {}", "b() {}"], true);
        assert!(pretty_multi.starts_with("class C extends B {"));
        assert!(pretty_multi.contains("a() {}"));
        assert!(pretty_multi.contains("b() {}"));
        assert!(pretty_multi.ends_with('}'));
        // Anonymous class expression heritage
        assert_eq!(
            class_extends_skeleton(None, Some("Base"), &[], true),
            "class extends Base {}"
        );
    }

    #[test]
    fn continue71_export_named_from_dual_oracle_drives_shipped() {
        assert_eq!(export_keyword(), "export ");
        assert_eq!(from_keyword(true), " from ");
        assert_eq!(from_keyword(false), "from");

        // Specifier fragments from shipped helpers
        assert_eq!(specifier_alias_fragment("a", "a"), "a");
        assert_eq!(specifier_alias_fragment("a", "b"), "a as b");
        assert_eq!(named_specifier_braces(&["a", "b as c"], true), "{ a, b as c }");
        assert_eq!(named_specifier_braces(&["a", "b as c"], false), "{a,b as c}");
        assert_eq!(
            named_specifier_list_interior(&["x", "y"], false),
            "x,y"
        );

        // Pretty + single quotes
        assert_eq!(
            export_named_from_skeleton(&["a", "b as c"], "./m.js", true, true, true),
            "export { a, b as c } from './m.js';"
        );
        // Pretty + double quotes
        assert_eq!(
            export_named_from_skeleton(&["a"], "./m.js", true, true, false),
            "export { a } from \"./m.js\";"
        );
        // Minify always double-quotes module (compressor residual)
        assert_eq!(
            export_named_from_skeleton(&["a", "b as c"], "./m.js", false, true, true),
            "export {a,b as c} from \"./m.js\";"
        );
        // No semi
        assert_eq!(
            export_named_from_skeleton(&["x"], "lib", true, false, false),
            "export { x } from \"lib\""
        );
        // Empty specifier list still valid re-export shell
        assert_eq!(
            export_named_from_skeleton(&[], "empty", false, true, false),
            "export {} from \"empty\";"
        );

        // Source token drives string_literal_token
        assert_eq!(
            export_from_source_token("mod", true, true),
            string_literal_token("mod", true, true)
        );
        assert_eq!(
            export_from_source_token("mod", false, true),
            string_literal_token("mod", false, false)
        );
    }

    #[test]
    fn continue71_export_namespace_and_all_from_dual_oracle() {
        assert_eq!(
            export_namespace_from_skeleton("ns", "./m.js", true, true, true),
            "export * as ns from './m.js';"
        );
        assert_eq!(
            export_namespace_from_skeleton("ns", "./m.js", false, true, true),
            "export * as ns from \"./m.js\";"
        );
        assert_eq!(
            export_all_from_skeleton("./all.js", true, true, false),
            "export * from \"./all.js\";"
        );
        assert_eq!(
            export_all_from_skeleton("./all.js", true, true, true),
            "export * from './all.js';"
        );
        assert_eq!(
            export_all_from_skeleton("./all.js", false, true, true),
            "export * from \"./all.js\";"
        );
        assert_eq!(
            export_all_from_skeleton("x", false, false, false),
            "export * from \"x\""
        );
    }

    #[test]
    fn continue71_super_member_dual_oracle() {
        assert_eq!(super_static_member_skeleton("foo"), "super.foo");
        assert_eq!(super_computed_member_skeleton("k"), "super[k]");
        assert_eq!(super_computed_member_skeleton("\"x\""), "super[\"x\"]");
        assert_eq!(super_member_skeleton("bar", false), "super.bar");
        assert_eq!(super_member_skeleton("idx", true), "super[idx]");
        assert_eq!(super_call_skeleton(""), "super()");
        assert_eq!(super_call_skeleton("a, b"), "super(a, b)");
    }
}

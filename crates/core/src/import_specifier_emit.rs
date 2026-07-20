//! Pure ImportSpecifier + ImportDefaultSpecifier + ImportNamespaceSpecifier +
//! ExportSpecifier dual-oracle emission — residual pure **continue87** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–86:
//! - ImportSpecifier dual-oracle composing real
//!   `continue38_import_specifier_skeleton` + `specifier_alias_fragment`
//! - ImportDefaultSpecifier dual-oracle composing real
//!   `continue34_import_default_specifier_skeleton`
//! - ImportNamespaceSpecifier dual-oracle composing real
//!   `continue34_import_namespace_specifier_skeleton`
//! - ExportSpecifier dual-oracle composing real
//!   `continue38_export_specifier_skeleton`
//! - Named-specifier braces dual-oracle composing real
//!   `named_specifier_braces` (pretty vs minify)
//! - Composed full import declaration residual shells via
//!   `import_declaration_full_skeleton`
//!
//! Intentionally does **not** re-wrap continue64–86 partition shells
//! (ImportDeclaration continue81 stays separate for try/throw/import stmt
//! surface; export-from continue71 stays separate for `from` heritage forms;
//! function/class/this continue86 stays separate). Composes real shipped pure
//! helpers from import/export specifier bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::ident_literal_emit::{
    named_specifier_braces, specifier_alias_fragment, specifier_list_sep,
};
use crate::literal_widen_emit::{
    continue34_import_default_namespace_skeleton, continue34_import_default_specifier_skeleton,
    continue34_import_namespace_specifier_skeleton, continue38_export_specifier_skeleton,
    continue38_import_specifier_skeleton,
};
use crate::try_import_emit::import_declaration_full_skeleton;

/// Dual-oracle residual: continue87 related AST type catalog.
pub const CONTINUE87_RELATED_TYPES: &[&str] = &[
    "ImportSpecifier",
    "ImportDefaultSpecifier",
    "ImportNamespaceSpecifier",
    "ExportSpecifier",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_import_specifier_related_type(t: &str) -> bool {
    CONTINUE87_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue87_import_specifier_type(t: &str) -> bool {
    t == "ImportSpecifier"
}

#[must_use]
pub fn is_continue87_import_default_specifier_type(t: &str) -> bool {
    t == "ImportDefaultSpecifier"
}

#[must_use]
pub fn is_continue87_import_namespace_specifier_type(t: &str) -> bool {
    t == "ImportNamespaceSpecifier"
}

#[must_use]
pub fn is_continue87_export_specifier_type(t: &str) -> bool {
    t == "ExportSpecifier"
}

// ── ImportSpecifier dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ImportSpecifier skeleton composing real
/// [`continue38_import_specifier_skeleton`].
#[must_use]
pub fn continue87_import_specifier_skeleton(imported: &str, local: Option<&str>) -> String {
    continue38_import_specifier_skeleton(imported, local)
}

/// Dual-oracle ImportSpecifier via alias fragment helper (same surface).
#[must_use]
pub fn continue87_import_specifier_alias(imported: &str, local: &str) -> String {
    specifier_alias_fragment(imported, local)
}

/// Convenience pretty bare import specifier.
#[must_use]
pub fn import_specifier_bare(imported: &str) -> String {
    continue87_import_specifier_skeleton(imported, None)
}

/// Convenience aliased import specifier.
#[must_use]
pub fn import_specifier_aliased(imported: &str, local: &str) -> String {
    continue87_import_specifier_skeleton(imported, Some(local))
}

// ── ImportDefaultSpecifier dual-oracle ──────────────────────────────────────

/// Dual-oracle ImportDefaultSpecifier skeleton composing real
/// [`continue34_import_default_specifier_skeleton`].
#[must_use]
pub fn continue87_import_default_specifier_skeleton(local: &str) -> String {
    continue34_import_default_specifier_skeleton(local)
}

/// Convenience alias.
#[must_use]
pub fn import_default_specifier(local: &str) -> String {
    continue87_import_default_specifier_skeleton(local)
}

// ── ImportNamespaceSpecifier dual-oracle ────────────────────────────────────

/// Dual-oracle ImportNamespaceSpecifier skeleton composing real
/// [`continue34_import_namespace_specifier_skeleton`].
#[must_use]
pub fn continue87_import_namespace_specifier_skeleton(local: &str) -> String {
    continue34_import_namespace_specifier_skeleton(local)
}

/// Convenience alias.
#[must_use]
pub fn import_namespace_specifier(local: &str) -> String {
    continue87_import_namespace_specifier_skeleton(local)
}

// ── ExportSpecifier dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ExportSpecifier skeleton composing real
/// [`continue38_export_specifier_skeleton`].
#[must_use]
pub fn continue87_export_specifier_skeleton(local: &str, exported: Option<&str>) -> String {
    continue38_export_specifier_skeleton(local, exported)
}

/// Convenience bare export specifier.
#[must_use]
pub fn export_specifier_bare(local: &str) -> String {
    continue87_export_specifier_skeleton(local, None)
}

/// Convenience aliased export specifier.
#[must_use]
pub fn export_specifier_aliased(local: &str, exported: &str) -> String {
    continue87_export_specifier_skeleton(local, Some(exported))
}

// ── Named braces + full import declaration dual-oracle ──────────────────────

/// Dual-oracle named-specifier braces composing real [`named_specifier_braces`].
#[must_use]
pub fn continue87_named_specifier_braces(specifiers: &[&str], pretty: bool) -> String {
    named_specifier_braces(specifiers, pretty)
}

/// Dual-oracle full import declaration composing real
/// [`import_declaration_full_skeleton`].
#[must_use]
pub fn continue87_import_declaration_skeleton(
    default_local: Option<&str>,
    namespace_local: Option<&str>,
    named: &[&str],
    source: &str,
    pretty: bool,
    semi: bool,
    single_quote: bool,
) -> String {
    import_declaration_full_skeleton(
        default_local,
        namespace_local,
        named,
        source,
        pretty,
        semi,
        single_quote,
    )
}

/// Dual-oracle residual: default + namespace combined shell.
#[must_use]
pub fn continue87_import_default_namespace(default_local: &str, ns_local: &str, source: &str) -> String {
    continue34_import_default_namespace_skeleton(default_local, ns_local, source)
}

/// Dual-oracle residual: named-only import pretty/minify poles.
#[must_use]
pub fn continue87_import_named_only(named: &[&str], source: &str, pretty: bool) -> String {
    continue87_import_declaration_skeleton(None, None, named, source, pretty, true, true)
}

/// Dual-oracle residual: side-effect import shell.
#[must_use]
pub fn continue87_import_side_effect(source: &str, pretty: bool) -> String {
    continue87_import_declaration_skeleton(None, None, &[], source, pretty, true, false)
}

/// Dual-oracle residual: composed default + named import.
#[must_use]
pub fn continue87_import_default_named(
    default_local: &str,
    named: &[&str],
    source: &str,
    pretty: bool,
) -> String {
    continue87_import_declaration_skeleton(
        Some(default_local),
        None,
        named,
        source,
        pretty,
        true,
        true,
    )
}

/// Dual-oracle residual: specifier list separator poles.
#[must_use]
pub fn continue87_specifier_list_sep(pretty: bool) -> &'static str {
    specifier_list_sep(pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ident_literal_emit::{named_specifier_braces, specifier_alias_fragment};
    use crate::literal_widen_emit::{
        continue34_import_default_specifier_skeleton, continue34_import_namespace_specifier_skeleton,
        continue38_export_specifier_skeleton, continue38_import_specifier_skeleton,
    };
    use crate::try_import_emit::import_declaration_full_skeleton;

    #[test]
    fn continue87_type_catalog() {
        assert_eq!(CONTINUE87_RELATED_TYPES.len(), 4);
        assert!(is_import_specifier_related_type("ImportSpecifier"));
        assert!(is_import_specifier_related_type("ImportDefaultSpecifier"));
        assert!(is_import_specifier_related_type("ImportNamespaceSpecifier"));
        assert!(is_import_specifier_related_type("ExportSpecifier"));
        assert!(!is_import_specifier_related_type("ImportDeclaration"));
        assert!(!is_import_specifier_related_type("ExportNamedDeclaration"));
        assert!(!is_import_specifier_related_type("FunctionDeclaration"));
        assert!(!is_import_specifier_related_type("ExportAllDeclaration"));
        assert!(is_continue87_import_specifier_type("ImportSpecifier"));
        assert!(is_continue87_import_default_specifier_type(
            "ImportDefaultSpecifier"
        ));
        assert!(is_continue87_import_namespace_specifier_type(
            "ImportNamespaceSpecifier"
        ));
        assert!(is_continue87_export_specifier_type("ExportSpecifier"));
        assert!(!is_continue87_import_specifier_type("ExportSpecifier"));
        assert!(!is_continue87_export_specifier_type("ImportSpecifier"));
        assert!(!is_continue87_import_default_specifier_type(
            "ImportNamespaceSpecifier"
        ));
    }

    #[test]
    fn continue87_specifier_dual_oracle() {
        assert_eq!(import_specifier_bare("useState"), "useState");
        assert_eq!(import_specifier_aliased("useState", "us"), "useState as us");
        assert_eq!(
            continue87_import_specifier_skeleton("foo", None),
            continue38_import_specifier_skeleton("foo", None)
        );
        assert_eq!(
            continue87_import_specifier_skeleton("foo", Some("bar")),
            continue38_import_specifier_skeleton("foo", Some("bar"))
        );
        assert_eq!(
            continue87_import_specifier_alias("a", "b"),
            specifier_alias_fragment("a", "b")
        );
        assert_eq!(continue87_import_specifier_alias("a", "a"), "a");
        assert_ne!(
            import_specifier_bare("x"),
            import_specifier_aliased("x", "y")
        );

        assert_eq!(import_default_specifier("React"), "React");
        assert_eq!(
            continue87_import_default_specifier_skeleton("React"),
            continue34_import_default_specifier_skeleton("React")
        );

        assert_eq!(import_namespace_specifier("ns"), "* as ns");
        assert_eq!(
            continue87_import_namespace_specifier_skeleton("fs"),
            continue34_import_namespace_specifier_skeleton("fs")
        );
        assert_ne!(
            import_default_specifier("x"),
            import_namespace_specifier("x")
        );

        assert_eq!(export_specifier_bare("a"), "a");
        assert_eq!(export_specifier_aliased("a", "b"), "a as b");
        assert_eq!(
            continue87_export_specifier_skeleton("local", Some("exported")),
            continue38_export_specifier_skeleton("local", Some("exported"))
        );
        assert_eq!(
            continue87_export_specifier_skeleton("same", Some("same")),
            "same"
        );
        assert_ne!(export_specifier_bare("z"), export_specifier_aliased("z", "w"));
    }

    #[test]
    fn continue87_import_compose_dual_oracle() {
        assert_eq!(continue87_specifier_list_sep(true), ", ");
        assert_eq!(continue87_specifier_list_sep(false), ",");
        assert_ne!(
            continue87_specifier_list_sep(true),
            continue87_specifier_list_sep(false)
        );

        let braces_pretty = continue87_named_specifier_braces(&["a", "b as c"], true);
        let braces_mini = continue87_named_specifier_braces(&["a", "b as c"], false);
        assert_eq!(braces_pretty, named_specifier_braces(&["a", "b as c"], true));
        assert_eq!(braces_mini, named_specifier_braces(&["a", "b as c"], false));
        assert_eq!(braces_pretty, "{ a, b as c }");
        assert_eq!(braces_mini, "{a,b as c}");
        assert_ne!(braces_pretty, braces_mini);

        let named_pretty = continue87_import_named_only(&["useState", "useMemo as um"], "react", true);
        let named_mini = continue87_import_named_only(&["useState", "useMemo as um"], "react", false);
        assert_eq!(
            named_pretty,
            import_declaration_full_skeleton(
                None,
                None,
                &["useState", "useMemo as um"],
                "react",
                true,
                true,
                true
            )
        );
        assert!(named_pretty.contains("import"));
        assert!(named_pretty.contains("from"));
        assert_ne!(named_pretty, named_mini);

        let mixed = continue87_import_default_named(
            "React",
            &["useState"],
            "react",
            true,
        );
        assert_eq!(
            mixed,
            "import React, { useState } from 'react';"
        );
        assert!(mixed.contains("React"));
        assert!(mixed.contains("useState"));

        let side = continue87_import_side_effect("side-effect", true);
        assert_eq!(side, "import \"side-effect\";");

        let def_ns = continue87_import_default_namespace("Def", "NS", "\"./m.js\"");
        assert_eq!(def_ns, "import Def, * as NS from \"./m.js\";");

        let ns_only = continue87_import_declaration_skeleton(
            None,
            Some("fs"),
            &[],
            "node:fs",
            false,
            true,
            false,
        );
        assert_eq!(ns_only, "import * as fs from \"node:fs\";");
        assert_eq!(
            ns_only,
            import_declaration_full_skeleton(None, Some("fs"), &[], "node:fs", false, true, false)
        );
    }
}

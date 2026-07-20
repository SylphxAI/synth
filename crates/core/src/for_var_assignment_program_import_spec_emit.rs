//! Pure ForStatement + VariableDeclaration + VariableDeclarator +
//! AssignmentPattern + Program + ImportDefaultSpecifier +
//! ImportNamespaceSpecifier dual-oracle emission — residual pure
//! **continue101** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–100:
//! - ForStatement dual-oracle composing real `continue34_for_skeleton`
//! - VariableDeclaration dual-oracle composing real
//!   `continue34_variable_declaration_skeleton`
//! - VariableDeclarator dual-oracle composing real
//!   `continue34_variable_declarator_skeleton`
//! - AssignmentPattern dual-oracle composing real
//!   `continue34_assignment_pattern_skeleton`
//! - Program dual-oracle composing real `continue34_program_skeleton`
//! - ImportDefaultSpecifier dual-oracle composing real
//!   `continue34_import_default_specifier_skeleton`
//! - ImportNamespaceSpecifier dual-oracle composing real
//!   `continue34_import_namespace_specifier_skeleton`
//! - Composed for/var/assignment/program/import-spec residual shells
//!
//! Intentionally does **not** re-wrap continue79 var/return/if poles,
//! continue80 for/while/switch poles, continue85 program poles, continue87
//! import-specifier poles, or continue100 meta/import/chain poles. Composes
//! real shipped pure helpers from continue34 bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue34_assignment_pattern_skeleton, continue34_for_skeleton,
    continue34_import_default_namespace_skeleton, continue34_import_default_specifier_skeleton,
    continue34_import_namespace_specifier_skeleton, continue34_program_skeleton,
    continue34_variable_declaration_skeleton, continue34_variable_declarator_skeleton,
};

/// Dual-oracle residual: continue101 related AST type catalog.
pub const CONTINUE101_RELATED_TYPES: &[&str] = &[
    "ForStatement",
    "VariableDeclaration",
    "VariableDeclarator",
    "AssignmentPattern",
    "Program",
    "ImportDefaultSpecifier",
    "ImportNamespaceSpecifier",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_for_var_assignment_program_import_spec_related_type(t: &str) -> bool {
    CONTINUE101_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue101_for_statement_type(t: &str) -> bool {
    t == "ForStatement"
}

#[must_use]
pub fn is_continue101_variable_declaration_type(t: &str) -> bool {
    t == "VariableDeclaration"
}

#[must_use]
pub fn is_continue101_variable_declarator_type(t: &str) -> bool {
    t == "VariableDeclarator"
}

#[must_use]
pub fn is_continue101_assignment_pattern_type(t: &str) -> bool {
    t == "AssignmentPattern"
}

#[must_use]
pub fn is_continue101_program_type(t: &str) -> bool {
    t == "Program"
}

#[must_use]
pub fn is_continue101_import_default_specifier_type(t: &str) -> bool {
    t == "ImportDefaultSpecifier"
}

#[must_use]
pub fn is_continue101_import_namespace_specifier_type(t: &str) -> bool {
    t == "ImportNamespaceSpecifier"
}

#[must_use]
pub fn is_continue101_var_family_type(t: &str) -> bool {
    matches!(
        t,
        "VariableDeclaration" | "VariableDeclarator" | "AssignmentPattern"
    )
}

#[must_use]
pub fn is_continue101_import_spec_family_type(t: &str) -> bool {
    matches!(t, "ImportDefaultSpecifier" | "ImportNamespaceSpecifier")
}

// ── ForStatement dual-oracle ────────────────────────────────────────────────

/// Dual-oracle ForStatement skeleton composing real [`continue34_for_skeleton`].
#[must_use]
pub fn continue101_for_skeleton(init: &str, test: &str, update: &str, body: &str) -> String {
    continue34_for_skeleton(init, test, update, body)
}

/// Dual-oracle ForStatement pretty alias.
#[must_use]
pub fn continue101_for_pretty(init: &str, test: &str, update: &str, body: &str) -> String {
    continue101_for_skeleton(init, test, update, body)
}

/// Dual-oracle ForStatement minify alias.
#[must_use]
pub fn continue101_for_minify(init: &str, test: &str, update: &str, body: &str) -> String {
    continue101_for_skeleton(init, test, update, body)
}

// ── VariableDeclaration dual-oracle ─────────────────────────────────────────

/// Dual-oracle VariableDeclaration skeleton composing real
/// [`continue34_variable_declaration_skeleton`].
#[must_use]
pub fn continue101_variable_declaration_skeleton(kind: &str, decls: &str) -> String {
    continue34_variable_declaration_skeleton(kind, decls)
}

/// Dual-oracle VariableDeclaration pretty alias.
#[must_use]
pub fn continue101_variable_declaration_pretty(kind: &str, decls: &str) -> String {
    continue101_variable_declaration_skeleton(kind, decls)
}

/// Dual-oracle VariableDeclaration minify alias.
#[must_use]
pub fn continue101_variable_declaration_minify(kind: &str, decls: &str) -> String {
    continue101_variable_declaration_skeleton(kind, decls)
}

// ── VariableDeclarator dual-oracle ──────────────────────────────────────────

/// Dual-oracle VariableDeclarator skeleton composing real
/// [`continue34_variable_declarator_skeleton`].
#[must_use]
pub fn continue101_variable_declarator_skeleton(id: &str, init: Option<&str>) -> String {
    continue34_variable_declarator_skeleton(id, init)
}

/// Dual-oracle VariableDeclarator with initializer.
#[must_use]
pub fn continue101_variable_declarator(id: &str, init: &str) -> String {
    continue101_variable_declarator_skeleton(id, Some(init))
}

/// Dual-oracle bare VariableDeclarator (no initializer).
#[must_use]
pub fn continue101_variable_declarator_bare(id: &str) -> String {
    continue101_variable_declarator_skeleton(id, None)
}

/// Dual-oracle VariableDeclarator pretty alias.
#[must_use]
pub fn continue101_variable_declarator_pretty(id: &str, init: Option<&str>) -> String {
    continue101_variable_declarator_skeleton(id, init)
}

/// Dual-oracle VariableDeclarator minify alias.
#[must_use]
pub fn continue101_variable_declarator_minify(id: &str, init: Option<&str>) -> String {
    continue101_variable_declarator_skeleton(id, init)
}

// ── AssignmentPattern dual-oracle ───────────────────────────────────────────

/// Dual-oracle AssignmentPattern skeleton composing real
/// [`continue34_assignment_pattern_skeleton`].
#[must_use]
pub fn continue101_assignment_pattern_skeleton(left: &str, right: &str) -> String {
    continue34_assignment_pattern_skeleton(left, right)
}

/// Dual-oracle AssignmentPattern pretty alias.
#[must_use]
pub fn continue101_assignment_pattern_pretty(left: &str, right: &str) -> String {
    continue101_assignment_pattern_skeleton(left, right)
}

/// Dual-oracle AssignmentPattern minify alias.
#[must_use]
pub fn continue101_assignment_pattern_minify(left: &str, right: &str) -> String {
    continue101_assignment_pattern_skeleton(left, right)
}

// ── Program dual-oracle ─────────────────────────────────────────────────────

/// Dual-oracle Program skeleton composing real [`continue34_program_skeleton`].
#[must_use]
pub fn continue101_program_skeleton(body: &str) -> String {
    continue34_program_skeleton(body)
}

/// Dual-oracle Program pretty alias.
#[must_use]
pub fn continue101_program_pretty(body: &str) -> String {
    continue101_program_skeleton(body)
}

/// Dual-oracle Program minify alias.
#[must_use]
pub fn continue101_program_minify(body: &str) -> String {
    continue101_program_skeleton(body)
}

// ── ImportDefaultSpecifier dual-oracle ──────────────────────────────────────

/// Dual-oracle ImportDefaultSpecifier skeleton composing real
/// [`continue34_import_default_specifier_skeleton`].
#[must_use]
pub fn continue101_import_default_specifier_skeleton(local: &str) -> String {
    continue34_import_default_specifier_skeleton(local)
}

/// Dual-oracle ImportDefaultSpecifier pretty alias.
#[must_use]
pub fn continue101_import_default_specifier_pretty(local: &str) -> String {
    continue101_import_default_specifier_skeleton(local)
}

/// Dual-oracle ImportDefaultSpecifier minify alias.
#[must_use]
pub fn continue101_import_default_specifier_minify(local: &str) -> String {
    continue101_import_default_specifier_skeleton(local)
}

// ── ImportNamespaceSpecifier dual-oracle ────────────────────────────────────

/// Dual-oracle ImportNamespaceSpecifier skeleton composing real
/// [`continue34_import_namespace_specifier_skeleton`].
#[must_use]
pub fn continue101_import_namespace_specifier_skeleton(local: &str) -> String {
    continue34_import_namespace_specifier_skeleton(local)
}

/// Dual-oracle ImportNamespaceSpecifier pretty alias.
#[must_use]
pub fn continue101_import_namespace_specifier_pretty(local: &str) -> String {
    continue101_import_namespace_specifier_skeleton(local)
}

/// Dual-oracle ImportNamespaceSpecifier minify alias.
#[must_use]
pub fn continue101_import_namespace_specifier_minify(local: &str) -> String {
    continue101_import_namespace_specifier_skeleton(local)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: `const x = 1;` via declaration skeleton.
#[must_use]
pub fn continue101_const_init(id: &str, init: &str) -> String {
    let decl = continue101_variable_declarator(id, init);
    continue101_variable_declaration_skeleton("const", &decl)
}

/// Dual-oracle residual: `let a = b = 0` style for-init with assignment pattern.
#[must_use]
pub fn continue101_for_with_var(
    kind: &str,
    id: &str,
    init: &str,
    test: &str,
    update: &str,
    body: &str,
) -> String {
    let decl = continue101_variable_declarator(id, init);
    let var = continue101_variable_declaration_skeleton(kind, &decl);
    // strip trailing `;` for for-init slot (declaration skeleton ends with `;`)
    let init_slot = var.trim_end_matches(';');
    continue101_for_skeleton(init_slot, test, update, body)
}

/// Dual-oracle residual: default param-like assignment pattern in declarator.
#[must_use]
pub fn continue101_declarator_with_default(id: &str, default: &str, value: &str) -> String {
    let pat = continue101_assignment_pattern_skeleton(id, default);
    continue101_variable_declarator(&pat, value)
}

/// Dual-oracle residual: program of one const declaration.
#[must_use]
pub fn continue101_program_const(id: &str, init: &str) -> String {
    let stmt = continue101_const_init(id, init);
    continue101_program_skeleton(&format!("{stmt}\n"))
}

/// Dual-oracle residual: `import Def, * as NS from "src";` combined shell.
#[must_use]
pub fn continue101_import_default_namespace(default_local: &str, ns_local: &str, source: &str) -> String {
    continue34_import_default_namespace_skeleton(default_local, ns_local, source)
}

/// Dual-oracle residual: default + namespace locals composed from pure specs.
#[must_use]
pub fn continue101_import_specs_pair(default_local: &str, ns_local: &str) -> String {
    format!(
        "{}, {}",
        continue101_import_default_specifier_skeleton(default_local),
        continue101_import_namespace_specifier_skeleton(ns_local)
    )
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue101_stmt_sep(pretty: bool) -> &'static str {
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
        continue34_assignment_pattern_skeleton, continue34_for_skeleton,
        continue34_import_default_specifier_skeleton, continue34_import_namespace_specifier_skeleton,
        continue34_program_skeleton, continue34_variable_declaration_skeleton,
        continue34_variable_declarator_skeleton,
    };

    #[test]
    fn continue101_type_catalog() {
        assert_eq!(CONTINUE101_RELATED_TYPES.len(), 7);
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "ForStatement"
        ));
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "VariableDeclaration"
        ));
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "VariableDeclarator"
        ));
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "AssignmentPattern"
        ));
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "Program"
        ));
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "ImportDefaultSpecifier"
        ));
        assert!(is_for_var_assignment_program_import_spec_related_type(
            "ImportNamespaceSpecifier"
        ));
        assert!(!is_for_var_assignment_program_import_spec_related_type(
            "MetaProperty"
        ));
        assert!(!is_for_var_assignment_program_import_spec_related_type(
            "SwitchStatement"
        ));

        assert!(is_continue101_for_statement_type("ForStatement"));
        assert!(!is_continue101_for_statement_type("WhileStatement"));
        assert!(is_continue101_variable_declaration_type(
            "VariableDeclaration"
        ));
        assert!(is_continue101_variable_declarator_type(
            "VariableDeclarator"
        ));
        assert!(is_continue101_assignment_pattern_type("AssignmentPattern"));
        assert!(is_continue101_program_type("Program"));
        assert!(is_continue101_import_default_specifier_type(
            "ImportDefaultSpecifier"
        ));
        assert!(is_continue101_import_namespace_specifier_type(
            "ImportNamespaceSpecifier"
        ));
        assert!(is_continue101_var_family_type("VariableDeclaration"));
        assert!(is_continue101_var_family_type("AssignmentPattern"));
        assert!(!is_continue101_var_family_type("Program"));
        assert!(is_continue101_import_spec_family_type(
            "ImportDefaultSpecifier"
        ));
        assert!(is_continue101_import_spec_family_type(
            "ImportNamespaceSpecifier"
        ));
        assert!(!is_continue101_import_spec_family_type("Program"));
    }

    #[test]
    fn continue101_for_var_assignment_emit() {
        assert_eq!(
            continue101_for_skeleton("let i = 0", "i < 3", "i++", "{}"),
            "for (let i = 0; i < 3; i++) {}"
        );
        assert_eq!(
            continue101_for_skeleton("let i = 0", "i < 3", "i++", "{}"),
            continue34_for_skeleton("let i = 0", "i < 3", "i++", "{}")
        );
        assert_eq!(
            continue101_for_pretty("a", "b", "c", "d"),
            continue101_for_minify("a", "b", "c", "d")
        );

        assert_eq!(
            continue101_variable_declaration_skeleton("const", "x = 1"),
            "const x = 1;"
        );
        assert_eq!(
            continue101_variable_declaration_skeleton("const", "x = 1"),
            continue34_variable_declaration_skeleton("const", "x = 1")
        );
        assert_eq!(
            continue101_variable_declaration_pretty("let", "a, b"),
            continue101_variable_declaration_minify("let", "a, b")
        );

        assert_eq!(
            continue101_variable_declarator_skeleton("x", Some("1")),
            "x = 1"
        );
        assert_eq!(
            continue101_variable_declarator_skeleton("x", Some("1")),
            continue34_variable_declarator_skeleton("x", Some("1"))
        );
        assert_eq!(continue101_variable_declarator("y", "2"), "y = 2");
        assert_eq!(continue101_variable_declarator_bare("z"), "z");
        assert_eq!(
            continue101_variable_declarator_skeleton("z", None),
            continue34_variable_declarator_skeleton("z", None)
        );
        assert_eq!(
            continue101_variable_declarator_pretty("a", Some("b")),
            continue101_variable_declarator_minify("a", Some("b"))
        );

        assert_eq!(
            continue101_assignment_pattern_skeleton("x", "0"),
            "x = 0"
        );
        assert_eq!(
            continue101_assignment_pattern_skeleton("x", "0"),
            continue34_assignment_pattern_skeleton("x", "0")
        );
        assert_eq!(
            continue101_assignment_pattern_pretty("a", "b"),
            continue101_assignment_pattern_minify("a", "b")
        );
    }

    #[test]
    fn continue101_program_import_spec_emit() {
        assert_eq!(
            continue101_program_skeleton("const x = 1;\n"),
            "const x = 1;\n"
        );
        assert_eq!(
            continue101_program_skeleton("const x = 1;\n"),
            continue34_program_skeleton("const x = 1;\n")
        );
        assert_eq!(
            continue101_program_pretty("body"),
            continue101_program_minify("body")
        );

        assert_eq!(
            continue101_import_default_specifier_skeleton("React"),
            "React"
        );
        assert_eq!(
            continue101_import_default_specifier_skeleton("React"),
            continue34_import_default_specifier_skeleton("React")
        );
        assert_eq!(
            continue101_import_default_specifier_pretty("A"),
            continue101_import_default_specifier_minify("A")
        );

        assert_eq!(
            continue101_import_namespace_specifier_skeleton("ns"),
            "* as ns"
        );
        assert_eq!(
            continue101_import_namespace_specifier_skeleton("ns"),
            continue34_import_namespace_specifier_skeleton("ns")
        );
        assert_eq!(
            continue101_import_namespace_specifier_pretty("m"),
            continue101_import_namespace_specifier_minify("m")
        );
    }

    #[test]
    fn continue101_composed_residual_shells() {
        assert_eq!(continue101_const_init("x", "1"), "const x = 1;");
        assert_eq!(
            continue101_for_with_var("let", "i", "0", "i < 2", "i++", "{}"),
            "for (let i = 0; i < 2; i++) {}"
        );
        assert_eq!(
            continue101_declarator_with_default("x", "0", "y"),
            "x = 0 = y"
        );
        assert_eq!(
            continue101_program_const("n", "42"),
            "const n = 42;\n"
        );
        assert_eq!(
            continue101_import_default_namespace("React", "ReactAll", "\"react\""),
            "import React, * as ReactAll from \"react\";"
        );
        assert_eq!(
            continue101_import_specs_pair("Def", "NS"),
            "Def, * as NS"
        );
        assert_eq!(continue101_stmt_sep(true), " ");
        assert_eq!(continue101_stmt_sep(false), "");
        assert_ne!(continue101_stmt_sep(true), continue101_stmt_sep(false));
    }
}

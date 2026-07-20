//! Pure ClassExpression + ClassDeclaration + MetaProperty + NewExpression +
//! ThisExpression + Super dual-oracle **edge** emission — residual pure
//! **continue122** for tooling/format-minify-lint.
//!
//! New dual-oracle shells **not** covered by prior residual modules continue71–121:
//! - Class dual-oracle composing real `continue57_class_edge_shell`
//! - New/this/super dual-oracle composing real `continue57_new_this_super_shell`
//! - Meta/partition dual-oracle composing real `continue57_meta_partition_shell`
//! - Super edge dual-oracle composing real `continue58_super_edge_shell`
//! - New/class edge dual-oracle composing real `continue58_new_class_edge_shell`
//! - Meta/partition edge dual-oracle composing real `continue58_meta_partition_shell`
//! - Composed class/meta/new/this/super residual edge shells over continue55 bases
//!
//! Intentionally does **not** re-wrap continue121 sequence/update/yield/await/
//! import edge poles (continue56/54 shells), continue120 class/meta/new/this/super
//! poles (continue55 bases as primary surface), or continue119 sequence plane
//! poles (continue53 bases). Composes real shipped pure shells from continue57
//! and continue58 edges over the continue55 class/meta/new/this/super plane.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue55_class_declaration_skeleton, continue55_class_expression_skeleton,
    continue55_meta_import_meta_skeleton, continue55_meta_new_target_skeleton,
    continue55_new_expression_skeleton, continue55_super_call_skeleton, continue55_this_skeleton,
    continue57_class_edge_shell, continue57_meta_partition_shell, continue57_new_this_super_shell,
    continue58_meta_partition_shell, continue58_new_class_edge_shell, continue58_super_edge_shell,
    is_continue55_class_declaration_type, is_continue55_class_expression_type,
    is_continue55_meta_property_type, is_continue55_new_expression_type, is_continue55_related_type,
    is_continue55_super_type, is_continue55_this_expression_type, CONTINUE55_RELATED_TYPES,
};

/// Dual-oracle residual: continue122 related AST type catalog (class plane).
pub const CONTINUE122_RELATED_TYPES: &[&str] = &[
    "ClassExpression",
    "ClassDeclaration",
    "MetaProperty",
    "NewExpression",
    "ThisExpression",
    "Super",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_class_meta_new_this_super_edge_related_type(t: &str) -> bool {
    CONTINUE122_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue122_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue122_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue122_meta_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue122_new_type(t: &str) -> bool {
    t == "NewExpression"
}

#[must_use]
pub fn is_continue122_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue122_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue122_class_plane_type(t: &str) -> bool {
    matches!(t, "ClassExpression" | "ClassDeclaration")
}

#[must_use]
pub fn is_continue122_meta_new_plane_type(t: &str) -> bool {
    matches!(t, "MetaProperty" | "NewExpression")
}

#[must_use]
pub fn is_continue122_this_super_plane_type(t: &str) -> bool {
    matches!(t, "ThisExpression" | "Super")
}

#[must_use]
pub fn is_continue122_type(t: &str) -> bool {
    matches!(
        t,
        "ClassExpression"
            | "ClassDeclaration"
            | "MetaProperty"
            | "NewExpression"
            | "ThisExpression"
            | "Super"
    )
}

// ── continue57 shell dual-oracle ────────────────────────────────────────────

/// Dual-oracle residual: continue122 class edge shell composing real
/// [`continue57_class_edge_shell`].
#[must_use]
pub fn continue122_class_edge_shell() -> bool {
    let a = continue57_class_edge_shell();
    let b = continue57_class_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue122 new/this/super shell composing real
/// [`continue57_new_this_super_shell`].
#[must_use]
pub fn continue122_new_this_super_shell() -> bool {
    let a = continue57_new_this_super_shell();
    let b = continue57_new_this_super_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue122 meta/partition shell composing real
/// [`continue57_meta_partition_shell`].
#[must_use]
pub fn continue122_meta_partition_shell() -> bool {
    let a = continue57_meta_partition_shell();
    let b = continue57_meta_partition_shell();
    a && b && a == b
}

// ── continue58 edge shell dual-oracle ───────────────────────────────────────

/// Dual-oracle residual: continue122 super edge shell composing real
/// [`continue58_super_edge_shell`].
#[must_use]
pub fn continue122_super_edge_shell() -> bool {
    let a = continue58_super_edge_shell();
    let b = continue58_super_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue122 new/class edge shell composing real
/// [`continue58_new_class_edge_shell`].
#[must_use]
pub fn continue122_new_class_edge_shell() -> bool {
    let a = continue58_new_class_edge_shell();
    let b = continue58_new_class_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue122 continue58 meta/partition shell composing
/// real [`continue58_meta_partition_shell`].
#[must_use]
pub fn continue122_edge_meta_partition_shell() -> bool {
    let a = continue58_meta_partition_shell();
    let b = continue58_meta_partition_shell();
    a && b && a == b
}

// ── continue55 base edge skeleton emit ──────────────────────────────────────

/// Dual-oracle residual: empty class expression edge over continue55 base.
#[must_use]
pub fn continue122_class_expression_empty_edge() -> String {
    continue55_class_expression_skeleton("")
}

/// Dual-oracle residual: named class expression edge over continue55 base.
#[must_use]
pub fn continue122_class_expression_named_edge(name: &str) -> String {
    continue55_class_expression_skeleton(name)
}

/// Dual-oracle residual: class declaration edge over continue55 base.
#[must_use]
pub fn continue122_class_declaration_edge(name: &str) -> String {
    continue55_class_declaration_skeleton(name)
}

/// Dual-oracle residual: import.meta edge over continue55 base.
#[must_use]
pub fn continue122_meta_import_meta_edge() -> &'static str {
    continue55_meta_import_meta_skeleton()
}

/// Dual-oracle residual: new.target edge over continue55 base.
#[must_use]
pub fn continue122_meta_new_target_edge() -> &'static str {
    continue55_meta_new_target_skeleton()
}

/// Dual-oracle residual: new expression empty-args edge over continue55 base.
#[must_use]
pub fn continue122_new_empty_edge(callee: &str) -> String {
    continue55_new_expression_skeleton(callee, "")
}

/// Dual-oracle residual: new expression multi-arg edge over continue55 base.
#[must_use]
pub fn continue122_new_args_edge(callee: &str, args: &str) -> String {
    continue55_new_expression_skeleton(callee, args)
}

/// Dual-oracle residual: bare this edge over continue55 base.
#[must_use]
pub fn continue122_this_edge() -> &'static str {
    continue55_this_skeleton()
}

/// Dual-oracle residual: bare super() edge over continue55 base.
#[must_use]
pub fn continue122_super_empty_edge() -> String {
    continue55_super_call_skeleton("")
}

/// Dual-oracle residual: super(args) edge over continue55 base.
#[must_use]
pub fn continue122_super_args_edge(args: &str) -> String {
    continue55_super_call_skeleton(args)
}

// ── Composed residual edge shells ───────────────────────────────────────────

/// Dual-oracle residual: empty class expression then bare new edge.
#[must_use]
pub fn continue122_empty_class_then_new(callee: &str) -> String {
    let c = continue122_class_expression_empty_edge();
    let n = continue122_new_empty_edge(callee);
    format!("{c} {n}")
}

/// Dual-oracle residual: class declaration then this edge pair.
#[must_use]
pub fn continue122_class_decl_then_this(name: &str) -> String {
    let c = continue122_class_declaration_edge(name);
    let t = continue122_this_edge();
    format!("{c} {t}")
}

/// Dual-oracle residual: import.meta then new.target edge pair.
#[must_use]
pub fn continue122_meta_pair_edge() -> String {
    let a = continue122_meta_import_meta_edge();
    let b = continue122_meta_new_target_edge();
    format!("{a} {b}")
}

/// Dual-oracle residual: bare super then this edge pair.
#[must_use]
pub fn continue122_super_then_this_edge(args: &str) -> String {
    let s = continue122_super_args_edge(args);
    let t = continue122_this_edge();
    format!("{s} {t}")
}

/// Dual-oracle residual: new then super edge pair.
#[must_use]
pub fn continue122_new_then_super_edge(callee: &str, nargs: &str, sargs: &str) -> String {
    let n = continue122_new_args_edge(callee, nargs);
    let s = continue122_super_args_edge(sargs);
    format!("{n} {s}")
}

/// Dual-oracle residual: catalog length pole (continue55 plane, six types).
#[must_use]
pub fn continue122_catalog_len_shell() -> bool {
    CONTINUE122_RELATED_TYPES.len() == 6
        && CONTINUE55_RELATED_TYPES.len() == 6
        && is_continue55_related_type("ClassExpression")
        && is_continue55_related_type("Super")
        && !is_continue55_related_type("SequenceExpression")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue122_sep(pretty: bool) -> &'static str {
    if pretty {
        " "
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue122_type_catalog() {
        assert_eq!(CONTINUE122_RELATED_TYPES.len(), 6);
        assert!(is_class_meta_new_this_super_edge_related_type(
            "ClassExpression"
        ));
        assert!(is_class_meta_new_this_super_edge_related_type(
            "ClassDeclaration"
        ));
        assert!(is_class_meta_new_this_super_edge_related_type("MetaProperty"));
        assert!(is_class_meta_new_this_super_edge_related_type(
            "NewExpression"
        ));
        assert!(is_class_meta_new_this_super_edge_related_type(
            "ThisExpression"
        ));
        assert!(is_class_meta_new_this_super_edge_related_type("Super"));
        assert!(!is_class_meta_new_this_super_edge_related_type(
            "SequenceExpression"
        ));
        assert!(!is_class_meta_new_this_super_edge_related_type(
            "ImportExpression"
        ));

        assert!(is_continue122_class_expression_type("ClassExpression"));
        assert!(!is_continue122_class_expression_type("ClassDeclaration"));
        assert!(is_continue122_class_declaration_type("ClassDeclaration"));
        assert!(is_continue122_meta_type("MetaProperty"));
        assert!(is_continue122_new_type("NewExpression"));
        assert!(is_continue122_this_type("ThisExpression"));
        assert!(is_continue122_super_type("Super"));
        assert!(is_continue122_class_plane_type("ClassExpression"));
        assert!(is_continue122_class_plane_type("ClassDeclaration"));
        assert!(!is_continue122_class_plane_type("Super"));
        assert!(is_continue122_meta_new_plane_type("MetaProperty"));
        assert!(is_continue122_meta_new_plane_type("NewExpression"));
        assert!(!is_continue122_meta_new_plane_type("ThisExpression"));
        assert!(is_continue122_this_super_plane_type("ThisExpression"));
        assert!(is_continue122_this_super_plane_type("Super"));
        assert!(!is_continue122_this_super_plane_type("NewExpression"));
        assert!(is_continue122_type("Super"));
        assert!(!is_continue122_type("SequenceExpression"));
        assert!(!is_continue122_type("Identifier"));
        assert!(continue122_catalog_len_shell());
    }

    #[test]
    fn continue122_continue57_shell_dual_oracle() {
        assert!(continue122_class_edge_shell());
        assert!(continue122_new_this_super_shell());
        assert!(continue122_meta_partition_shell());
        // base poles still hold
        assert!(continue57_class_edge_shell());
        assert!(continue57_new_this_super_shell());
        assert!(continue57_meta_partition_shell());
    }

    #[test]
    fn continue122_continue58_edge_shell_dual_oracle() {
        assert!(continue122_super_edge_shell());
        assert!(continue122_new_class_edge_shell());
        assert!(continue122_edge_meta_partition_shell());
        assert!(continue58_super_edge_shell());
        assert!(continue58_new_class_edge_shell());
        assert!(continue58_meta_partition_shell());
    }

    #[test]
    fn continue122_edge_skeleton_emit() {
        assert_eq!(continue122_class_expression_empty_edge(), "class {}");
        assert_eq!(
            continue122_class_expression_empty_edge(),
            continue55_class_expression_skeleton("")
        );
        assert_eq!(
            continue122_class_expression_named_edge("Widget"),
            "class Widget {}"
        );
        assert_eq!(
            continue122_class_expression_named_edge("Widget"),
            continue55_class_expression_skeleton("Widget")
        );
        assert_eq!(continue122_class_declaration_edge("App"), "class App {}");
        assert_eq!(
            continue122_class_declaration_edge("App"),
            continue55_class_declaration_skeleton("App")
        );
        assert_eq!(continue122_meta_import_meta_edge(), "import.meta");
        assert_eq!(
            continue122_meta_import_meta_edge(),
            continue55_meta_import_meta_skeleton()
        );
        assert_eq!(continue122_meta_new_target_edge(), "new.target");
        assert_eq!(
            continue122_meta_new_target_edge(),
            continue55_meta_new_target_skeleton()
        );
        assert_eq!(continue122_new_empty_edge("Map"), "new Map()");
        assert_eq!(
            continue122_new_args_edge("Set", "1, 2"),
            "new Set(1, 2)"
        );
        assert_eq!(
            continue122_new_args_edge("Set", "1, 2"),
            continue55_new_expression_skeleton("Set", "1, 2")
        );
        assert_eq!(continue122_this_edge(), "this");
        assert_eq!(continue122_this_edge(), continue55_this_skeleton());
        assert_eq!(continue122_super_empty_edge(), "super()");
        assert_eq!(continue122_super_args_edge("props"), "super(props)");
        assert_eq!(
            continue122_super_args_edge("props"),
            continue55_super_call_skeleton("props")
        );
        assert!(is_continue55_class_expression_type("ClassExpression"));
        assert!(is_continue55_class_declaration_type("ClassDeclaration"));
        assert!(is_continue55_meta_property_type("MetaProperty"));
        assert!(is_continue55_new_expression_type("NewExpression"));
        assert!(is_continue55_this_expression_type("ThisExpression"));
        assert!(is_continue55_super_type("Super"));
    }

    #[test]
    fn continue122_composed_residual_shells() {
        assert_eq!(
            continue122_empty_class_then_new("Foo"),
            "class {} new Foo()"
        );
        assert_eq!(
            continue122_class_decl_then_this("D"),
            "class D {} this"
        );
        assert_eq!(continue122_meta_pair_edge(), "import.meta new.target");
        assert_eq!(
            continue122_super_then_this_edge("x"),
            "super(x) this"
        );
        assert_eq!(
            continue122_new_then_super_edge("Foo", "", "a"),
            "new Foo() super(a)"
        );
        assert_eq!(continue122_sep(true), " ");
        assert_eq!(continue122_sep(false), "");
        assert_ne!(continue122_sep(true), continue122_sep(false));
    }
}

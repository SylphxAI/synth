//! Pure ClassExpression + ClassDeclaration + MetaProperty + NewExpression +
//! ThisExpression + Super dual-oracle emission — residual pure **continue120**
//! for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–119:
//! - ClassExpression dual-oracle composing real
//!   `continue55_class_expression_skeleton`
//! - ClassDeclaration dual-oracle composing real
//!   `continue55_class_declaration_skeleton`
//! - MetaProperty import.meta dual-oracle composing real
//!   `continue55_meta_import_meta_skeleton`
//! - MetaProperty new.target dual-oracle composing real
//!   `continue55_meta_new_target_skeleton`
//! - NewExpression dual-oracle composing real
//!   `continue55_new_expression_skeleton`
//! - ThisExpression dual-oracle composing real `continue55_this_skeleton`
//! - Super call dual-oracle composing real `continue55_super_call_skeleton`
//! - Composed class/meta/new/this/super residual shells
//!
//! Intentionally does **not** re-wrap continue119 sequence/update/yield/await/
//! chain/import poles (continue53 bases), continue118 super/this/meta/template
//! poles (continue51 bases), or continue114 class/import/export/new/this/super/
//! meta poles (continue47 bases). Composes real shipped pure helpers from
//! continue55 class/meta/new/this/super bases. Full engines remain product dens.
//! NO authority_rust / ts_deleted. dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue55_class_declaration_skeleton, continue55_class_expression_skeleton,
    continue55_meta_import_meta_skeleton, continue55_meta_new_target_skeleton,
    continue55_new_expression_skeleton, continue55_super_call_skeleton,
    continue55_this_skeleton,
};

/// Dual-oracle residual: continue120 related AST type catalog.
pub const CONTINUE120_RELATED_TYPES: &[&str] = &[
    "ClassExpression",
    "ClassDeclaration",
    "MetaProperty",
    "NewExpression",
    "ThisExpression",
    "Super",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_class_meta_new_this_super_related_type(t: &str) -> bool {
    CONTINUE120_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue120_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue120_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue120_meta_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue120_new_type(t: &str) -> bool {
    t == "NewExpression"
}

#[must_use]
pub fn is_continue120_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue120_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue120_class_plane_type(t: &str) -> bool {
    matches!(t, "ClassExpression" | "ClassDeclaration")
}

#[must_use]
pub fn is_continue120_meta_new_plane_type(t: &str) -> bool {
    matches!(t, "MetaProperty" | "NewExpression")
}

#[must_use]
pub fn is_continue120_this_super_plane_type(t: &str) -> bool {
    matches!(t, "ThisExpression" | "Super")
}

#[must_use]
pub fn is_continue120_type(t: &str) -> bool {
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

// ── ClassExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ClassExpression skeleton composing real
/// [`continue55_class_expression_skeleton`].
#[must_use]
pub fn continue120_class_expression_skeleton(name: &str) -> String {
    continue55_class_expression_skeleton(name)
}

/// Dual-oracle ClassExpression pretty alias.
#[must_use]
pub fn continue120_class_expression_pretty(name: &str) -> String {
    continue120_class_expression_skeleton(name)
}

/// Dual-oracle ClassExpression minify alias.
#[must_use]
pub fn continue120_class_expression_minify(name: &str) -> String {
    continue120_class_expression_skeleton(name)
}

// ── ClassDeclaration dual-oracle ────────────────────────────────────────────

/// Dual-oracle ClassDeclaration skeleton composing real
/// [`continue55_class_declaration_skeleton`].
#[must_use]
pub fn continue120_class_declaration_skeleton(name: &str) -> String {
    continue55_class_declaration_skeleton(name)
}

/// Dual-oracle ClassDeclaration pretty alias.
#[must_use]
pub fn continue120_class_declaration_pretty(name: &str) -> String {
    continue120_class_declaration_skeleton(name)
}

/// Dual-oracle ClassDeclaration minify alias.
#[must_use]
pub fn continue120_class_declaration_minify(name: &str) -> String {
    continue120_class_declaration_skeleton(name)
}

// ── MetaProperty dual-oracle ────────────────────────────────────────────────

/// Dual-oracle import.meta skeleton composing real
/// [`continue55_meta_import_meta_skeleton`].
#[must_use]
pub fn continue120_meta_import_meta_skeleton() -> &'static str {
    continue55_meta_import_meta_skeleton()
}

/// Dual-oracle import.meta pretty alias.
#[must_use]
pub fn continue120_meta_import_meta_pretty() -> &'static str {
    continue120_meta_import_meta_skeleton()
}

/// Dual-oracle import.meta minify alias.
#[must_use]
pub fn continue120_meta_import_meta_minify() -> &'static str {
    continue120_meta_import_meta_skeleton()
}

/// Dual-oracle new.target skeleton composing real
/// [`continue55_meta_new_target_skeleton`].
#[must_use]
pub fn continue120_meta_new_target_skeleton() -> &'static str {
    continue55_meta_new_target_skeleton()
}

/// Dual-oracle new.target pretty alias.
#[must_use]
pub fn continue120_meta_new_target_pretty() -> &'static str {
    continue120_meta_new_target_skeleton()
}

/// Dual-oracle new.target minify alias.
#[must_use]
pub fn continue120_meta_new_target_minify() -> &'static str {
    continue120_meta_new_target_skeleton()
}

// ── NewExpression dual-oracle ───────────────────────────────────────────────

/// Dual-oracle NewExpression skeleton composing real
/// [`continue55_new_expression_skeleton`].
#[must_use]
pub fn continue120_new_expression_skeleton(callee: &str, args: &str) -> String {
    continue55_new_expression_skeleton(callee, args)
}

/// Dual-oracle NewExpression pretty alias.
#[must_use]
pub fn continue120_new_expression_pretty(callee: &str, args: &str) -> String {
    continue120_new_expression_skeleton(callee, args)
}

/// Dual-oracle NewExpression minify alias.
#[must_use]
pub fn continue120_new_expression_minify(callee: &str, args: &str) -> String {
    continue120_new_expression_skeleton(callee, args)
}

// ── ThisExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThisExpression skeleton composing real
/// [`continue55_this_skeleton`].
#[must_use]
pub fn continue120_this_skeleton() -> &'static str {
    continue55_this_skeleton()
}

/// Dual-oracle ThisExpression pretty alias.
#[must_use]
pub fn continue120_this_pretty() -> &'static str {
    continue120_this_skeleton()
}

/// Dual-oracle ThisExpression minify alias.
#[must_use]
pub fn continue120_this_minify() -> &'static str {
    continue120_this_skeleton()
}

// ── Super dual-oracle ───────────────────────────────────────────────────────

/// Dual-oracle Super call skeleton composing real
/// [`continue55_super_call_skeleton`].
#[must_use]
pub fn continue120_super_call_skeleton(args: &str) -> String {
    continue55_super_call_skeleton(args)
}

/// Dual-oracle Super call pretty alias.
#[must_use]
pub fn continue120_super_call_pretty(args: &str) -> String {
    continue120_super_call_skeleton(args)
}

/// Dual-oracle Super call minify alias.
#[must_use]
pub fn continue120_super_call_minify(args: &str) -> String {
    continue120_super_call_skeleton(args)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: class expression then new.
#[must_use]
pub fn continue120_class_expr_then_new(name: &str, callee: &str, args: &str) -> String {
    let c = continue120_class_expression_skeleton(name);
    let n = continue120_new_expression_skeleton(callee, args);
    format!("{c} {n}")
}

/// Dual-oracle residual: class declaration then this.
#[must_use]
pub fn continue120_class_decl_then_this(name: &str) -> String {
    let c = continue120_class_declaration_skeleton(name);
    let t = continue120_this_skeleton();
    format!("{c} {t}")
}

/// Dual-oracle residual: import.meta then new.target.
#[must_use]
pub fn continue120_meta_pair() -> String {
    let a = continue120_meta_import_meta_skeleton();
    let b = continue120_meta_new_target_skeleton();
    format!("{a} {b}")
}

/// Dual-oracle residual: super() then this.
#[must_use]
pub fn continue120_super_then_this(args: &str) -> String {
    let s = continue120_super_call_skeleton(args);
    let t = continue120_this_skeleton();
    format!("{s} {t}")
}

/// Dual-oracle residual: new then super.
#[must_use]
pub fn continue120_new_then_super(callee: &str, nargs: &str, sargs: &str) -> String {
    let n = continue120_new_expression_skeleton(callee, nargs);
    let s = continue120_super_call_skeleton(sargs);
    format!("{n} {s}")
}

/// Dual-oracle residual: anonymous class seed.
#[must_use]
pub fn continue120_class_expression_empty() -> String {
    continue120_class_expression_skeleton("")
}

/// Dual-oracle residual: bare this seed.
#[must_use]
pub fn continue120_this_seed() -> &'static str {
    continue120_this_skeleton()
}

/// Dual-oracle residual: bare super() seed.
#[must_use]
pub fn continue120_super_empty() -> String {
    continue120_super_call_skeleton("")
}

/// Dual-oracle residual: bare new Foo() seed.
#[must_use]
pub fn continue120_new_empty(callee: &str) -> String {
    continue120_new_expression_skeleton(callee, "")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue120_sep(pretty: bool) -> &'static str {
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
        continue55_class_declaration_skeleton, continue55_class_expression_skeleton,
        continue55_meta_import_meta_skeleton, continue55_meta_new_target_skeleton,
        continue55_new_expression_skeleton, continue55_super_call_skeleton,
        continue55_this_skeleton,
    };

    #[test]
    fn continue120_type_catalog() {
        assert_eq!(CONTINUE120_RELATED_TYPES.len(), 6);
        assert!(is_class_meta_new_this_super_related_type("ClassExpression"));
        assert!(is_class_meta_new_this_super_related_type("ClassDeclaration"));
        assert!(is_class_meta_new_this_super_related_type("MetaProperty"));
        assert!(is_class_meta_new_this_super_related_type("NewExpression"));
        assert!(is_class_meta_new_this_super_related_type("ThisExpression"));
        assert!(is_class_meta_new_this_super_related_type("Super"));
        assert!(!is_class_meta_new_this_super_related_type("SequenceExpression"));
        assert!(!is_class_meta_new_this_super_related_type("ImportExpression"));

        assert!(is_continue120_class_expression_type("ClassExpression"));
        assert!(!is_continue120_class_expression_type("ClassDeclaration"));
        assert!(is_continue120_class_declaration_type("ClassDeclaration"));
        assert!(is_continue120_meta_type("MetaProperty"));
        assert!(is_continue120_new_type("NewExpression"));
        assert!(is_continue120_this_type("ThisExpression"));
        assert!(is_continue120_super_type("Super"));
        assert!(is_continue120_class_plane_type("ClassExpression"));
        assert!(is_continue120_class_plane_type("ClassDeclaration"));
        assert!(!is_continue120_class_plane_type("Super"));
        assert!(is_continue120_meta_new_plane_type("MetaProperty"));
        assert!(is_continue120_meta_new_plane_type("NewExpression"));
        assert!(!is_continue120_meta_new_plane_type("ThisExpression"));
        assert!(is_continue120_this_super_plane_type("ThisExpression"));
        assert!(is_continue120_this_super_plane_type("Super"));
        assert!(!is_continue120_this_super_plane_type("NewExpression"));
        assert!(is_continue120_type("Super"));
        assert!(!is_continue120_type("SequenceExpression"));
        assert!(!is_continue120_type("Identifier"));
    }

    #[test]
    fn continue120_class_meta_emit() {
        assert_eq!(continue120_class_expression_skeleton(""), "class {}");
        assert_eq!(continue120_class_expression_skeleton("C"), "class C {}");
        assert_eq!(
            continue120_class_expression_skeleton("C"),
            continue55_class_expression_skeleton("C")
        );
        assert_eq!(
            continue120_class_expression_pretty("X"),
            continue120_class_expression_minify("X")
        );
        assert_eq!(continue120_class_expression_empty(), "class {}");

        assert_eq!(continue120_class_declaration_skeleton("D"), "class D {}");
        assert_eq!(
            continue120_class_declaration_skeleton("D"),
            continue55_class_declaration_skeleton("D")
        );
        assert_eq!(
            continue120_class_declaration_pretty("Y"),
            continue120_class_declaration_minify("Y")
        );

        assert_eq!(continue120_meta_import_meta_skeleton(), "import.meta");
        assert_eq!(
            continue120_meta_import_meta_skeleton(),
            continue55_meta_import_meta_skeleton()
        );
        assert_eq!(
            continue120_meta_import_meta_pretty(),
            continue120_meta_import_meta_minify()
        );

        assert_eq!(continue120_meta_new_target_skeleton(), "new.target");
        assert_eq!(
            continue120_meta_new_target_skeleton(),
            continue55_meta_new_target_skeleton()
        );
        assert_eq!(
            continue120_meta_new_target_pretty(),
            continue120_meta_new_target_minify()
        );
        assert_eq!(continue120_meta_pair(), "import.meta new.target");
    }

    #[test]
    fn continue120_new_this_super_emit() {
        assert_eq!(continue120_new_expression_skeleton("Foo", ""), "new Foo()");
        assert_eq!(
            continue120_new_expression_skeleton("Foo", "1, 2"),
            "new Foo(1, 2)"
        );
        assert_eq!(
            continue120_new_expression_skeleton("Foo", "1"),
            continue55_new_expression_skeleton("Foo", "1")
        );
        assert_eq!(
            continue120_new_expression_pretty("Bar", "x"),
            continue120_new_expression_minify("Bar", "x")
        );
        assert_eq!(continue120_new_empty("Foo"), "new Foo()");

        assert_eq!(continue120_this_skeleton(), "this");
        assert_eq!(continue120_this_skeleton(), continue55_this_skeleton());
        assert_eq!(continue120_this_pretty(), continue120_this_minify());
        assert_eq!(continue120_this_seed(), "this");

        assert_eq!(continue120_super_call_skeleton(""), "super()");
        assert_eq!(continue120_super_call_skeleton("x"), "super(x)");
        assert_eq!(
            continue120_super_call_skeleton("a"),
            continue55_super_call_skeleton("a")
        );
        assert_eq!(
            continue120_super_call_pretty("y"),
            continue120_super_call_minify("y")
        );
        assert_eq!(continue120_super_empty(), "super()");
    }

    #[test]
    fn continue120_composed_residual_shells() {
        assert_eq!(
            continue120_class_expr_then_new("C", "Foo", "1"),
            "class C {} new Foo(1)"
        );
        assert_eq!(
            continue120_class_decl_then_this("D"),
            "class D {} this"
        );
        assert_eq!(continue120_super_then_this("x"), "super(x) this");
        assert_eq!(
            continue120_new_then_super("Foo", "", "a"),
            "new Foo() super(a)"
        );
        assert_eq!(continue120_sep(true), " ");
        assert_eq!(continue120_sep(false), "");
        assert_ne!(continue120_sep(true), continue120_sep(false));
    }
}

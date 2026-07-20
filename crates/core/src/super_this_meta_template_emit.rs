//! Pure Super + ThisExpression + MetaProperty + TaggedTemplateExpression +
//! TemplateLiteral + TemplateElement dual-oracle emission — residual pure
//! **continue118** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–117:
//! - Super call dual-oracle composing real `continue51_super_call_skeleton`
//! - Super member dual-oracle composing real `continue51_super_member_skeleton`
//! - ThisExpression member dual-oracle composing real
//!   `continue51_this_member_skeleton`
//! - MetaProperty import.meta dual-oracle composing real
//!   `continue51_import_meta_skeleton`
//! - MetaProperty new.target dual-oracle composing real
//!   `continue51_new_target_skeleton`
//! - TemplateLiteral dual-oracle composing real
//!   `continue51_template_literal_skeleton`
//! - TaggedTemplateExpression dual-oracle composing real
//!   `continue51_tagged_template_skeleton`
//! - Composed super/this/meta/template residual shells
//!
//! Intentionally does **not** re-wrap continue117 class private/method/
//! accessor/object poles (continue50 bases), continue116 class property/
//! static/decorator/method poles (continue49 bases), or continue115 optional/
//! import/await/yield/private poles (continue48 bases). Composes real shipped
//! pure helpers from continue51 super/this/meta/template bases. Full engines
//! remain product residual. NO authority_rust / ts_deleted. pure residual ≠ authority flip;
//! PreferRust OFF.

use crate::literal_widen_emit::{
    continue51_import_meta_skeleton, continue51_new_target_skeleton,
    continue51_super_call_skeleton, continue51_super_member_skeleton,
    continue51_tagged_template_skeleton, continue51_template_literal_skeleton,
    continue51_this_member_skeleton,
};

/// Dual-oracle residual: continue118 related AST type catalog.
pub const CONTINUE118_RELATED_TYPES: &[&str] = &[
    "Super",
    "ThisExpression",
    "MetaProperty",
    "TaggedTemplateExpression",
    "TemplateLiteral",
    "TemplateElement",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_super_this_meta_template_related_type(t: &str) -> bool {
    CONTINUE118_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue118_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue118_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue118_meta_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue118_tagged_template_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

#[must_use]
pub fn is_continue118_template_literal_type(t: &str) -> bool {
    t == "TemplateLiteral"
}

#[must_use]
pub fn is_continue118_template_element_type(t: &str) -> bool {
    t == "TemplateElement"
}

#[must_use]
pub fn is_continue118_meta_plane_type(t: &str) -> bool {
    matches!(t, "MetaProperty" | "Super" | "ThisExpression")
}

#[must_use]
pub fn is_continue118_template_plane_type(t: &str) -> bool {
    matches!(
        t,
        "TaggedTemplateExpression" | "TemplateLiteral" | "TemplateElement"
    )
}

#[must_use]
pub fn is_continue118_type(t: &str) -> bool {
    matches!(
        t,
        "Super"
            | "ThisExpression"
            | "MetaProperty"
            | "TaggedTemplateExpression"
            | "TemplateLiteral"
            | "TemplateElement"
    )
}

// ── Super call dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle Super call skeleton composing real
/// [`continue51_super_call_skeleton`].
#[must_use]
pub fn continue118_super_call_skeleton(args: &str) -> String {
    continue51_super_call_skeleton(args)
}

/// Dual-oracle Super call pretty alias.
#[must_use]
pub fn continue118_super_call_pretty(args: &str) -> String {
    continue118_super_call_skeleton(args)
}

/// Dual-oracle Super call minify alias.
#[must_use]
pub fn continue118_super_call_minify(args: &str) -> String {
    continue118_super_call_skeleton(args)
}

// ── Super member dual-oracle ────────────────────────────────────────────────

/// Dual-oracle Super member skeleton composing real
/// [`continue51_super_member_skeleton`].
#[must_use]
pub fn continue118_super_member_skeleton(name: &str) -> String {
    continue51_super_member_skeleton(name)
}

/// Dual-oracle Super member pretty alias.
#[must_use]
pub fn continue118_super_member_pretty(name: &str) -> String {
    continue118_super_member_skeleton(name)
}

/// Dual-oracle Super member minify alias.
#[must_use]
pub fn continue118_super_member_minify(name: &str) -> String {
    continue118_super_member_skeleton(name)
}

// ── ThisExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThisExpression member skeleton composing real
/// [`continue51_this_member_skeleton`].
#[must_use]
pub fn continue118_this_member_skeleton(name: &str) -> String {
    continue51_this_member_skeleton(name)
}

/// Dual-oracle ThisExpression member pretty alias.
#[must_use]
pub fn continue118_this_member_pretty(name: &str) -> String {
    continue118_this_member_skeleton(name)
}

/// Dual-oracle ThisExpression member minify alias.
#[must_use]
pub fn continue118_this_member_minify(name: &str) -> String {
    continue118_this_member_skeleton(name)
}

// ── MetaProperty import.meta dual-oracle ────────────────────────────────────

/// Dual-oracle import.meta skeleton composing real
/// [`continue51_import_meta_skeleton`].
#[must_use]
pub fn continue118_import_meta_skeleton() -> String {
    continue51_import_meta_skeleton()
}

/// Dual-oracle import.meta pretty alias.
#[must_use]
pub fn continue118_import_meta_pretty() -> String {
    continue118_import_meta_skeleton()
}

/// Dual-oracle import.meta minify alias.
#[must_use]
pub fn continue118_import_meta_minify() -> String {
    continue118_import_meta_skeleton()
}

// ── MetaProperty new.target dual-oracle ─────────────────────────────────────

/// Dual-oracle new.target skeleton composing real
/// [`continue51_new_target_skeleton`].
#[must_use]
pub fn continue118_new_target_skeleton() -> String {
    continue51_new_target_skeleton()
}

/// Dual-oracle new.target pretty alias.
#[must_use]
pub fn continue118_new_target_pretty() -> String {
    continue118_new_target_skeleton()
}

/// Dual-oracle new.target minify alias.
#[must_use]
pub fn continue118_new_target_minify() -> String {
    continue118_new_target_skeleton()
}

// ── TemplateLiteral dual-oracle ─────────────────────────────────────────────

/// Dual-oracle TemplateLiteral skeleton composing real
/// [`continue51_template_literal_skeleton`].
#[must_use]
pub fn continue118_template_literal_skeleton(cooked: &str) -> String {
    continue51_template_literal_skeleton(cooked)
}

/// Dual-oracle TemplateLiteral pretty alias.
#[must_use]
pub fn continue118_template_literal_pretty(cooked: &str) -> String {
    continue118_template_literal_skeleton(cooked)
}

/// Dual-oracle TemplateLiteral minify alias.
#[must_use]
pub fn continue118_template_literal_minify(cooked: &str) -> String {
    continue118_template_literal_skeleton(cooked)
}

// ── TaggedTemplateExpression dual-oracle ────────────────────────────────────

/// Dual-oracle TaggedTemplateExpression skeleton composing real
/// [`continue51_tagged_template_skeleton`].
#[must_use]
pub fn continue118_tagged_template_skeleton(tag: &str, cooked: &str) -> String {
    continue51_tagged_template_skeleton(tag, cooked)
}

/// Dual-oracle TaggedTemplateExpression pretty alias.
#[must_use]
pub fn continue118_tagged_template_pretty(tag: &str, cooked: &str) -> String {
    continue118_tagged_template_skeleton(tag, cooked)
}

/// Dual-oracle TaggedTemplateExpression minify alias.
#[must_use]
pub fn continue118_tagged_template_minify(tag: &str, cooked: &str) -> String {
    continue118_tagged_template_skeleton(tag, cooked)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: super call then super member seed.
#[must_use]
pub fn continue118_super_call_then_member(args: &str, name: &str) -> String {
    let c = continue118_super_call_skeleton(args);
    let m = continue118_super_member_skeleton(name);
    format!("{c} {m}")
}

/// Dual-oracle residual: this member then super member seed.
#[must_use]
pub fn continue118_this_then_super_member(this_name: &str, super_name: &str) -> String {
    let t = continue118_this_member_skeleton(this_name);
    let s = continue118_super_member_skeleton(super_name);
    format!("{t} {s}")
}

/// Dual-oracle residual: import.meta + new.target seed.
#[must_use]
pub fn continue118_meta_pair() -> String {
    let i = continue118_import_meta_skeleton();
    let n = continue118_new_target_skeleton();
    format!("{i} {n}")
}

/// Dual-oracle residual: template literal then tagged template seed.
#[must_use]
pub fn continue118_template_then_tagged(cooked: &str, tag: &str, tagged_cooked: &str) -> String {
    let lit = continue118_template_literal_skeleton(cooked);
    let tagged = continue118_tagged_template_skeleton(tag, tagged_cooked);
    format!("{lit} {tagged}")
}

/// Dual-oracle residual: super() bare call seed.
#[must_use]
pub fn continue118_super_call_empty() -> String {
    continue118_super_call_skeleton("")
}

/// Dual-oracle residual: empty template literal seed.
#[must_use]
pub fn continue118_template_literal_empty() -> String {
    continue118_template_literal_skeleton("")
}

/// Dual-oracle residual: empty tagged template seed.
#[must_use]
pub fn continue118_tagged_template_empty(tag: &str) -> String {
    continue118_tagged_template_skeleton(tag, "")
}

/// Dual-oracle residual: this.x seed helper.
#[must_use]
pub fn continue118_this_x() -> String {
    continue118_this_member_skeleton("x")
}

/// Dual-oracle residual: super.foo seed helper.
#[must_use]
pub fn continue118_super_foo() -> String {
    continue118_super_member_skeleton("foo")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue118_sep(pretty: bool) -> &'static str {
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
        continue51_import_meta_skeleton, continue51_new_target_skeleton,
        continue51_super_call_skeleton, continue51_super_member_skeleton,
        continue51_tagged_template_skeleton, continue51_template_literal_skeleton,
        continue51_this_member_skeleton,
    };

    #[test]
    fn continue118_type_catalog() {
        assert_eq!(CONTINUE118_RELATED_TYPES.len(), 6);
        assert!(is_super_this_meta_template_related_type("Super"));
        assert!(is_super_this_meta_template_related_type("ThisExpression"));
        assert!(is_super_this_meta_template_related_type("MetaProperty"));
        assert!(is_super_this_meta_template_related_type(
            "TaggedTemplateExpression"
        ));
        assert!(is_super_this_meta_template_related_type("TemplateLiteral"));
        assert!(is_super_this_meta_template_related_type("TemplateElement"));
        assert!(!is_super_this_meta_template_related_type(
            "ClassPrivateProperty"
        ));
        assert!(!is_super_this_meta_template_related_type("ObjectMethod"));

        assert!(is_continue118_super_type("Super"));
        assert!(!is_continue118_super_type("ThisExpression"));
        assert!(is_continue118_this_type("ThisExpression"));
        assert!(is_continue118_meta_type("MetaProperty"));
        assert!(is_continue118_tagged_template_type(
            "TaggedTemplateExpression"
        ));
        assert!(is_continue118_template_literal_type("TemplateLiteral"));
        assert!(is_continue118_template_element_type("TemplateElement"));
        assert!(is_continue118_meta_plane_type("Super"));
        assert!(is_continue118_meta_plane_type("ThisExpression"));
        assert!(is_continue118_meta_plane_type("MetaProperty"));
        assert!(!is_continue118_meta_plane_type("TemplateLiteral"));
        assert!(is_continue118_template_plane_type("TemplateLiteral"));
        assert!(is_continue118_template_plane_type(
            "TaggedTemplateExpression"
        ));
        assert!(is_continue118_template_plane_type("TemplateElement"));
        assert!(!is_continue118_template_plane_type("Super"));
        assert!(is_continue118_type("Super"));
        assert!(is_continue118_type("TemplateElement"));
        assert!(!is_continue118_type("ClassPrivateMethod"));
        assert!(!is_continue118_type("Identifier"));
    }

    #[test]
    fn continue118_super_this_emit() {
        assert_eq!(continue118_super_call_skeleton("a, b"), "super(a, b)");
        assert_eq!(
            continue118_super_call_skeleton("a, b"),
            continue51_super_call_skeleton("a, b")
        );
        assert_eq!(
            continue118_super_call_pretty(""),
            continue118_super_call_minify("")
        );
        assert_eq!(continue118_super_call_empty(), "super()");

        assert_eq!(continue118_super_member_skeleton("foo"), "super.foo");
        assert_eq!(
            continue118_super_member_skeleton("foo"),
            continue51_super_member_skeleton("foo")
        );
        assert_eq!(
            continue118_super_member_pretty("bar"),
            continue118_super_member_minify("bar")
        );
        assert_eq!(continue118_super_foo(), "super.foo");

        assert_eq!(continue118_this_member_skeleton("x"), "this.x");
        assert_eq!(
            continue118_this_member_skeleton("x"),
            continue51_this_member_skeleton("x")
        );
        assert_eq!(
            continue118_this_member_pretty("y"),
            continue118_this_member_minify("y")
        );
        assert_eq!(continue118_this_x(), "this.x");
    }

    #[test]
    fn continue118_meta_template_emit() {
        assert_eq!(continue118_import_meta_skeleton(), "import.meta");
        assert_eq!(
            continue118_import_meta_skeleton(),
            continue51_import_meta_skeleton()
        );
        assert_eq!(
            continue118_import_meta_pretty(),
            continue118_import_meta_minify()
        );

        assert_eq!(continue118_new_target_skeleton(), "new.target");
        assert_eq!(
            continue118_new_target_skeleton(),
            continue51_new_target_skeleton()
        );
        assert_eq!(
            continue118_new_target_pretty(),
            continue118_new_target_minify()
        );

        assert_eq!(continue118_template_literal_skeleton("hi"), "`hi`");
        assert_eq!(
            continue118_template_literal_skeleton("hi"),
            continue51_template_literal_skeleton("hi")
        );
        assert_eq!(
            continue118_template_literal_pretty("a"),
            continue118_template_literal_minify("a")
        );
        assert_eq!(continue118_template_literal_empty(), "``");

        assert_eq!(
            continue118_tagged_template_skeleton("css", "a{}"),
            "css`a{}`"
        );
        assert_eq!(
            continue118_tagged_template_skeleton("css", "a{}"),
            continue51_tagged_template_skeleton("css", "a{}")
        );
        assert_eq!(
            continue118_tagged_template_pretty("html", "x"),
            continue118_tagged_template_minify("html", "x")
        );
        assert_eq!(continue118_tagged_template_empty("html"), "html``");
    }

    #[test]
    fn continue118_composed_residual_shells() {
        assert_eq!(
            continue118_super_call_then_member("a", "foo"),
            "super(a) super.foo"
        );
        assert_eq!(
            continue118_this_then_super_member("x", "bar"),
            "this.x super.bar"
        );
        assert_eq!(continue118_meta_pair(), "import.meta new.target");
        assert_eq!(
            continue118_template_then_tagged("hi", "css", "a{}"),
            "`hi` css`a{}`"
        );
        assert_eq!(continue118_sep(true), " ");
        assert_eq!(continue118_sep(false), "");
        assert_ne!(continue118_sep(true), continue118_sep(false));
    }
}

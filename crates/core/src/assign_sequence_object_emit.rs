//! Pure AssignmentExpression + SequenceExpression + ObjectExpression +
//! ParenthesizedExpression dual-oracle emission — residual pure **continue78**
//! for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - AssignmentExpression full dual-oracle pretty/minify op spacing
//!   (continue15 `assignment_expression_skeleton` composed; continue76 explicitly
//!   excluded AssignmentExpression from binary/unary/logical surface)
//! - SequenceExpression dual-oracle pretty/minify list sep composing real
//!   `sequence_expression_skeleton` (continue15)
//! - ObjectExpression dual-oracle pretty/minify prop join + bracketSpacing +
//!   trailing comma driving real `object_skeleton` / `object_property_skeleton`
//! - ParenthesizedExpression dual-oracle driving real `paren_group`
//!
//! Intentionally does **not** re-wrap continue64–77 partition shells
//! (call/member/new/array continue77 stays separate; binary/unary continue76
//! stays separate). Composes real shipped pure helpers from
//! `assign_logical_update_emit` + `object_emit`.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::assign_logical_update_emit::{
    assignment_expression_skeleton, paren_group, sequence_expression_skeleton,
};
use crate::object_emit::{object_property_skeleton, object_skeleton};

/// Dual-oracle residual: continue78 related AST type catalog.
pub const CONTINUE78_RELATED_TYPES: &[&str] = &[
    "AssignmentExpression",
    "SequenceExpression",
    "ObjectExpression",
    "ParenthesizedExpression",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_assign_sequence_object_related_type(t: &str) -> bool {
    CONTINUE78_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue78_assignment_type(t: &str) -> bool {
    t == "AssignmentExpression"
}

#[must_use]
pub fn is_continue78_sequence_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue78_object_type(t: &str) -> bool {
    t == "ObjectExpression"
}

#[must_use]
pub fn is_continue78_parenthesized_type(t: &str) -> bool {
    t == "ParenthesizedExpression"
}

// ── AssignmentExpression dual-oracle ────────────────────────────────────────

/// Dual-oracle AssignmentExpression skeleton: `lhs` + op + `rhs`.
///
/// Pretty: `a = 1` / `a += 1`; minify: `a=1` / `a+=1`.
/// Drives real shipped [`assignment_expression_skeleton`].
#[must_use]
pub fn continue78_assignment_expression_skeleton(
    lhs: &str,
    op: &str,
    rhs: &str,
    pretty: bool,
) -> String {
    assignment_expression_skeleton(lhs, op, rhs, pretty)
}

/// Convenience: pretty assignment.
#[must_use]
pub fn assignment_expression_pretty(lhs: &str, op: &str, rhs: &str) -> String {
    continue78_assignment_expression_skeleton(lhs, op, rhs, true)
}

/// Convenience: minify assignment.
#[must_use]
pub fn assignment_expression_minify(lhs: &str, op: &str, rhs: &str) -> String {
    continue78_assignment_expression_skeleton(lhs, op, rhs, false)
}

// ── SequenceExpression dual-oracle ──────────────────────────────────────────

/// Dual-oracle SequenceExpression skeleton: `a, b` / `a,b`.
///
/// Drives real shipped [`sequence_expression_skeleton`].
#[must_use]
pub fn continue78_sequence_expression_skeleton(parts: &[&str], pretty: bool) -> String {
    sequence_expression_skeleton(parts, pretty)
}

/// Convenience: pretty sequence.
#[must_use]
pub fn sequence_expression_pretty(parts: &[&str]) -> String {
    continue78_sequence_expression_skeleton(parts, true)
}

/// Convenience: minify sequence.
#[must_use]
pub fn sequence_expression_minify(parts: &[&str]) -> String {
    continue78_sequence_expression_skeleton(parts, false)
}

// ── ObjectExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle ObjectExpression skeleton composing real [`object_skeleton`].
///
/// Pretty + bracketSpacing: `{ a: 1, b: 2 }`; minify: `{a:1,b:2}`.
#[must_use]
pub fn continue78_object_expression_skeleton(
    props: &[&str],
    pretty: bool,
    bracket_spacing: bool,
    trailing_comma: bool,
) -> String {
    object_skeleton(props, pretty, bracket_spacing, trailing_comma)
}

/// Dual-oracle object property: `key: value` / `key:value`.
///
/// Drives real shipped [`object_property_skeleton`].
#[must_use]
pub fn continue78_object_property_skeleton(key: &str, value: &str, pretty: bool) -> String {
    object_property_skeleton(key, value, pretty)
}

/// Convenience: pretty object with bracket spacing, no trailing comma.
#[must_use]
pub fn object_expression_pretty(props: &[&str]) -> String {
    continue78_object_expression_skeleton(props, true, true, false)
}

/// Convenience: minify object (no bracket spacing, no trailing comma).
#[must_use]
pub fn object_expression_minify(props: &[&str]) -> String {
    continue78_object_expression_skeleton(props, false, false, false)
}

/// Convenience: pretty object with trailing comma residual.
#[must_use]
pub fn object_expression_pretty_trailing(props: &[&str]) -> String {
    continue78_object_expression_skeleton(props, true, true, true)
}

// ── ParenthesizedExpression dual-oracle ─────────────────────────────────────

/// Dual-oracle ParenthesizedExpression: `(inner)`.
///
/// Pretty/minify share explicit paren placement (engines rarely re-parenthesize).
/// Drives real shipped [`paren_group`].
#[must_use]
pub fn continue78_parenthesized_expression_skeleton(inner: &str, pretty: bool) -> String {
    paren_group(inner, pretty)
}

/// Convenience: parenthesize.
#[must_use]
pub fn parenthesized_expression(inner: &str) -> String {
    continue78_parenthesized_expression_skeleton(inner, true)
}

/// Dual-oracle residual: parenthesized sequence `(a, b)` / `(a,b)`.
#[must_use]
pub fn continue78_parenthesized_sequence(parts: &[&str], pretty: bool) -> String {
    continue78_parenthesized_expression_skeleton(
        &continue78_sequence_expression_skeleton(parts, pretty),
        pretty,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assign_logical_update_emit::{
        assignment_expression_skeleton, paren_group, sequence_expression_skeleton,
    };
    use crate::object_emit::{object_property_skeleton, object_skeleton};

    #[test]
    fn continue78_type_catalog() {
        assert_eq!(CONTINUE78_RELATED_TYPES.len(), 4);
        assert!(is_assign_sequence_object_related_type("AssignmentExpression"));
        assert!(is_assign_sequence_object_related_type("SequenceExpression"));
        assert!(is_assign_sequence_object_related_type("ObjectExpression"));
        assert!(is_assign_sequence_object_related_type("ParenthesizedExpression"));
        assert!(!is_assign_sequence_object_related_type("BinaryExpression"));
        assert!(!is_assign_sequence_object_related_type("CallExpression"));
        assert!(!is_assign_sequence_object_related_type("LogicalExpression"));
        assert!(is_continue78_assignment_type("AssignmentExpression"));
        assert!(is_continue78_sequence_type("SequenceExpression"));
        assert!(is_continue78_object_type("ObjectExpression"));
        assert!(is_continue78_parenthesized_type("ParenthesizedExpression"));
        assert!(!is_continue78_assignment_type("SequenceExpression"));
        assert!(!is_continue78_object_type("ObjectPattern"));
    }

    #[test]
    fn continue78_assignment_expression_dual_oracle() {
        assert_eq!(assignment_expression_pretty("a", "=", "1"), "a = 1");
        assert_eq!(assignment_expression_minify("a", "=", "1"), "a=1");
        assert_eq!(assignment_expression_pretty("a", "+=", "b"), "a += b");
        assert_eq!(assignment_expression_minify("a", "+=", "b"), "a+=b");
        assert_eq!(assignment_expression_pretty("x", "&&=", "y"), "x &&= y");
        assert_eq!(assignment_expression_minify("x", "&&=", "y"), "x&&=y");
        // drives shipped continue15 skeleton
        assert_eq!(
            continue78_assignment_expression_skeleton("a", "=", "1", true),
            assignment_expression_skeleton("a", "=", "1", true)
        );
        assert_eq!(
            continue78_assignment_expression_skeleton("a", "=", "1", false),
            assignment_expression_skeleton("a", "=", "1", false)
        );
        // pretty ≠ minify spacing residual
        assert_ne!(
            assignment_expression_pretty("a", "=", "1"),
            assignment_expression_minify("a", "=", "1")
        );
    }

    #[test]
    fn continue78_sequence_and_paren_dual_oracle() {
        assert_eq!(sequence_expression_pretty(&["a", "b"]), "a, b");
        assert_eq!(sequence_expression_minify(&["a", "b"]), "a,b");
        assert_eq!(sequence_expression_pretty(&[]), "");
        assert_eq!(sequence_expression_pretty(&["x"]), "x");
        assert_eq!(
            continue78_sequence_expression_skeleton(&["a", "b", "c"], false),
            "a,b,c"
        );
        assert_eq!(
            continue78_sequence_expression_skeleton(&["a", "b"], true),
            sequence_expression_skeleton(&["a", "b"], true)
        );
        assert_eq!(
            continue78_sequence_expression_skeleton(&["a", "b"], false),
            sequence_expression_skeleton(&["a", "b"], false)
        );

        assert_eq!(parenthesized_expression("a + b"), "(a + b)");
        assert_eq!(
            continue78_parenthesized_expression_skeleton("x", false),
            paren_group("x", false)
        );
        assert_eq!(continue78_parenthesized_sequence(&["a", "b"], true), "(a, b)");
        assert_eq!(
            continue78_parenthesized_sequence(&["a", "b"], false),
            "(a,b)"
        );
    }

    #[test]
    fn continue78_object_expression_dual_oracle() {
        assert_eq!(object_expression_pretty(&[]), "{}");
        assert_eq!(object_expression_minify(&[]), "{}");

        let p1 = continue78_object_property_skeleton("a", "1", true);
        let p2 = continue78_object_property_skeleton("b", "2", true);
        assert_eq!(p1, "a: 1");
        assert_eq!(continue78_object_property_skeleton("a", "1", false), "a:1");
        assert_eq!(
            continue78_object_property_skeleton("a", "1", true),
            object_property_skeleton("a", "1", true)
        );

        let pretty_props = [p1.as_str(), p2.as_str()];
        assert_eq!(object_expression_pretty(&pretty_props), "{ a: 1, b: 2 }");

        let m1 = continue78_object_property_skeleton("a", "1", false);
        let m2 = continue78_object_property_skeleton("b", "2", false);
        let minify_props = [m1.as_str(), m2.as_str()];
        assert_eq!(object_expression_minify(&minify_props), "{a:1,b:2}");

        // trailing comma residual (bracketSpacing keeps space before `}`)
        assert_eq!(
            object_expression_pretty_trailing(&["a: 1"]),
            "{ a: 1, }"
        );
        assert_eq!(
            continue78_object_expression_skeleton(&["a:1", "b:2"], false, false, true),
            "{a:1,b:2,}"
        );
        // drives shipped object_skeleton
        assert_eq!(
            continue78_object_expression_skeleton(&["a: 1"], true, true, false),
            object_skeleton(&["a: 1"], true, true, false)
        );
        assert_eq!(
            continue78_object_expression_skeleton(&["a:1"], false, false, false),
            object_skeleton(&["a:1"], false, false, false)
        );
    }
}

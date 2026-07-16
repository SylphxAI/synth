//! Pure AssignmentExpression + UpdateExpression + MemberExpression +
//! OptionalMemberExpression + CallExpression + OptionalCallExpression +
//! NewExpression dual-oracle emission — residual pure **continue97** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–96:
//! - AssignmentExpression dual-oracle composing real
//!   `continue30_assignment_skeleton`
//! - UpdateExpression dual-oracle composing real
//!   `continue30_update_skeleton`
//! - MemberExpression / OptionalMemberExpression dual-oracle composing real
//!   `continue30_member_skeleton`
//! - CallExpression / OptionalCallExpression dual-oracle composing real
//!   `continue30_call_skeleton`
//! - NewExpression dual-oracle composing real `continue30_new_skeleton`
//! - Composed assignment/update/member/call/new residual shells
//!
//! Intentionally does **not** re-wrap continue15 assignment/logical/update,
//! continue77 call/member/new pretty-minify poles, continue76 update poles,
//! continue93 sequence/update/yield continue26 bases, or continue96
//! class/function/import/export poles. Composes real shipped pure helpers
//! from continue30 bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue30_assignment_skeleton, continue30_call_skeleton, continue30_member_skeleton,
    continue30_new_skeleton, continue30_update_skeleton,
};

/// Dual-oracle residual: continue97 related AST type catalog.
pub const CONTINUE97_RELATED_TYPES: &[&str] = &[
    "AssignmentExpression",
    "UpdateExpression",
    "MemberExpression",
    "OptionalMemberExpression",
    "CallExpression",
    "OptionalCallExpression",
    "NewExpression",
    "SequenceExpression",
    "SpreadElement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_assignment_update_member_call_related_type(t: &str) -> bool {
    CONTINUE97_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue97_assignment_type(t: &str) -> bool {
    t == "AssignmentExpression"
}

#[must_use]
pub fn is_continue97_update_type(t: &str) -> bool {
    t == "UpdateExpression"
}

#[must_use]
pub fn is_continue97_member_type(t: &str) -> bool {
    matches!(t, "MemberExpression" | "OptionalMemberExpression")
}

#[must_use]
pub fn is_continue97_call_type(t: &str) -> bool {
    matches!(t, "CallExpression" | "OptionalCallExpression")
}

#[must_use]
pub fn is_continue97_new_type(t: &str) -> bool {
    t == "NewExpression"
}

#[must_use]
pub fn is_continue97_sequence_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue97_spread_type(t: &str) -> bool {
    t == "SpreadElement"
}

// ── AssignmentExpression dual-oracle ────────────────────────────────────────

/// Dual-oracle AssignmentExpression skeleton composing real
/// [`continue30_assignment_skeleton`].
#[must_use]
pub fn continue97_assignment_skeleton(left: &str, op: &str, right: &str) -> String {
    continue30_assignment_skeleton(left, op, right)
}

/// Dual-oracle AssignmentExpression pretty alias.
#[must_use]
pub fn continue97_assignment_pretty(left: &str, op: &str, right: &str) -> String {
    continue97_assignment_skeleton(left, op, right)
}

/// Dual-oracle AssignmentExpression minify alias.
#[must_use]
pub fn continue97_assignment_minify(left: &str, op: &str, right: &str) -> String {
    continue97_assignment_skeleton(left, op, right)
}

// ── UpdateExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle UpdateExpression skeleton composing real
/// [`continue30_update_skeleton`].
#[must_use]
pub fn continue97_update_skeleton(arg: &str, op: &str, prefix: bool) -> String {
    continue30_update_skeleton(arg, op, prefix)
}

/// Dual-oracle UpdateExpression prefix alias.
#[must_use]
pub fn continue97_update_prefix(op: &str, arg: &str) -> String {
    continue97_update_skeleton(arg, op, true)
}

/// Dual-oracle UpdateExpression postfix alias.
#[must_use]
pub fn continue97_update_postfix(arg: &str, op: &str) -> String {
    continue97_update_skeleton(arg, op, false)
}

/// Dual-oracle UpdateExpression pretty alias.
#[must_use]
pub fn continue97_update_pretty(arg: &str, op: &str, prefix: bool) -> String {
    continue97_update_skeleton(arg, op, prefix)
}

/// Dual-oracle UpdateExpression minify alias.
#[must_use]
pub fn continue97_update_minify(arg: &str, op: &str, prefix: bool) -> String {
    continue97_update_skeleton(arg, op, prefix)
}

// ── MemberExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle MemberExpression skeleton composing real
/// [`continue30_member_skeleton`].
#[must_use]
pub fn continue97_member_skeleton(
    object: &str,
    property: &str,
    computed: bool,
    optional: bool,
) -> String {
    continue30_member_skeleton(object, property, computed, optional)
}

/// Dual-oracle MemberExpression static non-optional alias.
#[must_use]
pub fn continue97_member_static(object: &str, property: &str) -> String {
    continue97_member_skeleton(object, property, false, false)
}

/// Dual-oracle MemberExpression computed non-optional alias.
#[must_use]
pub fn continue97_member_computed(object: &str, property: &str) -> String {
    continue97_member_skeleton(object, property, true, false)
}

/// Dual-oracle OptionalMemberExpression static alias.
#[must_use]
pub fn continue97_member_optional(object: &str, property: &str) -> String {
    continue97_member_skeleton(object, property, false, true)
}

/// Dual-oracle OptionalMemberExpression computed alias.
#[must_use]
pub fn continue97_member_optional_computed(object: &str, property: &str) -> String {
    continue97_member_skeleton(object, property, true, true)
}

/// Dual-oracle MemberExpression pretty alias.
#[must_use]
pub fn continue97_member_pretty(
    object: &str,
    property: &str,
    computed: bool,
    optional: bool,
) -> String {
    continue97_member_skeleton(object, property, computed, optional)
}

/// Dual-oracle MemberExpression minify alias.
#[must_use]
pub fn continue97_member_minify(
    object: &str,
    property: &str,
    computed: bool,
    optional: bool,
) -> String {
    continue97_member_skeleton(object, property, computed, optional)
}

// ── CallExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle CallExpression skeleton composing real
/// [`continue30_call_skeleton`].
#[must_use]
pub fn continue97_call_skeleton(callee: &str, args: &str, optional: bool) -> String {
    continue30_call_skeleton(callee, args, optional)
}

/// Dual-oracle CallExpression non-optional alias.
#[must_use]
pub fn continue97_call(callee: &str, args: &str) -> String {
    continue97_call_skeleton(callee, args, false)
}

/// Dual-oracle OptionalCallExpression alias.
#[must_use]
pub fn continue97_optional_call(callee: &str, args: &str) -> String {
    continue97_call_skeleton(callee, args, true)
}

/// Dual-oracle CallExpression pretty alias.
#[must_use]
pub fn continue97_call_pretty(callee: &str, args: &str, optional: bool) -> String {
    continue97_call_skeleton(callee, args, optional)
}

/// Dual-oracle CallExpression minify alias.
#[must_use]
pub fn continue97_call_minify(callee: &str, args: &str, optional: bool) -> String {
    continue97_call_skeleton(callee, args, optional)
}

// ── NewExpression dual-oracle ───────────────────────────────────────────────

/// Dual-oracle NewExpression skeleton composing real
/// [`continue30_new_skeleton`].
#[must_use]
pub fn continue97_new_skeleton(callee: &str, args: &str) -> String {
    continue30_new_skeleton(callee, args)
}

/// Dual-oracle NewExpression pretty alias.
#[must_use]
pub fn continue97_new_pretty(callee: &str, args: &str) -> String {
    continue97_new_skeleton(callee, args)
}

/// Dual-oracle NewExpression minify alias.
#[must_use]
pub fn continue97_new_minify(callee: &str, args: &str) -> String {
    continue97_new_skeleton(callee, args)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: simple `a = b` assignment.
#[must_use]
pub fn continue97_assign_eq(left: &str, right: &str) -> String {
    continue97_assignment_skeleton(left, "=", right)
}

/// Dual-oracle residual: `a += b` compound assignment.
#[must_use]
pub fn continue97_assign_add_eq(left: &str, right: &str) -> String {
    continue97_assignment_skeleton(left, "+=", right)
}

/// Dual-oracle residual: prefix `++i`.
#[must_use]
pub fn continue97_pre_increment(arg: &str) -> String {
    continue97_update_prefix("++", arg)
}

/// Dual-oracle residual: postfix `i++`.
#[must_use]
pub fn continue97_post_increment(arg: &str) -> String {
    continue97_update_postfix(arg, "++")
}

/// Dual-oracle residual: prefix `--i`.
#[must_use]
pub fn continue97_pre_decrement(arg: &str) -> String {
    continue97_update_prefix("--", arg)
}

/// Dual-oracle residual: postfix `i--`.
#[must_use]
pub fn continue97_post_decrement(arg: &str) -> String {
    continue97_update_postfix(arg, "--")
}

/// Dual-oracle residual: method call `obj.method(args)`.
#[must_use]
pub fn continue97_method_call(object: &str, method: &str, args: &str) -> String {
    let member = continue97_member_static(object, method);
    continue97_call(&member, args)
}

/// Dual-oracle residual: optional method call `obj?.method?.(args)`.
#[must_use]
pub fn continue97_optional_method_call(object: &str, method: &str, args: &str) -> String {
    let member = continue97_member_optional(object, method);
    continue97_optional_call(&member, args)
}

/// Dual-oracle residual: `new Foo()` no-arg constructor.
#[must_use]
pub fn continue97_new_empty(callee: &str) -> String {
    continue97_new_skeleton(callee, "")
}

/// Dual-oracle residual: assign call result `left = callee(args)`.
#[must_use]
pub fn continue97_assign_call(left: &str, callee: &str, args: &str) -> String {
    let call = continue97_call(callee, args);
    continue97_assign_eq(left, &call)
}

/// Dual-oracle residual: assign new instance `left = new Ctor(args)`.
#[must_use]
pub fn continue97_assign_new(left: &str, callee: &str, args: &str) -> String {
    let neu = continue97_new_skeleton(callee, args);
    continue97_assign_eq(left, &neu)
}

/// Dual-oracle residual: member assign `obj.prop = value`.
#[must_use]
pub fn continue97_member_assign(object: &str, property: &str, value: &str) -> String {
    let member = continue97_member_static(object, property);
    continue97_assign_eq(&member, value)
}

/// Dual-oracle residual: computed member assign `obj[key] = value`.
#[must_use]
pub fn continue97_computed_member_assign(object: &str, property: &str, value: &str) -> String {
    let member = continue97_member_computed(object, property);
    continue97_assign_eq(&member, value)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue97_stmt_sep(pretty: bool) -> &'static str {
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
        continue30_assignment_skeleton, continue30_call_skeleton, continue30_member_skeleton,
        continue30_new_skeleton, continue30_update_skeleton,
    };

    #[test]
    fn continue97_type_catalog() {
        assert_eq!(CONTINUE97_RELATED_TYPES.len(), 9);
        assert!(is_assignment_update_member_call_related_type(
            "AssignmentExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "UpdateExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "MemberExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "OptionalMemberExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "CallExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "OptionalCallExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "NewExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "SequenceExpression"
        ));
        assert!(is_assignment_update_member_call_related_type(
            "SpreadElement"
        ));
        assert!(!is_assignment_update_member_call_related_type(
            "ClassDeclaration"
        ));
        assert!(!is_assignment_update_member_call_related_type(
            "IfStatement"
        ));

        assert!(is_continue97_assignment_type("AssignmentExpression"));
        assert!(!is_continue97_assignment_type("UpdateExpression"));
        assert!(is_continue97_update_type("UpdateExpression"));
        assert!(is_continue97_member_type("MemberExpression"));
        assert!(is_continue97_member_type("OptionalMemberExpression"));
        assert!(!is_continue97_member_type("CallExpression"));
        assert!(is_continue97_call_type("CallExpression"));
        assert!(is_continue97_call_type("OptionalCallExpression"));
        assert!(!is_continue97_call_type("NewExpression"));
        assert!(is_continue97_new_type("NewExpression"));
        assert!(is_continue97_sequence_type("SequenceExpression"));
        assert!(is_continue97_spread_type("SpreadElement"));
    }

    #[test]
    fn continue97_assignment_update_member_call_new_emit() {
        assert_eq!(continue97_assignment_skeleton("a", "=", "1"), "a = 1");
        assert_eq!(
            continue97_assignment_skeleton("a", "=", "1"),
            continue30_assignment_skeleton("a", "=", "1")
        );
        assert_eq!(
            continue97_assignment_skeleton("a", "+=", "b"),
            "a += b"
        );
        assert_eq!(
            continue97_assignment_pretty("x", "*=", "2"),
            continue97_assignment_minify("x", "*=", "2")
        );

        assert_eq!(continue97_update_skeleton("i", "++", true), "++i");
        assert_eq!(continue97_update_skeleton("i", "++", false), "i++");
        assert_eq!(
            continue97_update_skeleton("i", "++", true),
            continue30_update_skeleton("i", "++", true)
        );
        assert_eq!(continue97_update_prefix("--", "n"), "--n");
        assert_eq!(continue97_update_postfix("n", "--"), "n--");
        assert_eq!(
            continue97_update_pretty("k", "++", false),
            continue97_update_minify("k", "++", false)
        );

        assert_eq!(
            continue97_member_skeleton("obj", "x", false, false),
            "obj.x"
        );
        assert_eq!(
            continue97_member_skeleton("obj", "k", true, false),
            "obj[k]"
        );
        assert_eq!(
            continue97_member_skeleton("obj", "x", false, true),
            "obj?.x"
        );
        assert_eq!(
            continue97_member_skeleton("obj", "k", true, true),
            "obj?.[k]"
        );
        assert_eq!(
            continue97_member_skeleton("obj", "x", false, false),
            continue30_member_skeleton("obj", "x", false, false)
        );
        assert_eq!(continue97_member_static("a", "b"), "a.b");
        assert_eq!(continue97_member_computed("a", "0"), "a[0]");
        assert_eq!(continue97_member_optional("a", "b"), "a?.b");
        assert_eq!(
            continue97_member_optional_computed("a", "k"),
            "a?.[k]"
        );
        assert_eq!(
            continue97_member_pretty("o", "p", false, false),
            continue97_member_minify("o", "p", false, false)
        );

        assert_eq!(continue97_call_skeleton("f", "1, 2", false), "f(1, 2)");
        assert_eq!(continue97_call_skeleton("f", "", true), "f?.()");
        assert_eq!(
            continue97_call_skeleton("f", "x", false),
            continue30_call_skeleton("f", "x", false)
        );
        assert_eq!(continue97_call("g", "a"), "g(a)");
        assert_eq!(continue97_optional_call("g", "a"), "g?.(a)");
        assert_eq!(
            continue97_call_pretty("h", "", false),
            continue97_call_minify("h", "", false)
        );

        assert_eq!(continue97_new_skeleton("Foo", "a"), "new Foo(a)");
        assert_eq!(
            continue97_new_skeleton("Foo", "a"),
            continue30_new_skeleton("Foo", "a")
        );
        assert_eq!(
            continue97_new_pretty("Bar", ""),
            continue97_new_minify("Bar", "")
        );
    }

    #[test]
    fn continue97_composed_residual_shells() {
        assert_eq!(continue97_assign_eq("x", "1"), "x = 1");
        assert_eq!(continue97_assign_add_eq("x", "1"), "x += 1");
        assert_eq!(continue97_pre_increment("i"), "++i");
        assert_eq!(continue97_post_increment("i"), "i++");
        assert_eq!(continue97_pre_decrement("i"), "--i");
        assert_eq!(continue97_post_decrement("i"), "i--");
        assert_eq!(
            continue97_method_call("obj", "map", "fn"),
            "obj.map(fn)"
        );
        assert_eq!(
            continue97_optional_method_call("obj", "map", "fn"),
            "obj?.map?.(fn)"
        );
        assert_eq!(continue97_new_empty("Map"), "new Map()");
        assert_eq!(continue97_assign_call("x", "f", "1"), "x = f(1)");
        assert_eq!(
            continue97_assign_new("x", "Foo", "a"),
            "x = new Foo(a)"
        );
        assert_eq!(
            continue97_member_assign("obj", "x", "1"),
            "obj.x = 1"
        );
        assert_eq!(
            continue97_computed_member_assign("obj", "k", "1"),
            "obj[k] = 1"
        );
        assert_eq!(continue97_stmt_sep(true), " ");
        assert_eq!(continue97_stmt_sep(false), "");
        assert_ne!(continue97_stmt_sep(true), continue97_stmt_sep(false));
    }
}

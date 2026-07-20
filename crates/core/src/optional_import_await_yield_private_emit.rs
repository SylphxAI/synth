//! Pure OptionalMemberExpression + OptionalCallExpression + ChainExpression +
//! ImportExpression + AwaitExpression + YieldExpression + PrivateIdentifier +
//! PrivateName dual-oracle emission — residual pure **continue115** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–114:
//! - OptionalMemberExpression dual-oracle composing real
//!   `continue48_optional_member_skeleton`
//! - OptionalCallExpression dual-oracle composing real
//!   `continue48_optional_call_skeleton`
//! - ChainExpression dual-oracle wrapping optional member/call seeds
//! - ImportExpression dual-oracle composing real
//!   `continue48_import_expression_skeleton`
//! - AwaitExpression dual-oracle composing real `continue48_await_skeleton`
//! - YieldExpression dual-oracle composing real `continue48_yield_skeleton`
//! - PrivateIdentifier / PrivateName dual-oracle composing real
//!   `continue48_private_id_skeleton`
//! - Composed optional/import/await/yield/private residual shells
//!
//! Intentionally does **not** re-wrap continue114 class/import/export/new/this/
//! super/meta poles (continue47 bases), continue113 for/throw/label/empty poles
//! (continue46 bases), or continue112 do-while/switch/break/continue/try poles
//! (continue45 bases). Composes real shipped pure helpers from continue48
//! optional/await/yield/private bases. Full engines remain product residual. NO
//! authority_rust / ts_deleted. pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue48_await_skeleton, continue48_import_expression_skeleton,
    continue48_optional_call_skeleton, continue48_optional_member_skeleton,
    continue48_private_id_skeleton, continue48_yield_skeleton,
};

/// Dual-oracle residual: continue115 related AST type catalog.
pub const CONTINUE115_RELATED_TYPES: &[&str] = &[
    "OptionalMemberExpression",
    "OptionalCallExpression",
    "ChainExpression",
    "ImportExpression",
    "AwaitExpression",
    "YieldExpression",
    "PrivateIdentifier",
    "PrivateName",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_optional_import_await_yield_private_related_type(t: &str) -> bool {
    CONTINUE115_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue115_optional_member_type(t: &str) -> bool {
    t == "OptionalMemberExpression"
}

#[must_use]
pub fn is_continue115_optional_call_type(t: &str) -> bool {
    t == "OptionalCallExpression"
}

#[must_use]
pub fn is_continue115_chain_type(t: &str) -> bool {
    t == "ChainExpression"
}

#[must_use]
pub fn is_continue115_import_expression_type(t: &str) -> bool {
    t == "ImportExpression"
}

#[must_use]
pub fn is_continue115_await_type(t: &str) -> bool {
    t == "AwaitExpression"
}

#[must_use]
pub fn is_continue115_yield_type(t: &str) -> bool {
    t == "YieldExpression"
}

#[must_use]
pub fn is_continue115_private_id_type(t: &str) -> bool {
    t == "PrivateIdentifier"
}

#[must_use]
pub fn is_continue115_private_name_type(t: &str) -> bool {
    t == "PrivateName"
}

#[must_use]
pub fn is_continue115_private_type(t: &str) -> bool {
    matches!(t, "PrivateIdentifier" | "PrivateName")
}

#[must_use]
pub fn is_continue115_optional_type(t: &str) -> bool {
    matches!(
        t,
        "OptionalMemberExpression" | "OptionalCallExpression" | "ChainExpression"
    )
}

#[must_use]
pub fn is_continue115_type(t: &str) -> bool {
    matches!(
        t,
        "OptionalMemberExpression"
            | "OptionalCallExpression"
            | "ChainExpression"
            | "ImportExpression"
            | "AwaitExpression"
            | "YieldExpression"
            | "PrivateIdentifier"
            | "PrivateName"
    )
}

// ── OptionalMemberExpression dual-oracle ────────────────────────────────────

/// Dual-oracle OptionalMemberExpression skeleton composing real
/// [`continue48_optional_member_skeleton`].
#[must_use]
pub fn continue115_optional_member_skeleton(obj: &str, prop: &str) -> String {
    continue48_optional_member_skeleton(obj, prop)
}

/// Dual-oracle OptionalMemberExpression pretty alias.
#[must_use]
pub fn continue115_optional_member_pretty(obj: &str, prop: &str) -> String {
    continue115_optional_member_skeleton(obj, prop)
}

/// Dual-oracle OptionalMemberExpression minify alias.
#[must_use]
pub fn continue115_optional_member_minify(obj: &str, prop: &str) -> String {
    continue115_optional_member_skeleton(obj, prop)
}

// ── OptionalCallExpression dual-oracle ──────────────────────────────────────

/// Dual-oracle OptionalCallExpression skeleton composing real
/// [`continue48_optional_call_skeleton`].
#[must_use]
pub fn continue115_optional_call_skeleton(callee: &str, args: &str) -> String {
    continue48_optional_call_skeleton(callee, args)
}

/// Dual-oracle OptionalCallExpression pretty alias.
#[must_use]
pub fn continue115_optional_call_pretty(callee: &str, args: &str) -> String {
    continue115_optional_call_skeleton(callee, args)
}

/// Dual-oracle OptionalCallExpression minify alias.
#[must_use]
pub fn continue115_optional_call_minify(callee: &str, args: &str) -> String {
    continue115_optional_call_skeleton(callee, args)
}

// ── ChainExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ChainExpression transparent wrap of an optional seed.
/// ChainExpression is a pure wrapper in tooling emit — the seed is the body.
#[must_use]
pub fn continue115_chain_expression_skeleton(seed: &str) -> String {
    seed.to_string()
}

/// Dual-oracle ChainExpression pretty alias.
#[must_use]
pub fn continue115_chain_expression_pretty(seed: &str) -> String {
    continue115_chain_expression_skeleton(seed)
}

/// Dual-oracle ChainExpression minify alias.
#[must_use]
pub fn continue115_chain_expression_minify(seed: &str) -> String {
    continue115_chain_expression_skeleton(seed)
}

// ── ImportExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle ImportExpression skeleton composing real
/// [`continue48_import_expression_skeleton`].
#[must_use]
pub fn continue115_import_expression_skeleton(source: &str) -> String {
    continue48_import_expression_skeleton(source)
}

/// Dual-oracle ImportExpression pretty alias.
#[must_use]
pub fn continue115_import_expression_pretty(source: &str) -> String {
    continue115_import_expression_skeleton(source)
}

/// Dual-oracle ImportExpression minify alias.
#[must_use]
pub fn continue115_import_expression_minify(source: &str) -> String {
    continue115_import_expression_skeleton(source)
}

// ── AwaitExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle AwaitExpression skeleton composing real
/// [`continue48_await_skeleton`].
#[must_use]
pub fn continue115_await_skeleton(expr: &str) -> String {
    continue48_await_skeleton(expr)
}

/// Dual-oracle AwaitExpression pretty alias.
#[must_use]
pub fn continue115_await_pretty(expr: &str) -> String {
    continue115_await_skeleton(expr)
}

/// Dual-oracle AwaitExpression minify alias.
#[must_use]
pub fn continue115_await_minify(expr: &str) -> String {
    continue115_await_skeleton(expr)
}

// ── YieldExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle YieldExpression skeleton composing real
/// [`continue48_yield_skeleton`].
#[must_use]
pub fn continue115_yield_skeleton(expr: Option<&str>) -> String {
    continue48_yield_skeleton(expr)
}

/// Dual-oracle YieldExpression pretty alias.
#[must_use]
pub fn continue115_yield_pretty(expr: Option<&str>) -> String {
    continue115_yield_skeleton(expr)
}

/// Dual-oracle YieldExpression minify alias.
#[must_use]
pub fn continue115_yield_minify(expr: Option<&str>) -> String {
    continue115_yield_skeleton(expr)
}

// ── PrivateIdentifier dual-oracle ───────────────────────────────────────────

/// Dual-oracle PrivateIdentifier skeleton composing real
/// [`continue48_private_id_skeleton`].
#[must_use]
pub fn continue115_private_id_skeleton(name: &str) -> String {
    continue48_private_id_skeleton(name)
}

/// Dual-oracle PrivateIdentifier pretty alias.
#[must_use]
pub fn continue115_private_id_pretty(name: &str) -> String {
    continue115_private_id_skeleton(name)
}

/// Dual-oracle PrivateIdentifier minify alias.
#[must_use]
pub fn continue115_private_id_minify(name: &str) -> String {
    continue115_private_id_skeleton(name)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: chain of optional member.
#[must_use]
pub fn continue115_chain_optional_member(obj: &str, prop: &str) -> String {
    continue115_chain_expression_skeleton(&continue115_optional_member_skeleton(obj, prop))
}

/// Dual-oracle residual: chain of optional call.
#[must_use]
pub fn continue115_chain_optional_call(callee: &str, args: &str) -> String {
    continue115_chain_expression_skeleton(&continue115_optional_call_skeleton(callee, args))
}

/// Dual-oracle residual: await of dynamic import.
#[must_use]
pub fn continue115_await_import(source: &str) -> String {
    let import = continue115_import_expression_skeleton(source);
    continue115_await_skeleton(&import)
}

/// Dual-oracle residual: yield of optional member.
#[must_use]
pub fn continue115_yield_optional_member(obj: &str, prop: &str) -> String {
    let member = continue115_optional_member_skeleton(obj, prop);
    continue115_yield_skeleton(Some(&member))
}

/// Dual-oracle residual: bare yield seed.
#[must_use]
pub fn continue115_yield_bare() -> String {
    continue115_yield_skeleton(None)
}

/// Dual-oracle residual: await of optional call.
#[must_use]
pub fn continue115_await_optional_call(callee: &str, args: &str) -> String {
    let call = continue115_optional_call_skeleton(callee, args);
    continue115_await_skeleton(&call)
}

/// Dual-oracle residual: optional member then optional call compose.
#[must_use]
pub fn continue115_optional_member_then_call(
    obj: &str,
    prop: &str,
    args: &str,
) -> String {
    let member = continue115_optional_member_skeleton(obj, prop);
    continue115_optional_call_skeleton(&member, args)
}

/// Dual-oracle residual: private field access via optional member (`obj?.#x`).
#[must_use]
pub fn continue115_optional_private_member(obj: &str, name: &str) -> String {
    let private = continue115_private_id_skeleton(name);
    // Optional member of private keeps `?.` + `#name` (no extra dot: skeleton joins `?.{prop}`).
    continue115_optional_member_skeleton(obj, private.trim_start_matches('#'))
        .replacen(
            &format!("?.{}", private.trim_start_matches('#')),
            &format!("?.{private}"),
            1,
        )
}

/// Dual-oracle residual: private id alone seed.
#[must_use]
pub fn continue115_private_field(name: &str) -> String {
    continue115_private_id_skeleton(name)
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue115_sep(pretty: bool) -> &'static str {
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
        continue48_await_skeleton, continue48_import_expression_skeleton,
        continue48_optional_call_skeleton, continue48_optional_member_skeleton,
        continue48_private_id_skeleton, continue48_yield_skeleton,
    };

    #[test]
    fn continue115_type_catalog() {
        assert_eq!(CONTINUE115_RELATED_TYPES.len(), 8);
        assert!(is_optional_import_await_yield_private_related_type(
            "OptionalMemberExpression"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "OptionalCallExpression"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "ChainExpression"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "ImportExpression"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "AwaitExpression"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "YieldExpression"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "PrivateIdentifier"
        ));
        assert!(is_optional_import_await_yield_private_related_type(
            "PrivateName"
        ));
        assert!(!is_optional_import_await_yield_private_related_type(
            "ClassDeclaration"
        ));
        assert!(!is_optional_import_await_yield_private_related_type(
            "ForStatement"
        ));

        assert!(is_continue115_optional_member_type(
            "OptionalMemberExpression"
        ));
        assert!(!is_continue115_optional_member_type(
            "OptionalCallExpression"
        ));
        assert!(is_continue115_optional_call_type("OptionalCallExpression"));
        assert!(is_continue115_chain_type("ChainExpression"));
        assert!(is_continue115_import_expression_type("ImportExpression"));
        assert!(is_continue115_await_type("AwaitExpression"));
        assert!(is_continue115_yield_type("YieldExpression"));
        assert!(is_continue115_private_id_type("PrivateIdentifier"));
        assert!(is_continue115_private_name_type("PrivateName"));
        assert!(is_continue115_private_type("PrivateIdentifier"));
        assert!(is_continue115_private_type("PrivateName"));
        assert!(!is_continue115_private_type("Identifier"));
        assert!(is_continue115_optional_type("OptionalMemberExpression"));
        assert!(is_continue115_optional_type("OptionalCallExpression"));
        assert!(is_continue115_optional_type("ChainExpression"));
        assert!(!is_continue115_optional_type("ImportExpression"));
        assert!(is_continue115_type("OptionalMemberExpression"));
        assert!(is_continue115_type("PrivateName"));
        assert!(!is_continue115_type("ClassDeclaration"));
        assert!(!is_continue115_type("DoWhileStatement"));
    }

    #[test]
    fn continue115_optional_import_await_emit() {
        assert_eq!(
            continue115_optional_member_skeleton("obj", "x"),
            "obj?.x"
        );
        assert_eq!(
            continue115_optional_member_skeleton("obj", "x"),
            continue48_optional_member_skeleton("obj", "x")
        );
        assert_eq!(
            continue115_optional_member_pretty("a", "b"),
            continue115_optional_member_minify("a", "b")
        );

        assert_eq!(
            continue115_optional_call_skeleton("fn", "1"),
            "fn?.(1)"
        );
        assert_eq!(
            continue115_optional_call_skeleton("fn", "1, 2"),
            "fn?.(1, 2)"
        );
        assert_eq!(
            continue115_optional_call_skeleton("fn", "1"),
            continue48_optional_call_skeleton("fn", "1")
        );
        assert_eq!(
            continue115_optional_call_pretty("g", "a"),
            continue115_optional_call_minify("g", "a")
        );

        assert_eq!(
            continue115_chain_expression_skeleton("obj?.x"),
            "obj?.x"
        );
        assert_eq!(
            continue115_chain_expression_pretty("x?.(1)"),
            continue115_chain_expression_minify("x?.(1)")
        );

        assert_eq!(
            continue115_import_expression_skeleton("./m.js"),
            "import(\"./m.js\")"
        );
        assert_eq!(
            continue115_import_expression_skeleton("./m.js"),
            continue48_import_expression_skeleton("./m.js")
        );
        assert_eq!(
            continue115_import_expression_pretty("./z"),
            continue115_import_expression_minify("./z")
        );

        assert_eq!(continue115_await_skeleton("p"), "await p");
        assert_eq!(
            continue115_await_skeleton("p"),
            continue48_await_skeleton("p")
        );
        assert_eq!(
            continue115_await_pretty("x"),
            continue115_await_minify("x")
        );
    }

    #[test]
    fn continue115_yield_private_emit() {
        assert_eq!(continue115_yield_skeleton(Some("1")), "yield 1");
        assert_eq!(continue115_yield_skeleton(None), "yield");
        assert_eq!(
            continue115_yield_skeleton(Some("1")),
            continue48_yield_skeleton(Some("1"))
        );
        assert_eq!(
            continue115_yield_pretty(None),
            continue115_yield_minify(None)
        );

        assert_eq!(continue115_private_id_skeleton("x"), "#x");
        assert_eq!(
            continue115_private_id_skeleton("x"),
            continue48_private_id_skeleton("x")
        );
        assert_eq!(
            continue115_private_id_pretty("id"),
            continue115_private_id_minify("id")
        );
    }

    #[test]
    fn continue115_composed_residual_shells() {
        assert_eq!(
            continue115_chain_optional_member("obj", "prop"),
            "obj?.prop"
        );
        assert_eq!(
            continue115_chain_optional_call("fn", "a, b"),
            "fn?.(a, b)"
        );
        assert_eq!(
            continue115_await_import("./mod.js"),
            "await import(\"./mod.js\")"
        );
        assert_eq!(
            continue115_yield_optional_member("obj", "x"),
            "yield obj?.x"
        );
        assert_eq!(continue115_yield_bare(), "yield");
        assert_eq!(
            continue115_await_optional_call("load", ""),
            "await load?.()"
        );
        assert_eq!(
            continue115_optional_member_then_call("obj", "run", "1"),
            "obj?.run?.(1)"
        );
        assert_eq!(continue115_private_field("secret"), "#secret");
        assert_eq!(
            continue115_optional_private_member("obj", "x"),
            "obj?.#x"
        );
        assert_eq!(continue115_sep(true), " ");
        assert_eq!(continue115_sep(false), "");
        assert_ne!(continue115_sep(true), continue115_sep(false));
    }
}

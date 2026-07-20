//! Pure CallExpression + MemberExpression + NewExpression + ArrayExpression
//! dual-oracle emission — residual pure **continue77** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - CallExpression full dual-oracle pretty/minify arg-list spacing
//!   (continue8 `call_skeleton` always used `", "` — compressor residual uses bare `,`)
//! - MemberExpression static/computed dual-oracle composing real `member_access_*`
//! - NewExpression dual-oracle driving real `new_expression_skeleton` /
//!   `new_keyword` (pretty `new Foo(a, b)` vs minify `newFoo(a,b)`)
//! - ArrayExpression dual-oracle pretty/minify element join + optional trailing
//!   comma (continue8 `array_skeleton` always used `", "`)
//!
//! Intentionally does **not** re-wrap continue64–76 partition shells
//! (optional-chain continue73 stays separate; binary/unary continue76 stays separate).
//! Composes real shipped pure helpers from `call_member_emit` + `new_await_chain_emit`.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::call_member_emit::{
    call_args_close, call_args_open, member_access_close, member_access_open,
};
use crate::new_await_chain_emit::{new_expression_skeleton, new_keyword};

/// Dual-oracle residual: continue77 related AST type catalog.
pub const CONTINUE77_RELATED_TYPES: &[&str] = &[
    "CallExpression",
    "MemberExpression",
    "NewExpression",
    "ArrayExpression",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_call_member_new_related_type(t: &str) -> bool {
    CONTINUE77_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue77_call_type(t: &str) -> bool {
    t == "CallExpression"
}

#[must_use]
pub fn is_continue77_member_type(t: &str) -> bool {
    t == "MemberExpression"
}

#[must_use]
pub fn is_continue77_new_type(t: &str) -> bool {
    t == "NewExpression"
}

#[must_use]
pub fn is_continue77_array_type(t: &str) -> bool {
    t == "ArrayExpression"
}

// ── List separators (printer vs compressor residual) ────────────────────────

/// Dual-oracle residual: list element separator.
/// - pretty: `", "` (printer residual)
/// - minify: `","` (compressor residual)
#[must_use]
pub fn continue77_list_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Dual-oracle residual: join already-rendered fragments with pretty/minify sep.
#[must_use]
pub fn continue77_join_list(parts: &[&str], pretty: bool) -> String {
    parts.join(continue77_list_sep(pretty))
}

// ── CallExpression dual-oracle ──────────────────────────────────────────────

/// Dual-oracle CallExpression skeleton: `callee(arg, …)`.
///
/// Widens continue8 `call_skeleton` (always `", "`) with real pretty/minify
/// arg-list spacing dual-oracle.
#[must_use]
pub fn continue77_call_expression_skeleton(callee: &str, args: &[&str], pretty: bool) -> String {
    format!(
        "{callee}{}{}{}",
        call_args_open(),
        continue77_join_list(args, pretty),
        call_args_close()
    )
}

/// Convenience: pretty call.
#[must_use]
pub fn call_expression_pretty(callee: &str, args: &[&str]) -> String {
    continue77_call_expression_skeleton(callee, args, true)
}

/// Convenience: minify call.
#[must_use]
pub fn call_expression_minify(callee: &str, args: &[&str]) -> String {
    continue77_call_expression_skeleton(callee, args, false)
}

// ── MemberExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle MemberExpression skeleton.
///
/// Static: `obj.prop`; computed: `obj[prop]`. Pretty/minify share the same
/// placement (no spacing residual) — dual-oracle via computed flag + shipped
/// `member_access_open` / `member_access_close`.
#[must_use]
pub fn continue77_member_expression_skeleton(
    object: &str,
    property: &str,
    computed: bool,
) -> String {
    format!(
        "{object}{}{property}{}",
        member_access_open(computed),
        member_access_close(computed)
    )
}

/// Convenience: static member.
#[must_use]
pub fn member_expression_static(object: &str, property: &str) -> String {
    continue77_member_expression_skeleton(object, property, false)
}

/// Convenience: computed member.
#[must_use]
pub fn member_expression_computed(object: &str, property: &str) -> String {
    continue77_member_expression_skeleton(object, property, true)
}

/// Dual-oracle residual: member then call (`obj.m(a)` / `obj.m(a)` minify args).
#[must_use]
pub fn continue77_member_then_call(
    object: &str,
    property: &str,
    computed: bool,
    args: &[&str],
    pretty: bool,
) -> String {
    let member = continue77_member_expression_skeleton(object, property, computed);
    continue77_call_expression_skeleton(&member, args, pretty)
}

// ── NewExpression dual-oracle ───────────────────────────────────────────────

/// Dual-oracle NewExpression skeleton.
///
/// Drives real shipped [`new_expression_skeleton`] / [`new_keyword`]:
/// - pretty: `new Foo(a, b)`
/// - minify: `newFoo(a,b)`
#[must_use]
pub fn continue77_new_expression_skeleton(callee: &str, args: &[&str], pretty: bool) -> String {
    new_expression_skeleton(callee, args, pretty)
}

/// Convenience: pretty new.
#[must_use]
pub fn new_expression_pretty(callee: &str, args: &[&str]) -> String {
    continue77_new_expression_skeleton(callee, args, true)
}

/// Convenience: minify new.
#[must_use]
pub fn new_expression_minify(callee: &str, args: &[&str]) -> String {
    continue77_new_expression_skeleton(callee, args, false)
}

/// Dual-oracle residual: bare `new` keyword token via shipped helper.
#[must_use]
pub fn continue77_new_keyword(pretty: bool) -> &'static str {
    new_keyword(pretty)
}

// ── ArrayExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle ArrayExpression skeleton: `[a, b]` / `[a,b]` + optional trailing comma.
///
/// Widens continue8 `array_skeleton` (always `", "`) with real pretty/minify
/// element join dual-oracle.
#[must_use]
pub fn continue77_array_expression_skeleton(
    elements: &[&str],
    pretty: bool,
    trailing_comma: bool,
) -> String {
    let mut interior = continue77_join_list(elements, pretty);
    if trailing_comma && !elements.is_empty() {
        interior.push(',');
    }
    format!("[{interior}]")
}

/// Convenience: pretty array (no trailing comma).
#[must_use]
pub fn array_expression_pretty(elements: &[&str]) -> String {
    continue77_array_expression_skeleton(elements, true, false)
}

/// Convenience: minify array (no trailing comma).
#[must_use]
pub fn array_expression_minify(elements: &[&str]) -> String {
    continue77_array_expression_skeleton(elements, false, false)
}

/// Convenience: pretty array with trailing comma residual.
#[must_use]
pub fn array_expression_pretty_trailing(elements: &[&str]) -> String {
    continue77_array_expression_skeleton(elements, true, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call_member_emit::{call_skeleton, member_skeleton};
    use crate::new_await_chain_emit::{new_expression_skeleton, new_keyword};

    #[test]
    fn continue77_type_catalog() {
        assert_eq!(CONTINUE77_RELATED_TYPES.len(), 4);
        assert!(is_call_member_new_related_type("CallExpression"));
        assert!(is_call_member_new_related_type("MemberExpression"));
        assert!(is_call_member_new_related_type("NewExpression"));
        assert!(is_call_member_new_related_type("ArrayExpression"));
        assert!(!is_call_member_new_related_type("OptionalCallExpression"));
        assert!(!is_call_member_new_related_type("BinaryExpression"));
        assert!(is_continue77_call_type("CallExpression"));
        assert!(is_continue77_member_type("MemberExpression"));
        assert!(is_continue77_new_type("NewExpression"));
        assert!(is_continue77_array_type("ArrayExpression"));
        assert!(!is_continue77_call_type("NewExpression"));
        assert!(!is_continue77_array_type("CallExpression"));
    }

    #[test]
    fn continue77_call_expression_dual_oracle() {
        // empty args identical
        assert_eq!(call_expression_pretty("foo", &[]), "foo()");
        assert_eq!(call_expression_minify("foo", &[]), "foo()");
        // pretty spaces after commas
        assert_eq!(call_expression_pretty("foo", &["1", "2"]), "foo(1, 2)");
        // minify glues commas (widens continue8 fixed ", ")
        assert_eq!(call_expression_minify("foo", &["1", "2"]), "foo(1,2)");
        assert_eq!(
            continue77_call_expression_skeleton("bar.baz", &["a", "b", "c"], false),
            "bar.baz(a,b,c)"
        );
        // pretty path matches continue8 call_skeleton for multi-arg
        assert_eq!(
            call_expression_pretty("foo", &["1", "2"]),
            call_skeleton("foo", &["1", "2"])
        );
        // minify path diverges from continue8 fixed sep
        assert_ne!(
            call_expression_minify("foo", &["1", "2"]),
            call_skeleton("foo", &["1", "2"])
        );
        assert_eq!(continue77_list_sep(true), ", ");
        assert_eq!(continue77_list_sep(false), ",");
        assert_eq!(continue77_join_list(&["x", "y"], true), "x, y");
        assert_eq!(continue77_join_list(&["x", "y"], false), "x,y");
    }

    #[test]
    fn continue77_member_and_new_dual_oracle() {
        assert_eq!(member_expression_static("obj", "prop"), "obj.prop");
        assert_eq!(member_expression_computed("obj", "k"), "obj[k]");
        // drives shipped member_access tokens (same as continue8 member_skeleton)
        assert_eq!(
            continue77_member_expression_skeleton("a", "b", false),
            member_skeleton("a", "b", false)
        );
        assert_eq!(
            continue77_member_expression_skeleton("a", "b", true),
            member_skeleton("a", "b", true)
        );
        assert_eq!(
            continue77_member_then_call("obj", "m", false, &["1", "2"], true),
            "obj.m(1, 2)"
        );
        assert_eq!(
            continue77_member_then_call("obj", "m", false, &["1", "2"], false),
            "obj.m(1,2)"
        );
        assert_eq!(
            continue77_member_then_call("obj", "k", true, &["x"], false),
            "obj[k](x)"
        );

        // new dual-oracle drives shipped skeleton
        assert_eq!(new_expression_pretty("Foo", &["a", "b"]), "new Foo(a, b)");
        assert_eq!(new_expression_minify("Foo", &["a", "b"]), "newFoo(a,b)");
        assert_eq!(
            continue77_new_expression_skeleton("Foo", &["a", "b"], true),
            new_expression_skeleton("Foo", &["a", "b"], true)
        );
        assert_eq!(
            continue77_new_expression_skeleton("Foo", &["a", "b"], false),
            new_expression_skeleton("Foo", &["a", "b"], false)
        );
        assert_eq!(continue77_new_keyword(true), new_keyword(true));
        assert_eq!(continue77_new_keyword(false), new_keyword(false));
        assert_eq!(new_expression_pretty("X", &[]), "new X()");
        assert_eq!(new_expression_minify("X", &[]), "newX()");
    }

    #[test]
    fn continue77_array_expression_dual_oracle() {
        assert_eq!(array_expression_pretty(&[]), "[]");
        assert_eq!(array_expression_minify(&[]), "[]");
        assert_eq!(array_expression_pretty(&["a", "b"]), "[a, b]");
        assert_eq!(array_expression_minify(&["a", "b"]), "[a,b]");
        assert_eq!(
            continue77_array_expression_skeleton(&["a", "b", "c"], false, false),
            "[a,b,c]"
        );
        // trailing comma residual
        assert_eq!(array_expression_pretty_trailing(&["a"]), "[a,]");
        assert_eq!(
            continue77_array_expression_skeleton(&["a", "b"], true, true),
            "[a, b,]"
        );
        assert_eq!(
            continue77_array_expression_skeleton(&["a", "b"], false, true),
            "[a,b,]"
        );
        // empty + trailing still empty
        assert_eq!(
            continue77_array_expression_skeleton(&[], true, true),
            "[]"
        );
    }
}

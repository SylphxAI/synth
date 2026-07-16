//! Pure OptionalMemberExpression + OptionalCallExpression + ChainExpression
//! dual-oracle emission —
//! residual pure **continue73** for tooling/format-minify-lint fragment.
//!
//! New AST emit skeletons **not** covered by prior dens modules:
//! - OptionalMemberExpression full dual-oracle (static `?.` vs computed `?.[`,
//!   optional flag on/off) — widens fixed continue48 single non-computed form
//! - OptionalCallExpression with real arg list + pretty/minify seps
//!   (prior shells used a single pre-joined `args` string)
//! - ChainExpression transparent wrap composed with optional member/call
//!
//! Intentionally does **not** re-wrap continue64–72 partition shells.
//! Composes real shipped pure helpers (`optional_chain_open`,
//! `optional_chain_close`, `optional_chain`, `function_param_sep`,
//! `call_args_open`/`close`).
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::call_member_emit::{call_args_close, call_args_open};
use crate::function_decl_emit::function_param_sep;
use crate::member_access_kind::{optional_chain_close, optional_chain_open};

/// Dual-oracle residual: continue73 related AST type catalog.
pub const CONTINUE73_RELATED_TYPES: &[&str] = &[
    "OptionalMemberExpression",
    "OptionalCallExpression",
    "ChainExpression",
    "MemberExpression",
    "CallExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_optional_chain_related_type(t: &str) -> bool {
    CONTINUE73_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue73_optional_member_type(t: &str) -> bool {
    t == "OptionalMemberExpression"
}

#[must_use]
pub fn is_continue73_optional_call_type(t: &str) -> bool {
    t == "OptionalCallExpression"
}

#[must_use]
pub fn is_continue73_chain_expression_type(t: &str) -> bool {
    t == "ChainExpression"
}

#[must_use]
pub fn is_continue73_member_type(t: &str) -> bool {
    t == "MemberExpression"
}

#[must_use]
pub fn is_continue73_call_type(t: &str) -> bool {
    t == "CallExpression"
}

// ── arg list interior (pretty / minify) ─────────────────────────────────────

/// Dual-oracle arg list interior for call forms.
/// Pretty: `, `; minify: `,`.
#[must_use]
pub fn arg_list_interior(args: &[&str], pretty: bool) -> String {
    let sep = function_param_sep(pretty);
    let mut out = String::with_capacity(
        args.iter().map(|a| a.len()).sum::<usize>() + sep.len() * args.len().saturating_sub(1),
    );
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(a);
    }
    out
}

// ── OptionalMemberExpression dual-oracle ────────────────────────────────────

/// Dual-oracle OptionalMemberExpression / MemberExpression skeleton.
///
/// - optional+static: `obj?.prop`
/// - optional+computed: `obj?.[expr]`
/// - non-optional+static: `obj.prop`
/// - non-optional+computed: `obj[expr]`
///
/// Drives real shipped: [`optional_chain_open`], [`optional_chain_close`].
#[must_use]
pub fn optional_member_skeleton(
    object: &str,
    property: &str,
    optional: bool,
    computed: bool,
) -> String {
    let mut out = String::with_capacity(object.len() + property.len() + 4);
    out.push_str(object);
    out.push_str(optional_chain_open(optional, computed));
    out.push_str(property);
    out.push_str(optional_chain_close(computed));
    out
}

/// Convenience: optional static member `obj?.prop`.
#[must_use]
pub fn optional_member_static(object: &str, property: &str) -> String {
    optional_member_skeleton(object, property, true, false)
}

/// Convenience: optional computed member `obj?.[expr]`.
#[must_use]
pub fn optional_member_computed(object: &str, property: &str) -> String {
    optional_member_skeleton(object, property, true, true)
}

// ── OptionalCallExpression dual-oracle ──────────────────────────────────────

/// Dual-oracle OptionalCallExpression open between callee and args.
/// Optional: `?.(` ; non-optional: `(`.
#[must_use]
pub fn optional_call_open(optional: bool) -> &'static str {
    if optional {
        "?.("
    } else {
        call_args_open()
    }
}

/// Dual-oracle OptionalCallExpression skeleton with real arg list seps.
///
/// Pretty optional: `fn?.(a, b)`
/// Minify optional: `fn?.(a,b)`
/// Non-optional call residual: `fn(a, b)` / `fn(a,b)`
///
/// Drives real shipped: [`optional_chain`] token presence, [`function_param_sep`],
/// [`call_args_close`].
#[must_use]
pub fn optional_call_skeleton(
    callee: &str,
    args: &[&str],
    optional: bool,
    pretty: bool,
) -> String {
    let interior = arg_list_interior(args, pretty);
    let mut out = String::with_capacity(callee.len() + interior.len() + 4);
    out.push_str(callee);
    out.push_str(optional_call_open(optional));
    out.push_str(&interior);
    out.push_str(call_args_close());
    out
}

/// Convenience: pretty optional call.
#[must_use]
pub fn optional_call_pretty(callee: &str, args: &[&str]) -> String {
    optional_call_skeleton(callee, args, true, true)
}

/// Convenience: minify optional call.
#[must_use]
pub fn optional_call_minify(callee: &str, args: &[&str]) -> String {
    optional_call_skeleton(callee, args, true, false)
}

// ── ChainExpression dual-oracle ─────────────────────────────────────────────

/// ChainExpression is a transparent wrapper in print/compress — emit inner as-is.
/// Dual-oracle residual: identity over already-rendered optional chain interior.
/// Named continue73_* to avoid clashing with continue16 `chain_expression_skeleton`.
#[must_use]
pub fn continue73_chain_expression_skeleton(inner: &str) -> String {
    inner.to_owned()
}

/// Dual-oracle: compose optional member inside ChainExpression.
#[must_use]
pub fn chain_optional_member(
    object: &str,
    property: &str,
    optional: bool,
    computed: bool,
) -> String {
    continue73_chain_expression_skeleton(&optional_member_skeleton(
        object, property, optional, computed,
    ))
}

/// Dual-oracle: compose optional call inside ChainExpression.
#[must_use]
pub fn chain_optional_call(callee: &str, args: &[&str], optional: bool, pretty: bool) -> String {
    continue73_chain_expression_skeleton(&optional_call_skeleton(
        callee, args, optional, pretty,
    ))
}

/// Dual-oracle residual: chained optional member then call
/// `obj?.prop?.(a, b)` (caller-owned fragments).
#[must_use]
pub fn optional_member_then_call(
    object: &str,
    property: &str,
    args: &[&str],
    member_computed: bool,
    pretty: bool,
) -> String {
    let member = optional_member_skeleton(object, property, true, member_computed);
    optional_call_skeleton(&member, args, true, pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call_member_emit::{call_args_close, call_args_open, optional_chain};
    use crate::function_decl_emit::function_param_sep;
    use crate::member_access_kind::{optional_chain_close, optional_chain_open};

    #[test]
    fn continue73_type_catalog() {
        assert_eq!(CONTINUE73_RELATED_TYPES.len(), 5);
        assert!(is_optional_chain_related_type("OptionalMemberExpression"));
        assert!(is_optional_chain_related_type("OptionalCallExpression"));
        assert!(is_optional_chain_related_type("ChainExpression"));
        assert!(is_optional_chain_related_type("MemberExpression"));
        assert!(is_optional_chain_related_type("CallExpression"));
        assert!(!is_optional_chain_related_type("BinaryExpression"));
        assert!(!is_optional_chain_related_type("ImportDeclaration"));
        assert!(is_continue73_optional_member_type("OptionalMemberExpression"));
        assert!(is_continue73_optional_call_type("OptionalCallExpression"));
        assert!(is_continue73_chain_expression_type("ChainExpression"));
        assert!(is_continue73_member_type("MemberExpression"));
        assert!(is_continue73_call_type("CallExpression"));
        assert!(!is_continue73_optional_member_type("MemberExpression"));
        assert!(!is_continue73_optional_call_type("CallExpression"));
    }

    #[test]
    fn continue73_optional_member_dual_oracle_drives_shipped() {
        // Real shipped optional chain helpers remain authority for tokens.
        assert_eq!(optional_chain(), "?.");
        assert_eq!(optional_chain_open(true, false), "?.");
        assert_eq!(optional_chain_open(true, true), "?.[");
        assert_eq!(optional_chain_open(false, false), ".");
        assert_eq!(optional_chain_open(false, true), "[");
        assert_eq!(optional_chain_close(true), "]");
        assert_eq!(optional_chain_close(false), "");

        // Optional static
        assert_eq!(optional_member_static("obj", "prop"), "obj?.prop");
        assert_eq!(
            optional_member_skeleton("obj", "prop", true, false),
            "obj?.prop"
        );
        // Optional computed
        assert_eq!(optional_member_computed("obj", "k"), "obj?.[k]");
        assert_eq!(
            optional_member_skeleton("obj", "k", true, true),
            "obj?.[k]"
        );
        // Non-optional residual (MemberExpression path)
        assert_eq!(
            optional_member_skeleton("obj", "prop", false, false),
            "obj.prop"
        );
        assert_eq!(
            optional_member_skeleton("obj", "k", false, true),
            "obj[k]"
        );
        // Nested object
        assert_eq!(
            optional_member_skeleton("a?.b", "c", true, false),
            "a?.b?.c"
        );
    }

    #[test]
    fn continue73_optional_call_dual_oracle() {
        assert_eq!(function_param_sep(true), ", ");
        assert_eq!(function_param_sep(false), ",");
        assert_eq!(call_args_open(), "(");
        assert_eq!(call_args_close(), ")");
        assert_eq!(optional_call_open(true), "?.(");
        assert_eq!(optional_call_open(false), "(");
        assert_eq!(arg_list_interior(&["a", "b"], true), "a, b");
        assert_eq!(arg_list_interior(&["a", "b"], false), "a,b");
        assert_eq!(arg_list_interior(&[], true), "");

        // Pretty optional call
        assert_eq!(optional_call_pretty("fn", &[]), "fn?.()");
        assert_eq!(optional_call_pretty("fn", &["1"]), "fn?.(1)");
        assert_eq!(optional_call_pretty("fn", &["a", "b"]), "fn?.(a, b)");
        // Minify optional call
        assert_eq!(optional_call_minify("fn", &[]), "fn?.()");
        assert_eq!(optional_call_minify("fn", &["a", "b"]), "fn?.(a,b)");
        // Non-optional call residual
        assert_eq!(
            optional_call_skeleton("fn", &["x", "y"], false, true),
            "fn(x, y)"
        );
        assert_eq!(
            optional_call_skeleton("fn", &["x", "y"], false, false),
            "fn(x,y)"
        );
    }

    #[test]
    fn continue73_chain_expression_and_compose() {
        // ChainExpression is transparent
        assert_eq!(continue73_chain_expression_skeleton("obj?.x"), "obj?.x");
        assert_eq!(
            chain_optional_member("obj", "x", true, false),
            "obj?.x"
        );
        assert_eq!(
            chain_optional_member("obj", "k", true, true),
            "obj?.[k]"
        );
        assert_eq!(
            chain_optional_call("fn", &["a"], true, true),
            "fn?.(a)"
        );
        assert_eq!(
            chain_optional_call("fn", &["a", "b"], true, false),
            "fn?.(a,b)"
        );

        // Compose member then call
        assert_eq!(
            optional_member_then_call("obj", "run", &["x"], false, true),
            "obj?.run?.(x)"
        );
        assert_eq!(
            optional_member_then_call("obj", "k", &["a", "b"], true, false),
            "obj?.[k]?.(a,b)"
        );
        assert_eq!(
            optional_member_then_call("obj", "run", &[], false, true),
            "obj?.run?.()"
        );
    }
}

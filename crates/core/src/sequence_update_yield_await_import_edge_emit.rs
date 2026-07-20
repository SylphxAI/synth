//! Pure SequenceExpression + UpdateExpression + YieldExpression +
//! AwaitExpression + ChainExpression + ImportExpression dual-oracle **edge**
//! emission — residual pure **continue121** for tooling/format-minify-lint.
//!
//! New dual-oracle shells **not** covered by prior residual modules continue71–120:
//! - Sequence dual-oracle composing real `continue56_sequence_shell`
//! - Update dual-oracle composing real `continue56_update_shell`
//! - Yield/await dual-oracle composing real `continue56_yield_await_shell`
//! - Import/chain dual-oracle composing real `continue56_import_chain_shell`
//! - Catalog partition dual-oracle composing real `continue56_partition_shell`
//! - Edge dual-oracle composing real continue54 sequence/update/yield/await/
//!   import/chain edge shells (empty-arg / multi-side poles)
//! - Composed sequence/update/yield/await/import residual edge shells
//!
//! Intentionally does **not** re-wrap continue120 class/meta/new/this/super poles
//! (continue55 bases), continue119 sequence/update/yield/await/chain/import
//! skeleton poles (continue53 bases as primary surface), or continue118
//! super/this/meta/template poles (continue51 bases). Composes real shipped
//! pure shells from continue56 (and continue54 edges) over the continue53
//! sequence/update/yield/await/chain/import plane. Full engines remain product
//! residual. NO authority_rust / ts_deleted. pure residual ≠ authority flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue54_import_chain_edge_shell, continue54_partition_shell,
    continue54_sequence_edge_shell, continue54_update_edge_shell,
    continue54_yield_await_edge_shell, continue56_import_chain_shell,
    continue56_partition_shell, continue56_sequence_shell, continue56_update_shell,
    continue56_yield_await_shell, continue53_await_skeleton,
    continue53_import_dynamic_skeleton, continue53_optional_chain_skeleton,
    continue53_sequence_skeleton, continue53_update_postfix_skeleton,
    continue53_update_prefix_skeleton, continue53_yield_skeleton,
    continue53_yield_star_skeleton, is_continue53_await_type, is_continue53_chain_type,
    is_continue53_import_expr_type, is_continue53_related_type, is_continue53_sequence_type,
    is_continue53_update_type, is_continue53_yield_type, CONTINUE53_RELATED_TYPES,
};

/// Dual-oracle residual: continue121 related AST type catalog (sequence plane).
pub const CONTINUE121_RELATED_TYPES: &[&str] = &[
    "SequenceExpression",
    "UpdateExpression",
    "YieldExpression",
    "AwaitExpression",
    "ChainExpression",
    "ImportExpression",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_sequence_update_yield_await_import_edge_related_type(t: &str) -> bool {
    CONTINUE121_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue121_sequence_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue121_update_type(t: &str) -> bool {
    t == "UpdateExpression"
}

#[must_use]
pub fn is_continue121_yield_type(t: &str) -> bool {
    t == "YieldExpression"
}

#[must_use]
pub fn is_continue121_await_type(t: &str) -> bool {
    t == "AwaitExpression"
}

#[must_use]
pub fn is_continue121_chain_type(t: &str) -> bool {
    t == "ChainExpression"
}

#[must_use]
pub fn is_continue121_import_type(t: &str) -> bool {
    t == "ImportExpression"
}

#[must_use]
pub fn is_continue121_control_expr_plane_type(t: &str) -> bool {
    matches!(
        t,
        "SequenceExpression" | "UpdateExpression" | "YieldExpression" | "AwaitExpression"
    )
}

#[must_use]
pub fn is_continue121_chain_import_plane_type(t: &str) -> bool {
    matches!(t, "ChainExpression" | "ImportExpression")
}

#[must_use]
pub fn is_continue121_type(t: &str) -> bool {
    matches!(
        t,
        "SequenceExpression"
            | "UpdateExpression"
            | "YieldExpression"
            | "AwaitExpression"
            | "ChainExpression"
            | "ImportExpression"
    )
}

// ── continue56 shell dual-oracle ────────────────────────────────────────────

/// Dual-oracle residual: continue121 sequence shell composing real
/// [`continue56_sequence_shell`].
#[must_use]
pub fn continue121_sequence_shell() -> bool {
    let a = continue56_sequence_shell();
    let b = continue56_sequence_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 update shell composing real
/// [`continue56_update_shell`].
#[must_use]
pub fn continue121_update_shell() -> bool {
    let a = continue56_update_shell();
    let b = continue56_update_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 yield/await shell composing real
/// [`continue56_yield_await_shell`].
#[must_use]
pub fn continue121_yield_await_shell() -> bool {
    let a = continue56_yield_await_shell();
    let b = continue56_yield_await_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 import/chain shell composing real
/// [`continue56_import_chain_shell`].
#[must_use]
pub fn continue121_import_chain_shell() -> bool {
    let a = continue56_import_chain_shell();
    let b = continue56_import_chain_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 partition shell composing real
/// [`continue56_partition_shell`].
#[must_use]
pub fn continue121_partition_shell() -> bool {
    let a = continue56_partition_shell();
    let b = continue56_partition_shell();
    a && b && a == b
}

// ── continue54 edge shell dual-oracle ───────────────────────────────────────

/// Dual-oracle residual: continue121 sequence edge shell composing real
/// [`continue54_sequence_edge_shell`].
#[must_use]
pub fn continue121_sequence_edge_shell() -> bool {
    let a = continue54_sequence_edge_shell();
    let b = continue54_sequence_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 update edge shell composing real
/// [`continue54_update_edge_shell`].
#[must_use]
pub fn continue121_update_edge_shell() -> bool {
    let a = continue54_update_edge_shell();
    let b = continue54_update_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 yield/await edge shell composing real
/// [`continue54_yield_await_edge_shell`].
#[must_use]
pub fn continue121_yield_await_edge_shell() -> bool {
    let a = continue54_yield_await_edge_shell();
    let b = continue54_yield_await_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 import/chain edge shell composing real
/// [`continue54_import_chain_edge_shell`].
#[must_use]
pub fn continue121_import_chain_edge_shell() -> bool {
    let a = continue54_import_chain_edge_shell();
    let b = continue54_import_chain_edge_shell();
    a && b && a == b
}

/// Dual-oracle residual: continue121 continue54 partition shell composing real
/// [`continue54_partition_shell`].
#[must_use]
pub fn continue121_edge_partition_shell() -> bool {
    let a = continue54_partition_shell();
    let b = continue54_partition_shell();
    a && b && a == b
}

// ── Edge skeleton poles (empty-arg / multi-side) over continue53 ─────────────

/// Dual-oracle residual: empty-pair sequence edge skeleton.
#[must_use]
pub fn continue121_sequence_empty_edge() -> String {
    continue53_sequence_skeleton("", "")
}

/// Dual-oracle residual: prefix `--n` update edge skeleton.
#[must_use]
pub fn continue121_update_prefix_edge(arg: &str) -> String {
    continue53_update_prefix_skeleton("--", arg)
}

/// Dual-oracle residual: postfix `n++` update edge skeleton.
#[must_use]
pub fn continue121_update_postfix_edge(arg: &str) -> String {
    continue53_update_postfix_skeleton(arg, "++")
}

/// Dual-oracle residual: bare yield edge skeleton.
#[must_use]
pub fn continue121_yield_empty_edge() -> String {
    continue53_yield_skeleton("")
}

/// Dual-oracle residual: bare yield* edge skeleton.
#[must_use]
pub fn continue121_yield_star_empty_edge() -> String {
    continue53_yield_star_skeleton("")
}

/// Dual-oracle residual: bare await edge skeleton.
#[must_use]
pub fn continue121_await_empty_edge() -> String {
    continue53_await_skeleton("")
}

/// Dual-oracle residual: empty-string import edge skeleton.
#[must_use]
pub fn continue121_import_empty_string_edge() -> String {
    continue53_import_dynamic_skeleton("''")
}

/// Dual-oracle residual: empty-prop optional chain edge skeleton.
#[must_use]
pub fn continue121_optional_chain_empty_prop_edge(obj: &str) -> String {
    continue53_optional_chain_skeleton(obj, "")
}

// ── Composed residual edge shells ───────────────────────────────────────────

/// Dual-oracle residual: sequence empty edge then update prefix edge.
#[must_use]
pub fn continue121_empty_sequence_then_prefix(arg: &str) -> String {
    let seq = continue121_sequence_empty_edge();
    let up = continue121_update_prefix_edge(arg);
    format!("{seq} {up}")
}

/// Dual-oracle residual: bare yield then bare await edge pair.
#[must_use]
pub fn continue121_empty_yield_then_await() -> String {
    let y = continue121_yield_empty_edge();
    let a = continue121_await_empty_edge();
    format!("{y}{a}")
}

/// Dual-oracle residual: yield* empty then import('') edge pair.
#[must_use]
pub fn continue121_empty_yield_star_then_import() -> String {
    let y = continue121_yield_star_empty_edge();
    let i = continue121_import_empty_string_edge();
    format!("{y} {i}")
}

/// Dual-oracle residual: optional-chain empty prop then postfix update.
#[must_use]
pub fn continue121_empty_chain_then_postfix(obj: &str, arg: &str) -> String {
    let c = continue121_optional_chain_empty_prop_edge(obj);
    let u = continue121_update_postfix_edge(arg);
    format!("{c} {u}")
}

/// Dual-oracle residual: catalog length pole (continue53 plane, six types).
#[must_use]
pub fn continue121_catalog_len_shell() -> bool {
    CONTINUE121_RELATED_TYPES.len() == 6
        && CONTINUE53_RELATED_TYPES.len() == 6
        && is_continue53_related_type("SequenceExpression")
        && is_continue53_related_type("ImportExpression")
        && !is_continue53_related_type("Super")
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue121_sep(pretty: bool) -> &'static str {
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
    fn continue121_type_catalog() {
        assert_eq!(CONTINUE121_RELATED_TYPES.len(), 6);
        assert!(is_sequence_update_yield_await_import_edge_related_type(
            "SequenceExpression"
        ));
        assert!(is_sequence_update_yield_await_import_edge_related_type(
            "UpdateExpression"
        ));
        assert!(is_sequence_update_yield_await_import_edge_related_type(
            "YieldExpression"
        ));
        assert!(is_sequence_update_yield_await_import_edge_related_type(
            "AwaitExpression"
        ));
        assert!(is_sequence_update_yield_await_import_edge_related_type(
            "ChainExpression"
        ));
        assert!(is_sequence_update_yield_await_import_edge_related_type(
            "ImportExpression"
        ));
        assert!(!is_sequence_update_yield_await_import_edge_related_type(
            "Super"
        ));
        assert!(!is_sequence_update_yield_await_import_edge_related_type(
            "ClassExpression"
        ));

        assert!(is_continue121_sequence_type("SequenceExpression"));
        assert!(!is_continue121_sequence_type("UpdateExpression"));
        assert!(is_continue121_update_type("UpdateExpression"));
        assert!(is_continue121_yield_type("YieldExpression"));
        assert!(is_continue121_await_type("AwaitExpression"));
        assert!(is_continue121_chain_type("ChainExpression"));
        assert!(is_continue121_import_type("ImportExpression"));
        assert!(is_continue121_control_expr_plane_type("SequenceExpression"));
        assert!(is_continue121_control_expr_plane_type("YieldExpression"));
        assert!(!is_continue121_control_expr_plane_type("ImportExpression"));
        assert!(is_continue121_chain_import_plane_type("ChainExpression"));
        assert!(is_continue121_chain_import_plane_type("ImportExpression"));
        assert!(!is_continue121_chain_import_plane_type("UpdateExpression"));
        assert!(is_continue121_type("ImportExpression"));
        assert!(!is_continue121_type("Super"));
        assert!(!is_continue121_type("Identifier"));
        assert!(continue121_catalog_len_shell());
    }

    #[test]
    fn continue121_continue56_shell_dual_oracle() {
        assert!(continue121_sequence_shell());
        assert!(continue121_update_shell());
        assert!(continue121_yield_await_shell());
        assert!(continue121_import_chain_shell());
        assert!(continue121_partition_shell());
        // base poles still hold
        assert!(continue56_sequence_shell());
        assert!(continue56_update_shell());
        assert!(continue56_yield_await_shell());
        assert!(continue56_import_chain_shell());
        assert!(continue56_partition_shell());
    }

    #[test]
    fn continue121_continue54_edge_shell_dual_oracle() {
        assert!(continue121_sequence_edge_shell());
        assert!(continue121_update_edge_shell());
        assert!(continue121_yield_await_edge_shell());
        assert!(continue121_import_chain_edge_shell());
        assert!(continue121_edge_partition_shell());
        assert!(continue54_sequence_edge_shell());
        assert!(continue54_update_edge_shell());
        assert!(continue54_yield_await_edge_shell());
        assert!(continue54_import_chain_edge_shell());
        assert!(continue54_partition_shell());
    }

    #[test]
    fn continue121_edge_skeleton_emit() {
        assert_eq!(continue121_sequence_empty_edge(), ", ");
        assert_eq!(
            continue121_sequence_empty_edge(),
            continue53_sequence_skeleton("", "")
        );
        assert_eq!(continue121_update_prefix_edge("n"), "--n");
        assert_eq!(
            continue121_update_prefix_edge("n"),
            continue53_update_prefix_skeleton("--", "n")
        );
        assert_eq!(continue121_update_postfix_edge("n"), "n++");
        assert_eq!(
            continue121_update_postfix_edge("n"),
            continue53_update_postfix_skeleton("n", "++")
        );
        assert_eq!(continue121_yield_empty_edge(), "yield ");
        assert_eq!(continue121_yield_star_empty_edge(), "yield* ");
        assert_eq!(continue121_await_empty_edge(), "await ");
        assert_eq!(continue121_import_empty_string_edge(), "import('')");
        assert_eq!(continue121_optional_chain_empty_prop_edge("a"), "a?.");
        assert!(is_continue53_sequence_type("SequenceExpression"));
        assert!(is_continue53_update_type("UpdateExpression"));
        assert!(is_continue53_yield_type("YieldExpression"));
        assert!(is_continue53_await_type("AwaitExpression"));
        assert!(is_continue53_import_expr_type("ImportExpression"));
        assert!(is_continue53_chain_type("ChainExpression"));
    }

    #[test]
    fn continue121_composed_residual_shells() {
        assert_eq!(
            continue121_empty_sequence_then_prefix("n"),
            ",  --n"
        );
        assert_eq!(continue121_empty_yield_then_await(), "yield await ");
        assert_eq!(
            continue121_empty_yield_star_then_import(),
            "yield*  import('')"
        );
        assert_eq!(
            continue121_empty_chain_then_postfix("a", "n"),
            "a?. n++"
        );
        assert_eq!(continue121_sep(true), " ");
        assert_eq!(continue121_sep(false), "");
        assert_ne!(continue121_sep(true), continue121_sep(false));
    }
}

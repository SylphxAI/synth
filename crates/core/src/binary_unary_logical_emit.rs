//! Pure BinaryExpression + UnaryExpression + LogicalExpression + UpdateExpression
//! dual-oracle emission — residual pure **continue76** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules:
//! - BinaryExpression full dual-oracle pretty/minify via shipped
//!   `binary_operator_token`, **plus** compressor residual that always spaces
//!   word ops `in` / `instanceof` even in minify (continue31/41 were pretty-only
//!   fixed `a + b` shells; unary_binary_emit tokens omit the word-op minify rule)
//! - UnaryExpression full dual-oracle driving real `unary_operator_token`
//!   (word ops `typeof`/`void`/`delete` always trailing space; symbolic glued)
//! - LogicalExpression dual-oracle (`&&` / `||` / `??`) composing binary spacing
//! - UpdateExpression dual-oracle prefix/postfix via shipped `update_prefix_token`
//!
//! Intentionally does **not** re-wrap continue64–75 partition shells.
//! Composes real shipped pure helpers from `unary_binary_emit`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::unary_binary_emit::{
    binary_operator_token, is_logical_operator, unary_operator_token, update_prefix_token,
};

/// Dual-oracle residual: continue76 related AST type catalog.
pub const CONTINUE76_RELATED_TYPES: &[&str] = &[
    "BinaryExpression",
    "UnaryExpression",
    "LogicalExpression",
    "UpdateExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_binary_unary_logical_related_type(t: &str) -> bool {
    CONTINUE76_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue76_binary_type(t: &str) -> bool {
    t == "BinaryExpression"
}

#[must_use]
pub fn is_continue76_unary_type(t: &str) -> bool {
    t == "UnaryExpression"
}

#[must_use]
pub fn is_continue76_logical_type(t: &str) -> bool {
    t == "LogicalExpression"
}

#[must_use]
pub fn is_continue76_update_type(t: &str) -> bool {
    t == "UpdateExpression"
}

// ── BinaryExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle residual: word binary operators that need spaces in **both**
/// pretty and minify (compressor residual: `in`, `instanceof`).
#[must_use]
pub fn binary_is_word_operator(op: &str) -> bool {
    matches!(op, "in" | "instanceof")
}

/// Dual-oracle binary operator fragment.
///
/// - pretty: always ` {op} ` (printer residual)
/// - minify word ops (`in`/`instanceof`): still ` {op} ` (compressor residual)
/// - minify symbolic: bare `op` via shipped [`binary_operator_token`]
#[must_use]
pub fn continue76_binary_operator_token(op: &str, pretty: bool) -> String {
    if binary_is_word_operator(op) {
        format!(" {op} ")
    } else {
        binary_operator_token(op, pretty)
    }
}

/// Dual-oracle BinaryExpression skeleton: `left` + op + `right`.
///
/// Widens continue31/41 pretty-only `a + b` fixed shells with real pretty/minify
/// dual-oracle and word-op minify spacing.
#[must_use]
pub fn continue76_binary_expression_skeleton(
    left: &str,
    op: &str,
    right: &str,
    pretty: bool,
) -> String {
    format!(
        "{left}{}{right}",
        continue76_binary_operator_token(op, pretty)
    )
}

/// Convenience: pretty binary.
#[must_use]
pub fn binary_expression_pretty(left: &str, op: &str, right: &str) -> String {
    continue76_binary_expression_skeleton(left, op, right, true)
}

/// Convenience: minify binary.
#[must_use]
pub fn binary_expression_minify(left: &str, op: &str, right: &str) -> String {
    continue76_binary_expression_skeleton(left, op, right, false)
}

// ── UnaryExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle UnaryExpression skeleton.
///
/// Prefix (default): word ops keep trailing space via shipped
/// [`unary_operator_token`]; symbolic glues (`!x`, `-x`).
/// Postfix residual (rare for pure unaries; models `arg op` form).
#[must_use]
pub fn continue76_unary_expression_skeleton(
    op: &str,
    argument: &str,
    prefix: bool,
    pretty: bool,
) -> String {
    if prefix {
        format!("{}{argument}", unary_operator_token(op, pretty))
    } else {
        // postfix: operator after argument (no leading space residual)
        format!("{argument}{op}")
    }
}

/// Convenience: prefix unary pretty.
#[must_use]
pub fn unary_expression_pretty(op: &str, argument: &str) -> String {
    continue76_unary_expression_skeleton(op, argument, true, true)
}

/// Convenience: prefix unary minify.
#[must_use]
pub fn unary_expression_minify(op: &str, argument: &str) -> String {
    continue76_unary_expression_skeleton(op, argument, true, false)
}

// ── LogicalExpression dual-oracle ───────────────────────────────────────────

/// Dual-oracle LogicalExpression skeleton: `left` + `&&`/`||`/`??` + `right`.
///
/// Non-logical ops still emit with binary spacing for dual-oracle completeness
/// (caller should pass a logical op; fence via [`is_logical_operator`]).
#[must_use]
pub fn continue76_logical_expression_skeleton(
    left: &str,
    op: &str,
    right: &str,
    pretty: bool,
) -> String {
    let _ = is_logical_operator(op);
    // Logical ops are never word-ops; reuse binary dual-oracle token path.
    continue76_binary_expression_skeleton(left, op, right, pretty)
}

/// Convenience: pretty logical.
#[must_use]
pub fn logical_expression_pretty(left: &str, op: &str, right: &str) -> String {
    continue76_logical_expression_skeleton(left, op, right, true)
}

/// Convenience: minify logical.
#[must_use]
pub fn logical_expression_minify(left: &str, op: &str, right: &str) -> String {
    continue76_logical_expression_skeleton(left, op, right, false)
}

// ── UpdateExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle UpdateExpression: prefix `++x` / postfix `x++`.
/// Pretty and minify share the same token placement (no spacing residual).
/// Drives real shipped [`update_prefix_token`].
#[must_use]
pub fn continue76_update_expression_skeleton(operand: &str, op: &str, prefix: bool) -> String {
    let (before, after) = update_prefix_token(prefix, op);
    format!("{before}{operand}{after}")
}

/// Convenience: prefix `++`.
#[must_use]
pub fn update_prefix_inc(operand: &str) -> String {
    continue76_update_expression_skeleton(operand, "++", true)
}

/// Convenience: postfix `++`.
#[must_use]
pub fn update_postfix_inc(operand: &str) -> String {
    continue76_update_expression_skeleton(operand, "++", false)
}

/// Convenience: prefix `--`.
#[must_use]
pub fn update_prefix_dec(operand: &str) -> String {
    continue76_update_expression_skeleton(operand, "--", true)
}

/// Convenience: postfix `--`.
#[must_use]
pub fn update_postfix_dec(operand: &str) -> String {
    continue76_update_expression_skeleton(operand, "--", false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unary_binary_emit::{
        binary_operator_token, unary_is_word_operator, unary_operator_token,
    };

    #[test]
    fn continue76_type_catalog() {
        assert_eq!(CONTINUE76_RELATED_TYPES.len(), 4);
        assert!(is_binary_unary_logical_related_type("BinaryExpression"));
        assert!(is_binary_unary_logical_related_type("UnaryExpression"));
        assert!(is_binary_unary_logical_related_type("LogicalExpression"));
        assert!(is_binary_unary_logical_related_type("UpdateExpression"));
        assert!(!is_binary_unary_logical_related_type("AssignmentExpression"));
        assert!(!is_binary_unary_logical_related_type("TemplateLiteral"));
        assert!(is_continue76_binary_type("BinaryExpression"));
        assert!(is_continue76_unary_type("UnaryExpression"));
        assert!(is_continue76_logical_type("LogicalExpression"));
        assert!(is_continue76_update_type("UpdateExpression"));
        assert!(!is_continue76_binary_type("LogicalExpression"));
        assert!(!is_continue76_logical_type("BinaryExpression"));
    }

    #[test]
    fn continue76_binary_expression_dual_oracle() {
        // pretty always spaces
        assert_eq!(binary_expression_pretty("a", "+", "b"), "a + b");
        assert_eq!(binary_expression_pretty("a", "===", "b"), "a === b");
        // minify symbolic glues
        assert_eq!(binary_expression_minify("a", "+", "b"), "a+b");
        assert_eq!(binary_expression_minify("a", "===", "b"), "a===b");
        // word ops always spaced (compressor residual) — widens unary_binary bare path
        assert!(binary_is_word_operator("in"));
        assert!(binary_is_word_operator("instanceof"));
        assert!(!binary_is_word_operator("+"));
        assert_eq!(binary_expression_pretty("k", "in", "obj"), "k in obj");
        assert_eq!(binary_expression_minify("k", "in", "obj"), "k in obj");
        assert_eq!(
            binary_expression_minify("x", "instanceof", "Foo"),
            "x instanceof Foo"
        );
        // drives shipped token for non-word
        assert_eq!(
            continue76_binary_operator_token("+", true),
            binary_operator_token("+", true)
        );
        assert_eq!(
            continue76_binary_operator_token("+", false),
            binary_operator_token("+", false)
        );
        // multi-char residual
        assert_eq!(
            continue76_binary_expression_skeleton("foo.bar", "**", "2", false),
            "foo.bar**2"
        );
    }

    #[test]
    fn continue76_unary_expression_dual_oracle() {
        // symbolic glued pretty/minify
        assert_eq!(unary_expression_pretty("!", "x"), "!x");
        assert_eq!(unary_expression_minify("!", "x"), "!x");
        assert_eq!(unary_expression_pretty("-", "n"), "-n");
        assert_eq!(unary_expression_minify("~", "bits"), "~bits");
        // word ops keep trailing space (drives shipped unary_operator_token)
        assert!(unary_is_word_operator("typeof"));
        assert_eq!(unary_expression_pretty("typeof", "x"), "typeof x");
        assert_eq!(unary_expression_minify("typeof", "x"), "typeof x");
        assert_eq!(unary_expression_pretty("void", "0"), "void 0");
        assert_eq!(unary_expression_pretty("delete", "obj.k"), "delete obj.k");
        assert_eq!(
            continue76_unary_expression_skeleton("typeof", "x", true, true),
            format!("{}x", unary_operator_token("typeof", true))
        );
        // postfix residual
        assert_eq!(
            continue76_unary_expression_skeleton("!", "x", false, true),
            "x!"
        );
    }

    #[test]
    fn continue76_logical_and_update_dual_oracle() {
        assert_eq!(logical_expression_pretty("a", "&&", "b"), "a && b");
        assert_eq!(logical_expression_minify("a", "&&", "b"), "a&&b");
        assert_eq!(logical_expression_pretty("a", "||", "b"), "a || b");
        assert_eq!(logical_expression_minify("a", "||", "b"), "a||b");
        assert_eq!(logical_expression_pretty("a", "??", "b"), "a ?? b");
        assert_eq!(logical_expression_minify("a", "??", "b"), "a??b");

        assert_eq!(update_prefix_inc("i"), "++i");
        assert_eq!(update_postfix_inc("i"), "i++");
        assert_eq!(update_prefix_dec("j"), "--j");
        assert_eq!(update_postfix_dec("j"), "j--");
        assert_eq!(
            continue76_update_expression_skeleton("x", "++", true),
            "++x"
        );
        assert_eq!(
            continue76_update_expression_skeleton("x", "--", false),
            "x--"
        );
    }
}

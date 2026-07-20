//! Pure ObjectPattern + RestElement + SpreadElement + ArrayPattern +
//! AssignmentPattern + ForAwaitStatement dual-oracle emission — residual pure
//! **continue84** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - ObjectPattern dual-oracle empty/properties pretty/minify composing real
//!   `object_pattern_skeleton` (continue19 base)
//! - RestElement / SpreadElement dual-oracle `...arg` tokens
//! - ArrayPattern dual-oracle empty + elements pretty/minify
//! - AssignmentPattern dual-oracle pretty/minify `left = right` / `left=right`
//! - ForAwaitStatement dual-oracle pretty/minify composing real
//!   `for_await_of_statement_skeleton`
//! - Composed destructure / for-await residual shells
//!
//! Intentionally does **not** re-wrap continue64–83 partition shells
//! (yield/meta/await continue83 stays separate; property/static continue82 stays
//! separate; object-expression continue78 / array-expression continue77 /
//! tagged-template continue75 stay separate). Composes real shipped pure helpers
//! from `pattern_sequence_emit` + assignment-pattern spacing.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::pattern_sequence_emit::{
    for_await_of_statement_skeleton, object_pattern_assign_property, object_pattern_shorthand,
    object_pattern_skeleton, rest_element_token, spread_element_token,
};

/// Dual-oracle residual: continue84 related AST type catalog.
pub const CONTINUE84_RELATED_TYPES: &[&str] = &[
    "ObjectPattern",
    "RestElement",
    "SpreadElement",
    "ArrayPattern",
    "AssignmentPattern",
    "ForAwaitStatement",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_pattern_rest_for_await_related_type(t: &str) -> bool {
    CONTINUE84_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue84_object_pattern_type(t: &str) -> bool {
    t == "ObjectPattern"
}

#[must_use]
pub fn is_continue84_rest_element_type(t: &str) -> bool {
    t == "RestElement"
}

#[must_use]
pub fn is_continue84_spread_element_type(t: &str) -> bool {
    t == "SpreadElement"
}

#[must_use]
pub fn is_continue84_array_pattern_type(t: &str) -> bool {
    t == "ArrayPattern"
}

#[must_use]
pub fn is_continue84_assignment_pattern_type(t: &str) -> bool {
    t == "AssignmentPattern"
}

#[must_use]
pub fn is_continue84_for_await_type(t: &str) -> bool {
    t == "ForAwaitStatement"
}

// ── ObjectPattern dual-oracle ───────────────────────────────────────────────

/// Dual-oracle ObjectPattern skeleton composing real [`object_pattern_skeleton`].
#[must_use]
pub fn continue84_object_pattern_skeleton(properties: &[&str], pretty: bool) -> String {
    object_pattern_skeleton(properties, pretty)
}

/// Convenience: pretty object pattern.
#[must_use]
pub fn object_pattern_pretty(properties: &[&str]) -> String {
    continue84_object_pattern_skeleton(properties, true)
}

/// Convenience: minify object pattern.
#[must_use]
pub fn object_pattern_minify(properties: &[&str]) -> String {
    continue84_object_pattern_skeleton(properties, false)
}

/// Convenience: empty object pattern.
#[must_use]
pub fn object_pattern_empty() -> String {
    continue84_object_pattern_skeleton(&[], true)
}

// ── RestElement / SpreadElement dual-oracle ─────────────────────────────────

/// Dual-oracle RestElement token composing real [`rest_element_token`].
#[must_use]
pub fn continue84_rest_element_skeleton(arg: &str) -> String {
    rest_element_token(arg)
}

/// Dual-oracle SpreadElement token composing real [`spread_element_token`].
#[must_use]
pub fn continue84_spread_element_skeleton(arg: &str) -> String {
    spread_element_token(arg)
}

/// Convenience aliases.
#[must_use]
pub fn rest_element(arg: &str) -> String {
    continue84_rest_element_skeleton(arg)
}

#[must_use]
pub fn spread_element(arg: &str) -> String {
    continue84_spread_element_skeleton(arg)
}

// ── ArrayPattern dual-oracle ────────────────────────────────────────────────

/// Dual-oracle ArrayPattern skeleton: `[a, , b, ...rest]` elements already rendered.
/// Pretty inserts spaces after `[` / before `]` and between elements (mirrors
/// object-pattern dual-oracle spacing poles); minify is tight.
#[must_use]
pub fn continue84_array_pattern_skeleton(elements: &[&str], pretty: bool) -> String {
    if elements.is_empty() {
        return "[]".to_string();
    }
    if pretty {
        format!("[ {} ]", elements.join(", "))
    } else {
        format!("[{}]", elements.join(","))
    }
}

/// Convenience: pretty array pattern.
#[must_use]
pub fn array_pattern_pretty(elements: &[&str]) -> String {
    continue84_array_pattern_skeleton(elements, true)
}

/// Convenience: minify array pattern.
#[must_use]
pub fn array_pattern_minify(elements: &[&str]) -> String {
    continue84_array_pattern_skeleton(elements, false)
}

/// Convenience: empty array pattern.
#[must_use]
pub fn array_pattern_empty() -> String {
    continue84_array_pattern_skeleton(&[], true)
}

// ── AssignmentPattern dual-oracle ───────────────────────────────────────────

/// Dual-oracle AssignmentPattern: pretty `left = right` vs minify `left=right`.
#[must_use]
pub fn continue84_assignment_pattern_skeleton(left: &str, right: &str, pretty: bool) -> String {
    // Reuse object-pattern assign spacing (same ` = ` / `=` dual-oracle poles).
    object_pattern_assign_property(left, right, pretty)
}

/// Convenience: pretty assignment pattern.
#[must_use]
pub fn assignment_pattern_pretty(left: &str, right: &str) -> String {
    continue84_assignment_pattern_skeleton(left, right, true)
}

/// Convenience: minify assignment pattern.
#[must_use]
pub fn assignment_pattern_minify(left: &str, right: &str) -> String {
    continue84_assignment_pattern_skeleton(left, right, false)
}

// ── ForAwaitStatement dual-oracle ───────────────────────────────────────────

/// Dual-oracle ForAwaitStatement skeleton composing real
/// [`for_await_of_statement_skeleton`].
#[must_use]
pub fn continue84_for_await_of_skeleton(
    left: &str,
    right: &str,
    body: &str,
    pretty: bool,
) -> String {
    for_await_of_statement_skeleton(left, right, body, pretty)
}

/// Convenience: pretty for-await-of.
#[must_use]
pub fn for_await_of_pretty(left: &str, right: &str, body: &str) -> String {
    continue84_for_await_of_skeleton(left, right, body, true)
}

/// Convenience: minify for-await-of.
#[must_use]
pub fn for_await_of_minify(left: &str, right: &str, body: &str) -> String {
    continue84_for_await_of_skeleton(left, right, body, false)
}

// ── Composed dual-oracle shells ─────────────────────────────────────────────

/// Dual-oracle residual: object pattern with shorthand + assign + rest.
#[must_use]
pub fn continue84_object_destructure_shell(pretty: bool) -> String {
    let assign = continue84_assignment_pattern_skeleton("b", "1", pretty);
    let rest = continue84_rest_element_skeleton("rest");
    let a = object_pattern_shorthand("a");
    continue84_object_pattern_skeleton(&[a.as_str(), assign.as_str(), rest.as_str()], pretty)
}

/// Dual-oracle residual: array pattern with hole + rest.
#[must_use]
pub fn continue84_array_destructure_shell(pretty: bool) -> String {
    let rest = continue84_rest_element_skeleton("xs");
    continue84_array_pattern_skeleton(&["a", "", "b", rest.as_str()], pretty)
}

/// Dual-oracle residual: `for await (const {a, ...r} of src) body`.
#[must_use]
pub fn continue84_for_await_object_pattern(
    source: &str,
    body: &str,
    pretty: bool,
) -> String {
    let pat = continue84_object_destructure_shell(pretty);
    let left = if pretty {
        format!("const {pat}")
    } else {
        format!("const{pat}")
    };
    continue84_for_await_of_skeleton(&left, source, body, pretty)
}

/// Dual-oracle residual: spread inside array pattern-like expression shell.
#[must_use]
pub fn continue84_spread_in_array_shell(arg: &str, pretty: bool) -> String {
    let sp = continue84_spread_element_skeleton(arg);
    continue84_array_pattern_skeleton(&[sp.as_str()], pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern_sequence_emit::{
        for_await_of_statement_skeleton, object_pattern_skeleton, rest_element_token,
        spread_element_token,
    };

    #[test]
    fn continue84_type_catalog() {
        assert_eq!(CONTINUE84_RELATED_TYPES.len(), 6);
        assert!(is_pattern_rest_for_await_related_type("ObjectPattern"));
        assert!(is_pattern_rest_for_await_related_type("RestElement"));
        assert!(is_pattern_rest_for_await_related_type("SpreadElement"));
        assert!(is_pattern_rest_for_await_related_type("ArrayPattern"));
        assert!(is_pattern_rest_for_await_related_type("AssignmentPattern"));
        assert!(is_pattern_rest_for_await_related_type("ForAwaitStatement"));
        assert!(!is_pattern_rest_for_await_related_type("ObjectExpression"));
        assert!(!is_pattern_rest_for_await_related_type("ArrayExpression"));
        assert!(!is_pattern_rest_for_await_related_type("ForOfStatement"));
        assert!(!is_pattern_rest_for_await_related_type("YieldExpression"));
        assert!(is_continue84_object_pattern_type("ObjectPattern"));
        assert!(is_continue84_rest_element_type("RestElement"));
        assert!(is_continue84_spread_element_type("SpreadElement"));
        assert!(is_continue84_array_pattern_type("ArrayPattern"));
        assert!(is_continue84_assignment_pattern_type("AssignmentPattern"));
        assert!(is_continue84_for_await_type("ForAwaitStatement"));
        assert!(!is_continue84_object_pattern_type("ObjectExpression"));
        assert!(!is_continue84_for_await_type("ForOfStatement"));
    }

    #[test]
    fn continue84_object_rest_spread_dual_oracle() {
        assert_eq!(object_pattern_empty(), "{}");
        assert_eq!(object_pattern_pretty(&["a", "b"]), "{ a, b }");
        assert_eq!(object_pattern_minify(&["a", "b"]), "{a,b}");
        assert_eq!(
            continue84_object_pattern_skeleton(&["x"], true),
            object_pattern_skeleton(&["x"], true)
        );
        assert_ne!(
            object_pattern_pretty(&["a"]),
            object_pattern_minify(&["a"])
        );

        assert_eq!(rest_element("rest"), "...rest");
        assert_eq!(spread_element("xs"), "...xs");
        assert_eq!(
            continue84_rest_element_skeleton("r"),
            rest_element_token("r")
        );
        assert_eq!(
            continue84_spread_element_skeleton("s"),
            spread_element_token("s")
        );
        assert_eq!(rest_element("a"), spread_element("a"));
    }

    #[test]
    fn continue84_array_assign_for_await_compose_dual_oracle() {
        assert_eq!(array_pattern_empty(), "[]");
        assert_eq!(array_pattern_pretty(&["a", "b"]), "[ a, b ]");
        assert_eq!(array_pattern_minify(&["a", "b"]), "[a,b]");
        assert_eq!(array_pattern_pretty(&["x"]), "[ x ]");
        assert_eq!(array_pattern_minify(&["x"]), "[x]");
        assert_ne!(
            array_pattern_pretty(&["x"]),
            array_pattern_minify(&["x"])
        );

        assert_eq!(assignment_pattern_pretty("x", "0"), "x = 0");
        assert_eq!(assignment_pattern_minify("x", "0"), "x=0");
        assert_ne!(
            assignment_pattern_pretty("a", "1"),
            assignment_pattern_minify("a", "1")
        );

        assert_eq!(
            for_await_of_pretty("x", "asyncIter", "{}"),
            "for await (x of asyncIter) {}"
        );
        assert_eq!(
            for_await_of_minify("x", "it", "body()"),
            "forawait(xofit)body()"
        );
        assert_eq!(
            continue84_for_await_of_skeleton("x", "it", "{}", true),
            for_await_of_statement_skeleton("x", "it", "{}", true)
        );
        assert_ne!(
            for_await_of_pretty("a", "b", "{}"),
            for_await_of_minify("a", "b", "{}")
        );

        let obj = continue84_object_destructure_shell(true);
        assert!(obj.contains("a"));
        assert!(obj.contains("b = 1") || obj.contains("b=1"));
        assert!(obj.contains("...rest"));
        let obj_mini = continue84_object_destructure_shell(false);
        assert!(obj_mini.contains("...rest"));
        assert_ne!(obj, obj_mini);

        let arr = continue84_array_destructure_shell(true);
        assert!(arr.starts_with('['));
        assert!(arr.contains("...xs"));
        let arr_mini = continue84_array_destructure_shell(false);
        assert_ne!(arr, arr_mini);

        let fa = continue84_for_await_object_pattern("src", "{}", true);
        assert!(fa.starts_with("for await"));
        assert!(fa.contains("const"));
        assert!(fa.contains("src"));
        let fa_mini = continue84_for_await_object_pattern("src", "{}", false);
        assert!(fa_mini.starts_with("forawait"));
        assert_ne!(fa, fa_mini);

        let sp = continue84_spread_in_array_shell("ys", true);
        assert_eq!(sp, "[ ...ys ]");
        let sp_mini = continue84_spread_in_array_shell("ys", false);
        assert_eq!(sp_mini, "[...ys]");
        assert_ne!(sp, sp_mini);
    }
}

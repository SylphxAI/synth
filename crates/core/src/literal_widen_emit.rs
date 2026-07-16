//! Pure residual continue21: BigIntLiteral / RegExpLiteral / DirectiveLiteral emit.
//! Intentional ts_only plugins retained. NO authority_rust / ts_deleted invent.

/// Type guards for continue21 related AST node types.
#[must_use]
pub fn is_literal_widen_related_type(t: &str) -> bool {
    matches!(
        t,
        "BigIntLiteral" | "RegExpLiteral" | "DirectiveLiteral" | "Directive"
    )
}

#[must_use]
pub fn is_continue21_bigint_literal_type(t: &str) -> bool {
    t == "BigIntLiteral"
}

#[must_use]
pub fn is_continue21_regexp_literal_type(t: &str) -> bool {
    t == "RegExpLiteral"
}

#[must_use]
pub fn is_continue21_directive_literal_type(t: &str) -> bool {
    t == "DirectiveLiteral" || t == "Directive"
}

/// Dual-oracle BigIntLiteral emit residual (`1n`).
#[must_use]
pub fn continue21_bigint_literal_skeleton(value: &str) -> String {
    let v = value.trim();
    if v.ends_with('n') {
        v.to_string()
    } else {
        format!("{v}n")
    }
}

/// Dual-oracle RegExpLiteral emit residual (`/foo/gi`).
#[must_use]
pub fn continue21_regexp_literal_skeleton(pattern: &str, flags: &str) -> String {
    format!("/{pattern}/{flags}")
}

/// Dual-oracle DirectiveLiteral residual (`"use strict";`).
#[must_use]
pub fn continue21_directive_literal_skeleton(value: &str, pretty: bool) -> String {
    let body = format!("\"{value}\"");
    if pretty {
        format!("{body};")
    } else {
        format!("{body};")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue21_literal_widen_emit() {
        assert!(is_literal_widen_related_type("BigIntLiteral"));
        assert!(is_literal_widen_related_type("RegExpLiteral"));
        assert!(is_literal_widen_related_type("DirectiveLiteral"));
        assert!(is_continue21_bigint_literal_type("BigIntLiteral"));
        assert!(is_continue21_regexp_literal_type("RegExpLiteral"));
        assert!(is_continue21_directive_literal_type("Directive"));
        assert!(!is_literal_widen_related_type("NumericLiteral"));
        assert_eq!(continue21_bigint_literal_skeleton("42"), "42n");
        assert_eq!(continue21_bigint_literal_skeleton("7n"), "7n");
        assert_eq!(continue21_regexp_literal_skeleton("ab+", "gi"), "/ab+/gi");
        assert_eq!(
            continue21_directive_literal_skeleton("use strict", true),
            "\"use strict\";"
        );
    }
}

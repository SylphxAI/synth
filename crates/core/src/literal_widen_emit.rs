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


// ── continue22 pure residual: Null/Boolean/Numeric/String literal emit ──

/// Type guards for continue22 primitive literal AST node types.
#[must_use]
pub fn is_continue22_primitive_literal_type(t: &str) -> bool {
    matches!(
        t,
        "NullLiteral" | "BooleanLiteral" | "NumericLiteral" | "StringLiteral"
    )
}

#[must_use]
pub fn is_continue22_null_literal_type(t: &str) -> bool {
    t == "NullLiteral"
}

#[must_use]
pub fn is_continue22_boolean_literal_type(t: &str) -> bool {
    t == "BooleanLiteral"
}

#[must_use]
pub fn is_continue22_numeric_literal_type(t: &str) -> bool {
    t == "NumericLiteral"
}

#[must_use]
pub fn is_continue22_string_literal_type(t: &str) -> bool {
    t == "StringLiteral"
}

/// Dual-oracle NullLiteral emit residual.
#[must_use]
pub fn continue22_null_literal_skeleton() -> &'static str {
    "null"
}

/// Dual-oracle BooleanLiteral emit residual.
#[must_use]
pub fn continue22_boolean_literal_skeleton(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

/// Dual-oracle NumericLiteral emit residual (raw value string).
#[must_use]
pub fn continue22_numeric_literal_skeleton(raw: &str) -> String {
    raw.trim().to_string()
}

/// Dual-oracle StringLiteral emit residual with quote style.
#[must_use]
pub fn continue22_string_literal_skeleton(value: &str, double_quote: bool) -> String {
    if double_quote {
        format!("\"{value}\"")
    } else {
        format!("'{value}'")
    }
}


// ── continue23 pure residual: TemplateLiteral / TaggedTemplateExpression emit ──

/// Type guards for continue23 template-related AST node types.
#[must_use]
pub fn is_continue23_template_related_type(t: &str) -> bool {
    matches!(t, "TemplateLiteral" | "TaggedTemplateExpression" | "TemplateElement")
}

#[must_use]
pub fn is_continue23_template_literal_type(t: &str) -> bool {
    t == "TemplateLiteral"
}

#[must_use]
pub fn is_continue23_tagged_template_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

#[must_use]
pub fn is_continue23_template_element_type(t: &str) -> bool {
    t == "TemplateElement"
}

/// Dual-oracle residual: empty template literal skeleton.
#[must_use]
pub fn continue23_empty_template_literal_skeleton() -> &'static str {
    "``"
}

/// Dual-oracle residual: cooked template element fragment emit (no expressions).
#[must_use]
pub fn continue23_template_element_cooked(cooked: &str) -> String {
    cooked.to_string()
}

/// Dual-oracle residual: template literal with single cooked quasi and no expressions.
#[must_use]
pub fn continue23_template_literal_no_expr_skeleton(cooked: &str) -> String {
    format!("`{cooked}`")
}

/// Dual-oracle residual: tagged template skeleton tag + empty template.
#[must_use]
pub fn continue23_tagged_template_skeleton(tag: &str) -> String {
    format!("{tag}``")
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

    #[test]
    fn continue22_primitive_literal_emit() {
        assert!(is_continue22_primitive_literal_type("NullLiteral"));
        assert!(is_continue22_primitive_literal_type("BooleanLiteral"));
        assert!(is_continue22_primitive_literal_type("NumericLiteral"));
        assert!(is_continue22_primitive_literal_type("StringLiteral"));
        assert!(!is_continue22_primitive_literal_type("BigIntLiteral"));
        assert!(is_continue22_null_literal_type("NullLiteral"));
        assert!(is_continue22_boolean_literal_type("BooleanLiteral"));
        assert!(is_continue22_numeric_literal_type("NumericLiteral"));
        assert!(is_continue22_string_literal_type("StringLiteral"));
        assert_eq!(continue22_null_literal_skeleton(), "null");
        assert_eq!(continue22_boolean_literal_skeleton(true), "true");
        assert_eq!(continue22_boolean_literal_skeleton(false), "false");
        assert_eq!(continue22_numeric_literal_skeleton(" 3.14 "), "3.14");
        assert_eq!(continue22_string_literal_skeleton("hi", true), "\"hi\"");
        assert_eq!(continue22_string_literal_skeleton("hi", false), "'hi'");
    }

    #[test]
    fn continue23_template_literal_emit() {
        assert!(is_continue23_template_related_type("TemplateLiteral"));
        assert!(is_continue23_template_related_type("TaggedTemplateExpression"));
        assert!(is_continue23_template_related_type("TemplateElement"));
        assert!(!is_continue23_template_related_type("StringLiteral"));
        assert!(is_continue23_template_literal_type("TemplateLiteral"));
        assert!(is_continue23_tagged_template_type("TaggedTemplateExpression"));
        assert!(is_continue23_template_element_type("TemplateElement"));
        assert_eq!(continue23_empty_template_literal_skeleton(), "``");
        assert_eq!(continue23_template_element_cooked("hi"), "hi");
        assert_eq!(continue23_template_literal_no_expr_skeleton("hello"), "`hello`");
        assert_eq!(continue23_tagged_template_skeleton("String.raw"), "String.raw``");
    }

}

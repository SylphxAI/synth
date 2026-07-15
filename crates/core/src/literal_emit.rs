//! Pure literal token emission —
//! mirrors printLiteral / compressLiteral in format + minify packages.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! Does not escape contents — parity with simple TS printer/compressor paths.
//! NO full formatter engine, NO authority_rust / ts_deleted.

use crate::format_indent::quote_string_literal;

/// Emit a string literal under format or minify quote policy.
/// `minify=true` forces double quotes (TS compressLiteral); else respects `single_quote`.
#[must_use]
pub fn emit_string_literal(value: &str, single_quote: bool, minify: bool) -> String {
    if minify {
        quote_string_literal(value, false)
    } else {
        quote_string_literal(value, single_quote)
    }
}

/// Prefer raw non-string representation when present (TS: String(raw) for non-strings).
/// Empty raw falls through to `fallback`.
#[must_use]
pub fn emit_raw_or_fallback(raw: Option<&str>, fallback: &str) -> String {
    match raw {
        Some(r) if !r.is_empty() => r.to_string(),
        _ => fallback.to_string(),
    }
}

/// Full literal emission pure branch.
/// - When `value_is_string`, uses quote policy (raw ignored for strings, matching TS).
/// - Else prefers `raw` when present, else `value_repr` (JSON.stringify stand-in).
#[must_use]
pub fn emit_literal(
    value_is_string: bool,
    value_str: Option<&str>,
    raw: Option<&str>,
    single_quote: bool,
    minify: bool,
) -> String {
    if value_is_string {
        let v = value_str.unwrap_or("");
        return emit_string_literal(v, single_quote, minify);
    }
    if let Some(r) = raw {
        return r.to_string();
    }
    value_str.unwrap_or("").to_string()
}

/// Boolean / null / undefined / number token from a typed tag (pure keyword map).
/// Unknown tags return empty (caller supplies raw/value).
#[must_use]
pub fn emit_primitive_keyword(kind: &str) -> Option<&'static str> {
    match kind {
        "true" | "boolean_true" => Some("true"),
        "false" | "boolean_false" => Some("false"),
        "null" => Some("null"),
        "undefined" => Some("undefined"),
        "nan" | "NaN" => Some("NaN"),
        "infinity" | "Infinity" => Some("Infinity"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_quote_policy() {
        assert_eq!(emit_string_literal("hi", false, false), "\"hi\"");
        assert_eq!(emit_string_literal("hi", true, false), "'hi'");
        // minify always double
        assert_eq!(emit_string_literal("hi", true, true), "\"hi\"");
        assert_eq!(emit_string_literal("hi", false, true), "\"hi\"");
    }

    #[test]
    fn raw_and_full_emit() {
        assert_eq!(emit_raw_or_fallback(Some("42"), "0"), "42");
        assert_eq!(emit_raw_or_fallback(Some(""), "0"), "0");
        assert_eq!(emit_raw_or_fallback(None, "0"), "0");

        assert_eq!(
            emit_literal(true, Some("x"), Some("'x'"), true, false),
            "'x'"
        );
        assert_eq!(
            emit_literal(true, Some("x"), Some("'x'"), false, true),
            "\"x\""
        );
        assert_eq!(
            emit_literal(false, Some("99"), Some("0x63"), false, false),
            "0x63"
        );
        assert_eq!(
            emit_literal(false, Some("99"), None, false, false),
            "99"
        );
        assert_eq!(emit_literal(false, None, None, false, false), "");
    }

    #[test]
    fn primitive_keywords() {
        assert_eq!(emit_primitive_keyword("true"), Some("true"));
        assert_eq!(emit_primitive_keyword("false"), Some("false"));
        assert_eq!(emit_primitive_keyword("null"), Some("null"));
        assert_eq!(emit_primitive_keyword("undefined"), Some("undefined"));
        assert_eq!(emit_primitive_keyword("NaN"), Some("NaN"));
        assert_eq!(emit_primitive_keyword("Infinity"), Some("Infinity"));
        assert_eq!(emit_primitive_keyword("string"), None);
    }
}

//! Pure method-definition kind emission helpers —
//! mirrors `printMethodDefinition` / `compressMethodDefinition`
//! (`kind && kind !== 'method'` write branch) in format/minify tooling.
//! Residual pure continue for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Whether a method `kind` field should be written as a prefix token.
/// TS: `if (kind && kind !== 'method') write(\`${kind} \`)`.
/// Empty / missing / `"method"` → false; `"get"` | `"set"` | `"constructor"` → true.
#[must_use]
pub fn should_emit_method_kind(kind: Option<&str>) -> bool {
    matches!(kind, Some(k) if !k.is_empty() && k != "method")
}

/// Normalize method kind for emission (None / empty / "method" → "").
/// Non-empty other kinds returned as-is for caller to pass to `method_kind_prefix`.
#[must_use]
pub fn normalize_method_kind(kind: Option<&str>) -> &str {
    match kind {
        Some(k) if should_emit_method_kind(Some(k)) => k,
        _ => "",
    }
}

/// Method params + body glue after the key (pretty → `() `; minify → `()`).
/// TS format: write('() '); minify: write('()').
/// Note: `fn_sig_emit::method_params` is the full `() ` / `()` token — this is an alias
/// kept for method-definition call-site clarity.
#[must_use]
pub fn method_after_key(pretty: bool) -> &'static str {
    if pretty {
        "() "
    } else {
        "()"
    }
}

/// True when method value child should be walked for a BlockStatement body.
/// Pure presence check: value exists when FunctionExpression index is Some.
#[must_use]
pub fn method_has_value(value_index: Option<usize>) -> bool {
    value_index.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_kind_decision() {
        assert!(!should_emit_method_kind(None));
        assert!(!should_emit_method_kind(Some("")));
        assert!(!should_emit_method_kind(Some("method")));
        assert!(should_emit_method_kind(Some("get")));
        assert!(should_emit_method_kind(Some("set")));
        assert!(should_emit_method_kind(Some("constructor")));
    }

    #[test]
    fn normalize_and_after_key() {
        assert_eq!(normalize_method_kind(None), "");
        assert_eq!(normalize_method_kind(Some("method")), "");
        assert_eq!(normalize_method_kind(Some("get")), "get");
        assert_eq!(method_after_key(true), "() ");
        assert_eq!(method_after_key(false), "()");
        assert!(method_has_value(Some(1)));
        assert!(!method_has_value(None));
    }
}

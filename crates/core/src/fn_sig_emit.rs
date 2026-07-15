//! Pure function/method signature emission glue —
//! mirrors printer/compressor FunctionDeclaration + MethodDefinition close-before-body.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Close of parameter list before the function/method body.
/// Format: `) ` (space before `{` is sometimes separate via body_brace_open);
/// TS format FunctionDeclaration writes `) ` then body; minify writes `)`.
#[must_use]
pub fn params_close_before_body(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// Empty method parameter list token (no inspected params in TS path).
/// Format MethodDefinition: `() `; minify: `()`.
#[must_use]
pub fn method_params(pretty: bool) -> &'static str {
    if pretty {
        "() "
    } else {
        "()"
    }
}

/// Space before a named function/class identifier after the keyword.
/// TS: `function${name ? \` ${name}\` : ''}` / `class ${name} `.
#[must_use]
pub fn named_id_prefix(has_name: bool) -> &'static str {
    if has_name {
        " "
    } else {
        ""
    }
}

/// Render optional name after keyword (function/class), including leading space.
/// Returns empty string when name is None/empty.
#[must_use]
pub fn emit_named_id(name: Option<&str>) -> String {
    match name {
        Some(n) if !n.is_empty() => format!(" {n}"),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_close_and_method() {
        assert_eq!(params_close_before_body(true), ") ");
        assert_eq!(params_close_before_body(false), ")");
        assert_eq!(method_params(true), "() ");
        assert_eq!(method_params(false), "()");
    }

    #[test]
    fn named_id() {
        assert_eq!(named_id_prefix(true), " ");
        assert_eq!(named_id_prefix(false), "");
        assert_eq!(emit_named_id(Some("foo")), " foo");
        assert_eq!(emit_named_id(Some("")), "");
        assert_eq!(emit_named_id(None), "");
    }
}

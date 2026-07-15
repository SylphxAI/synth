//! Pure format/minify emit helpers — member access, keyword prefixes, unary word ops.
//! Mirrors printer/compressor branches in `packages/synth-js-format` / `synth-js-minify`.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Word unary / keyword operators that need a trailing space before the operand.
const WORD_UNARY_OPS: &[&str] = &["typeof", "delete", "void", "await", "new", "yield", "throw"];

/// Whether a unary/prefix operator needs a trailing space (word ops).
#[must_use]
pub fn unary_needs_trailing_space(operator: &str) -> bool {
    WORD_UNARY_OPS.contains(&operator)
}

/// Render unary operator with optional trailing space.
#[must_use]
pub fn format_unary_operator(operator: &str) -> String {
    if unary_needs_trailing_space(operator) {
        format!("{operator} ")
    } else {
        operator.to_string()
    }
}

/// Member-access open delimiter (TS compress/print MemberExpression).
/// `computed=true` → `[`; else `.`.
#[must_use]
pub fn member_access_open(computed: bool) -> &'static str {
    if computed { "[" } else { "." }
}

/// Member-access close delimiter (`]` when computed, else empty).
#[must_use]
pub fn member_access_close(computed: bool) -> &'static str {
    if computed { "]" } else { "" }
}

/// `async ` prefix when function/method is async.
#[must_use]
pub fn async_prefix(is_async: bool) -> &'static str {
    if is_async { "async " } else { "" }
}

/// Generator `*` suffix after `function` keyword.
#[must_use]
pub fn generator_suffix(is_generator: bool) -> &'static str {
    if is_generator { "*" } else { "" }
}

/// Method kind prefix (`get `, `set `, empty for plain method).
/// TS: write kind only when kind && kind !== 'method'.
#[must_use]
pub fn method_kind_prefix(kind: &str) -> &'static str {
    match kind {
        "get" => "get ",
        "set" => "set ",
        "constructor" => "constructor ",
        _ => "", // "method" | unknown
    }
}

/// Export declaration prefix (`export ` / `export default `).
#[must_use]
pub fn export_prefix(is_default: bool) -> &'static str {
    if is_default {
        "export default "
    } else {
        "export "
    }
}

/// Variable declaration kind token with trailing space (default `const `).
#[must_use]
pub fn var_kind_token(kind: Option<&str>) -> String {
    let k = kind.filter(|s| !s.is_empty()).unwrap_or("const");
    format!("{k} ")
}

/// Whether printing would exceed print width (pure wrap decision).
/// `current_col` is 0-based column before emission; `addition` is chars to write.
#[must_use]
pub fn exceeds_print_width(current_col: usize, addition: usize, print_width: usize) -> bool {
    if print_width == 0 {
        return false;
    }
    current_col.saturating_add(addition) > print_width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unary_word_ops() {
        assert!(unary_needs_trailing_space("typeof"));
        assert!(unary_needs_trailing_space("delete"));
        assert!(unary_needs_trailing_space("void"));
        assert!(unary_needs_trailing_space("await"));
        assert!(!unary_needs_trailing_space("!"));
        assert!(!unary_needs_trailing_space("-"));
        assert!(!unary_needs_trailing_space("++"));
        assert_eq!(format_unary_operator("typeof"), "typeof ");
        assert_eq!(format_unary_operator("!"), "!");
        assert_eq!(format_unary_operator("await"), "await ");
    }

    #[test]
    fn member_access() {
        assert_eq!(member_access_open(false), ".");
        assert_eq!(member_access_open(true), "[");
        assert_eq!(member_access_close(false), "");
        assert_eq!(member_access_close(true), "]");
    }

    #[test]
    fn prefixes() {
        assert_eq!(async_prefix(true), "async ");
        assert_eq!(async_prefix(false), "");
        assert_eq!(generator_suffix(true), "*");
        assert_eq!(generator_suffix(false), "");
        assert_eq!(method_kind_prefix("get"), "get ");
        assert_eq!(method_kind_prefix("set"), "set ");
        assert_eq!(method_kind_prefix("method"), "");
        assert_eq!(method_kind_prefix("constructor"), "constructor ");
        assert_eq!(export_prefix(false), "export ");
        assert_eq!(export_prefix(true), "export default ");
        assert_eq!(var_kind_token(Some("let")), "let ");
        assert_eq!(var_kind_token(None), "const ");
        assert_eq!(var_kind_token(Some("")), "const ");
    }

    #[test]
    fn print_width() {
        assert!(!exceeds_print_width(0, 10, 80));
        assert!(exceeds_print_width(75, 10, 80));
        assert!(!exceeds_print_width(70, 10, 80));
        assert!(!exceeds_print_width(100, 5, 0)); // disabled
        assert!(exceeds_print_width(80, 1, 80));
    }
}

//! Pure format/minify style helpers — mirrors printer/compressor option branches
//! in `packages/synth-js-format` and `packages/synth-js-minify`.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Semicolon suffix (TS: options.semi).
#[must_use]
pub fn semi_suffix(semi: bool) -> &'static str {
    if semi { ";" } else { "" }
}

/// Opening brace with optional bracket spacing (TS: `{ ` vs `{`).
#[must_use]
pub fn bracket_open(bracket_spacing: bool) -> &'static str {
    if bracket_spacing { "{ " } else { "{" }
}

/// Closing brace with optional bracket spacing (TS: ` }` vs `}`).
#[must_use]
pub fn bracket_close(bracket_spacing: bool) -> &'static str {
    if bracket_spacing { " }" } else { "}" }
}

/// Word operators that need surrounding spaces even when minifying.
const SPACE_OPS: &[&str] = &["in", "instanceof"];

/// Whether binary operator needs surrounding spaces under minify (TS compressBinaryExpression).
#[must_use]
pub fn binary_op_needs_space(operator: &str) -> bool {
    SPACE_OPS.contains(&operator)
}

/// Render binary operator with format-style spaces (` a + b ` style for non-word ops too when format).
/// `spaced=true` → always surround with spaces; minify uses needs_space only.
#[must_use]
pub fn format_binary_operator(operator: &str, spaced: bool) -> String {
    if spaced || binary_op_needs_space(operator) {
        format!(" {operator} ")
    } else {
        operator.to_string()
    }
}

/// Arrow-function param parens decision (TS: arrowParens always|avoid).
/// `always=true` → always wrap; else wrap only when param_count != 1.
#[must_use]
pub fn arrow_needs_parens(always: bool, param_count: usize) -> bool {
    if always {
        true
    } else {
        param_count != 1
    }
}

/// Trailing comma decision for list contexts.
/// mode: "none" | "es5" | "all" (unknown → none).
/// `es5_context` = object/array (ES5-valid); multi-arg / function params need "all".
#[must_use]
pub fn wants_trailing_comma(mode: &str, es5_context: bool) -> bool {
    match mode {
        "all" => true,
        "es5" => es5_context,
        _ => false,
    }
}

/// Statement separator between top-level statements (format uses "\n\n"; minify "").
#[must_use]
pub fn statement_separator(pretty: bool) -> &'static str {
    if pretty { "\n\n" } else { "" }
}

/// Join list items with comma + optional space (format: ", "; minify: ",").
#[must_use]
pub fn list_joiner(pretty: bool) -> &'static str {
    if pretty { ", " } else { "," }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semi_and_brackets() {
        assert_eq!(semi_suffix(true), ";");
        assert_eq!(semi_suffix(false), "");
        assert_eq!(bracket_open(true), "{ ");
        assert_eq!(bracket_open(false), "{");
        assert_eq!(bracket_close(true), " }");
        assert_eq!(bracket_close(false), "}");
    }

    #[test]
    fn binary_ops() {
        assert!(binary_op_needs_space("in"));
        assert!(binary_op_needs_space("instanceof"));
        assert!(!binary_op_needs_space("+"));
        assert_eq!(format_binary_operator("+", true), " + ");
        assert_eq!(format_binary_operator("+", false), "+");
        assert_eq!(format_binary_operator("in", false), " in ");
        assert_eq!(format_binary_operator("instanceof", true), " instanceof ");
    }

    #[test]
    fn arrow_parens_and_trailing() {
        assert!(arrow_needs_parens(true, 1));
        assert!(arrow_needs_parens(true, 0));
        assert!(!arrow_needs_parens(false, 1));
        assert!(arrow_needs_parens(false, 0));
        assert!(arrow_needs_parens(false, 2));

        assert!(!wants_trailing_comma("none", true));
        assert!(wants_trailing_comma("es5", true));
        assert!(!wants_trailing_comma("es5", false));
        assert!(wants_trailing_comma("all", false));
        assert!(!wants_trailing_comma("unknown", true));
    }

    #[test]
    fn separators() {
        assert_eq!(statement_separator(true), "\n\n");
        assert_eq!(statement_separator(false), "");
        assert_eq!(list_joiner(true), ", ");
        assert_eq!(list_joiner(false), ",");
    }
}

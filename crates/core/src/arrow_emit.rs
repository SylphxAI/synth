//! Pure arrow-function emission helpers —
//! mirrors `packages/synth-js-format/src/printer.ts` printArrowFunction and
//! `packages/synth-js-minify/src/compressor.ts` compressArrowFunction.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Arrow operator token (pretty → ` => `; minify → `=>`).
#[must_use]
pub fn arrow_token(pretty: bool) -> &'static str {
    if pretty {
        " => "
    } else {
        "=>"
    }
}

/// Whether arrow parameter list needs parentheses.
///
/// TS minify: wrap when `param_count != 1` **or** mangling is enabled
/// (mangle rewrites identifiers; sole-param bare form is suppressed).
/// Format path typically forces parens via `arrowParens: 'always'`
/// (see `arrow_needs_parens` in `format_style`).
///
/// `force_parens` covers mangle / arrowParens-always.
#[must_use]
pub fn arrow_params_need_parens(param_count: usize, force_parens: bool) -> bool {
    force_parens || param_count != 1
}

/// Open delimiter for arrow params (`(` when parens required, else empty).
#[must_use]
pub fn arrow_params_open(need_parens: bool) -> &'static str {
    if need_parens {
        "("
    } else {
        ""
    }
}

/// Close of arrow params + arrow operator.
///
/// - parens + pretty → `) => `
/// - parens + minify → `)=>`
/// - bare + pretty → ` => `
/// - bare + minify → `=>`
#[must_use]
pub fn arrow_after_params(pretty: bool, need_parens: bool) -> &'static str {
    match (need_parens, pretty) {
        (true, true) => ") => ",
        (true, false) => ")=>",
        (false, true) => " => ",
        (false, false) => "=>",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_token_pretty_vs_minify() {
        assert_eq!(arrow_token(true), " => ");
        assert_eq!(arrow_token(false), "=>");
    }

    #[test]
    fn params_need_parens() {
        assert!(!arrow_params_need_parens(1, false));
        assert!(arrow_params_need_parens(1, true)); // force (mangle / always)
        assert!(arrow_params_need_parens(0, false));
        assert!(arrow_params_need_parens(2, false));
        assert!(arrow_params_need_parens(2, true));
    }

    #[test]
    fn open_and_after() {
        assert_eq!(arrow_params_open(true), "(");
        assert_eq!(arrow_params_open(false), "");
        assert_eq!(arrow_after_params(true, true), ") => ");
        assert_eq!(arrow_after_params(false, true), ")=>");
        assert_eq!(arrow_after_params(true, false), " => ");
        assert_eq!(arrow_after_params(false, false), "=>");
    }
}

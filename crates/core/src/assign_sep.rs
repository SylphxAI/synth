//! Pure assignment / property / call separator helpers —
//! mirrors printer (pretty) vs compressor (compact) spacing branches.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Assignment operator between lhs and rhs (pretty → ` = `; minify → `=`).
#[must_use]
pub fn assign_op(pretty: bool) -> &'static str {
    if pretty {
        " = "
    } else {
        "="
    }
}

/// Object property key/value separator (pretty → `: `; minify → `:`).
#[must_use]
pub fn prop_sep(pretty: bool) -> &'static str {
    if pretty {
        ": "
    } else {
        ":"
    }
}

/// Call / group open paren.
#[must_use]
pub fn call_open() -> &'static str {
    "("
}

/// Call / group close paren.
#[must_use]
pub fn call_close() -> &'static str {
    ")"
}

/// Array open bracket.
#[must_use]
pub fn array_open() -> &'static str {
    "["
}

/// Array close bracket.
#[must_use]
pub fn array_close() -> &'static str {
    "]"
}

/// Space after `function` keyword before name or `*` / `(` (TS always writes `function ` or `function*`).
/// When generator, the `*` is handled by `generator_suffix`; this is the base keyword.
#[must_use]
pub fn function_keyword(is_generator: bool) -> &'static str {
    if is_generator {
        "function*"
    } else {
        "function"
    }
}

/// Space between function keyword (or name) and params list open.
/// Named: `function foo(` needs space before name handled by caller; between name and `(` none.
/// Anonymous: `function(` minify vs `function (` pretty — rare; TS format uses `function name(`.
#[must_use]
pub fn function_params_glue(pretty: bool, has_name: bool) -> &'static str {
    if has_name {
        ""
    } else if pretty {
        " "
    } else {
        ""
    }
}

/// Empty object/array token (no elements).
#[must_use]
pub fn empty_braces(kind: &str) -> &'static str {
    match kind {
        "array" => "[]",
        _ => "{}", // object | unknown
    }
}

/// Whether to insert a space before `{` after `)` in function/method bodies (format style).
/// TS: `) {` pretty; `){` minify.
#[must_use]
pub fn body_brace_open(pretty: bool) -> &'static str {
    if pretty {
        " {"
    } else {
        "{"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_and_prop() {
        assert_eq!(assign_op(true), " = ");
        assert_eq!(assign_op(false), "=");
        assert_eq!(prop_sep(true), ": ");
        assert_eq!(prop_sep(false), ":");
    }

    #[test]
    fn delimiters() {
        assert_eq!(call_open(), "(");
        assert_eq!(call_close(), ")");
        assert_eq!(array_open(), "[");
        assert_eq!(array_close(), "]");
        assert_eq!(empty_braces("array"), "[]");
        assert_eq!(empty_braces("object"), "{}");
        assert_eq!(empty_braces("other"), "{}");
    }

    #[test]
    fn function_and_body() {
        assert_eq!(function_keyword(false), "function");
        assert_eq!(function_keyword(true), "function*");
        assert_eq!(function_params_glue(true, false), " ");
        assert_eq!(function_params_glue(false, false), "");
        assert_eq!(function_params_glue(true, true), "");
        assert_eq!(body_brace_open(true), " {");
        assert_eq!(body_brace_open(false), "{");
    }
}

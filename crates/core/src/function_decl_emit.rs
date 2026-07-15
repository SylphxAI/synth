//! Pure FunctionDeclaration emission helpers —
//! residual pure continue10 for tooling/format-minify-lint fragment.
//! Mirrors `printFunctionDeclaration` / `compressFunctionDeclaration` in
//! `packages/synth-js-format` / `synth-js-minify` (async / function / generator /
//! name / params / body glue). Full engines remain product dens.
//! NO authority_rust / ts_deleted.
#![allow(dead_code)]

/// `async ` prefix when `node.data.async` is true.
#[must_use]
pub fn async_prefix(is_async: bool) -> &'static str {
    if is_async {
        "async "
    } else {
        ""
    }
}

/// Core `function` keyword (always present for FunctionDeclaration).
#[must_use]
pub fn function_keyword() -> &'static str {
    "function"
}

/// Generator star after `function` (TS: `function*`).
#[must_use]
pub fn generator_star(is_generator: bool) -> &'static str {
    if is_generator {
        "*"
    } else {
        ""
    }
}

/// Name fragment after function keyword: pretty/minify both use ` ${name}` when present.
#[must_use]
pub fn function_name_fragment(name: Option<&str>) -> String {
    match name {
        Some(n) if !n.is_empty() => format!(" {n}"),
        _ => String::new(),
    }
}

/// Params open `(` (shared).
#[must_use]
pub fn function_params_open() -> &'static str {
    "("
}

/// Param list separator: pretty `, ` vs minify `,`.
#[must_use]
pub fn function_param_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Close of params before body: pretty `) ` vs minify `)`.
#[must_use]
pub fn function_params_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// Build FunctionDeclaration head skeleton up to (and including) params close.
/// `params` are already-rendered param fragments; `name` optional id (pre-mangle).
#[must_use]
pub fn function_declaration_head(
    is_async: bool,
    is_generator: bool,
    name: Option<&str>,
    params: &[&str],
    pretty: bool,
) -> String {
    let mut out = String::with_capacity(32 + params.iter().map(|p| p.len()).sum::<usize>());
    out.push_str(async_prefix(is_async));
    out.push_str(function_keyword());
    out.push_str(generator_star(is_generator));
    out.push_str(&function_name_fragment(name));
    out.push_str(function_params_open());
    for (i, p) in params.iter().enumerate() {
        if i > 0 {
            out.push_str(function_param_sep(pretty));
        }
        out.push_str(p);
    }
    out.push_str(function_params_close(pretty));
    out
}

/// Full FunctionDeclaration skeleton: head + body fragment.
#[must_use]
pub fn function_declaration_skeleton(
    is_async: bool,
    is_generator: bool,
    name: Option<&str>,
    params: &[&str],
    body: &str,
    pretty: bool,
) -> String {
    let mut out = function_declaration_head(is_async, is_generator, name, params, pretty);
    out.push_str(body);
    out
}

/// Type guard for FunctionDeclaration.
#[must_use]
pub fn is_function_declaration_type(t: &str) -> bool {
    t == "FunctionDeclaration"
}

/// Type guard for function-like (declaration or expression).
#[must_use]
pub fn is_function_like_type(t: &str) -> bool {
    matches!(
        t,
        "FunctionDeclaration" | "FunctionExpression" | "ArrowFunctionExpression"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_and_keyword() {
        assert_eq!(async_prefix(true), "async ");
        assert_eq!(async_prefix(false), "");
        assert_eq!(function_keyword(), "function");
        assert_eq!(generator_star(true), "*");
        assert_eq!(generator_star(false), "");
        assert_eq!(function_name_fragment(Some("foo")), " foo");
        assert_eq!(function_name_fragment(None), "");
        assert_eq!(function_params_open(), "(");
        assert_eq!(function_param_sep(true), ", ");
        assert_eq!(function_param_sep(false), ",");
        assert_eq!(function_params_close(true), ") ");
        assert_eq!(function_params_close(false), ")");
    }

    #[test]
    fn skeleton_pretty_vs_minify() {
        let pretty = function_declaration_skeleton(
            true,
            true,
            Some("gen"),
            &["a", "b"],
            "{ return a; }",
            true,
        );
        assert_eq!(pretty, "async function* gen(a, b) { return a; }");

        let minify =
            function_declaration_skeleton(false, false, Some("f"), &["x"], "{return x;}", false);
        assert_eq!(minify, "function f(x){return x;}");

        let anon = function_declaration_head(false, false, None, &[], true);
        assert_eq!(anon, "function() ");
    }

    #[test]
    fn type_guards() {
        assert!(is_function_declaration_type("FunctionDeclaration"));
        assert!(!is_function_declaration_type("MethodDefinition"));
        assert!(is_function_like_type("ArrowFunctionExpression"));
        assert!(!is_function_like_type("ClassDeclaration"));
    }
}

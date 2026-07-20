//! Pure ArrowFunctionExpression + MethodDefinition(static/async/params/computed)
//! + ExportDefaultDeclaration dual-oracle emission —
//! residual pure **continue72** for tooling/format-minify-lint fragment.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - ArrowFunctionExpression full dual-oracle (async, arrowParens, param sep,
//!   pretty ` => ` vs minify `=>`) — widens fixed continue29/41 single forms
//! - MethodDefinition with static / async / generator / computed key / real
//!   params list (prior method_definition_skeleton always emits empty `()`)
//! - ExportDefaultDeclaration dual-oracle for function / class / expression
//!   declaration fragments (+ optional semi)
//!
//! Intentionally does **not** re-wrap continue64–71 partition shells.
//! Composes real shipped pure helpers (`async_prefix`, `arrow_*`,
//! `static_prefix`, `export_keyword`, `export_default_fragment`,
//! `function_param_sep`, `computed_property_key`, `method_kind_prefix`).
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::arrow_emit::{arrow_after_params, arrow_params_need_parens, arrow_params_open};
use crate::export_class_emit::{export_default_fragment, export_keyword};
use crate::format_emit::{async_prefix, generator_suffix, method_kind_prefix as format_method_kind};
use crate::format_style::arrow_needs_parens;
use crate::function_decl_emit::function_param_sep;
use crate::property_static_emit::{computed_property_key, static_prefix};

/// Dual-oracle residual: continue72 related AST type catalog.
pub const CONTINUE72_RELATED_TYPES: &[&str] = &[
    "ArrowFunctionExpression",
    "MethodDefinition",
    "FunctionExpression",
    "ExportDefaultDeclaration",
    "FunctionDeclaration",
    "ClassDeclaration",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_arrow_method_default_related_type(t: &str) -> bool {
    CONTINUE72_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue72_arrow_type(t: &str) -> bool {
    t == "ArrowFunctionExpression"
}

#[must_use]
pub fn is_continue72_method_definition_type(t: &str) -> bool {
    t == "MethodDefinition"
}

#[must_use]
pub fn is_continue72_function_expression_type(t: &str) -> bool {
    t == "FunctionExpression"
}

#[must_use]
pub fn is_continue72_export_default_type(t: &str) -> bool {
    t == "ExportDefaultDeclaration"
}

#[must_use]
pub fn is_continue72_function_declaration_type(t: &str) -> bool {
    t == "FunctionDeclaration"
}

#[must_use]
pub fn is_continue72_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

// ── ArrowFunctionExpression dual-oracle ─────────────────────────────────────

/// Dual-oracle param list interior for arrow / method forms.
/// Pretty: `, `; minify: `,`.
#[must_use]
pub fn param_list_interior(params: &[&str], pretty: bool) -> String {
    let sep = function_param_sep(pretty);
    let mut out = String::with_capacity(
        params.iter().map(|p| p.len()).sum::<usize>() + sep.len() * params.len().saturating_sub(1),
    );
    for (i, p) in params.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(p);
    }
    out
}

/// Whether arrow params need parens under dual-oracle policy.
///
/// - Pretty path: honors `arrow_parens_always` via [`arrow_needs_parens`].
/// - Minify path: bare single param only when `!mangle` (mirrors compressor);
///   force parens when `mangle` is true.
#[must_use]
pub fn arrow_dual_needs_parens(
    pretty: bool,
    arrow_parens_always: bool,
    param_count: usize,
    mangle: bool,
) -> bool {
    if pretty {
        arrow_needs_parens(arrow_parens_always, param_count)
    } else {
        // compressArrowFunction: single param without parens iff !mangle
        arrow_params_need_parens(param_count, mangle)
    }
}

/// Dual-oracle ArrowFunctionExpression skeleton.
///
/// Pretty (arrowParens always): `(a, b) => body` / `async (x) => body`
/// Pretty (avoid, 1 param): `x => body`
/// Minify: denser seps + `=>` without spaces; optional bare single param.
///
/// Drives real shipped: [`async_prefix`], [`arrow_params_open`],
/// [`arrow_after_params`], [`arrow_token`], [`function_param_sep`].
#[must_use]
pub fn arrow_function_skeleton(
    is_async: bool,
    params: &[&str],
    body: &str,
    pretty: bool,
    arrow_parens_always: bool,
    mangle: bool,
) -> String {
    let need_parens =
        arrow_dual_needs_parens(pretty, arrow_parens_always, params.len(), mangle);
    let interior = param_list_interior(params, pretty);
    let mut out = String::with_capacity(
        16 + interior.len() + body.len() + if is_async { 6 } else { 0 },
    );
    out.push_str(async_prefix(is_async));
    out.push_str(arrow_params_open(need_parens));
    out.push_str(&interior);
    // arrow_after_params already embeds the arrow token when need_parens is set;
    // when bare (no parens), it returns ` => ` / `=>` alone.
    out.push_str(arrow_after_params(pretty, need_parens));
    out.push_str(body);
    out
}

/// Convenience: pretty always-parens arrow (printer default `arrowParens: 'always'`).
#[must_use]
pub fn arrow_function_pretty_always(params: &[&str], body: &str, is_async: bool) -> String {
    arrow_function_skeleton(is_async, params, body, true, true, false)
}

/// Convenience: minify dual-oracle (no mangle).
#[must_use]
pub fn arrow_function_minify(params: &[&str], body: &str, is_async: bool) -> String {
    arrow_function_skeleton(is_async, params, body, false, false, false)
}

/// Empty / zero-param arrow dual-oracle residual (engines use `() => {}` / `()=>{}`).
#[must_use]
pub fn arrow_empty_skeleton(pretty: bool) -> &'static str {
    if pretty {
        "() => {}"
    } else {
        "()=>{}"
    }
}

// ── MethodDefinition dual-oracle (static / async / generator / computed / params) ─

/// Dual-oracle method key fragment: computed `[expr]` vs bare id.
#[must_use]
pub fn method_key_fragment(key: &str, computed: bool) -> String {
    if computed {
        computed_property_key(key)
    } else {
        key.to_owned()
    }
}

/// Method params open/close glue: pretty `(a, b) ` vs minify `(a,b)`.
#[must_use]
pub fn method_params_fragment(params: &[&str], pretty: bool) -> String {
    let interior = param_list_interior(params, pretty);
    if pretty {
        format!("({interior}) ")
    } else {
        format!("({interior})")
    }
}

/// Dual-oracle MethodDefinition skeleton with static/async/generator/kind/
/// computed key / real params — residual not covered by empty-`()` shells.
///
/// Pretty: `static async *get [k](a, b) {body}`
/// Minify: denser param sep, no space before body brace.
///
/// Drives real shipped: [`static_prefix`], [`async_prefix`],
/// [`generator_suffix`], [`format_method_kind`], [`computed_property_key`],
/// [`function_param_sep`].
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn method_definition_full_skeleton(
    kind: Option<&str>,
    key: &str,
    params: &[&str],
    body: Option<&str>,
    is_static: bool,
    is_async: bool,
    is_generator: bool,
    computed: bool,
    pretty: bool,
) -> String {
    let kind_tok = match kind {
        Some(k) if !k.is_empty() && k != "method" => format_method_kind(k),
        _ => "",
    };
    let key_frag = method_key_fragment(key, computed);
    let params_frag = method_params_fragment(params, pretty);
    let mut out = String::with_capacity(
        24 + key_frag.len()
            + params_frag.len()
            + body.map_or(0, str::len)
            + kind_tok.len(),
    );
    out.push_str(static_prefix(is_static));
    out.push_str(async_prefix(is_async));
    // generator star sits after kind for get/set-free methods; for kind prefixes
    // (get/set) generators are rare — still emit star before key (babel order:
    // static async * name).
    out.push_str(kind_tok);
    out.push_str(generator_suffix(is_generator));
    out.push_str(&key_frag);
    out.push_str(&params_frag);
    if let Some(b) = body {
        out.push_str(b);
    }
    out
}

// ── ExportDefaultDeclaration dual-oracle ────────────────────────────────────

/// Dual-oracle ExportDefaultDeclaration skeleton: `export default <decl>[;]`.
///
/// Pretty honors `semi` for expression defaults; minify always adds `;` for
/// non-block-bodied declarations (compressor residual). Block-bodied
/// function/class fragments ending in `}` never get an extra semi — parity
/// with `printExportDeclaration` / `compressExportDeclaration`.
///
/// Drives real shipped: [`export_keyword`], [`export_default_fragment`].
#[must_use]
pub fn export_default_declaration_skeleton(
    declaration: &str,
    pretty: bool,
    semi: bool,
) -> String {
    let mut out = String::with_capacity(16 + declaration.len() + 1);
    out.push_str(export_keyword());
    out.push_str(export_default_fragment(true));
    out.push_str(declaration);
    let block_bodied = declaration.ends_with('}');
    if !block_bodied {
        let add_semi = if pretty { semi } else { true };
        if add_semi && !declaration.ends_with(';') {
            out.push(';');
        }
    }
    out
}

/// Export default function declaration dual-oracle residual.
/// `head` is already-rendered function head+body (from function_declaration_skeleton).
#[must_use]
pub fn export_default_function_skeleton(function_frag: &str, pretty: bool, semi: bool) -> String {
    export_default_declaration_skeleton(function_frag, pretty, semi)
}

/// Export default class declaration dual-oracle residual.
#[must_use]
pub fn export_default_class_skeleton(class_frag: &str, pretty: bool, semi: bool) -> String {
    export_default_declaration_skeleton(class_frag, pretty, semi)
}

/// Export default expression dual-oracle residual (`export default expr;`).
#[must_use]
pub fn export_default_expression_skeleton(expr: &str, pretty: bool, semi: bool) -> String {
    export_default_declaration_skeleton(expr, pretty, semi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrow_emit::{arrow_after_params, arrow_params_need_parens, arrow_token};
    use crate::export_class_emit::{
        class_declaration_skeleton, export_declaration_skeleton, export_default_fragment,
        export_keyword,
    };
    use crate::format_emit::{async_prefix, generator_suffix, method_kind_prefix};
    use crate::format_style::arrow_needs_parens;
    use crate::function_decl_emit::{function_declaration_skeleton, function_param_sep};
    use crate::property_static_emit::{computed_property_key, static_prefix};

    #[test]
    fn continue72_type_catalog() {
        assert_eq!(CONTINUE72_RELATED_TYPES.len(), 6);
        assert!(is_arrow_method_default_related_type("ArrowFunctionExpression"));
        assert!(is_arrow_method_default_related_type("MethodDefinition"));
        assert!(is_arrow_method_default_related_type("ExportDefaultDeclaration"));
        assert!(is_arrow_method_default_related_type("FunctionExpression"));
        assert!(is_arrow_method_default_related_type("FunctionDeclaration"));
        assert!(is_arrow_method_default_related_type("ClassDeclaration"));
        assert!(!is_arrow_method_default_related_type("ImportDeclaration"));
        assert!(!is_arrow_method_default_related_type("BinaryExpression"));
        assert!(is_continue72_arrow_type("ArrowFunctionExpression"));
        assert!(is_continue72_method_definition_type("MethodDefinition"));
        assert!(is_continue72_function_expression_type("FunctionExpression"));
        assert!(is_continue72_export_default_type("ExportDefaultDeclaration"));
        assert!(is_continue72_function_declaration_type("FunctionDeclaration"));
        assert!(is_continue72_class_declaration_type("ClassDeclaration"));
        assert!(!is_continue72_arrow_type("FunctionExpression"));
    }

    #[test]
    fn continue72_arrow_dual_oracle_drives_shipped() {
        // Real shipped arrow helpers remain authority for tokens / parens.
        assert_eq!(arrow_token(true), " => ");
        assert_eq!(arrow_token(false), "=>");
        assert_eq!(arrow_after_params(true, true), ") => ");
        assert_eq!(arrow_after_params(false, true), ")=>");
        assert_eq!(arrow_after_params(true, false), " => ");
        assert_eq!(arrow_after_params(false, false), "=>");
        assert!(arrow_needs_parens(true, 1));
        assert!(!arrow_needs_parens(false, 1));
        assert!(arrow_params_need_parens(1, true)); // mangle forces parens
        assert!(!arrow_params_need_parens(1, false));
        assert_eq!(function_param_sep(true), ", ");
        assert_eq!(function_param_sep(false), ",");
        assert_eq!(async_prefix(true), "async ");
        assert_eq!(async_prefix(false), "");

        // Pretty always-parens (printer default)
        assert_eq!(
            arrow_function_pretty_always(&["x"], "x + 1", false),
            "(x) => x + 1"
        );
        assert_eq!(
            arrow_function_pretty_always(&["a", "b"], "a + b", false),
            "(a, b) => a + b"
        );
        assert_eq!(
            arrow_function_pretty_always(&["x"], "x", true),
            "async (x) => x"
        );
        // Pretty avoid: bare single param
        assert_eq!(
            arrow_function_skeleton(false, &["x"], "x * 2", true, false, false),
            "x => x * 2"
        );
        // Minify dual-oracle
        assert_eq!(
            arrow_function_minify(&["x"], "x+1", false),
            "x=>x+1"
        );
        assert_eq!(
            arrow_function_minify(&["a", "b"], "a+b", false),
            "(a,b)=>a+b"
        );
        assert_eq!(
            arrow_function_minify(&["x"], "x", true),
            "async x=>x"
        );
        // Minify + mangle forces parens on single param
        assert_eq!(
            arrow_function_skeleton(false, &["x"], "x", false, false, true),
            "(x)=>x"
        );
        // Empty residual
        assert_eq!(arrow_empty_skeleton(true), "() => {}");
        assert_eq!(arrow_empty_skeleton(false), "()=>{}");
        // Zero params always parenthesized
        assert_eq!(
            arrow_function_skeleton(false, &[], "{}", true, false, false),
            "() => {}"
        );
        assert_eq!(
            arrow_function_skeleton(false, &[], "{}", false, false, false),
            "()=>{}"
        );
        assert_eq!(param_list_interior(&["a", "b"], true), "a, b");
        assert_eq!(param_list_interior(&["a", "b"], false), "a,b");
        assert!(arrow_dual_needs_parens(true, true, 1, false));
        assert!(!arrow_dual_needs_parens(true, false, 1, false));
        assert!(!arrow_dual_needs_parens(false, false, 1, false));
        assert!(arrow_dual_needs_parens(false, false, 1, true));
    }

    #[test]
    fn continue72_method_definition_full_dual_oracle() {
        assert_eq!(static_prefix(true), "static ");
        assert_eq!(static_prefix(false), "");
        assert_eq!(generator_suffix(true), "*");
        assert_eq!(generator_suffix(false), "");
        assert_eq!(method_kind_prefix("get"), "get ");
        assert_eq!(method_kind_prefix("set"), "set ");
        assert_eq!(method_kind_prefix("method"), "");
        assert_eq!(computed_property_key("k"), "[k]");
        assert_eq!(method_key_fragment("run", false), "run");
        assert_eq!(method_key_fragment("k", true), "[k]");
        assert_eq!(method_params_fragment(&["a", "b"], true), "(a, b) ");
        assert_eq!(method_params_fragment(&["a", "b"], false), "(a,b)");
        assert_eq!(method_params_fragment(&[], true), "() ");
        assert_eq!(method_params_fragment(&[], false), "()");

        // Plain method pretty
        assert_eq!(
            method_definition_full_skeleton(
                Some("method"),
                "run",
                &[],
                Some("{}"),
                false,
                false,
                false,
                false,
                true
            ),
            "run() {}"
        );
        // Minify plain
        assert_eq!(
            method_definition_full_skeleton(
                None,
                "run",
                &["a", "b"],
                Some("{return a+b;}"),
                false,
                false,
                false,
                false,
                false
            ),
            "run(a,b){return a+b;}"
        );
        // get / set
        assert_eq!(
            method_definition_full_skeleton(
                Some("get"),
                "name",
                &[],
                Some("{return this._n;}"),
                false,
                false,
                false,
                false,
                true
            ),
            "get name() {return this._n;}"
        );
        assert_eq!(
            method_definition_full_skeleton(
                Some("set"),
                "name",
                &["v"],
                Some("{}"),
                false,
                false,
                false,
                false,
                false
            ),
            "set name(v){}"
        );
        // static async generator computed
        assert_eq!(
            method_definition_full_skeleton(
                Some("method"),
                "k",
                &["x"],
                Some("{}"),
                true,
                true,
                true,
                true,
                true
            ),
            "static async *[k](x) {}"
        );
        // static get minify
        assert_eq!(
            method_definition_full_skeleton(
                Some("get"),
                "id",
                &[],
                Some("{return 1;}"),
                true,
                false,
                false,
                false,
                false
            ),
            "static get id(){return 1;}"
        );
    }

    #[test]
    fn continue72_export_default_dual_oracle() {
        assert_eq!(export_keyword(), "export ");
        assert_eq!(export_default_fragment(true), "default ");
        assert_eq!(export_default_fragment(false), "");
        assert_eq!(
            export_declaration_skeleton(true, Some("foo")),
            "export default foo"
        );

        // Expression default + semi
        assert_eq!(
            export_default_expression_skeleton("42", true, true),
            "export default 42;"
        );
        assert_eq!(
            export_default_expression_skeleton("42", true, false),
            "export default 42"
        );
        // Minify always semi for expressions
        assert_eq!(
            export_default_expression_skeleton("42", false, false),
            "export default 42;"
        );
        assert_eq!(
            export_default_expression_skeleton("foo", false, true),
            "export default foo;"
        );

        // Function declaration (block-bodied — no trailing semi)
        let fn_pretty = function_declaration_skeleton(false, false, Some("foo"), &[], "{}", true);
        assert_eq!(fn_pretty, "function foo() {}");
        assert_eq!(
            export_default_function_skeleton(&fn_pretty, true, true),
            "export default function foo() {}"
        );
        let fn_minify =
            function_declaration_skeleton(true, false, None, &["a"], "{return a;}", false);
        assert_eq!(fn_minify, "async function(a){return a;}");
        assert_eq!(
            export_default_function_skeleton(&fn_minify, false, true),
            "export default async function(a){return a;}"
        );

        // Class declaration block-bodied
        let class_pretty = class_declaration_skeleton(Some("C"), &[], true);
        assert_eq!(class_pretty, "class C {}");
        assert_eq!(
            export_default_class_skeleton(&class_pretty, true, true),
            "export default class C {}"
        );
        let class_minify = class_declaration_skeleton(None, &["m(){}"], false);
        assert_eq!(
            export_default_class_skeleton(&class_minify, false, true),
            format!("export default {class_minify}")
        );
    }
}

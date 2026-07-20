//! Pure FunctionDeclaration + FunctionExpression + ClassDeclaration +
//! ClassExpression + ThisExpression + Super dual-oracle emission — residual
//! pure **continue86** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior residual modules continue71–85:
//! - FunctionDeclaration dual-oracle pretty/minify composing real
//!   `function_declaration_skeleton` (function_decl base)
//! - FunctionExpression dual-oracle (same skeleton surface; expression form)
//! - ClassDeclaration dual-oracle pretty/minify composing real
//!   `class_declaration_skeleton` (export_class base)
//! - ClassExpression dual-oracle (same skeleton surface; expression form)
//! - ThisExpression dual-oracle composing real `this_token`
//! - Super dual-oracle composing real `super_token`
//! - Composed function/class/this/super residual shells
//!
//! Intentionally does **not** re-wrap continue64–85 partition shells
//! (ident/block/program continue85 stays separate; pattern/rest continue84
//! stays separate; yield/meta continue83 stays separate; class-extends +
//! export-from continue71 stays separate for heritage/`from` forms). Composes
//! real shipped pure helpers from function/class/this/super bases.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::assign_logical_update_emit::{super_token, this_token};
use crate::export_class_emit::class_declaration_skeleton;
use crate::function_decl_emit::function_declaration_skeleton;

/// Dual-oracle residual: continue86 related AST type catalog.
pub const CONTINUE86_RELATED_TYPES: &[&str] = &[
    "FunctionDeclaration",
    "FunctionExpression",
    "ClassDeclaration",
    "ClassExpression",
    "ThisExpression",
    "Super",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_function_class_this_related_type(t: &str) -> bool {
    CONTINUE86_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue86_function_declaration_type(t: &str) -> bool {
    t == "FunctionDeclaration"
}

#[must_use]
pub fn is_continue86_function_expression_type(t: &str) -> bool {
    t == "FunctionExpression"
}

#[must_use]
pub fn is_continue86_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}

#[must_use]
pub fn is_continue86_class_expression_type(t: &str) -> bool {
    t == "ClassExpression"
}

#[must_use]
pub fn is_continue86_this_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue86_super_type(t: &str) -> bool {
    t == "Super"
}

// ── FunctionDeclaration / FunctionExpression dual-oracle ────────────────────

/// Dual-oracle FunctionDeclaration skeleton composing real
/// [`function_declaration_skeleton`].
#[must_use]
pub fn continue86_function_declaration_skeleton(
    is_async: bool,
    is_generator: bool,
    name: Option<&str>,
    params: &[&str],
    body: &str,
    pretty: bool,
) -> String {
    function_declaration_skeleton(is_async, is_generator, name, params, body, pretty)
}

/// Dual-oracle FunctionExpression skeleton (same pure head+body surface as
/// declaration; name optional for named function expressions).
#[must_use]
pub fn continue86_function_expression_skeleton(
    is_async: bool,
    is_generator: bool,
    name: Option<&str>,
    params: &[&str],
    body: &str,
    pretty: bool,
) -> String {
    function_declaration_skeleton(is_async, is_generator, name, params, body, pretty)
}

/// Convenience: pretty named function declaration.
#[must_use]
pub fn function_declaration_pretty(name: &str, params: &[&str], body: &str) -> String {
    continue86_function_declaration_skeleton(false, false, Some(name), params, body, true)
}

/// Convenience: minify named function declaration.
#[must_use]
pub fn function_declaration_minify(name: &str, params: &[&str], body: &str) -> String {
    continue86_function_declaration_skeleton(false, false, Some(name), params, body, false)
}

/// Convenience: async generator function expression pretty.
#[must_use]
pub fn async_generator_function_expression_pretty(
    name: Option<&str>,
    params: &[&str],
    body: &str,
) -> String {
    continue86_function_expression_skeleton(true, true, name, params, body, true)
}

// ── ClassDeclaration / ClassExpression dual-oracle ──────────────────────────

/// Dual-oracle ClassDeclaration skeleton composing real
/// [`class_declaration_skeleton`].
#[must_use]
pub fn continue86_class_declaration_skeleton(
    name: Option<&str>,
    methods: &[&str],
    pretty: bool,
) -> String {
    class_declaration_skeleton(name, methods, pretty)
}

/// Dual-oracle ClassExpression skeleton (same pure class keyword + body surface).
#[must_use]
pub fn continue86_class_expression_skeleton(
    name: Option<&str>,
    methods: &[&str],
    pretty: bool,
) -> String {
    class_declaration_skeleton(name, methods, pretty)
}

/// Convenience: pretty named class.
#[must_use]
pub fn class_declaration_pretty(name: &str, methods: &[&str]) -> String {
    continue86_class_declaration_skeleton(Some(name), methods, true)
}

/// Convenience: minify named class.
#[must_use]
pub fn class_declaration_minify(name: &str, methods: &[&str]) -> String {
    continue86_class_declaration_skeleton(Some(name), methods, false)
}

/// Convenience: anonymous class expression minify.
#[must_use]
pub fn class_expression_anon_minify(methods: &[&str]) -> String {
    continue86_class_expression_skeleton(None, methods, false)
}

// ── ThisExpression / Super dual-oracle ──────────────────────────────────────

/// Dual-oracle ThisExpression skeleton composing real [`this_token`].
#[must_use]
pub fn continue86_this_expression_skeleton() -> &'static str {
    this_token()
}

/// Dual-oracle Super skeleton composing real [`super_token`].
#[must_use]
pub fn continue86_super_skeleton() -> &'static str {
    super_token()
}

/// Convenience aliases.
#[must_use]
pub fn this_expression() -> &'static str {
    continue86_this_expression_skeleton()
}

#[must_use]
pub fn super_expression() -> &'static str {
    continue86_super_skeleton()
}

// ── Composed dual-oracle shells ─────────────────────────────────────────────

/// Dual-oracle residual: method body using `this` + `super` identifiers.
#[must_use]
pub fn continue86_this_super_method_body(pretty: bool) -> String {
    let this_t = continue86_this_expression_skeleton();
    let super_t = continue86_super_skeleton();
    if pretty {
        format!("{{ return {this_t}.{super_t}; }}")
    } else {
        format!("{{return {this_t}.{super_t};}}")
    }
}

/// Dual-oracle residual: class with method body referencing this/super shells.
#[must_use]
pub fn continue86_class_with_this_super(name: &str, pretty: bool) -> String {
    let body = continue86_this_super_method_body(pretty);
    let method = if pretty {
        format!("m() {body}")
    } else {
        format!("m(){body}")
    };
    continue86_class_declaration_skeleton(Some(name), &[method.as_str()], pretty)
}

/// Dual-oracle residual: function declaration returning `this`.
#[must_use]
pub fn continue86_function_returns_this(name: &str, pretty: bool) -> String {
    let this_t = continue86_this_expression_skeleton();
    let body = if pretty {
        format!("{{ return {this_t}; }}")
    } else {
        format!("{{return {this_t};}}")
    };
    continue86_function_declaration_skeleton(false, false, Some(name), &[], &body, pretty)
}

/// Dual-oracle residual: async function expression calling `super`.
#[must_use]
pub fn continue86_async_fn_calls_super(pretty: bool) -> String {
    let super_t = continue86_super_skeleton();
    let body = if pretty {
        format!("{{ return {super_t}; }}")
    } else {
        format!("{{return {super_t};}}")
    };
    continue86_function_expression_skeleton(true, false, None, &[], &body, pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assign_logical_update_emit::{super_token, this_token};
    use crate::export_class_emit::class_declaration_skeleton;
    use crate::function_decl_emit::function_declaration_skeleton;

    #[test]
    fn continue86_type_catalog() {
        assert_eq!(CONTINUE86_RELATED_TYPES.len(), 6);
        assert!(is_function_class_this_related_type("FunctionDeclaration"));
        assert!(is_function_class_this_related_type("FunctionExpression"));
        assert!(is_function_class_this_related_type("ClassDeclaration"));
        assert!(is_function_class_this_related_type("ClassExpression"));
        assert!(is_function_class_this_related_type("ThisExpression"));
        assert!(is_function_class_this_related_type("Super"));
        assert!(!is_function_class_this_related_type("Identifier"));
        assert!(!is_function_class_this_related_type("BlockStatement"));
        assert!(!is_function_class_this_related_type("ArrowFunctionExpression"));
        assert!(!is_function_class_this_related_type("MethodDefinition"));
        assert!(is_continue86_function_declaration_type("FunctionDeclaration"));
        assert!(is_continue86_function_expression_type("FunctionExpression"));
        assert!(is_continue86_class_declaration_type("ClassDeclaration"));
        assert!(is_continue86_class_expression_type("ClassExpression"));
        assert!(is_continue86_this_type("ThisExpression"));
        assert!(is_continue86_super_type("Super"));
        assert!(!is_continue86_this_type("Super"));
        assert!(!is_continue86_super_type("ThisExpression"));
        assert!(!is_continue86_function_declaration_type("FunctionExpression"));
        assert!(!is_continue86_class_expression_type("ClassDeclaration"));
    }

    #[test]
    fn continue86_function_class_dual_oracle() {
        assert_eq!(
            function_declaration_pretty("f", &["a", "b"], "{ return a; }"),
            "function f(a, b) { return a; }"
        );
        assert_eq!(
            function_declaration_minify("f", &["a", "b"], "{return a;}"),
            "function f(a,b){return a;}"
        );
        assert_ne!(
            function_declaration_pretty("f", &["a"], "{}"),
            function_declaration_minify("f", &["a"], "{}")
        );
        assert_eq!(
            continue86_function_declaration_skeleton(
                true,
                true,
                Some("gen"),
                &["x"],
                "{ yield x; }",
                true
            ),
            function_declaration_skeleton(true, true, Some("gen"), &["x"], "{ yield x; }", true)
        );
        assert_eq!(
            continue86_function_expression_skeleton(
                false,
                false,
                None,
                &[],
                "{}",
                false
            ),
            function_declaration_skeleton(false, false, None, &[], "{}", false)
        );
        assert_eq!(
            async_generator_function_expression_pretty(Some("g"), &["a"], "{ yield a; }"),
            "async function* g(a) { yield a; }"
        );

        assert_eq!(class_declaration_pretty("Foo", &[]), "class Foo {}");
        assert_eq!(class_declaration_minify("Foo", &[]), "class Foo {}");
        assert_eq!(
            class_declaration_pretty("Bar", &["m() {}"]),
            class_declaration_skeleton(Some("Bar"), &["m() {}"], true)
        );
        assert_eq!(
            continue86_class_declaration_skeleton(Some("A"), &["x() {}"], false),
            class_declaration_skeleton(Some("A"), &["x() {}"], false)
        );
        assert_eq!(
            continue86_class_expression_skeleton(None, &[], true),
            class_declaration_skeleton(None, &[], true)
        );
        let named_pretty = class_declaration_pretty("C", &["m() {}", "n() {}"]);
        let named_mini = class_declaration_minify("C", &["m() {}", "n() {}"]);
        // method sep differs pretty vs minify when multiple methods
        assert_ne!(named_pretty, named_mini);
        assert_eq!(class_expression_anon_minify(&[]), "class {}");
    }

    #[test]
    fn continue86_this_super_compose_dual_oracle() {
        assert_eq!(this_expression(), "this");
        assert_eq!(super_expression(), "super");
        assert_eq!(continue86_this_expression_skeleton(), this_token());
        assert_eq!(continue86_super_skeleton(), super_token());
        assert_ne!(this_expression(), super_expression());

        let body_pretty = continue86_this_super_method_body(true);
        assert_eq!(body_pretty, "{ return this.super; }");
        let body_mini = continue86_this_super_method_body(false);
        assert_eq!(body_mini, "{return this.super;}");
        assert_ne!(body_pretty, body_mini);

        let cls_pretty = continue86_class_with_this_super("App", true);
        assert!(cls_pretty.starts_with("class App"));
        assert!(cls_pretty.contains("this"));
        assert!(cls_pretty.contains("super"));
        let cls_mini = continue86_class_with_this_super("App", false);
        assert!(cls_mini.contains("this"));
        assert_ne!(cls_pretty, cls_mini);

        let fn_pretty = continue86_function_returns_this("id", true);
        assert_eq!(fn_pretty, "function id() { return this; }");
        let fn_mini = continue86_function_returns_this("id", false);
        assert_eq!(fn_mini, "function id(){return this;}");
        assert_ne!(fn_pretty, fn_mini);

        let async_pretty = continue86_async_fn_calls_super(true);
        assert!(async_pretty.starts_with("async function"));
        assert!(async_pretty.contains("super"));
        let async_mini = continue86_async_fn_calls_super(false);
        assert!(async_mini.starts_with("async function"));
        assert_ne!(async_pretty, async_mini);
    }
}

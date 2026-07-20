//! Pure ObjectMethod + ConditionalExpression + ForOf(await) dual-oracle emission —
//! residual pure **continue74** for tooling/format-minify-lint fragment.
//!
//! New AST emit skeletons **not** covered by prior residual modules:
//! - ObjectMethod full dual-oracle (kind get/set/method, async, generator,
//!   computed key, real param list, pretty/minify seps) — widens fixed
//!   continue50 pretty-only `get name() { body }` shells
//! - ConditionalExpression full dual-oracle pretty/minify via shipped
//!   `ternary_pair` (prior continue27 only emits spaced pretty form)
//! - ForOfStatement with `await` flag dual-oracle driving real `for_open` /
//!   `for_of_token` / `for_close` (loop_switch for_of lacked await; continue27
//!   for-await was a fixed pretty string)
//!
//! Intentionally does **not** re-wrap continue64–73 partition shells.
//! Composes real shipped pure helpers (`method_kind_prefix`, `async_prefix`,
//! `generator_suffix`, `computed_property_key`, `function_param_sep`,
//! `ternary_pair`, `for_open`/`for_close`/`for_of_token`).
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//! pure residual ≠ authority flip; PreferRust OFF.

use crate::arrow_method_default_emit::{method_key_fragment, method_params_fragment};
use crate::conditional_spread_emit::ternary_pair;
use crate::format_emit::{async_prefix, generator_suffix, method_kind_prefix as format_method_kind};
use crate::loop_template_emit::{for_close, for_of_token, for_open};

/// Dual-oracle residual: continue74 related AST type catalog.
pub const CONTINUE74_RELATED_TYPES: &[&str] = &[
    "ObjectMethod",
    "ObjectProperty",
    "Property",
    "ConditionalExpression",
    "ForOfStatement",
    "ForInStatement",
];

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_object_method_conditional_related_type(t: &str) -> bool {
    CONTINUE74_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue74_object_method_type(t: &str) -> bool {
    t == "ObjectMethod"
}

#[must_use]
pub fn is_continue74_object_property_type(t: &str) -> bool {
    t == "ObjectProperty" || t == "Property"
}

#[must_use]
pub fn is_continue74_conditional_type(t: &str) -> bool {
    t == "ConditionalExpression"
}

#[must_use]
pub fn is_continue74_for_of_type(t: &str) -> bool {
    t == "ForOfStatement"
}

#[must_use]
pub fn is_continue74_for_in_type(t: &str) -> bool {
    t == "ForInStatement"
}

// ── ObjectMethod dual-oracle ────────────────────────────────────────────────

/// Dual-oracle ObjectMethod skeleton inside object literals.
///
/// Pretty method: `async *run(a, b) {body}`
/// Pretty getter: `get name() {body}`
/// Pretty setter: `set name(v) {body}`
/// Minify: denser param sep, no space before body brace.
/// Computed: `get [k]() {}` / `async [k]() {}`
///
/// Drives real shipped: [`format_method_kind`], [`async_prefix`],
/// [`generator_suffix`], [`method_key_fragment`], [`method_params_fragment`].
///
/// Note: ObjectMethod is never `static` (class MethodDefinition is continue72).
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn object_method_skeleton(
    kind: Option<&str>,
    key: &str,
    params: &[&str],
    body: Option<&str>,
    is_async: bool,
    is_generator: bool,
    computed: bool,
    pretty: bool,
) -> String {
    let kind_tok = match kind {
        Some(k) if !k.is_empty() && k != "method" && k != "init" => format_method_kind(k),
        _ => "",
    };
    let key_frag = method_key_fragment(key, computed);
    let params_frag = method_params_fragment(params, pretty);
    let mut out = String::with_capacity(
        16 + key_frag.len()
            + params_frag.len()
            + body.map_or(0, str::len)
            + kind_tok.len(),
    );
    // Babel/print order for object methods: async * get? key(params) body
    // get/set rarely combine with async/generator; still compose prefixes.
    out.push_str(async_prefix(is_async));
    out.push_str(kind_tok);
    out.push_str(generator_suffix(is_generator));
    out.push_str(&key_frag);
    out.push_str(&params_frag);
    if let Some(b) = body {
        out.push_str(b);
    }
    out
}

/// Convenience: pretty object method.
#[must_use]
pub fn object_method_pretty(key: &str, params: &[&str], body: &str) -> String {
    object_method_skeleton(None, key, params, Some(body), false, false, false, true)
}

/// Convenience: minify object method.
#[must_use]
pub fn object_method_minify(key: &str, params: &[&str], body: &str) -> String {
    object_method_skeleton(None, key, params, Some(body), false, false, false, false)
}

/// Convenience: pretty getter.
#[must_use]
pub fn object_getter_skeleton(key: &str, body: &str, pretty: bool) -> String {
    object_method_skeleton(Some("get"), key, &[], Some(body), false, false, false, pretty)
}

/// Convenience: pretty/minify setter.
#[must_use]
pub fn object_setter_skeleton(key: &str, param: &str, body: &str, pretty: bool) -> String {
    object_method_skeleton(
        Some("set"),
        key,
        &[param],
        Some(body),
        false,
        false,
        false,
        pretty,
    )
}

// ── ConditionalExpression dual-oracle ───────────────────────────────────────

/// Dual-oracle ConditionalExpression skeleton.
///
/// Pretty: `test ? consequent : alternate`
/// Minify: `test?consequent:alternate`
///
/// Drives real shipped: [`ternary_pair`].
/// Named continue74_* to avoid clashing with continue16 `conditional_expression_skeleton`.
#[must_use]
pub fn continue74_conditional_expression_skeleton(
    test: &str,
    consequent: &str,
    alternate: &str,
    pretty: bool,
) -> String {
    let (q, colon) = ternary_pair(pretty);
    let mut out = String::with_capacity(test.len() + consequent.len() + alternate.len() + 8);
    out.push_str(test);
    out.push_str(q);
    out.push_str(consequent);
    out.push_str(colon);
    out.push_str(alternate);
    out
}

/// Convenience: pretty ternary.
#[must_use]
pub fn conditional_expression_pretty(test: &str, consequent: &str, alternate: &str) -> String {
    continue74_conditional_expression_skeleton(test, consequent, alternate, true)
}

/// Convenience: minify ternary.
#[must_use]
pub fn conditional_expression_minify(test: &str, consequent: &str, alternate: &str) -> String {
    continue74_conditional_expression_skeleton(test, consequent, alternate, false)
}

// ── ForOfStatement (+ await) dual-oracle ────────────────────────────────────

/// Dual-oracle ForOfStatement skeleton with optional await.
///
/// Pretty: `for (left of right) body` / `for await (left of right) body`
/// Minify: `for(left of right)body` / `for await(left of right)body`
///
/// Drives real shipped: [`for_open`], [`for_of_token`], [`for_close`].
/// Await keyword sits between `for` and `(` (ES2018 for-await-of).
#[must_use]
pub fn for_of_await_skeleton(
    left: &str,
    right: &str,
    body: &str,
    awaited: bool,
    pretty: bool,
) -> String {
    let mut out = String::with_capacity(left.len() + right.len() + body.len() + 24);
    if awaited {
        // `for await (` pretty / `for await(` minify — not `forawait(`.
        out.push_str(if pretty { "for await (" } else { "for await(" });
    } else {
        out.push_str(for_open(pretty));
    }
    out.push_str(left);
    out.push_str(for_of_token());
    out.push_str(right);
    // for_close pretty = ") " ; minify = ")"
    let close = for_close(pretty);
    if pretty {
        out.push_str(close.trim_end());
        out.push(' ');
        out.push_str(body);
    } else {
        out.push_str(close);
        out.push_str(body);
    }
    out
}

/// Convenience: non-await for-of pretty.
#[must_use]
pub fn for_of_pretty(left: &str, right: &str, body: &str) -> String {
    for_of_await_skeleton(left, right, body, false, true)
}

/// Convenience: await for-of minify.
#[must_use]
pub fn for_await_of_minify(left: &str, right: &str, body: &str) -> String {
    for_of_await_skeleton(left, right, body, true, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conditional_spread_emit::{ternary_colon, ternary_pair, ternary_q};
    use crate::format_emit::{async_prefix, generator_suffix, method_kind_prefix};
    use crate::loop_template_emit::{for_close, for_of_token, for_open};

    #[test]
    fn continue74_type_catalog() {
        assert_eq!(CONTINUE74_RELATED_TYPES.len(), 6);
        assert!(is_object_method_conditional_related_type("ObjectMethod"));
        assert!(is_object_method_conditional_related_type("ObjectProperty"));
        assert!(is_object_method_conditional_related_type("Property"));
        assert!(is_object_method_conditional_related_type("ConditionalExpression"));
        assert!(is_object_method_conditional_related_type("ForOfStatement"));
        assert!(is_object_method_conditional_related_type("ForInStatement"));
        assert!(!is_object_method_conditional_related_type("ArrowFunctionExpression"));
        assert!(!is_object_method_conditional_related_type("OptionalMemberExpression"));
        assert!(is_continue74_object_method_type("ObjectMethod"));
        assert!(is_continue74_object_property_type("Property"));
        assert!(is_continue74_object_property_type("ObjectProperty"));
        assert!(is_continue74_conditional_type("ConditionalExpression"));
        assert!(is_continue74_for_of_type("ForOfStatement"));
        assert!(is_continue74_for_in_type("ForInStatement"));
        assert!(!is_continue74_object_method_type("MethodDefinition"));
        assert!(!is_continue74_conditional_type("LogicalExpression"));
    }

    #[test]
    fn continue74_object_method_dual_oracle_drives_shipped() {
        assert_eq!(method_kind_prefix("get"), "get ");
        assert_eq!(method_kind_prefix("set"), "set ");
        assert_eq!(async_prefix(true), "async ");
        assert_eq!(async_prefix(false), "");
        assert_eq!(generator_suffix(true), "*");
        assert_eq!(generator_suffix(false), "");

        // Pretty plain method
        assert_eq!(
            object_method_pretty("run", &["a", "b"], "{}"),
            "run(a, b) {}"
        );
        // Minify param sep
        assert_eq!(
            object_method_minify("run", &["a", "b"], "{}"),
            "run(a,b){}"
        );
        // Getter / setter
        assert_eq!(
            object_getter_skeleton("name", "{return this._n;}", true),
            "get name() {return this._n;}"
        );
        assert_eq!(
            object_getter_skeleton("name", "{return this._n;}", false),
            "get name(){return this._n;}"
        );
        assert_eq!(
            object_setter_skeleton("name", "v", "{}", true),
            "set name(v) {}"
        );
        assert_eq!(
            object_setter_skeleton("name", "v", "{}", false),
            "set name(v){}"
        );
        // async + generator + computed
        assert_eq!(
            object_method_skeleton(
                None,
                "k",
                &["x"],
                Some("{}"),
                true,
                true,
                true,
                true
            ),
            "async *[k](x) {}"
        );
        assert_eq!(
            object_method_skeleton(
                Some("get"),
                "id",
                &[],
                Some("{return 1;}"),
                false,
                false,
                true,
                false
            ),
            "get [id](){return 1;}"
        );
        // kind "init" / "method" treated as bare method
        assert_eq!(
            object_method_skeleton(Some("init"), "x", &[], Some("{}"), false, false, false, true),
            "x() {}"
        );
        assert_eq!(
            object_method_skeleton(
                Some("method"),
                "x",
                &[],
                Some("{}"),
                false,
                false,
                false,
                false
            ),
            "x(){}"
        );
    }

    #[test]
    fn continue74_conditional_dual_oracle() {
        assert_eq!(ternary_q(true), " ? ");
        assert_eq!(ternary_q(false), "?");
        assert_eq!(ternary_colon(true), " : ");
        assert_eq!(ternary_colon(false), ":");
        assert_eq!(ternary_pair(true), (" ? ", " : "));
        assert_eq!(ternary_pair(false), ("?", ":"));

        assert_eq!(
            conditional_expression_pretty("a", "b", "c"),
            "a ? b : c"
        );
        assert_eq!(
            conditional_expression_minify("a", "b", "c"),
            "a?b:c"
        );
        assert_eq!(
            continue74_conditional_expression_skeleton("x > 0", "1", "0", true),
            "x > 0 ? 1 : 0"
        );
        assert_eq!(
            continue74_conditional_expression_skeleton("x>0", "1", "0", false),
            "x>0?1:0"
        );
        // nested alternate residual
        assert_eq!(
            conditional_expression_pretty("t", "a", "b ? c : d"),
            "t ? a : b ? c : d"
        );
    }

    #[test]
    fn continue74_for_of_await_dual_oracle() {
        assert_eq!(for_open(true), "for (");
        assert_eq!(for_open(false), "for(");
        assert_eq!(for_close(true), ") ");
        assert_eq!(for_close(false), ")");
        assert_eq!(for_of_token(), " of ");

        // Non-await
        assert_eq!(for_of_pretty("x", "arr", "{}"), "for (x of arr) {}");
        assert_eq!(
            for_of_await_skeleton("x", "arr", "body()", false, false),
            "for(x of arr)body()"
        );
        // Await pretty / minify — uses real spacing, not forawait concat
        assert_eq!(
            for_of_await_skeleton("x", "asyncIter", "{}", true, true),
            "for await (x of asyncIter) {}"
        );
        assert_eq!(
            for_await_of_minify("x", "it", "body()"),
            "for await(x of it)body()"
        );
        assert_eq!(
            for_of_await_skeleton("const [a]", "xs", "{use(a);}", true, true),
            "for await (const [a] of xs) {use(a);}"
        );
    }
}

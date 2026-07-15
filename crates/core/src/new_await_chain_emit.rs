//! Pure New / Await / Chain / Yield / ImportExpression emission —
//! residual pure continue16 for tooling/format-minify-lint fragment.
//! Mirrors printer/compressor skeletons for expression forms not covered by
//! prior continue modules. Full engines remain product dens.
//! NO authority_rust / ts_deleted. intentional ts_only×3 retained.

/// Whether node type is NewExpression.
#[must_use]
pub fn is_new_expression_type(t: &str) -> bool {
    t == "NewExpression"
}

/// Whether node type is AwaitExpression.
#[must_use]
pub fn is_await_expression_type(t: &str) -> bool {
    t == "AwaitExpression"
}

/// Whether node type is YieldExpression.
#[must_use]
pub fn is_yield_expression_type(t: &str) -> bool {
    t == "YieldExpression"
}

/// Whether node type is ChainExpression (optional-chain wrapper).
#[must_use]
pub fn is_chain_expression_type(t: &str) -> bool {
    t == "ChainExpression"
}

/// Whether node type is ImportExpression (`import()` dynamic).
#[must_use]
pub fn is_import_expression_type(t: &str) -> bool {
    t == "ImportExpression"
}

/// Whether node type is TaggedTemplateExpression.
#[must_use]
pub fn is_tagged_template_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

/// Whether node type is ConditionalExpression.
#[must_use]
pub fn is_conditional_expression_type(t: &str) -> bool {
    t == "ConditionalExpression"
}

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_new_await_chain_related_type(t: &str) -> bool {
    matches!(
        t,
        "NewExpression"
            | "AwaitExpression"
            | "YieldExpression"
            | "ChainExpression"
            | "ImportExpression"
            | "TaggedTemplateExpression"
            | "ConditionalExpression"
    )
}

/// `new` keyword with spacing (pretty → `new `; minify → `new`).
/// Callee is expected to follow; pretty always leaves a space after `new`.
#[must_use]
pub fn new_keyword(pretty: bool) -> &'static str {
    if pretty {
        "new "
    } else {
        "new"
    }
}

/// NewExpression skeleton: `new Callee(args)` with pretty/minify arg join.
/// Minify glues `new` to callee when callee starts with identifier/char;
/// pretty keeps `new `.
#[must_use]
pub fn new_expression_skeleton(callee: &str, args: &[&str], pretty: bool) -> String {
    let args_joined = if pretty {
        args.join(", ")
    } else {
        args.join(",")
    };
    if pretty {
        format!("new {callee}({args_joined})")
    } else {
        format!("new{callee}({args_joined})")
    }
}

/// AwaitExpression skeleton: pretty `await expr`; minify `await expr`
/// (TS compressor keeps a space after `await` keyword).
#[must_use]
pub fn await_expression_skeleton(argument: &str, _pretty: bool) -> String {
    format!("await {argument}")
}

/// YieldExpression skeleton.
/// - `delegate=true` → `yield* expr` (pretty space around `*`; minify `yield*expr`)
/// - bare yield without arg → `yield`
#[must_use]
pub fn yield_expression_skeleton(
    argument: Option<&str>,
    delegate: bool,
    pretty: bool,
) -> String {
    match (argument, delegate) {
        (None, _) => "yield".to_string(),
        (Some(arg), true) if pretty => format!("yield* {arg}"),
        (Some(arg), true) => format!("yield*{arg}"),
        (Some(arg), false) => format!("yield {arg}"),
    }
}

/// Dynamic `import(source)` skeleton (pretty/minify identical — engines omit
/// inner spaces around the source).
#[must_use]
pub fn import_expression_skeleton(source: &str, _pretty: bool) -> String {
    format!("import({source})")
}

/// ChainExpression is a transparent wrapper in print — emit inner as-is.
#[must_use]
pub fn chain_expression_skeleton(inner: &str) -> String {
    inner.to_string()
}

/// Tagged template skeleton: `tag\`quasi\`` (caller supplies quasi body with
/// backticks already). Pretty/minify identical for tag adjacency.
#[must_use]
pub fn tagged_template_skeleton(tag: &str, quasi_with_backticks: &str) -> String {
    format!("{tag}{quasi_with_backticks}")
}

/// ConditionalExpression skeleton: pretty `a ? b : c`; minify `a?b:c`.
#[must_use]
pub fn conditional_expression_skeleton(
    test: &str,
    consequent: &str,
    alternate: &str,
    pretty: bool,
) -> String {
    if pretty {
        format!("{test} ? {consequent} : {alternate}")
    } else {
        format!("{test}?{consequent}:{alternate}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_predicates() {
        assert!(is_new_expression_type("NewExpression"));
        assert!(is_await_expression_type("AwaitExpression"));
        assert!(is_yield_expression_type("YieldExpression"));
        assert!(is_chain_expression_type("ChainExpression"));
        assert!(is_import_expression_type("ImportExpression"));
        assert!(is_tagged_template_type("TaggedTemplateExpression"));
        assert!(is_conditional_expression_type("ConditionalExpression"));
        assert!(is_new_await_chain_related_type("NewExpression"));
        assert!(is_new_await_chain_related_type("AwaitExpression"));
        assert!(!is_new_await_chain_related_type("Identifier"));
    }

    #[test]
    fn new_await_import() {
        assert_eq!(new_keyword(true), "new ");
        assert_eq!(new_keyword(false), "new");
        assert_eq!(
            new_expression_skeleton("Foo", &["a", "b"], true),
            "new Foo(a, b)"
        );
        assert_eq!(
            new_expression_skeleton("Foo", &["a", "b"], false),
            "newFoo(a,b)"
        );
        assert_eq!(await_expression_skeleton("x", true), "await x");
        assert_eq!(await_expression_skeleton("x", false), "await x");
        assert_eq!(import_expression_skeleton("'./m'", true), "import('./m')");
        assert_eq!(import_expression_skeleton("'./m'", false), "import('./m')");
    }

    #[test]
    fn yield_chain_conditional() {
        assert_eq!(yield_expression_skeleton(None, false, true), "yield");
        assert_eq!(
            yield_expression_skeleton(Some("v"), false, true),
            "yield v"
        );
        assert_eq!(
            yield_expression_skeleton(Some("v"), true, true),
            "yield* v"
        );
        assert_eq!(
            yield_expression_skeleton(Some("v"), true, false),
            "yield*v"
        );
        assert_eq!(chain_expression_skeleton("a?.b"), "a?.b");
        assert_eq!(
            tagged_template_skeleton("html", "`<div>`"),
            "html`<div>`"
        );
        assert_eq!(
            conditional_expression_skeleton("a", "b", "c", true),
            "a ? b : c"
        );
        assert_eq!(
            conditional_expression_skeleton("a", "b", "c", false),
            "a?b:c"
        );
    }
}

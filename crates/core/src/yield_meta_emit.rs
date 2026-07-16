//! Pure residual continue20: YieldExpression / MetaProperty / ImportExpression emit.
//!
//! Dual-oracle under tooling/format-minify-lint. Intentional ts_only plugins retained.
//! NO authority_rust invent / ts_deleted.

#![allow(dead_code)]
/// Type guards for continue20 related AST node types.
#[must_use]
pub fn is_yield_meta_related_type(t: &str) -> bool {
    matches!(
        t,
        "YieldExpression" | "MetaProperty" | "ImportExpression" | "AwaitExpression"
    )
}

#[must_use]
pub fn is_continue20_yield_expression_type(t: &str) -> bool {
    t == "YieldExpression"
}

#[must_use]
pub fn is_continue20_meta_property_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue20_import_expression_type(t: &str) -> bool {
    t == "ImportExpression"
}

/// `yield` / `yield*` skeleton dual-oracle.
#[must_use]
pub fn continue20_yield_expression_skeleton(
    argument: Option<&str>,
    delegate: bool,
    pretty: bool,
) -> String {
    match (argument, delegate, pretty) {
        (None, _, _) => "yield".to_string(),
        (Some(arg), true, true) => format!("yield* {arg}"),
        (Some(arg), true, false) => format!("yield*{arg}"),
        (Some(arg), false, true) => format!("yield {arg}"),
        (Some(arg), false, false) => format!("yield{arg}"),
    }
}

/// `import.meta` / `new.target` meta property dual-oracle.
#[must_use]
pub fn continue20_meta_property_skeleton(meta: &str, property: &str) -> String {
    format!("{meta}.{property}")
}

/// Dynamic `import(source)` skeleton dual-oracle.
#[must_use]
pub fn continue20_import_expression_skeleton(source: &str, _pretty: bool) -> String {
    format!("import({source})")
}

/// Await expression skeleton dual-oracle residual (paired with yield/meta wave).
#[must_use]
pub fn continue20_await_expression_skeleton(argument: &str, pretty: bool) -> String {
    if pretty {
        format!("await {argument}")
    } else {
        format!("await{argument}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue20_yield_meta_import_emit() {
        assert!(is_yield_meta_related_type("YieldExpression"));
        assert!(is_yield_meta_related_type("MetaProperty"));
        assert!(is_yield_meta_related_type("ImportExpression"));
        assert!(is_continue20_yield_expression_type("YieldExpression"));
        assert!(is_continue20_meta_property_type("MetaProperty"));
        assert!(is_continue20_import_expression_type("ImportExpression"));
        assert!(!is_yield_meta_related_type("ForStatement"));

        assert_eq!(continue20_yield_expression_skeleton(None, false, true), "yield");
        assert_eq!(
            continue20_yield_expression_skeleton(Some("x"), false, true),
            "yield x"
        );
        assert_eq!(
            continue20_yield_expression_skeleton(Some("xs"), true, true),
            "yield* xs"
        );
        assert_eq!(
            continue20_yield_expression_skeleton(Some("xs"), true, false),
            "yield*xs"
        );

        assert_eq!(continue20_meta_property_skeleton("import", "meta"), "import.meta");
        assert_eq!(continue20_meta_property_skeleton("new", "target"), "new.target");
        assert_eq!(
            continue20_import_expression_skeleton("'./m.js'", true),
            "import('./m.js')"
        );
        assert_eq!(continue20_await_expression_skeleton("p", true), "await p");
        assert_eq!(continue20_await_expression_skeleton("p", false), "awaitp");
    }
}

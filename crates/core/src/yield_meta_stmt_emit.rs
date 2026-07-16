//! Pure YieldExpression + MetaProperty + ImportExpression + AwaitExpression
//! dual-oracle emission — residual pure **continue83** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules:
//! - YieldExpression dual-oracle bare/arg/delegate pretty/minify composing real
//!   `continue20_yield_expression_skeleton` (continue20 base)
//! - MetaProperty dual-oracle `import.meta` / `new.target`
//! - ImportExpression dual-oracle source-only (options form stays continue75)
//! - AwaitExpression dual-oracle pretty/minify spacing
//! - Composed yield-then-await / meta+yield residual shells
//!
//! Intentionally does **not** re-wrap continue64–82 partition shells
//! (property/static continue82 stays separate; try/throw/import continue81 stays
//! separate; template/import-options continue75 stays separate). Composes real
//! shipped pure helpers from `yield_meta_emit`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::yield_meta_emit::{
    continue20_await_expression_skeleton, continue20_import_expression_skeleton,
    continue20_meta_property_skeleton, continue20_yield_expression_skeleton,
};

/// Dual-oracle residual: continue83 related AST type catalog.
pub const CONTINUE83_RELATED_TYPES: &[&str] = &[
    "YieldExpression",
    "MetaProperty",
    "ImportExpression",
    "AwaitExpression",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_yield_meta_stmt_related_type(t: &str) -> bool {
    CONTINUE83_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue83_yield_type(t: &str) -> bool {
    t == "YieldExpression"
}

#[must_use]
pub fn is_continue83_meta_property_type(t: &str) -> bool {
    t == "MetaProperty"
}

#[must_use]
pub fn is_continue83_import_expression_type(t: &str) -> bool {
    t == "ImportExpression"
}

#[must_use]
pub fn is_continue83_await_type(t: &str) -> bool {
    t == "AwaitExpression"
}

// ── YieldExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle YieldExpression skeleton composing real
/// [`continue20_yield_expression_skeleton`].
#[must_use]
pub fn continue83_yield_expression_skeleton(
    argument: Option<&str>,
    delegate: bool,
    pretty: bool,
) -> String {
    continue20_yield_expression_skeleton(argument, delegate, pretty)
}

/// Convenience: bare `yield`.
#[must_use]
pub fn yield_bare() -> String {
    continue83_yield_expression_skeleton(None, false, true)
}

/// Convenience: pretty yield with argument.
#[must_use]
pub fn yield_arg_pretty(arg: &str) -> String {
    continue83_yield_expression_skeleton(Some(arg), false, true)
}

/// Convenience: minify yield with argument.
#[must_use]
pub fn yield_arg_minify(arg: &str) -> String {
    continue83_yield_expression_skeleton(Some(arg), false, false)
}

/// Convenience: pretty yield* delegate.
#[must_use]
pub fn yield_delegate_pretty(arg: &str) -> String {
    continue83_yield_expression_skeleton(Some(arg), true, true)
}

/// Convenience: minify yield* delegate.
#[must_use]
pub fn yield_delegate_minify(arg: &str) -> String {
    continue83_yield_expression_skeleton(Some(arg), true, false)
}

// ── MetaProperty dual-oracle ────────────────────────────────────────────────

/// Dual-oracle MetaProperty skeleton composing real
/// [`continue20_meta_property_skeleton`].
#[must_use]
pub fn continue83_meta_property_skeleton(meta: &str, property: &str) -> String {
    continue20_meta_property_skeleton(meta, property)
}

/// Convenience: `import.meta`.
#[must_use]
pub fn import_meta() -> String {
    continue83_meta_property_skeleton("import", "meta")
}

/// Convenience: `new.target`.
#[must_use]
pub fn new_target() -> String {
    continue83_meta_property_skeleton("new", "target")
}

// ── ImportExpression dual-oracle ────────────────────────────────────────────

/// Dual-oracle ImportExpression skeleton composing real
/// [`continue20_import_expression_skeleton`] (source-only; options → continue75).
#[must_use]
pub fn continue83_import_expression_skeleton(source: &str, pretty: bool) -> String {
    continue20_import_expression_skeleton(source, pretty)
}

/// Convenience: pretty dynamic import.
#[must_use]
pub fn import_expression_pretty(source: &str) -> String {
    continue83_import_expression_skeleton(source, true)
}

/// Convenience: minify dynamic import (same source form; pretty flag retained).
#[must_use]
pub fn import_expression_minify(source: &str) -> String {
    continue83_import_expression_skeleton(source, false)
}

// ── AwaitExpression dual-oracle ─────────────────────────────────────────────

/// Dual-oracle AwaitExpression skeleton composing real
/// [`continue20_await_expression_skeleton`].
#[must_use]
pub fn continue83_await_expression_skeleton(argument: &str, pretty: bool) -> String {
    continue20_await_expression_skeleton(argument, pretty)
}

/// Convenience: pretty await.
#[must_use]
pub fn await_pretty(argument: &str) -> String {
    continue83_await_expression_skeleton(argument, true)
}

/// Convenience: minify await.
#[must_use]
pub fn await_minify(argument: &str) -> String {
    continue83_await_expression_skeleton(argument, false)
}

// ── Composed dual-oracle shells ─────────────────────────────────────────────

/// Dual-oracle residual: `await (yield arg)` pretty composition.
#[must_use]
pub fn continue83_await_yield(arg: &str, pretty: bool) -> String {
    let y = continue83_yield_expression_skeleton(Some(arg), false, pretty);
    continue83_await_expression_skeleton(&y, pretty)
}

/// Dual-oracle residual: meta property then yield of property access shell.
#[must_use]
pub fn continue83_meta_then_yield(meta: &str, property: &str, pretty: bool) -> String {
    let m = continue83_meta_property_skeleton(meta, property);
    continue83_yield_expression_skeleton(Some(&m), false, pretty)
}

/// Dual-oracle residual: dynamic import of meta-relative source shell.
#[must_use]
pub fn continue83_import_meta_url_like(pretty: bool) -> String {
    let meta = import_meta();
    let source = format!("{meta}.url");
    continue83_import_expression_skeleton(&format!("'{source}'"), pretty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yield_meta_emit::{
        continue20_await_expression_skeleton, continue20_import_expression_skeleton,
        continue20_meta_property_skeleton, continue20_yield_expression_skeleton,
    };

    #[test]
    fn continue83_type_catalog() {
        assert_eq!(CONTINUE83_RELATED_TYPES.len(), 4);
        assert!(is_yield_meta_stmt_related_type("YieldExpression"));
        assert!(is_yield_meta_stmt_related_type("MetaProperty"));
        assert!(is_yield_meta_stmt_related_type("ImportExpression"));
        assert!(is_yield_meta_stmt_related_type("AwaitExpression"));
        assert!(!is_yield_meta_stmt_related_type("ForStatement"));
        assert!(!is_yield_meta_stmt_related_type("PropertyDefinition"));
        assert!(!is_yield_meta_stmt_related_type("TryStatement"));
        assert!(is_continue83_yield_type("YieldExpression"));
        assert!(is_continue83_meta_property_type("MetaProperty"));
        assert!(is_continue83_import_expression_type("ImportExpression"));
        assert!(is_continue83_await_type("AwaitExpression"));
        assert!(!is_continue83_yield_type("AwaitExpression"));
        assert!(!is_continue83_import_expression_type("ImportDeclaration"));
    }

    #[test]
    fn continue83_yield_meta_dual_oracle() {
        assert_eq!(yield_bare(), "yield");
        assert_eq!(yield_arg_pretty("x"), "yield x");
        assert_eq!(yield_arg_minify("x"), "yieldx");
        assert_eq!(yield_delegate_pretty("xs"), "yield* xs");
        assert_eq!(yield_delegate_minify("xs"), "yield*xs");
        assert_eq!(
            continue83_yield_expression_skeleton(Some("v"), true, true),
            continue20_yield_expression_skeleton(Some("v"), true, true)
        );
        assert_ne!(yield_arg_pretty("a"), yield_arg_minify("a"));
        assert_ne!(yield_delegate_pretty("a"), yield_delegate_minify("a"));

        assert_eq!(import_meta(), "import.meta");
        assert_eq!(new_target(), "new.target");
        assert_eq!(
            continue83_meta_property_skeleton("import", "meta"),
            continue20_meta_property_skeleton("import", "meta")
        );
        assert_eq!(
            continue83_meta_property_skeleton("new", "target"),
            "new.target"
        );
    }

    #[test]
    fn continue83_await_import_compose_dual_oracle() {
        assert_eq!(await_pretty("p"), "await p");
        assert_eq!(await_minify("p"), "awaitp");
        assert_eq!(
            continue83_await_expression_skeleton("q", true),
            continue20_await_expression_skeleton("q", true)
        );
        assert_ne!(await_pretty("x"), await_minify("x"));

        assert_eq!(
            import_expression_pretty("'./m.js'"),
            "import('./m.js')"
        );
        assert_eq!(
            import_expression_minify("'./m.js'"),
            "import('./m.js')"
        );
        assert_eq!(
            continue83_import_expression_skeleton("'./x'", true),
            continue20_import_expression_skeleton("'./x'", true)
        );

        let ay_pretty = continue83_await_yield("v", true);
        assert_eq!(ay_pretty, "await yield v");
        let ay_mini = continue83_await_yield("v", false);
        assert_eq!(ay_mini, "awaityieldv");
        assert_ne!(ay_pretty, ay_mini);

        let meta_y = continue83_meta_then_yield("import", "meta", true);
        assert_eq!(meta_y, "yield import.meta");
        let meta_y_mini = continue83_meta_then_yield("import", "meta", false);
        assert_eq!(meta_y_mini, "yieldimport.meta");

        let imp = continue83_import_meta_url_like(true);
        assert!(imp.starts_with("import("));
        assert!(imp.contains("import.meta.url"));
    }
}

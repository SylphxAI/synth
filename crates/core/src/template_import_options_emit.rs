//! Pure TemplateLiteral + TaggedTemplateExpression + ImportExpression(options)
//! dual-oracle emission — residual pure **continue75** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules:
//! - TemplateLiteral full dual-oracle (quasis + `${expr}` via shipped
//!   `template_tick` / `template_expr_open` / `template_expr_close`) —
//!   widens continue7 tick-only + continue23/25 fixed shells
//! - TaggedTemplateExpression dual-oracle driving real template body assembly
//!   (prior continue19 only concatenates pre-rendered quasi strings)
//! - ImportExpression with optional second-arg options dual-oracle
//!   `import(s)` / `import(s, opts)` pretty/minify seps (prior only `import(s)`)
//!
//! Intentionally does **not** re-wrap continue64–74 partition shells.
//! Composes real shipped pure helpers from `loop_template_emit`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::loop_template_emit::{template_expr_close, template_expr_open, template_tick};

/// Dual-oracle residual: continue75 related AST type catalog.
pub const CONTINUE75_RELATED_TYPES: &[&str] = &[
    "TemplateLiteral",
    "TemplateElement",
    "TaggedTemplateExpression",
    "ImportExpression",
    "Import",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_template_import_options_related_type(t: &str) -> bool {
    CONTINUE75_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue75_template_literal_type(t: &str) -> bool {
    t == "TemplateLiteral"
}

#[must_use]
pub fn is_continue75_template_element_type(t: &str) -> bool {
    t == "TemplateElement"
}

#[must_use]
pub fn is_continue75_tagged_template_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

#[must_use]
pub fn is_continue75_import_expression_type(t: &str) -> bool {
    t == "ImportExpression" || t == "Import"
}

// ── TemplateLiteral dual-oracle ─────────────────────────────────────────────

/// Dual-oracle TemplateElement cooked quasi fragment (no ticks).
#[must_use]
pub fn template_element_cooked(cooked: &str) -> String {
    cooked.to_owned()
}

/// Dual-oracle expression interpolation: `${expr}` / minify identical delimiters.
/// Drives real shipped [`template_expr_open`] / [`template_expr_close`].
#[must_use]
pub fn template_interpolation(expr: &str) -> String {
    format!(
        "{}{}{}",
        template_expr_open(),
        expr,
        template_expr_close()
    )
}

/// Dual-oracle TemplateLiteral skeleton from interleaved quasis and expressions.
///
/// Invariant (ESTree): `quasis.len() == expressions.len() + 1`.
/// Pretty and minify share `${}` delimiters; only caller-supplied expr/quasi
/// spacing differs (engines do not insert spaces inside template bodies).
///
/// Example: quasis=`["a", "b", "c"]`, exprs=`["x", "y"]` → `` `a${x}b${y}c` ``
#[must_use]
pub fn continue75_template_literal_skeleton(quasis: &[&str], expressions: &[&str]) -> String {
    let mut out = String::from(template_tick());
    let n_expr = expressions.len();
    for (i, q) in quasis.iter().enumerate() {
        out.push_str(q);
        if i < n_expr {
            out.push_str(&template_interpolation(expressions[i]));
        }
    }
    // If caller under-supplied quasis, still close the tick (fail-soft residual).
    if quasis.is_empty() {
        for e in expressions {
            out.push_str(&template_interpolation(e));
        }
    }
    out.push_str(template_tick());
    out
}

/// Convenience: empty template `` ` ` `` (zero-length quasi).
#[must_use]
pub fn template_literal_empty() -> String {
    continue75_template_literal_skeleton(&[""], &[])
}

/// Convenience: no-expression cooked template `` `hello` ``.
#[must_use]
pub fn template_literal_cooked(cooked: &str) -> String {
    continue75_template_literal_skeleton(&[cooked], &[])
}

/// Convenience: single interpolation `` `pre${expr}post` ``.
#[must_use]
pub fn template_literal_single(pre: &str, expr: &str, post: &str) -> String {
    continue75_template_literal_skeleton(&[pre, post], &[expr])
}

// ── TaggedTemplateExpression dual-oracle ────────────────────────────────────

/// Dual-oracle TaggedTemplateExpression: `tag` + TemplateLiteral body.
///
/// Pretty/minify: no space between tag and template (TS printer residual).
/// Drives real [`template_literal_skeleton`] so tagged form cannot drift from
/// bare TemplateLiteral dens.
#[must_use]
pub fn continue75_tagged_template_skeleton(tag: &str, quasis: &[&str], expressions: &[&str]) -> String {
    format!(
        "{}{}",
        tag,
        continue75_template_literal_skeleton(quasis, expressions)
    )
}

/// Convenience: simple tagged cooked `` tag`body` ``.
#[must_use]
pub fn tagged_template_cooked(tag: &str, cooked: &str) -> String {
    continue75_tagged_template_skeleton(tag, &[cooked], &[])
}

/// Convenience: tagged single interpolation.
#[must_use]
pub fn tagged_template_single(tag: &str, pre: &str, expr: &str, post: &str) -> String {
    continue75_tagged_template_skeleton(tag, &[pre, post], &[expr])
}

// ── ImportExpression dual-oracle (optional options arg) ─────────────────────

/// Dual-oracle ImportExpression with optional second-arg options object.
///
/// - no options: `import(source)`
/// - with options pretty: `import(source, options)`
/// - with options minify: `import(source,options)` (no space after comma)
///
/// Source and options are pre-rendered fragments (string literal / object).
#[must_use]
pub fn continue75_import_expression_options_skeleton(
    source: &str,
    options: Option<&str>,
    pretty: bool,
) -> String {
    match options {
        Some(opts) if !opts.is_empty() => {
            let sep = if pretty { ", " } else { "," };
            format!("import({source}{sep}{opts})")
        }
        _ => format!("import({source})"),
    }
}

/// Convenience: bare dynamic import (pretty/minify identical).
#[must_use]
pub fn import_expression_bare(source: &str) -> String {
    continue75_import_expression_options_skeleton(source, None, true)
}

/// Convenience: import with options pretty form.
#[must_use]
pub fn import_expression_with_options_pretty(source: &str, options: &str) -> String {
    continue75_import_expression_options_skeleton(source, Some(options), true)
}

/// Convenience: import with options minify form.
#[must_use]
pub fn import_expression_with_options_minify(source: &str, options: &str) -> String {
    continue75_import_expression_options_skeleton(source, Some(options), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loop_template_emit::{template_expr_close, template_expr_open, template_tick};

    #[test]
    fn continue75_type_catalog() {
        assert_eq!(CONTINUE75_RELATED_TYPES.len(), 5);
        assert!(is_template_import_options_related_type("TemplateLiteral"));
        assert!(is_template_import_options_related_type("TemplateElement"));
        assert!(is_template_import_options_related_type("TaggedTemplateExpression"));
        assert!(is_template_import_options_related_type("ImportExpression"));
        assert!(is_template_import_options_related_type("Import"));
        assert!(!is_template_import_options_related_type("ObjectMethod"));
        assert!(!is_template_import_options_related_type("ConditionalExpression"));
        assert!(is_continue75_template_literal_type("TemplateLiteral"));
        assert!(is_continue75_template_element_type("TemplateElement"));
        assert!(is_continue75_tagged_template_type("TaggedTemplateExpression"));
        assert!(is_continue75_import_expression_type("ImportExpression"));
        assert!(is_continue75_import_expression_type("Import"));
        assert!(!is_continue75_template_literal_type("TemplateElement"));
        assert!(!is_continue75_import_expression_type("ImportDeclaration"));
    }

    #[test]
    fn continue75_template_literal_dual_oracle_drives_shipped() {
        assert_eq!(template_tick(), "`");
        assert_eq!(template_expr_open(), "${");
        assert_eq!(template_expr_close(), "}");
        assert_eq!(template_interpolation("x"), "${x}");
        assert_eq!(template_element_cooked("hi"), "hi");

        assert_eq!(template_literal_empty(), "``");
        assert_eq!(template_literal_cooked("hello"), "`hello`");
        assert_eq!(template_literal_single("a", "x", "b"), "`a${x}b`");
        assert_eq!(
            continue75_template_literal_skeleton(&["pre-", "-mid-", "-post"], &["a", "b"]),
            "`pre-${a}-mid-${b}-post`"
        );
        // only quasis
        assert_eq!(continue75_template_literal_skeleton(&["only"], &[]), "`only`");
        // multi-char expr residual
        assert_eq!(
            template_literal_single("", "foo.bar()", ""),
            "`${foo.bar()}`"
        );
    }

    #[test]
    fn continue75_tagged_template_dual_oracle() {
        assert_eq!(tagged_template_cooked("css", ".x{}"), "css`.x{}`");
        assert_eq!(
            tagged_template_single("html", "<p>", "name", "</p>"),
            "html`<p>${name}</p>`"
        );
        assert_eq!(
            continue75_tagged_template_skeleton("tag", &["a", "b"], &["e"]),
            "tag`a${e}b`"
        );
        // no space between tag and tick (printer residual)
        assert_eq!(tagged_template_cooked("f", ""), "f``");
        assert_eq!(
            continue75_tagged_template_skeleton("String.raw", &["\\n", ""], &["x"]),
            "String.raw`\\n${x}`"
        );
    }

    #[test]
    fn continue75_import_expression_options_dual_oracle() {
        assert_eq!(import_expression_bare("'./m.js'"), "import('./m.js')");
        assert_eq!(
            continue75_import_expression_options_skeleton("'./m.js'", None, false),
            "import('./m.js')"
        );
        assert_eq!(
            import_expression_with_options_pretty("'./m.js'", "{ with: { type: 'json' } }"),
            "import('./m.js', { with: { type: 'json' } })"
        );
        assert_eq!(
            import_expression_with_options_minify("'./m.js'", "{with:{type:'json'}}"),
            "import('./m.js',{with:{type:'json'}})"
        );
        // empty options treated as bare
        assert_eq!(
            continue75_import_expression_options_skeleton("s", Some(""), true),
            "import(s)"
        );
        // assert options sep dual-oracle
        assert_eq!(
            continue75_import_expression_options_skeleton("s", Some("o"), true),
            "import(s, o)"
        );
        assert_eq!(
            continue75_import_expression_options_skeleton("s", Some("o"), false),
            "import(s,o)"
        );
    }
}

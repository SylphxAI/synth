//! Pure Identifier / Literal dual-oracle emission helpers —
//! residual pure continue14 for tooling/format-minify-lint fragment.
//! Mirrors `printIdentifier` / `compressIdentifier` and
//! `printLiteral` / `compressLiteral` in
//! `packages/synth-js-format` / `synth-js-minify`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//!
//! Pretty path: Identifier name as-is; string Literal honors singleQuote.
//! Minify path: Identifier uses precomputed mangled name when mangle is on;
//! string Literal always double-quoted (TS compressor).
#![allow(dead_code)]

use crate::format_indent::quote_string_literal;

/// Whether node type is Identifier.
#[must_use]
pub fn is_identifier_node_type(t: &str) -> bool {
    t == "Identifier"
}

/// Whether node type is Literal (ESTree plain Literal, not TemplateLiteral).
#[must_use]
pub fn is_literal_node_type(t: &str) -> bool {
    t == "Literal"
}

/// Whether a node type is covered by this residual dens surface.
#[must_use]
pub fn is_ident_literal_related_type(t: &str) -> bool {
    matches!(t, "Identifier" | "Literal")
}

/// Pretty Identifier token: name as written (TS `printIdentifier`).
/// Empty / missing name → empty string (parity: TS only writes when name truthy).
#[must_use]
pub fn identifier_token_pretty(name: Option<&str>) -> String {
    match name {
        Some(n) if !n.is_empty() => n.to_string(),
        _ => String::new(),
    }
}

/// Minify Identifier token: when `mangle_enabled`, use `mangled` (precomputed
/// via `NameMangler` / `generate_mangled_name`); else pass-through original.
/// Reserved names must be passed through as `mangled == original` by caller.
#[must_use]
pub fn identifier_token_minify(
    name: Option<&str>,
    mangle_enabled: bool,
    mangled: Option<&str>,
) -> String {
    let original = match name {
        Some(n) if !n.is_empty() => n,
        _ => return String::new(),
    };
    if mangle_enabled {
        match mangled {
            Some(m) if !m.is_empty() => m.to_string(),
            _ => original.to_string(),
        }
    } else {
        original.to_string()
    }
}

/// Dual-path Identifier emission.
/// `pretty=true` → original name; else minify/mangle path.
#[must_use]
pub fn identifier_token(
    name: Option<&str>,
    pretty: bool,
    mangle_enabled: bool,
    mangled: Option<&str>,
) -> String {
    if pretty {
        identifier_token_pretty(name)
    } else {
        identifier_token_minify(name, mangle_enabled, mangled)
    }
}

/// Dual-oracle string Literal quote policy.
/// Pretty: honors `single_quote`; minify: always double quotes.
#[must_use]
pub fn string_literal_token(value: &str, single_quote: bool, pretty: bool) -> String {
    if pretty {
        quote_string_literal(value, single_quote)
    } else {
        quote_string_literal(value, false)
    }
}

/// Non-string Literal: prefer `raw` when present (both pretty + minify),
/// else fall back to `value_repr` (JSON.stringify stand-in).
#[must_use]
pub fn non_string_literal_token(raw: Option<&str>, value_repr: Option<&str>) -> String {
    match raw {
        Some(r) if !r.is_empty() => r.to_string(),
        _ => value_repr.unwrap_or("").to_string(),
    }
}

/// Full Literal dual-oracle branch.
/// - String values: quote policy (pretty singleQuote vs minify double).
/// - Non-string: prefer raw, else value_repr.
/// Mirrors TS: when raw is defined and value is string → rewrite quotes;
/// when raw defined and non-string → write raw; else JSON.stringify(value).
#[must_use]
pub fn literal_token(
    value_is_string: bool,
    value_str: Option<&str>,
    raw: Option<&str>,
    single_quote: bool,
    pretty: bool,
) -> String {
    if value_is_string {
        return string_literal_token(value_str.unwrap_or(""), single_quote, pretty);
    }
    // TS: if raw !== undefined write String(raw) for non-strings even when empty.
    if let Some(r) = raw {
        return r.to_string();
    }
    value_str.unwrap_or("").to_string()
}

/// Named import / export specifier separator: pretty `, ` vs minify `,`.
#[must_use]
pub fn specifier_list_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Build `{ a, b as c }` style named specifier list interior (already-rendered
/// fragments). Dual-oracle separator only — brace spacing left to caller.
#[must_use]
pub fn named_specifier_list_interior(specifiers: &[&str], pretty: bool) -> String {
    let sep = specifier_list_sep(pretty);
    let mut out = String::with_capacity(
        specifiers.iter().map(|s| s.len()).sum::<usize>()
            + sep.len() * specifiers.len().saturating_sub(1),
    );
    for (i, s) in specifiers.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(s);
    }
    out
}

/// `import X as Y` / `export X as Y` local alias fragment.
/// When `imported == local`, emit bare local (shorthand); else `imported as local`.
#[must_use]
pub fn specifier_alias_fragment(imported: &str, local: &str) -> String {
    if imported == local || local.is_empty() {
        imported.to_string()
    } else if imported.is_empty() {
        local.to_string()
    } else {
        format!("{imported} as {local}")
    }
}

/// Full named-specifier brace skeleton: `{a, b as c}` (no outer spaces in minify;
/// pretty keeps space after `{` / before `}` when non-empty — Prettier-ish residual).
#[must_use]
pub fn named_specifier_braces(specifiers: &[&str], pretty: bool) -> String {
    if specifiers.is_empty() {
        return "{}".to_string();
    }
    let interior = named_specifier_list_interior(specifiers, pretty);
    if pretty {
        format!("{{ {interior} }}")
    } else {
        format!("{{{interior}}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mangle::{generate_mangled_name, NameMangler};

    #[test]
    fn type_guards() {
        assert!(is_identifier_node_type("Identifier"));
        assert!(!is_identifier_node_type("Literal"));
        assert!(is_literal_node_type("Literal"));
        assert!(!is_literal_node_type("TemplateLiteral"));
        assert!(is_ident_literal_related_type("Identifier"));
        assert!(is_ident_literal_related_type("Literal"));
        assert!(!is_ident_literal_related_type("BinaryExpression"));
    }

    #[test]
    fn identifier_pretty_and_minify_mangle() {
        assert_eq!(identifier_token_pretty(Some("foo")), "foo");
        assert_eq!(identifier_token_pretty(None), "");
        assert_eq!(identifier_token_pretty(Some("")), "");

        assert_eq!(
            identifier_token_minify(Some("foo"), false, Some("a")),
            "foo"
        );
        assert_eq!(
            identifier_token_minify(Some("foo"), true, Some("a")),
            "a"
        );
        // missing mangled falls back to original
        assert_eq!(identifier_token_minify(Some("foo"), true, None), "foo");

        let mut m = NameMangler::new(["window".into()]);
        let mangled = m.mangle("longName", true);
        assert_eq!(mangled, generate_mangled_name(0));
        assert_eq!(
            identifier_token(Some("longName"), true, true, Some(&mangled)),
            "longName"
        );
        assert_eq!(
            identifier_token(Some("longName"), false, true, Some(&mangled)),
            mangled
        );
        // reserved: mangler returns original
        let reserved = m.mangle("window", true);
        assert_eq!(reserved, "window");
        assert_eq!(
            identifier_token(Some("window"), false, true, Some(&reserved)),
            "window"
        );
    }

    #[test]
    fn string_literal_quote_dual_oracle() {
        assert_eq!(string_literal_token("hi", true, true), "'hi'");
        assert_eq!(string_literal_token("hi", false, true), "\"hi\"");
        // minify always double
        assert_eq!(string_literal_token("hi", true, false), "\"hi\"");
        assert_eq!(string_literal_token("hi", false, false), "\"hi\"");
    }

    #[test]
    fn literal_token_full_branch() {
        // string path
        assert_eq!(
            literal_token(true, Some("x"), Some("'x'"), true, true),
            "'x'"
        );
        assert_eq!(
            literal_token(true, Some("x"), Some("'x'"), true, false),
            "\"x\""
        );
        // non-string prefers raw
        assert_eq!(
            literal_token(false, Some("42"), Some("0x2a"), false, true),
            "0x2a"
        );
        // empty raw still preferred when Some (TS raw !== undefined)
        assert_eq!(literal_token(false, Some("0"), Some(""), false, true), "");
        // no raw → value_str
        assert_eq!(
            literal_token(false, Some("3.14"), None, false, false),
            "3.14"
        );
        assert_eq!(non_string_literal_token(Some("1e3"), Some("1000")), "1e3");
        assert_eq!(non_string_literal_token(None, Some("1000")), "1000");
    }

    #[test]
    fn named_specifier_lists() {
        assert_eq!(specifier_list_sep(true), ", ");
        assert_eq!(specifier_list_sep(false), ",");
        assert_eq!(specifier_alias_fragment("foo", "foo"), "foo");
        assert_eq!(specifier_alias_fragment("foo", "bar"), "foo as bar");
        assert_eq!(specifier_alias_fragment("foo", ""), "foo");
        assert_eq!(
            named_specifier_list_interior(&["a", "b as c"], true),
            "a, b as c"
        );
        assert_eq!(
            named_specifier_list_interior(&["a", "b as c"], false),
            "a,b as c"
        );
        assert_eq!(named_specifier_braces(&[], true), "{}");
        assert_eq!(named_specifier_braces(&["a"], true), "{ a }");
        assert_eq!(named_specifier_braces(&["a", "b"], false), "{a,b}");
        assert_eq!(
            named_specifier_braces(&["foo as bar"], true),
            "{ foo as bar }"
        );
    }
}

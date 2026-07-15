//! Pure Try / Catch / Throw / Labeled / Break / Continue / With + full Import
//! residual pure continue17 for tooling/format-minify-lint fragment.
//! Complements loop_template / method_array_import / ident_literal skeletons.
//! Full engines remain product dens. Intentional ts_only plugins retained.
//! NO authority_rust / ts_deleted.

use crate::ident_literal_emit::{named_specifier_braces, string_literal_token};

/// Whether node type is TryStatement.
#[must_use]
pub fn is_try_statement_type(t: &str) -> bool {
    t == "TryStatement"
}

/// Whether node type is CatchClause.
#[must_use]
pub fn is_catch_clause_type(t: &str) -> bool {
    t == "CatchClause"
}

/// Whether node type is ThrowStatement.
#[must_use]
pub fn is_throw_statement_type(t: &str) -> bool {
    t == "ThrowStatement"
}

/// Whether node type is LabeledStatement.
#[must_use]
pub fn is_labeled_statement_type(t: &str) -> bool {
    t == "LabeledStatement"
}

/// Whether node type is BreakStatement.
#[must_use]
pub fn is_break_statement_type(t: &str) -> bool {
    t == "BreakStatement"
}

/// Whether node type is ContinueStatement.
#[must_use]
pub fn is_continue_statement_type(t: &str) -> bool {
    t == "ContinueStatement"
}

/// Whether node type is WithStatement.
#[must_use]
pub fn is_with_statement_type(t: &str) -> bool {
    t == "WithStatement"
}

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_try_import_related_type(t: &str) -> bool {
    matches!(
        t,
        "TryStatement"
            | "CatchClause"
            | "ThrowStatement"
            | "LabeledStatement"
            | "BreakStatement"
            | "ContinueStatement"
            | "WithStatement"
            | "ImportDeclaration"
            | "ImportSpecifier"
            | "ImportDefaultSpecifier"
            | "ImportNamespaceSpecifier"
    )
}

/// `try ` keyword (always trailing space before block).
#[must_use]
pub fn try_token() -> &'static str {
    "try "
}

/// `catch` open (pretty → ` catch (`; minify → `catch(`) when param present.
#[must_use]
pub fn catch_open(pretty: bool, has_param: bool) -> &'static str {
    match (pretty, has_param) {
        (true, true) => " catch (",
        (false, true) => "catch(",
        (true, false) => " catch ",
        (false, false) => "catch",
    }
}

/// Close catch param before body (pretty → `) `; minify → `)` when param present).
#[must_use]
pub fn catch_param_close(pretty: bool, has_param: bool) -> &'static str {
    if !has_param {
        return if pretty { "" } else { "" };
    }
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// `finally ` keyword with spacing (pretty → ` finally `; minify → `finally`).
#[must_use]
pub fn finally_token(pretty: bool) -> &'static str {
    if pretty {
        " finally "
    } else {
        "finally"
    }
}

/// Throw statement skeleton: `throw ` + expr + optional semi.
#[must_use]
pub fn throw_statement_skeleton(expr: &str, semi: bool) -> String {
    let mut out = format!("throw {expr}");
    if semi {
        out.push(';');
    }
    out
}

/// Labeled statement skeleton: `label: ` + body (pretty keeps space after `:`).
#[must_use]
pub fn labeled_statement_skeleton(label: &str, body: &str, pretty: bool) -> String {
    if pretty {
        format!("{label}: {body}")
    } else {
        format!("{label}:{body}")
    }
}

/// Break / continue skeleton with optional label.
#[must_use]
pub fn break_continue_skeleton(kind: &str, label: Option<&str>, semi: bool) -> String {
    let kw = if kind == "continue" {
        "continue"
    } else {
        "break"
    };
    let mut out = match label.filter(|l| !l.is_empty()) {
        Some(l) => format!("{kw} {l}"),
        None => kw.to_string(),
    };
    if semi {
        out.push(';');
    }
    out
}

/// With statement open (pretty → `with (`; minify → `with(`).
#[must_use]
pub fn with_open(pretty: bool) -> &'static str {
    if pretty {
        "with ("
    } else {
        "with("
    }
}

/// Close with-object before body.
#[must_use]
pub fn with_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// Full try/catch/finally skeleton from already-rendered block interiors.
#[must_use]
pub fn try_statement_skeleton(
    try_block: &str,
    catch_param: Option<&str>,
    catch_block: Option<&str>,
    finally_block: Option<&str>,
    pretty: bool,
) -> String {
    let mut out = String::from(try_token());
    out.push_str(try_block);
    if let Some(cb) = catch_block {
        let has_param = catch_param.is_some_and(|p| !p.is_empty());
        out.push_str(catch_open(pretty, has_param));
        if let Some(p) = catch_param.filter(|p| !p.is_empty()) {
            out.push_str(p);
        }
        out.push_str(catch_param_close(pretty, has_param));
        out.push_str(cb);
    }
    if let Some(fb) = finally_block {
        out.push_str(finally_token(pretty));
        out.push_str(fb);
    }
    out
}

/// Full ImportDeclaration dual-oracle skeleton with default / namespace / named + from.
///
/// Dual-oracle of simplified printer vs compressor:
/// - pretty: `import Def, * as NS, { a, b as c } from 'mod';`
/// - minify: denser separators; always double-quoted module (compressor default)
#[must_use]
pub fn import_declaration_full_skeleton(
    default_local: Option<&str>,
    namespace_local: Option<&str>,
    named: &[&str],
    source: &str,
    pretty: bool,
    semi: bool,
    single_quote: bool,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(d) = default_local.filter(|s| !s.is_empty()) {
        parts.push(d.to_string());
    }
    if let Some(ns) = namespace_local.filter(|s| !s.is_empty()) {
        parts.push(format!("* as {ns}"));
    }
    if !named.is_empty() {
        parts.push(named_specifier_braces(named, pretty));
    }
    let quote_pretty = pretty;
    let src = string_literal_token(source, single_quote && pretty, quote_pretty);
    // minify compressor always double-quotes modules in this residual dens
    let src = if pretty {
        src
    } else {
        string_literal_token(source, false, false)
    };
    let mut out = String::from("import");
    if !parts.is_empty() {
        out.push(' ');
        let sep = if pretty { ", " } else { "," };
        for (i, p) in parts.iter().enumerate() {
            if i > 0 {
                out.push_str(sep);
            }
            out.push_str(p);
        }
        out.push_str(if pretty { " from " } else { " from " });
        out.push_str(&src);
    } else {
        // side-effect import: `import 'mod'`
        out.push(' ');
        out.push_str(&src);
    }
    if semi {
        out.push(';');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_guards() {
        assert!(is_try_statement_type("TryStatement"));
        assert!(is_catch_clause_type("CatchClause"));
        assert!(is_throw_statement_type("ThrowStatement"));
        assert!(is_try_import_related_type("ImportSpecifier"));
        assert!(!is_try_import_related_type("BinaryExpression"));
    }

    #[test]
    fn try_catch_throw_break_import() {
        assert_eq!(
            try_statement_skeleton("{}", Some("e"), Some("{}"), None, true),
            "try {} catch (e) {}"
        );
        assert_eq!(
            try_statement_skeleton("{}", Some("e"), Some("{}"), None, false),
            "try {}catch(e){}"
        );
        assert_eq!(
            try_statement_skeleton("{}", None, Some("{}"), Some("{}"), true),
            "try {} catch {} finally {}"
        );
        assert_eq!(throw_statement_skeleton("err", true), "throw err;");
        assert_eq!(
            labeled_statement_skeleton("loop", "break;", true),
            "loop: break;"
        );
        assert_eq!(
            labeled_statement_skeleton("loop", "break;", false),
            "loop:break;"
        );
        assert_eq!(break_continue_skeleton("break", None, true), "break;");
        assert_eq!(
            break_continue_skeleton("continue", Some("loop"), true),
            "continue loop;"
        );
        assert_eq!(with_open(true), "with (");
        assert_eq!(with_close(false), ")");

        assert_eq!(
            import_declaration_full_skeleton(
                Some("React"),
                None,
                &["useState", "useMemo as um"],
                "react",
                true,
                true,
                true
            ),
            "import React, { useState, useMemo as um } from 'react';"
        );
        assert_eq!(
            import_declaration_full_skeleton(None, Some("fs"), &[], "node:fs", false, true, false),
            "import * as fs from \"node:fs\";"
        );
        assert_eq!(
            import_declaration_full_skeleton(None, None, &[], "side-effect", true, true, false),
            "import \"side-effect\";"
        );
    }
}

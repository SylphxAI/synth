//! Pure object-expression / program-separator emission helpers —
//! residual pure continue9 for tooling/format-minify-lint fragment.
//! Mirrors `printObjectExpression` / `compressObjectExpression` and
//! statement separators in `packages/synth-js-format` / `synth-js-minify`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//!
//! Dual-oracle surface keeps small pure kernels even when siblings exist in
//! other emit modules — intentional residual dens (not dead product paths).
#![allow(dead_code)]

/// Pretty object open with optional `bracketSpacing` pad (`{ ` vs `{`).
#[must_use]
pub fn object_open_pad(bracket_spacing: bool) -> &'static str {
    if bracket_spacing {
        "{ "
    } else {
        "{"
    }
}

/// Pretty object close with optional `bracketSpacing` pad (` }` vs `}`).
#[must_use]
pub fn object_close_pad(bracket_spacing: bool) -> &'static str {
    if bracket_spacing {
        " }"
    } else {
        "}"
    }
}

/// Empty object literal `{}` (both pretty + minify; no spacing).
#[must_use]
pub fn empty_object() -> &'static str {
    "{}"
}

/// Property key/value separator: pretty `: `, minify `:`.
#[must_use]
pub fn object_property_colon(pretty: bool) -> &'static str {
    if pretty {
        ": "
    } else {
        ":"
    }
}

/// Element separator inside object properties: pretty `, `, minify `,`.
#[must_use]
pub fn object_prop_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Whether trailing comma applies for object props under format options.
/// Format: `trailingComma === 'all' || trailingComma === 'es5'` → true.
/// Minify compressor never emits object trailing commas.
#[must_use]
pub fn object_wants_trailing_comma(pretty: bool, trailing_comma: &str) -> bool {
    if !pretty {
        return false;
    }
    matches!(trailing_comma, "all" | "es5")
}

/// Program-level statement separator: pretty `\n\n`, minify empty (concat).
#[must_use]
pub fn program_statement_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n\n"
    } else {
        ""
    }
}

/// Block-level statement separator: pretty `\n`, minify empty.
#[must_use]
pub fn block_statement_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n"
    } else {
        ""
    }
}

/// Whether a node type is an object-expression related surface.
#[must_use]
pub fn is_object_related_type(ty: &str) -> bool {
    matches!(
        ty,
        "ObjectExpression" | "ObjectPattern" | "Property" | "SpreadElement" | "RestElement"
    )
}

/// Build object-expression emission skeleton from already-rendered props.
///
/// Pretty with bracketSpacing: `{ a: 1, b: 2 }` (+ optional trailing comma).
/// Minify: `{a:1,b:2}`.
#[must_use]
pub fn object_skeleton(props: &[&str], pretty: bool, bracket_spacing: bool, trailing_comma: bool) -> String {
    if props.is_empty() {
        return empty_object().to_string();
    }
    let mut out = String::with_capacity(props.iter().map(|p| p.len()).sum::<usize>() + 8);
    if pretty && bracket_spacing {
        out.push_str(object_open_pad(true));
    } else {
        out.push('{');
    }
    for (i, p) in props.iter().enumerate() {
        if i > 0 {
            out.push_str(object_prop_sep(pretty));
        }
        out.push_str(p);
    }
    if trailing_comma {
        out.push(',');
    }
    if pretty && bracket_spacing {
        out.push_str(object_close_pad(true));
    } else {
        out.push('}');
    }
    out
}

/// Build a single property emission: `key: value` (pretty) or `key:value` (minify).
#[must_use]
pub fn object_property_skeleton(key: &str, value: &str, pretty: bool) -> String {
    format!("{key}{}{value}", object_property_colon(pretty))
}

/// Shorthand property emission when key == value identifier (TS may still write
/// `key: value` in the simplified printer; residual kernel for future dens).
#[must_use]
pub fn shorthand_property(name: &str) -> String {
    name.to_string()
}

/// Variable declaration kind token with trailing space (const/let/var).
#[must_use]
pub fn var_kind_with_space(kind: &str) -> String {
    let k = match kind {
        "let" => "let",
        "var" => "var",
        _ => "const",
    };
    format!("{k} ")
}

/// Optional semicolon terminator (format option `semi`; minify always ends many stmts with `;`).
#[must_use]
pub fn semi_if(semi: bool) -> &'static str {
    if semi {
        ";"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_pads() {
        assert_eq!(empty_object(), "{}");
        assert_eq!(object_open_pad(true), "{ ");
        assert_eq!(object_close_pad(true), " }");
        assert_eq!(object_open_pad(false), "{");
        assert_eq!(object_close_pad(false), "}");
    }

    #[test]
    fn property_and_separators() {
        assert_eq!(object_property_colon(true), ": ");
        assert_eq!(object_property_colon(false), ":");
        assert_eq!(object_prop_sep(true), ", ");
        assert_eq!(object_prop_sep(false), ",");
        assert_eq!(object_property_skeleton("a", "1", true), "a: 1");
        assert_eq!(object_property_skeleton("a", "1", false), "a:1");
        assert_eq!(shorthand_property("x"), "x");
    }

    #[test]
    fn trailing_comma_policy() {
        assert!(object_wants_trailing_comma(true, "es5"));
        assert!(object_wants_trailing_comma(true, "all"));
        assert!(!object_wants_trailing_comma(true, "none"));
        assert!(!object_wants_trailing_comma(false, "es5"));
        assert!(!object_wants_trailing_comma(false, "all"));
    }

    #[test]
    fn object_skeletons() {
        assert_eq!(object_skeleton(&[], true, true, false), "{}");
        assert_eq!(
            object_skeleton(&["a: 1", "b: 2"], true, true, false),
            "{ a: 1, b: 2 }"
        );
        assert_eq!(
            object_skeleton(&["a: 1"], true, true, true),
            "{ a: 1, }"
        );
        assert_eq!(
            object_skeleton(&["a:1", "b:2"], false, false, false),
            "{a:1,b:2}"
        );
        assert_eq!(
            object_skeleton(&["a: 1"], true, false, false),
            "{a: 1}"
        );
    }

    #[test]
    fn program_and_var_helpers() {
        assert_eq!(program_statement_sep(true), "\n\n");
        assert_eq!(program_statement_sep(false), "");
        assert_eq!(block_statement_sep(true), "\n");
        assert_eq!(block_statement_sep(false), "");
        assert_eq!(var_kind_with_space("const"), "const ");
        assert_eq!(var_kind_with_space("let"), "let ");
        assert_eq!(var_kind_with_space("var"), "var ");
        assert_eq!(var_kind_with_space("unknown"), "const ");
        assert_eq!(semi_if(true), ";");
        assert_eq!(semi_if(false), "");
        assert!(is_object_related_type("ObjectExpression"));
        assert!(is_object_related_type("Property"));
        assert!(!is_object_related_type("ArrayExpression"));
    }
}

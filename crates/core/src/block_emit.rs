//! Pure block / list structure emission helpers —
//! mirrors printer BlockStatement + class body braces and trailing-comma
//! suffixes in `packages/synth-js-format` / `synth-js-minify`.
//! Residual pure continue for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Open a block body (`{` + optional newline when pretty and nonempty).
/// TS format: write('{'); if (children.length > 0) write('\n');
/// Minify: always `{` with no inner newline.
#[must_use]
pub fn block_open(pretty: bool, nonempty: bool) -> &'static str {
    if pretty && nonempty {
        "{\n"
    } else {
        "{"
    }
}

/// Close a block body (optional leading newline+indent handled by caller).
/// Pure close delimiter is always `}`.
#[must_use]
pub fn block_close() -> &'static str {
    "}"
}

/// Inner separator between block children (pretty → `\n`; minify → empty).
#[must_use]
pub fn block_item_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n"
    } else {
        ""
    }
}

/// Trailing comma after the last list item when policy wants it and items exist.
/// TS: if (trailingComma all|es5 && children.length > 0) write(',').
#[must_use]
pub fn trailing_comma_suffix(want: bool, item_count: usize) -> &'static str {
    if want && item_count > 0 {
        ","
    } else {
        ""
    }
}

/// Whether a unary operator is prefix (TS: `prefix !== false` → default true).
#[must_use]
pub fn unary_is_prefix(prefix_flag: Option<bool>) -> bool {
    prefix_flag.unwrap_or(true)
}

/// Object expression open with optional bracket-spacing space after `{`.
/// Empty objects use `empty_braces("object")` instead (caller decides).
#[must_use]
pub fn object_open(bracket_spacing: bool, nonempty: bool) -> &'static str {
    if !nonempty {
        "{"
    } else if bracket_spacing {
        "{ "
    } else {
        "{"
    }
}

/// Object expression close with optional bracket-spacing space before `}`.
#[must_use]
pub fn object_close(bracket_spacing: bool, nonempty: bool) -> &'static str {
    if !nonempty {
        "}"
    } else if bracket_spacing {
        " }"
    } else {
        "}"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_open_close_sep() {
        assert_eq!(block_open(true, true), "{\n");
        assert_eq!(block_open(true, false), "{");
        assert_eq!(block_open(false, true), "{");
        assert_eq!(block_close(), "}");
        assert_eq!(block_item_sep(true), "\n");
        assert_eq!(block_item_sep(false), "");
    }

    #[test]
    fn trailing_comma_and_unary() {
        assert_eq!(trailing_comma_suffix(true, 2), ",");
        assert_eq!(trailing_comma_suffix(true, 0), "");
        assert_eq!(trailing_comma_suffix(false, 3), "");
        assert!(unary_is_prefix(None));
        assert!(unary_is_prefix(Some(true)));
        assert!(!unary_is_prefix(Some(false)));
    }

    #[test]
    fn object_brackets() {
        assert_eq!(object_open(true, true), "{ ");
        assert_eq!(object_open(false, true), "{");
        assert_eq!(object_open(true, false), "{");
        assert_eq!(object_close(true, true), " }");
        assert_eq!(object_close(false, true), "}");
        assert_eq!(object_close(true, false), "}");
    }
}

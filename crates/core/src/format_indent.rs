//! Pure format indent + string-quote helpers — mirrors
//! `packages/synth-js-format/src/printer.ts` writeIndent / printLiteral quote style.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full formatter engine, NO authority_rust / ts_deleted.

/// Build indent string for a line (TS: writeIndent).
/// `use_tabs` → tab * level; else space * (level * tab_width).
#[must_use]
pub fn indent_string(level: usize, tab_width: usize, use_tabs: bool) -> String {
    if use_tabs {
        "\t".repeat(level)
    } else {
        " ".repeat(level.saturating_mul(tab_width))
    }
}

/// Quote a string value with single or double quotes (TS: printLiteral quote branch).
/// Does not escape contents — parity with simple TS printer path.
#[must_use]
pub fn quote_string_literal(value: &str, single_quote: bool) -> String {
    if single_quote {
        format!("'{value}'")
    } else {
        format!("\"{value}\"")
    }
}

/// Resolve end-of-line sequence (TS: FormatOptions.endOfLine defaults).
#[must_use]
pub fn end_of_line(kind: &str) -> &'static str {
    match kind {
        "crlf" => "\r\n",
        "cr" => "\r",
        // "lf" | "auto" | unknown → lf
        _ => "\n",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indent_spaces_and_tabs() {
        assert_eq!(indent_string(0, 2, false), "");
        assert_eq!(indent_string(1, 2, false), "  ");
        assert_eq!(indent_string(2, 4, false), "        ");
        assert_eq!(indent_string(2, 2, true), "\t\t");
        assert_eq!(indent_string(3, 0, false), "");
    }

    #[test]
    fn quote_single_and_double() {
        assert_eq!(quote_string_literal("hi", false), "\"hi\"");
        assert_eq!(quote_string_literal("hi", true), "'hi'");
        assert_eq!(quote_string_literal("", true), "''");
    }

    #[test]
    fn eol_variants() {
        assert_eq!(end_of_line("lf"), "\n");
        assert_eq!(end_of_line("crlf"), "\r\n");
        assert_eq!(end_of_line("cr"), "\r");
        assert_eq!(end_of_line("auto"), "\n");
        assert_eq!(end_of_line("unknown"), "\n");
    }
}

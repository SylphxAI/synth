//! Pure statement/control-flow keyword emission helpers —
//! mirrors `packages/synth-js-format/src/printer.ts` and
//! `packages/synth-js-minify/src/compressor.ts` return/if/else/class/import/await branches.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// `return` keyword with optional trailing space when an argument is present.
/// TS: write('return'); if (arg) write(' ').
#[must_use]
pub fn return_token(has_argument: bool) -> &'static str {
    if has_argument {
        "return "
    } else {
        "return"
    }
}

/// `if` open delimiter (pretty → `if (`; minify → `if(`).
#[must_use]
pub fn if_open(pretty: bool) -> &'static str {
    if pretty {
        "if ("
    } else {
        "if("
    }
}

/// Close of if-test before body (pretty → `) `; minify → `)`).
#[must_use]
pub fn if_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// `else` prefix between consequent and alternate (pretty → ` else `; minify → `else `).
#[must_use]
pub fn else_prefix(pretty: bool) -> &'static str {
    if pretty {
        " else "
    } else {
        "else "
    }
}

/// `class ` declaration keyword (format + minify share trailing space).
#[must_use]
pub fn class_prefix() -> &'static str {
    "class "
}

/// `import ` declaration keyword.
#[must_use]
pub fn import_prefix() -> &'static str {
    "import "
}

/// `await ` expression keyword (always needs trailing space before operand).
#[must_use]
pub fn await_prefix() -> &'static str {
    "await "
}

/// `new ` constructor call keyword.
#[must_use]
pub fn new_prefix() -> &'static str {
    "new "
}

/// Class name suffix space when a named class id is present (TS: `${name} `).
#[must_use]
pub fn class_name_suffix(has_name: bool) -> &'static str {
    if has_name {
        " "
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_tokens() {
        assert_eq!(return_token(true), "return ");
        assert_eq!(return_token(false), "return");
    }

    #[test]
    fn if_else_pretty_vs_minify() {
        assert_eq!(if_open(true), "if (");
        assert_eq!(if_open(false), "if(");
        assert_eq!(if_close(true), ") ");
        assert_eq!(if_close(false), ")");
        assert_eq!(else_prefix(true), " else ");
        assert_eq!(else_prefix(false), "else ");
    }

    #[test]
    fn decl_keywords() {
        assert_eq!(class_prefix(), "class ");
        assert_eq!(import_prefix(), "import ");
        assert_eq!(await_prefix(), "await ");
        assert_eq!(new_prefix(), "new ");
        assert_eq!(class_name_suffix(true), " ");
        assert_eq!(class_name_suffix(false), "");
    }
}

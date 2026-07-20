//! Pure loop / switch / template emission helpers —
//! residual pure continue7 for tooling/format-minify-lint fragment.
//! Mirrors common printer/compressor spacing branches for for/while/do/switch
//! and template-literal delimiters not covered by stmt_emit / conditional_spread.
//! Full engines remain product residual. NO authority_rust / ts_deleted.

/// `for` open (pretty → `for (`; minify → `for(`).
#[must_use]
pub fn for_open(pretty: bool) -> &'static str {
    if pretty {
        "for ("
    } else {
        "for("
    }
}

/// Close of for-header before body (pretty → `) `; minify → `)`).
#[must_use]
pub fn for_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// `for`…`of` keyword fragment (pretty → ` of `; minify → ` of ` — word ops keep spaces).
#[must_use]
pub fn for_of_token() -> &'static str {
    " of "
}

/// `for`…`in` keyword fragment.
#[must_use]
pub fn for_in_token() -> &'static str {
    " in "
}

/// `while` open (pretty → `while (`; minify → `while(`).
#[must_use]
pub fn while_open(pretty: bool) -> &'static str {
    if pretty {
        "while ("
    } else {
        "while("
    }
}

/// Close of while-test before body (pretty → `) `; minify → `)`).
#[must_use]
pub fn while_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// `do` token with trailing space before body (always `do ` when body follows).
#[must_use]
pub fn do_token() -> &'static str {
    "do "
}

/// `while` after do-body (pretty → ` while (`; minify → `while(`).
#[must_use]
pub fn do_while_open(pretty: bool) -> &'static str {
    if pretty {
        " while ("
    } else {
        "while("
    }
}

/// `switch` open (pretty → `switch (`; minify → `switch(`).
#[must_use]
pub fn switch_open(pretty: bool) -> &'static str {
    if pretty {
        "switch ("
    } else {
        "switch("
    }
}

/// Close of switch discriminant before body braces (pretty → `) `; minify → `)`).
#[must_use]
pub fn switch_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// `case ` label prefix.
#[must_use]
pub fn case_prefix() -> &'static str {
    "case "
}

/// `default` label (no value).
#[must_use]
pub fn default_label() -> &'static str {
    "default"
}

/// Case/default colon after label (pretty → `: `; minify → `:`).
#[must_use]
pub fn case_colon(pretty: bool) -> &'static str {
    if pretty {
        ": "
    } else {
        ":"
    }
}

/// `break` with optional label space (TS: `break` / `break label`).
#[must_use]
pub fn break_token(has_label: bool) -> &'static str {
    if has_label {
        "break "
    } else {
        "break"
    }
}

/// `continue` with optional label space.
#[must_use]
pub fn continue_token(has_label: bool) -> &'static str {
    if has_label {
        "continue "
    } else {
        "continue"
    }
}

/// Template literal outer backticks.
#[must_use]
pub fn template_tick() -> &'static str {
    "`"
}

/// Template expression open `${`.
#[must_use]
pub fn template_expr_open() -> &'static str {
    "${"
}

/// Template expression close `}`.
#[must_use]
pub fn template_expr_close() -> &'static str {
    "}"
}

/// Whether a node type is a loop statement (for dual-oracle type checks).
#[must_use]
pub fn is_loop_statement_type(ty: &str) -> bool {
    matches!(
        ty,
        "ForStatement"
            | "ForInStatement"
            | "ForOfStatement"
            | "WhileStatement"
            | "DoWhileStatement"
    )
}

/// Whether a node type is a switch-related statement.
#[must_use]
pub fn is_switch_related_type(ty: &str) -> bool {
    matches!(
        ty,
        "SwitchStatement" | "SwitchCase" | "BreakStatement" | "ContinueStatement"
    )
}

/// Whether a node type is a template literal / quasi.
#[must_use]
pub fn is_template_type(ty: &str) -> bool {
    matches!(
        ty,
        "TemplateLiteral" | "TemplateElement" | "TaggedTemplateExpression"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_while_pretty_vs_minify() {
        assert_eq!(for_open(true), "for (");
        assert_eq!(for_open(false), "for(");
        assert_eq!(for_close(true), ") ");
        assert_eq!(for_close(false), ")");
        assert_eq!(for_of_token(), " of ");
        assert_eq!(for_in_token(), " in ");
        assert_eq!(while_open(true), "while (");
        assert_eq!(while_open(false), "while(");
        assert_eq!(while_close(true), ") ");
        assert_eq!(while_close(false), ")");
        assert_eq!(do_token(), "do ");
        assert_eq!(do_while_open(true), " while (");
        assert_eq!(do_while_open(false), "while(");
    }

    #[test]
    fn switch_case_break() {
        assert_eq!(switch_open(true), "switch (");
        assert_eq!(switch_open(false), "switch(");
        assert_eq!(switch_close(true), ") ");
        assert_eq!(switch_close(false), ")");
        assert_eq!(case_prefix(), "case ");
        assert_eq!(default_label(), "default");
        assert_eq!(case_colon(true), ": ");
        assert_eq!(case_colon(false), ":");
        assert_eq!(break_token(false), "break");
        assert_eq!(break_token(true), "break ");
        assert_eq!(continue_token(false), "continue");
        assert_eq!(continue_token(true), "continue ");
    }

    #[test]
    fn template_and_type_guards() {
        assert_eq!(template_tick(), "`");
        assert_eq!(template_expr_open(), "${");
        assert_eq!(template_expr_close(), "}");
        assert!(is_loop_statement_type("ForOfStatement"));
        assert!(!is_loop_statement_type("IfStatement"));
        assert!(is_switch_related_type("SwitchCase"));
        assert!(is_template_type("TemplateLiteral"));
        assert!(!is_template_type("Literal"));
    }
}

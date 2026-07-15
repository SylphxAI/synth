//! Pure conditional / spread / sequence / throw emission helpers —
//! residual pure continue6 for tooling/format-minify-lint fragment.
//! Mirrors common printer/compressor spacing branches for expressions
//! not yet covered by unary_binary / stmt_emit (ternary, spread, sequence,
//! throw). Full engines remain product dens. NO authority_rust / ts_deleted.

/// Ternary `?` fragment (pretty → ` ? `; minify → `?`).
#[must_use]
pub fn ternary_q(pretty: bool) -> &'static str {
    if pretty {
        " ? "
    } else {
        "?"
    }
}

/// Ternary `:` fragment (pretty → ` : `; minify → `:`).
#[must_use]
pub fn ternary_colon(pretty: bool) -> &'static str {
    if pretty {
        " : "
    } else {
        ":"
    }
}

/// Full ternary operator pair as `(q, colon)` for pretty/minify.
#[must_use]
pub fn ternary_pair(pretty: bool) -> (&'static str, &'static str) {
    (ternary_q(pretty), ternary_colon(pretty))
}

/// Spread / rest element prefix (`...args`, `...rest`).
#[must_use]
pub fn spread_prefix() -> &'static str {
    "..."
}

/// Whether a node type is a spread/rest element (TS includes checks).
#[must_use]
pub fn is_spread_element_type(ty: &str) -> bool {
    matches!(
        ty,
        "SpreadElement" | "RestElement" | "JSXSpreadAttribute" | "JSXSpreadChild"
    )
}

/// Sequence / comma expression separator (pretty → `, `; minify → `,`).
#[must_use]
pub fn sequence_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// `throw` keyword with optional trailing space when an argument is present
/// (TS printers: `throw` + space + expr; bare `throw` is a syntax error so
/// argument is expected — still model both for dual-oracle completeness).
#[must_use]
pub fn throw_token(has_argument: bool) -> &'static str {
    if has_argument {
        "throw "
    } else {
        "throw"
    }
}

/// `try` keyword (always bare; body brace handled by block_emit).
#[must_use]
pub fn try_token() -> &'static str {
    "try"
}

/// `catch` open (pretty → ` catch (`; minify → `catch(`).
#[must_use]
pub fn catch_open(pretty: bool) -> &'static str {
    if pretty {
        " catch ("
    } else {
        "catch("
    }
}

/// Close catch param before body (pretty → `) `; minify → `)`).
#[must_use]
pub fn catch_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// Optional catch binding omitted form (optional catch binding ES2019):
/// pretty → ` catch `; minify → `catch`.
#[must_use]
pub fn catch_no_param(pretty: bool) -> &'static str {
    if pretty {
        " catch "
    } else {
        "catch"
    }
}

/// `finally` prefix (pretty → ` finally `; minify → `finally`).
#[must_use]
pub fn finally_prefix(pretty: bool) -> &'static str {
    if pretty {
        " finally "
    } else {
        "finally"
    }
}

/// Nullish coalescing / logical assignment already covered elsewhere;
/// this is the pure `??` token with pretty spacing.
#[must_use]
pub fn nullish_coalesce_token(pretty: bool) -> &'static str {
    if pretty {
        " ?? "
    } else {
        "??"
    }
}

/// Optional chain operator token (`.` vs `?.` pure fragment complement).
#[must_use]
pub fn optional_chain_dot(optional: bool) -> &'static str {
    if optional {
        "?."
    } else {
        "."
    }
}

/// Yield token with optional star + trailing space.
#[must_use]
pub fn yield_token(delegate: bool, has_argument: bool) -> String {
    let mut s = String::from("yield");
    if delegate {
        s.push('*');
    }
    if has_argument {
        s.push(' ');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ternary_pretty_vs_minify() {
        assert_eq!(ternary_q(true), " ? ");
        assert_eq!(ternary_q(false), "?");
        assert_eq!(ternary_colon(true), " : ");
        assert_eq!(ternary_colon(false), ":");
        assert_eq!(ternary_pair(true), (" ? ", " : "));
        assert_eq!(ternary_pair(false), ("?", ":"));
    }

    #[test]
    fn spread_and_sequence() {
        assert_eq!(spread_prefix(), "...");
        assert!(is_spread_element_type("SpreadElement"));
        assert!(is_spread_element_type("RestElement"));
        assert!(!is_spread_element_type("Identifier"));
        assert_eq!(sequence_sep(true), ", ");
        assert_eq!(sequence_sep(false), ",");
    }

    #[test]
    fn throw_try_catch_finally() {
        assert_eq!(throw_token(true), "throw ");
        assert_eq!(throw_token(false), "throw");
        assert_eq!(try_token(), "try");
        assert_eq!(catch_open(true), " catch (");
        assert_eq!(catch_open(false), "catch(");
        assert_eq!(catch_close(true), ") ");
        assert_eq!(catch_close(false), ")");
        assert_eq!(catch_no_param(true), " catch ");
        assert_eq!(catch_no_param(false), "catch");
        assert_eq!(finally_prefix(true), " finally ");
        assert_eq!(finally_prefix(false), "finally");
    }

    #[test]
    fn nullish_optional_yield() {
        assert_eq!(nullish_coalesce_token(true), " ?? ");
        assert_eq!(nullish_coalesce_token(false), "??");
        assert_eq!(optional_chain_dot(true), "?.");
        assert_eq!(optional_chain_dot(false), ".");
        assert_eq!(yield_token(false, true), "yield ");
        assert_eq!(yield_token(true, true), "yield* ");
        assert_eq!(yield_token(true, false), "yield*");
        assert_eq!(yield_token(false, false), "yield");
    }
}

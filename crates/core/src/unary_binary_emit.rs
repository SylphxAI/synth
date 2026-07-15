//! Pure unary/binary operator emission spacing —
//! mirrors `printUnaryExpression` / `printBinaryExpression` /
//! `compressUnaryExpression` / `compressBinaryExpression` spacing branches.
//! Residual pure continue for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Word-like unary operators that need a trailing space in pretty/minify
/// when followed by an identifier (typeof/void/delete/await).
#[must_use]
pub fn unary_is_word_operator(op: &str) -> bool {
    matches!(op, "typeof" | "void" | "delete" | "await" | "yield")
}

/// Whether a unary operator token is a pure prefix form (`!x`, `-x`, `typeof x`).
/// Update expressions (`++`/`--`) are position-sensitive — use [`update_prefix_token`].
#[must_use]
pub fn unary_is_always_prefix(op: &str) -> bool {
    !matches!(op, "++" | "--")
}

/// Emit the operator token for a unary expression.
/// Pretty word ops keep trailing space; symbolic never; minify word ops keep space.
#[must_use]
pub fn unary_operator_token(op: &str, pretty: bool) -> String {
    if unary_is_word_operator(op) {
        // Both pretty and minify write `typeof ` / `await ` etc.
        let _ = pretty;
        format!("{op} ")
    } else {
        op.to_string()
    }
}

/// Whether a binary operator is spaced in pretty mode.
/// TS format: `write(\` ${operator} \`)`; minify: `write(operator)`.
#[must_use]
pub fn binary_needs_spaces(pretty: bool) -> bool {
    pretty
}

/// Emit binary operator fragment (with or without surrounding spaces).
#[must_use]
pub fn binary_operator_token(op: &str, pretty: bool) -> String {
    if pretty {
        format!(" {op} ")
    } else {
        op.to_string()
    }
}

/// Assignment operators share binary spacing rules in the pure fragment.
#[must_use]
pub fn is_assignment_operator(op: &str) -> bool {
    matches!(
        op,
        "=" | "+="
            | "-="
            | "*="
            | "/="
            | "%="
            | "**="
            | "<<="
            | ">>="
            | ">>>="
            | "|="
            | "^="
            | "&="
            | "&&="
            | "||="
            | "??="
    )
}

/// Logical operators that are always spaced in pretty printers.
#[must_use]
pub fn is_logical_operator(op: &str) -> bool {
    matches!(op, "&&" | "||" | "??")
}

/// Update expression position: prefix `++x` vs postfix `x++`.
#[must_use]
pub fn update_prefix_token(prefix: bool, op: &str) -> (&'static str, &'static str) {
    if prefix {
        // op before operand
        if op == "++" {
            ("++", "")
        } else {
            ("--", "")
        }
    } else if op == "++" {
        ("", "++")
    } else {
        ("", "--")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unary_word_vs_symbolic() {
        assert!(unary_is_word_operator("typeof"));
        assert!(unary_is_word_operator("await"));
        assert!(!unary_is_word_operator("!"));
        assert_eq!(unary_operator_token("!", true), "!");
        assert_eq!(unary_operator_token("typeof", true), "typeof ");
        assert_eq!(unary_operator_token("typeof", false), "typeof ");
        assert_eq!(unary_operator_token("-", false), "-");
    }

    #[test]
    fn binary_spacing() {
        assert!(binary_needs_spaces(true));
        assert!(!binary_needs_spaces(false));
        assert_eq!(binary_operator_token("+", true), " + ");
        assert_eq!(binary_operator_token("+", false), "+");
        assert_eq!(binary_operator_token("===", true), " === ");
        assert!(is_assignment_operator("+="));
        assert!(is_logical_operator("&&"));
        assert!(!is_logical_operator("+"));
    }

    #[test]
    fn update_tokens() {
        assert_eq!(update_prefix_token(true, "++"), ("++", ""));
        assert_eq!(update_prefix_token(false, "++"), ("", "++"));
        assert_eq!(update_prefix_token(true, "--"), ("--", ""));
        assert_eq!(update_prefix_token(false, "--"), ("", "--"));
    }
}

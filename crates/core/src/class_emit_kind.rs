//! Pure class-declaration emission helpers —
//! mirrors `printClassDeclaration` / `compressClassDeclaration`
//! (prefix, name spacing, empty body) in format/minify tooling.
//! Residual pure continue for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Class keyword + trailing space written before the optional id.
/// TS minify/format: `write('class ')`.
#[must_use]
pub fn class_keyword() -> &'static str {
    "class "
}

/// Whether a class id should be written (string name present + non-empty).
#[must_use]
pub fn should_emit_class_name(name: Option<&str>) -> bool {
    matches!(name, Some(n) if !n.is_empty())
}

/// Trailing space after the class name before `{` / heritage.
/// TS minify: `` `${mangleName(name)} ` `` when name present.
#[must_use]
pub fn class_name_trailing_space(has_name: bool) -> &'static str {
    if has_name {
        " "
    } else {
        ""
    }
}

/// Empty class body token when no ClassBody child exists.
/// TS: `write('{}')` in the else branch of classBody find.
#[must_use]
pub fn empty_class_body() -> &'static str {
    "{}"
}

/// Open/close for a non-empty ClassBody.
#[must_use]
pub fn class_body_open() -> &'static str {
    "{"
}

#[must_use]
pub fn class_body_close() -> &'static str {
    "}"
}

/// Choose empty vs braced body based on whether a ClassBody child exists
/// and whether it has any method children.
#[must_use]
pub fn class_body_token(has_class_body: bool, method_count: usize) -> &'static str {
    if !has_class_body {
        return empty_class_body();
    }
    // Non-empty structure still uses open/close around methods; empty body
    // with ClassBody present still writes `{}` (open+close with zero methods).
    if method_count == 0 {
        empty_class_body()
    } else {
        // Caller walks methods between open/close — signal non-empty via open.
        class_body_open()
    }
}

/// True when the node type is ClassDeclaration / ClassExpression.
#[must_use]
pub fn is_class_type(node_type: &str) -> bool {
    matches!(node_type, "ClassDeclaration" | "ClassExpression")
}

/// True when the node type is ClassBody.
#[must_use]
pub fn is_class_body_type(node_type: &str) -> bool {
    node_type == "ClassBody"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_name_and_keyword() {
        assert_eq!(class_keyword(), "class ");
        assert!(should_emit_class_name(Some("Foo")));
        assert!(!should_emit_class_name(Some("")));
        assert!(!should_emit_class_name(None));
        assert_eq!(class_name_trailing_space(true), " ");
        assert_eq!(class_name_trailing_space(false), "");
    }

    #[test]
    fn body_tokens() {
        assert_eq!(empty_class_body(), "{}");
        assert_eq!(class_body_token(false, 0), "{}");
        assert_eq!(class_body_token(true, 0), "{}");
        assert_eq!(class_body_token(true, 2), "{");
        assert_eq!(class_body_close(), "}");
    }

    #[test]
    fn type_predicates() {
        assert!(is_class_type("ClassDeclaration"));
        assert!(is_class_type("ClassExpression"));
        assert!(!is_class_type("FunctionDeclaration"));
        assert!(is_class_body_type("ClassBody"));
    }
}

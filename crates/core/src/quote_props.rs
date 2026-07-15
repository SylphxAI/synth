//! Pure object-key quote decision —
//! mirrors FormatOptions.quoteProps: as-needed | consistent | preserve.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer engine, NO authority_rust / ts_deleted.

/// True when `key` is a bare identifier that does not require quotes in object literals.
/// Simplified ES identifier: starts with letter/_/$, then letter/digit/_/$.
#[must_use]
pub fn is_bare_identifier_key(key: &str) -> bool {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// Whether a property key must be quoted under `as-needed` mode.
#[must_use]
pub fn needs_quote_as_needed(key: &str) -> bool {
    !is_bare_identifier_key(key)
}

/// Quote decision for a single property key given mode.
/// - `as-needed`: quote only non-identifiers
/// - `consistent`: if any key in the object needs quotes (`any_needs_quote`), quote all
/// - `preserve`: honor `was_quoted` from source
/// - unknown → as-needed
#[must_use]
pub fn should_quote_prop(mode: &str, key: &str, was_quoted: bool, any_needs_quote: bool) -> bool {
    match mode {
        "preserve" => was_quoted,
        "consistent" => {
            if any_needs_quote {
                true
            } else {
                needs_quote_as_needed(key)
            }
        }
        // "as-needed" | unknown
        _ => needs_quote_as_needed(key),
    }
}

/// Emit a property key with optional quotes (no content escaping — parity with simple path).
#[must_use]
pub fn emit_prop_key(key: &str, quote: bool, single_quote: bool) -> String {
    if !quote {
        return key.to_string();
    }
    if single_quote {
        format!("'{key}'")
    } else {
        format!("\"{key}\"")
    }
}

/// Scan keys: true when any key needs quotes under as-needed (for consistent mode).
#[must_use]
pub fn any_key_needs_quote(keys: &[&str]) -> bool {
    keys.iter().any(|k| needs_quote_as_needed(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_identifiers() {
        assert!(is_bare_identifier_key("foo"));
        assert!(is_bare_identifier_key("_bar"));
        assert!(is_bare_identifier_key("$x"));
        assert!(is_bare_identifier_key("a1"));
        assert!(!is_bare_identifier_key(""));
        assert!(!is_bare_identifier_key("1a"));
        assert!(!is_bare_identifier_key("foo-bar"));
        assert!(!is_bare_identifier_key("foo bar"));
        // reserved words still match bare-identifier shape (as-needed does not ban keywords)
        assert!(is_bare_identifier_key("class"));
        assert!(needs_quote_as_needed("foo-bar"));
        assert!(!needs_quote_as_needed("foo"));
    }

    #[test]
    fn modes() {
        assert!(!should_quote_prop("as-needed", "foo", true, false));
        assert!(should_quote_prop("as-needed", "foo-bar", false, false));
        assert!(should_quote_prop("preserve", "foo", true, false));
        assert!(!should_quote_prop("preserve", "foo", false, false));
        // consistent: any needs quote → all quoted
        assert!(should_quote_prop("consistent", "foo", false, true));
        assert!(!should_quote_prop("consistent", "foo", false, false));
        assert!(should_quote_prop("consistent", "a-b", false, false));
        // unknown → as-needed
        assert!(!should_quote_prop("unknown", "ok", true, true));
    }

    #[test]
    fn emit_and_scan() {
        assert_eq!(emit_prop_key("foo", false, false), "foo");
        assert_eq!(emit_prop_key("foo", true, false), "\"foo\"");
        assert_eq!(emit_prop_key("foo", true, true), "'foo'");
        assert!(any_key_needs_quote(&["a", "b-c"]));
        assert!(!any_key_needs_quote(&["a", "b"]));
    }
}

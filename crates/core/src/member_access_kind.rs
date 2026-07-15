//! Pure member-access emission helpers —
//! mirrors `printMemberExpression` / `compressMemberExpression`
//! (computed `[]` vs static `.`) in format/minify tooling.
//! Residual pure continue for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Whether member access is computed (`obj[expr]`) vs static (`obj.prop`).
/// TS: `node.computed === true` (or data.computed).
#[must_use]
pub fn is_computed_member(computed: bool) -> bool {
    computed
}

/// Open token for member access: `[` when computed, `.` otherwise.
/// Minify + pretty share the same open tokens for this fragment.
#[must_use]
pub fn member_open(computed: bool) -> &'static str {
    if computed {
        "["
    } else {
        "."
    }
}

/// Close token for member access: `]` when computed, empty otherwise.
#[must_use]
pub fn member_close(computed: bool) -> &'static str {
    if computed {
        "]"
    } else {
        ""
    }
}

/// Emit open+close pair for a property key fragment (key itself is caller-owned).
/// Returns `(open, close)` for the pure decision surface.
#[must_use]
pub fn member_brackets(computed: bool) -> (&'static str, &'static str) {
    (member_open(computed), member_close(computed))
}

/// Optional chaining marker (`?.` / `?.[`) — pure presence.
/// TS optional-chain printers write `?.` before static or computed access.
#[must_use]
pub fn optional_chain_open(optional: bool, computed: bool) -> &'static str {
    match (optional, computed) {
        (true, true) => "?.[",
        (true, false) => "?.",
        (false, true) => "[",
        (false, false) => ".",
    }
}

/// Close for optional/computed combo (only computed needs `]`).
#[must_use]
pub fn optional_chain_close(computed: bool) -> &'static str {
    member_close(computed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computed_vs_static() {
        assert!(is_computed_member(true));
        assert!(!is_computed_member(false));
        assert_eq!(member_open(true), "[");
        assert_eq!(member_open(false), ".");
        assert_eq!(member_close(true), "]");
        assert_eq!(member_close(false), "");
        assert_eq!(member_brackets(true), ("[", "]"));
        assert_eq!(member_brackets(false), (".", ""));
    }

    #[test]
    fn optional_chain_tokens() {
        assert_eq!(optional_chain_open(true, false), "?.");
        assert_eq!(optional_chain_open(true, true), "?.[");
        assert_eq!(optional_chain_open(false, false), ".");
        assert_eq!(optional_chain_open(false, true), "[");
        assert_eq!(optional_chain_close(true), "]");
        assert_eq!(optional_chain_close(false), "");
    }
}

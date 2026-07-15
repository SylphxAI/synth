//! Pure lint rule activation + diagnostic severity resolution —
//! mirrors `packages/synth-lint/src/linter.ts#getActiveRules` / executeRule severity.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full linter engine, NO authority_rust / ts_deleted.

/// Whether a rule is enabled given config entry and rule default.
/// TS: `configValue === true || (configValue !== false && rule.enabled !== false)`
///
/// `config_enabled`:
/// - `Some(true)`  → force on
/// - `Some(false)` → force off
/// - `None`        → fall back to rule default (`rule_enabled_default`)
///
/// Note: string severities in config are handled separately via
/// [`resolve_diagnostic_severity`]; this function only models boolean enable flags.
#[must_use]
pub fn is_rule_enabled(config_enabled: Option<bool>, rule_enabled_default: bool) -> bool {
    match config_enabled {
        Some(true) => true,
        Some(false) => false,
        None => rule_enabled_default,
    }
}

/// Whether a rule applies to a language (TS: empty languages = all).
#[must_use]
pub fn language_applies(rule_languages: &[&str], language: &str) -> bool {
    if rule_languages.is_empty() {
        return true;
    }
    rule_languages.contains(&language)
}

/// Whether config language filter allows this language (TS: empty = all).
#[must_use]
pub fn config_language_allows(config_languages: &[&str], language: &str) -> bool {
    if config_languages.is_empty() {
        return true;
    }
    config_languages.contains(&language)
}

/// Combined pure activation check used by residual rule kernel.
#[must_use]
pub fn rule_is_active(
    config_enabled: Option<bool>,
    rule_enabled_default: bool,
    rule_languages: &[&str],
    config_languages: &[&str],
    language: &str,
) -> bool {
    is_rule_enabled(config_enabled, rule_enabled_default)
        && language_applies(rule_languages, language)
        && config_language_allows(config_languages, language)
}

/// Resolve diagnostic severity (TS executeRule):
/// `typeof configSeverity === 'string' ? configSeverity : diagnostic.severity || rule.severity`
///
/// - `config_severity_string` takes precedence when present/non-empty
/// - else diagnostic severity if present/non-empty
/// - else rule default severity
#[must_use]
pub fn resolve_diagnostic_severity<'a>(
    config_severity_string: Option<&'a str>,
    diagnostic_severity: Option<&'a str>,
    rule_severity: &'a str,
) -> &'a str {
    if let Some(s) = config_severity_string.filter(|s| !s.is_empty()) {
        return s;
    }
    if let Some(s) = diagnostic_severity.filter(|s| !s.is_empty()) {
        return s;
    }
    rule_severity
}

/// Whether a node type is in the rule's nodeTypes filter (empty = all).
#[must_use]
pub fn node_type_applies(rule_node_types: &[&str], node_type: &str) -> bool {
    if rule_node_types.is_empty() {
        return true;
    }
    rule_node_types.contains(&node_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enablement() {
        assert!(is_rule_enabled(Some(true), false));
        assert!(!is_rule_enabled(Some(false), true));
        assert!(is_rule_enabled(None, true));
        assert!(!is_rule_enabled(None, false));
    }

    #[test]
    fn languages() {
        assert!(language_applies(&[], "javascript"));
        assert!(language_applies(&["javascript", "typescript"], "javascript"));
        assert!(!language_applies(&["python"], "javascript"));
        assert!(config_language_allows(&[], "javascript"));
        assert!(!config_language_allows(&["python"], "javascript"));
        assert!(config_language_allows(&["javascript"], "javascript"));
    }

    #[test]
    fn active_combined() {
        assert!(rule_is_active(None, true, &[], &[], "js"));
        assert!(!rule_is_active(Some(false), true, &[], &[], "js"));
        assert!(!rule_is_active(None, true, &["python"], &[], "js"));
        assert!(!rule_is_active(None, true, &[], &["python"], "js"));
        assert!(rule_is_active(
            Some(true),
            false,
            &["javascript"],
            &["javascript"],
            "javascript"
        ));
    }

    #[test]
    fn severity_resolve() {
        assert_eq!(
            resolve_diagnostic_severity(Some("error"), Some("warning"), "info"),
            "error"
        );
        assert_eq!(
            resolve_diagnostic_severity(None, Some("warning"), "info"),
            "warning"
        );
        assert_eq!(
            resolve_diagnostic_severity(Some(""), Some("warning"), "info"),
            "warning"
        );
        assert_eq!(resolve_diagnostic_severity(None, None, "hint"), "hint");
        assert_eq!(
            resolve_diagnostic_severity(None, Some(""), "hint"),
            "hint"
        );
    }

    #[test]
    fn node_type_filter() {
        assert!(node_type_applies(&[], "IfStatement"));
        assert!(node_type_applies(&["IfStatement", "ForStatement"], "IfStatement"));
        assert!(!node_type_applies(&["IfStatement"], "WhileStatement"));
    }
}

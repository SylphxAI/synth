//! Pure console-call detectors — mirrors
//! `packages/synth-lint/src/rules/no-console.ts#isConsoleCall` pure branches.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full linter engine, NO authority_rust / ts_deleted.

/// CallExpression whose source text starts with `console.` (TS path).
#[must_use]
pub fn is_console_call_source(node_type: &str, source: &str) -> bool {
    node_type == "CallExpression" && source.starts_with("console.")
}

/// MemberExpression whose object name/string is `console`.
#[must_use]
pub fn is_console_member_object(node_type: &str, object_name: Option<&str>) -> bool {
    if node_type != "MemberExpression" {
        return false;
    }
    matches!(object_name, Some("console"))
}

/// Combined pure check used by residual rule kernel.
#[must_use]
pub fn is_console_call(
    node_type: &str,
    source: Option<&str>,
    object_name: Option<&str>,
) -> bool {
    if let Some(src) = source
        && is_console_call_source(node_type, src)
    {
        return true;
    }
    is_console_member_object(node_type, object_name)
}

/// Warning message parity with TS no-console rule.
#[must_use]
pub fn console_message(source: &str) -> String {
    format!("Unexpected console statement: {source}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_source_detection() {
        assert!(is_console_call_source("CallExpression", "console.log(1)"));
        assert!(is_console_call_source("CallExpression", "console.warn('x')"));
        assert!(!is_console_call_source("CallExpression", "logger.log(1)"));
        assert!(!is_console_call_source("Identifier", "console.log(1)"));
    }

    #[test]
    fn member_object_detection() {
        assert!(is_console_member_object("MemberExpression", Some("console")));
        assert!(!is_console_member_object("MemberExpression", Some("logger")));
        assert!(!is_console_member_object("MemberExpression", None));
        assert!(!is_console_member_object("CallExpression", Some("console")));
    }

    #[test]
    fn combined_and_message() {
        assert!(is_console_call(
            "CallExpression",
            Some("console.error(e)"),
            None
        ));
        assert!(is_console_call("MemberExpression", None, Some("console")));
        assert!(!is_console_call("CallExpression", Some("foo()"), None));
        assert_eq!(
            console_message("console.log(x)"),
            "Unexpected console statement: console.log(x)"
        );
    }
}

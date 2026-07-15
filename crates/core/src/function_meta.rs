//! Pure per-function metadata helpers —
//! mirrors `packages/synth-metrics/src/analyzer.ts` getFunctionName / params / LOC.
//! Residual pure deepen for tooling/metrics fragment.
//! AST traversal remains TS; this is the pure data-field + string arithmetic kernel.
//! NO authority_rust / ts_deleted.

/// Resolve function display name (TS getFunctionName).
/// Prefers `data.name`, then nested `data.id.name`, else `"<anonymous>"`.
#[must_use]
pub fn function_name(data_name: Option<&str>, id_name: Option<&str>) -> String {
    if let Some(n) = data_name.filter(|s| !s.is_empty()) {
        return n.to_string();
    }
    if let Some(n) = id_name.filter(|s| !s.is_empty()) {
        return n.to_string();
    }
    "<anonymous>".to_string()
}

/// Parameter count from data arrays (TS getFunctionParams).
/// Prefers `params_len` when `Some`, else `parameters_len`, else 0.
#[must_use]
pub fn function_param_count(params_len: Option<usize>, parameters_len: Option<usize>) -> usize {
    if let Some(n) = params_len {
        return n;
    }
    parameters_len.unwrap_or(0)
}

/// Lines of code for a function source slice (TS getFunctionLOC).
/// Counts `\n` separators + 1 when non-empty; empty → 0.
#[must_use]
pub fn function_loc_from_source(source_slice: &str) -> usize {
    if source_slice.is_empty() {
        return 0;
    }
    source_slice.split('\n').count()
}

/// Extract function source slice from full source + byte offsets (pure range).
/// Returns empty when offsets are inverted or out of range.
#[must_use]
pub fn function_source_slice(source: &str, start_offset: usize, end_offset: usize) -> &str {
    if start_offset > end_offset || end_offset > source.len() {
        return "";
    }
    // Guard UTF-8 char boundaries
    if !source.is_char_boundary(start_offset) || !source.is_char_boundary(end_offset) {
        return "";
    }
    &source[start_offset..end_offset]
}

/// Combined LOC from full source + span offsets.
#[must_use]
pub fn function_loc(source: &str, start_offset: usize, end_offset: usize) -> usize {
    function_loc_from_source(function_source_slice(source, start_offset, end_offset))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(function_name(Some("foo"), Some("id")), "foo");
        assert_eq!(function_name(None, Some("bar")), "bar");
        assert_eq!(function_name(Some(""), Some("bar")), "bar");
        assert_eq!(function_name(None, None), "<anonymous>");
        assert_eq!(function_name(Some(""), Some("")), "<anonymous>");
    }

    #[test]
    fn params() {
        assert_eq!(function_param_count(Some(3), Some(9)), 3);
        assert_eq!(function_param_count(None, Some(2)), 2);
        assert_eq!(function_param_count(None, None), 0);
        assert_eq!(function_param_count(Some(0), Some(5)), 0);
    }

    #[test]
    fn loc_and_slice() {
        assert_eq!(function_loc_from_source(""), 0);
        assert_eq!(function_loc_from_source("x"), 1);
        assert_eq!(function_loc_from_source("a\nb\nc"), 3);
        let src = "function f() {\n  return 1;\n}\n";
        let end = src.len() - 1; // exclude trailing newline
        assert_eq!(function_source_slice(src, 0, end), "function f() {\n  return 1;\n}");
        assert_eq!(function_loc(src, 0, end), 3);
        assert_eq!(function_source_slice(src, 10, 5), "");
        assert_eq!(function_source_slice(src, 0, 999), "");
    }
}

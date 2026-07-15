//! Pure severity filter + counts — mirrors
//! `packages/synth-lint/src/linter.ts#filterBySeverity` and counts aggregation.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full linter engine, NO authority_rust / ts_deleted.

/// Severity ladder (lower index = more severe). Matches TS order.
pub const SEVERITY_ORDER: &[&str] = &["error", "warning", "info", "hint"];

/// Rank of a severity string (0=error … 3=hint). Unknown → None.
#[must_use]
pub fn severity_rank(severity: &str) -> Option<usize> {
    SEVERITY_ORDER.iter().position(|&s| s == severity)
}

/// Keep diagnostic when its severity is at least as severe as min (TS: index <= minIndex).
/// When `min_severity` is None/empty, all pass.
#[must_use]
pub fn passes_min_severity(diagnostic_severity: &str, min_severity: Option<&str>) -> bool {
    let Some(min) = min_severity.filter(|s| !s.is_empty()) else {
        return true;
    };
    match (severity_rank(diagnostic_severity), severity_rank(min)) {
        (Some(d), Some(m)) => d <= m,
        _ => false,
    }
}

/// Filter severities by min threshold; returns kept severities in input order.
#[must_use]
pub fn filter_by_severity<'a>(
    severities: &[&'a str],
    min_severity: Option<&str>,
) -> Vec<&'a str> {
    severities
        .iter()
        .copied()
        .filter(|s| passes_min_severity(s, min_severity))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SeverityCounts {
    pub error: u32,
    pub warning: u32,
    pub info: u32,
    pub hint: u32,
}

/// Count diagnostics by severity (unknown keys ignored).
#[must_use]
pub fn count_by_severity(severities: &[&str]) -> SeverityCounts {
    let mut c = SeverityCounts::default();
    for s in severities {
        match *s {
            "error" => c.error += 1,
            "warning" => c.warning += 1,
            "info" => c.info += 1,
            "hint" => c.hint += 1,
            _ => {}
        }
    }
    c
}

/// Lint success = zero errors (TS: counts.error === 0).
#[must_use]
pub fn lint_success(counts: &SeverityCounts) -> bool {
    counts.error == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_and_filter() {
        assert_eq!(severity_rank("error"), Some(0));
        assert_eq!(severity_rank("hint"), Some(3));
        assert_eq!(severity_rank("unknown"), None);

        assert!(passes_min_severity("error", None));
        assert!(passes_min_severity("error", Some("warning")));
        assert!(passes_min_severity("warning", Some("warning")));
        assert!(!passes_min_severity("info", Some("warning")));
        assert!(!passes_min_severity("hint", Some("error")));

        let kept = filter_by_severity(&["error", "warning", "info", "hint"], Some("warning"));
        assert_eq!(kept, vec!["error", "warning"]);
    }

    #[test]
    fn counts_and_success() {
        let c = count_by_severity(&["error", "warning", "warning", "info", "hint", "bogus"]);
        assert_eq!(
            c,
            SeverityCounts {
                error: 1,
                warning: 2,
                info: 1,
                hint: 1
            }
        );
        assert!(!lint_success(&c));
        assert!(lint_success(&SeverityCounts {
            error: 0,
            warning: 3,
            info: 0,
            hint: 0
        }));
    }
}

//! Pure metrics ratio helpers — residual deepen for tooling/metrics arithmetic.
//! Complements metrics_basic / function_types without AST traversal.
//! NO full MetricsAnalyzer, NO authority_rust / ts_deleted.

/// Comment density = cloc / loc (empty loc → 0.0).
#[must_use]
pub fn comment_density(cloc: usize, loc: usize) -> f64 {
    if loc == 0 {
        0.0
    } else {
        cloc as f64 / loc as f64
    }
}

/// Blank density = blank / loc (empty loc → 0.0).
#[must_use]
pub fn blank_density(blank: usize, loc: usize) -> f64 {
    if loc == 0 {
        0.0
    } else {
        blank as f64 / loc as f64
    }
}

/// Source density = sloc / loc (empty loc → 0.0).
#[must_use]
pub fn source_density(sloc: usize, loc: usize) -> f64 {
    if loc == 0 {
        0.0
    } else {
        sloc as f64 / loc as f64
    }
}

/// Average nodes-per-function (empty functions → 0.0).
#[must_use]
pub fn avg_nodes_per_function(total_nodes: usize, function_count: usize) -> f64 {
    if function_count == 0 {
        0.0
    } else {
        total_nodes as f64 / function_count as f64
    }
}

/// Complexity density = cyclomatic / sloc (empty sloc → 0.0).
#[must_use]
pub fn complexity_density(cyclomatic: u32, sloc: usize) -> f64 {
    if sloc == 0 {
        0.0
    } else {
        f64::from(cyclomatic) / sloc as f64
    }
}

/// Sum of per-function cyclomatic complexities.
#[must_use]
pub fn sum_function_complexity(complexities: &[u32]) -> u32 {
    complexities.iter().fold(0u32, |a, &b| a.saturating_add(b))
}

/// Max of per-function complexities (empty → 0).
#[must_use]
pub fn max_function_complexity(complexities: &[u32]) -> u32 {
    complexities.iter().copied().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn densities() {
        assert!((comment_density(2, 10) - 0.2).abs() < 1e-12);
        assert_eq!(comment_density(1, 0), 0.0);
        assert!((blank_density(3, 10) - 0.3).abs() < 1e-12);
        assert!((source_density(5, 10) - 0.5).abs() < 1e-12);
        assert_eq!(source_density(0, 0), 0.0);
    }

    #[test]
    fn function_aggregates() {
        assert_eq!(avg_nodes_per_function(100, 0), 0.0);
        assert!((avg_nodes_per_function(100, 4) - 25.0).abs() < 1e-12);
        assert!((complexity_density(8, 40) - 0.2).abs() < 1e-12);
        assert_eq!(complexity_density(5, 0), 0.0);
        assert_eq!(sum_function_complexity(&[1, 2, 3]), 6);
        assert_eq!(sum_function_complexity(&[]), 0);
        assert_eq!(max_function_complexity(&[1, 5, 3]), 5);
        assert_eq!(max_function_complexity(&[]), 0);
    }
}

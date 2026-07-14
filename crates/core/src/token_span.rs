//! Pure token span helpers + token-stats formatting — mirrors
//! `packages/synth/src/types/token.ts` and `incremental-tokenizer.ts#formatTokenStats`.
//! FLEET-PRODUCTS-WAVE8 pure residual. NO authority_rust / ts_deleted.

/// Check if position (byte offset) is within inclusive token span [start, end].
#[must_use]
pub fn is_position_in_token(position: u32, start_offset: u32, end_offset: u32) -> bool {
    position >= start_offset && position <= end_offset
}

/// Token range overlap (inclusive ends).
#[must_use]
pub fn token_ranges_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    start1 <= end2 && start2 <= end1
}

/// Binary-search token index covering `offset` in sorted (start,end) spans.
#[must_use]
pub fn find_token_index_at_offset(spans: &[(u32, u32)], offset: u32) -> Option<usize> {
    if spans.is_empty() {
        return None;
    }
    let mut left: isize = 0;
    let mut right: isize = spans.len() as isize - 1;
    while left <= right {
        let mid = ((left + right) / 2) as usize;
        let (start, end) = spans[mid];
        if offset < start {
            right = mid as isize - 1;
        } else if offset > end {
            left = mid as isize + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

/// Tokenizer stats for display formatting.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenizerStats {
    pub total_tokens: u32,
    pub reused_tokens: u32,
    pub new_tokens: u32,
    pub affected_tokens: u32,
    pub time_ms: f64,
    pub reuse_rate: f64,
}

/// Format token stats string (TS: formatTokenStats).
#[must_use]
pub fn format_token_stats(stats: &TokenizerStats) -> String {
    format!(
        "Total: {} tokens, Reused: {} ({:.1}%), New: {}, Affected: {}, Time: {:.2}ms",
        stats.total_tokens,
        stats.reused_tokens,
        stats.reuse_rate * 100.0,
        stats.new_tokens,
        stats.affected_tokens,
        stats.time_ms
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_in_token() {
        assert!(is_position_in_token(5, 0, 10));
        assert!(is_position_in_token(0, 0, 10));
        assert!(is_position_in_token(10, 0, 10));
        assert!(!is_position_in_token(11, 0, 10));
    }

    #[test]
    fn ranges_overlap_inclusive() {
        assert!(token_ranges_overlap(0, 5, 5, 10));
        assert!(!token_ranges_overlap(0, 4, 5, 10));
        assert!(token_ranges_overlap(0, 10, 3, 7));
    }

    #[test]
    fn find_token_binary() {
        let spans = [(0, 3), (4, 7), (8, 12)];
        assert_eq!(find_token_index_at_offset(&spans, 5), Some(1));
        assert_eq!(find_token_index_at_offset(&spans, 0), Some(0));
        assert_eq!(find_token_index_at_offset(&spans, 12), Some(2));
        assert_eq!(find_token_index_at_offset(&spans, 13), None);
        assert_eq!(find_token_index_at_offset(&[], 0), None);
    }

    #[test]
    fn format_stats() {
        let s = TokenizerStats {
            total_tokens: 100,
            reused_tokens: 40,
            new_tokens: 60,
            affected_tokens: 10,
            time_ms: 1.5,
            reuse_rate: 0.4,
        };
        let out = format_token_stats(&s);
        assert!(out.contains("Total: 100 tokens"));
        assert!(out.contains("Reused: 40 (40.0%)"));
        assert!(out.contains("New: 60"));
        assert!(out.contains("Affected: 10"));
        assert!(out.contains("Time: 1.50ms"));
    }
}

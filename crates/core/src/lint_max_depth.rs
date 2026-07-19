//! Pure max-depth lint rule helpers — mirrors
//! `packages/synth-lint/src/rules/max-depth.ts`.
//! Depth is computed from parent-chain length; no AST visitor engine.
//! Pure maximum-depth lint policy.

pub const DEFAULT_MAX_DEPTH: u32 = 4;

/// Whether nesting depth exceeds the configured max.
#[must_use]
pub fn depth_exceeds(depth: u32, max_depth: u32) -> bool {
    depth > max_depth
}

/// Warning message parity with TS rule.
#[must_use]
pub fn max_depth_message(depth: u32, max_depth: u32) -> String {
    format!("Nesting depth of {depth} exceeds maximum allowed depth of {max_depth}")
}

/// Compute depth from a parent-id chain (root has parent None → depth 0).
/// `parent_of` returns parent id for a node id, or None at root.
#[must_use]
pub fn compute_depth_from_parents(start: u32, parent_of: &[(u32, Option<u32>)]) -> u32 {
    let map: std::collections::HashMap<u32, Option<u32>> =
        parent_of.iter().copied().collect();
    let mut depth = 0u32;
    let mut current = Some(start);
    let mut guard = 0u32;
    while let Some(id) = current {
        if guard > 10_000 {
            break;
        }
        guard += 1;
        match map.get(&id).copied().flatten() {
            Some(p) => {
                depth += 1;
                current = Some(p);
            }
            None => break,
        }
    }
    depth
}

/// Evaluate whether a node at `depth` should report under DEFAULT_MAX_DEPTH.
#[must_use]
pub fn should_report_default(depth: u32) -> bool {
    depth_exceeds(depth, DEFAULT_MAX_DEPTH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_rule() {
        assert!(!depth_exceeds(4, 4));
        assert!(depth_exceeds(5, 4));
        assert!(should_report_default(5));
        assert!(!should_report_default(4));
        let msg = max_depth_message(6, 4);
        assert!(msg.contains("6"));
        assert!(msg.contains("4"));
    }

    #[test]
    fn parent_chain() {
        // 0 root, 1→0, 2→1, 3→2
        let parents = vec![
            (0, None),
            (1, Some(0)),
            (2, Some(1)),
            (3, Some(2)),
        ];
        assert_eq!(compute_depth_from_parents(0, &parents), 0);
        assert_eq!(compute_depth_from_parents(1, &parents), 1);
        assert_eq!(compute_depth_from_parents(3, &parents), 3);
    }
}

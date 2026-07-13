//! Pure incremental-edit geometry (BW1 residual deepen for core/incremental-batch).
//!
//! Parity target: pure helpers from `packages/synth/src/incremental.ts`
//! (edit normalization, range overlap, affected-node detection). Re-parse /
//! parser callbacks stay TS until a WASM bridge ships. No I/O.

use crate::position::{Position, Span};
use crate::tree::{NodeId, Tree};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Text edit description (tree-sitter compatible; parity with TS `Edit`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {
    pub start_byte: u32,
    pub old_end_byte: u32,
    pub new_end_byte: u32,
    pub start_position: Position,
    pub old_end_position: Position,
    pub new_end_position: Position,
}

/// Simple edit for common cases (parity with TS `SimpleEdit`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEdit {
    pub start: u32,
    pub old_length: u32,
    pub new_length: u32,
}

/// Range affected by one or more edits (parity with TS `AffectedRange`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRange {
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_position: Position,
    pub end_position: Position,
}

/// Check if two closed-ish byte ranges overlap (TS: `start1 <= end2 && start2 <= end1`).
#[must_use]
pub fn ranges_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    start1 <= end2 && start2 <= end1
}

/// Convert byte offset to Position by scanning `source`.
///
/// Matches TS `IncrementalParser.offsetToPosition`: line/column start at 0
/// (TS incremental path; not 1-indexed document convention elsewhere).
#[must_use]
pub fn offset_to_position(source: &str, offset: u32) -> Position {
    let mut line: u32 = 0;
    let mut column: u32 = 0;
    let limit = offset as usize;
    for (i, ch) in source.char_indices() {
        if i >= limit {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    Position::new(line, column, offset)
}

/// Normalize SimpleEdit → full Edit using `source` for position fill.
#[must_use]
pub fn normalize_simple_edit(source: &str, simple: SimpleEdit) -> Edit {
    Edit {
        start_byte: simple.start,
        old_end_byte: simple.start.saturating_add(simple.old_length),
        new_end_byte: simple.start.saturating_add(simple.new_length),
        start_position: offset_to_position(source, simple.start),
        old_end_position: offset_to_position(source, simple.start.saturating_add(simple.old_length)),
        new_end_position: offset_to_position(source, simple.start.saturating_add(simple.new_length)),
    }
}

/// Find node ids whose span overlaps the edit's old byte range.
#[must_use]
pub fn find_overlapping_nodes(tree: &Tree, edit: &Edit) -> Vec<NodeId> {
    let mut overlapping = Vec::new();
    for node in tree.nodes() {
        let Some(span) = node.span else {
            continue;
        };
        let node_start = span.start.offset;
        let node_end = span.end.offset;
        if ranges_overlap(node_start, node_end, edit.start_byte, edit.old_end_byte) {
            overlapping.push(node.id);
        }
    }
    overlapping
}

/// Walk parents of `node_id` and insert them into `affected` (TS markParentsAsAffected).
pub fn mark_parents_as_affected(tree: &Tree, node_id: NodeId, affected: &mut BTreeSet<NodeId>) {
    let Ok(node) = tree.get_node(node_id) else {
        return;
    };
    let Some(parent) = node.parent else {
        return;
    };
    affected.insert(parent);
    mark_parents_as_affected(tree, parent, affected);
}

/// Find all nodes affected by edits (overlap + parent chain). Sorted ids for stability.
#[must_use]
pub fn find_affected_nodes(tree: &Tree, edits: &[Edit]) -> Vec<NodeId> {
    let mut affected: BTreeSet<NodeId> = BTreeSet::new();
    for edit in edits {
        for id in find_overlapping_nodes(tree, edit) {
            affected.insert(id);
            mark_parents_as_affected(tree, id, &mut affected);
        }
    }
    affected.into_iter().collect()
}

/// Union of edit ranges into a single AffectedRange (TS calculateAffectedRange).
/// Empty edits → zero range at origin.
#[must_use]
pub fn calculate_affected_range(edits: &[Edit]) -> AffectedRange {
    if edits.is_empty() {
        let z = Position::new(0, 0, 0);
        return AffectedRange {
            start_byte: 0,
            end_byte: 0,
            start_position: z,
            end_position: z,
        };
    }
    let mut min_start = u32::MAX;
    let mut max_end = 0u32;
    for edit in edits {
        min_start = min_start.min(edit.start_byte);
        max_end = max_end.max(edit.new_end_byte);
    }
    AffectedRange {
        start_byte: min_start,
        end_byte: max_end,
        start_position: edits[0].start_position,
        end_position: edits[edits.len() - 1].new_end_position,
    }
}

/// First node index whose span contains `offset` (TS findNodeAtOffset → arena index).
#[must_use]
pub fn find_node_at_offset(tree: &Tree, offset: u32) -> Option<NodeId> {
    for node in tree.nodes() {
        let Some(span) = node.span else {
            continue;
        };
        if span.start.offset <= offset && offset <= span.end.offset {
            return Some(node.id);
        }
    }
    None
}

/// Convenience: apply simple edit list → (normalized edits, affected ids, range).
#[must_use]
pub fn plan_edits(tree: &Tree, simple: &[SimpleEdit]) -> (Vec<Edit>, Vec<NodeId>, AffectedRange) {
    let source = tree.source();
    let edits: Vec<Edit> = simple
        .iter()
        .copied()
        .map(|s| normalize_simple_edit(&source, s))
        .collect();
    let affected = find_affected_nodes(tree, &edits);
    let range = calculate_affected_range(&edits);
    (edits, affected, range)
}

/// Helper to attach a span for tests / fixture loaders.
pub fn set_node_span(tree: &mut Tree, id: NodeId, span: Span) {
    if let Ok(node) = tree.get_node_mut(id) {
        node.span = Some(span);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;

    fn sample_spanned_tree() -> Tree {
        // source: "abcd\nefgh"  (9 chars + newline = 9? a b c d \n e f g h = 9)
        // offsets: a0 b1 c2 d3 \n4 e5 f6 g7 h8
        let mut tree = Tree::new("text", "abcd\nefgh");
        let root = tree.root_id();
        let left = tree.add_node(Node::new(0, "left").with_span(Span::from_coords(0, 0, 0, 0, 4, 4)));
        let right = tree.add_node(
            Node::new(0, "right").with_span(Span::from_coords(1, 0, 5, 1, 4, 9)),
        );
        let _ = tree.add_child(root, left);
        let _ = tree.add_child(root, right);
        // root span whole doc
        set_node_span(
            &mut tree,
            root,
            Span::from_coords(0, 0, 0, 1, 4, 9),
        );
        tree
    }

    #[test]
    fn ranges_overlap_basic() {
        assert!(ranges_overlap(0, 5, 3, 8));
        assert!(!ranges_overlap(0, 2, 3, 5));
        assert!(ranges_overlap(0, 5, 5, 10)); // boundary inclusive per TS
    }

    #[test]
    fn offset_to_position_newlines() {
        let src = "abcd\nefgh";
        let p = offset_to_position(src, 6);
        assert_eq!(p.line, 1);
        assert_eq!(p.column, 1); // 'f' is col 1 after e at col 0
        assert_eq!(p.offset, 6);
    }

    #[test]
    fn normalize_and_plan_edits() {
        let tree = sample_spanned_tree();
        let simple = SimpleEdit {
            start: 1,
            old_length: 2,
            new_length: 1,
        };
        let (edits, affected, range) = plan_edits(&tree, &[simple]);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].start_byte, 1);
        assert_eq!(edits[0].old_end_byte, 3);
        assert_eq!(edits[0].new_end_byte, 2);
        // left (0-4) overlaps, root parent marked
        assert!(affected.contains(&0));
        assert!(affected.contains(&1));
        assert!(!affected.contains(&2)); // right starts at 5
        assert_eq!(range.start_byte, 1);
        assert_eq!(range.end_byte, 2);
    }
}

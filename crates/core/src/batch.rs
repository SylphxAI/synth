//! Pure batch-processing helpers (BW1 residual deepen for core/incremental-batch).
//!
//! Parity target: pure grouping/chunking from `packages/synth/src/batch-processor.ts`.
//! Visitor callbacks stay at the call site (no wasm Fn bridge this tick).

use crate::tree::{NodeId, Tree};
use crate::traverse::{collect_ids, TraversalOrder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Default batch size (TS DEFAULT_BATCH_SIZE = 16).
pub const DEFAULT_BATCH_SIZE: usize = 16;

/// Options for batch chunking (parity with TS BatchProcessingOptions pure subset).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchProcessingOptions {
    pub batch_size: usize,
    pub group_by_type: bool,
}

impl Default for BatchProcessingOptions {
    fn default() -> Self {
        Self {
            batch_size: DEFAULT_BATCH_SIZE,
            group_by_type: true,
        }
    }
}

/// Group node ids by their type string (sorted keys for stability).
#[must_use]
pub fn group_node_ids_by_type(tree: &Tree, node_ids: &[NodeId]) -> BTreeMap<String, Vec<NodeId>> {
    let mut grouped: BTreeMap<String, Vec<NodeId>> = BTreeMap::new();
    for &id in node_ids {
        if let Ok(node) = tree.get_node(id) {
            grouped
                .entry(node.node_type.clone())
                .or_default()
                .push(id);
        }
    }
    grouped
}

/// Split a slice into batches of `batch_size` (last may be shorter).
#[must_use]
pub fn chunk_ids(ids: &[NodeId], batch_size: usize) -> Vec<Vec<NodeId>> {
    let size = batch_size.max(1);
    let mut out = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        let end = (i + size).min(ids.len());
        out.push(ids[i..end].to_vec());
        i = end;
    }
    out
}

/// Pre-order collect of all node ids (parity with batchTraverse collect phase).
#[must_use]
pub fn batch_preorder_ids(tree: &Tree) -> Vec<NodeId> {
    collect_ids(tree, tree.root_id(), TraversalOrder::PreOrder)
}

/// Pure plan: either type-grouped batches or flat batches of ids.
/// Returns ordered list of (optional type key, batch of ids).
#[must_use]
pub fn plan_batches(
    tree: &Tree,
    node_ids: &[NodeId],
    options: BatchProcessingOptions,
) -> Vec<(Option<String>, Vec<NodeId>)> {
    let size = options.batch_size.max(1);
    if options.group_by_type {
        let grouped = group_node_ids_by_type(tree, node_ids);
        let mut plans = Vec::new();
        for (ty, ids) in grouped {
            for batch in chunk_ids(&ids, size) {
                plans.push((Some(ty.clone()), batch));
            }
        }
        plans
    } else {
        chunk_ids(node_ids, size)
            .into_iter()
            .map(|b| (None, b))
            .collect()
    }
}

/// Select node ids whose type equals `type_name` via batched preorder plan (pure filter).
#[must_use]
pub fn batch_select_by_type(tree: &Tree, type_name: &str) -> Vec<NodeId> {
    batch_preorder_ids(tree)
        .into_iter()
        .filter(|&id| {
            tree.get_node(id)
                .map(|n| n.node_type == type_name)
                .unwrap_or(false)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;

    fn sample() -> Tree {
        let mut tree = Tree::new("markdown", "# hi");
        let root = tree.root_id();
        let h = tree.add_node(Node::new(0, "heading"));
        let p = tree.add_node(Node::new(0, "paragraph"));
        let t = tree.add_node(Node::new(0, "text"));
        let _ = tree.add_child(root, h);
        let _ = tree.add_child(root, p);
        let _ = tree.add_child(p, t);
        tree
    }

    #[test]
    fn group_and_chunk() {
        let tree = sample();
        let ids = batch_preorder_ids(&tree);
        assert_eq!(ids, vec![0, 1, 2, 3]);
        let grouped = group_node_ids_by_type(&tree, &ids);
        assert_eq!(grouped.get("text").unwrap(), &vec![3]);
        let chunks = chunk_ids(&ids, 2);
        assert_eq!(chunks, vec![vec![0, 1], vec![2, 3]]);
    }

    #[test]
    fn plan_batches_grouped() {
        let tree = sample();
        let ids = batch_preorder_ids(&tree);
        let plans = plan_batches(
            &tree,
            &ids,
            BatchProcessingOptions {
                batch_size: 1,
                group_by_type: true,
            },
        );
        // BTreeMap order: heading, paragraph, root, text
        assert_eq!(plans.len(), 4);
        assert_eq!(batch_select_by_type(&tree, "heading"), vec![1]);
    }
}

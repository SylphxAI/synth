//! Pure AST traversal orders (ADR-168 bulk deepen for core/ast-traverse-query).
//!
//! Parity target: `src/core/traverse.ts` id-collection semantics (no visitor
//! callbacks — those stay TS until a higher-level WASM bridge ships).

use crate::tree::{NodeId, Tree};

/// Traversal order (parity with TS `TraversalOrder`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalOrder {
    PreOrder,
    PostOrder,
    BreadthFirst,
}

/// Collect node ids under `start` (including `start`) in the given order.
#[must_use]
pub fn collect_ids(tree: &Tree, start: NodeId, order: TraversalOrder) -> Vec<NodeId> {
    let mut out = Vec::new();
    match order {
        TraversalOrder::PreOrder => collect_pre(tree, start, &mut out),
        TraversalOrder::PostOrder => collect_post(tree, start, &mut out),
        TraversalOrder::BreadthFirst => collect_bfs(tree, start, &mut out),
    }
    out
}

/// Pre-order walk of the whole tree from root.
#[must_use]
pub fn pre_order(tree: &Tree) -> Vec<NodeId> {
    collect_ids(tree, tree.root_id(), TraversalOrder::PreOrder)
}

/// Post-order walk of the whole tree from root.
#[must_use]
pub fn post_order(tree: &Tree) -> Vec<NodeId> {
    collect_ids(tree, tree.root_id(), TraversalOrder::PostOrder)
}

/// Breadth-first walk of the whole tree from root.
#[must_use]
pub fn breadth_first(tree: &Tree) -> Vec<NodeId> {
    collect_ids(tree, tree.root_id(), TraversalOrder::BreadthFirst)
}

/// Collect ids with max depth bound (depth of `start` = 0). Nodes deeper than
/// `max_depth` are omitted (parity with TS `options.maxDepth`).
#[must_use]
pub fn collect_ids_max_depth(
    tree: &Tree,
    start: NodeId,
    order: TraversalOrder,
    max_depth: usize,
) -> Vec<NodeId> {
    let mut out = Vec::new();
    match order {
        TraversalOrder::PreOrder => collect_pre_depth(tree, start, 0, max_depth, &mut out),
        TraversalOrder::PostOrder => collect_post_depth(tree, start, 0, max_depth, &mut out),
        TraversalOrder::BreadthFirst => collect_bfs_depth(tree, start, max_depth, &mut out),
    }
    out
}

fn children_of(tree: &Tree, id: NodeId) -> Vec<NodeId> {
    tree.get_node(id)
        .map(|n| n.children.clone())
        .unwrap_or_default()
}

fn collect_pre(tree: &Tree, id: NodeId, out: &mut Vec<NodeId>) {
    if tree.get_node(id).is_err() {
        return;
    }
    out.push(id);
    for child in children_of(tree, id) {
        collect_pre(tree, child, out);
    }
}

fn collect_post(tree: &Tree, id: NodeId, out: &mut Vec<NodeId>) {
    if tree.get_node(id).is_err() {
        return;
    }
    for child in children_of(tree, id) {
        collect_post(tree, child, out);
    }
    out.push(id);
}

fn collect_bfs(tree: &Tree, start: NodeId, out: &mut Vec<NodeId>) {
    if tree.get_node(start).is_err() {
        return;
    }
    let mut queue = std::collections::VecDeque::from([start]);
    while let Some(id) = queue.pop_front() {
        out.push(id);
        for child in children_of(tree, id) {
            queue.push_back(child);
        }
    }
}

fn collect_pre_depth(
    tree: &Tree,
    id: NodeId,
    depth: usize,
    max_depth: usize,
    out: &mut Vec<NodeId>,
) {
    if depth > max_depth || tree.get_node(id).is_err() {
        return;
    }
    out.push(id);
    for child in children_of(tree, id) {
        collect_pre_depth(tree, child, depth + 1, max_depth, out);
    }
}

fn collect_post_depth(
    tree: &Tree,
    id: NodeId,
    depth: usize,
    max_depth: usize,
    out: &mut Vec<NodeId>,
) {
    if depth > max_depth || tree.get_node(id).is_err() {
        return;
    }
    for child in children_of(tree, id) {
        collect_post_depth(tree, child, depth + 1, max_depth, out);
    }
    out.push(id);
}

fn collect_bfs_depth(tree: &Tree, start: NodeId, max_depth: usize, out: &mut Vec<NodeId>) {
    if tree.get_node(start).is_err() {
        return;
    }
    let mut queue = std::collections::VecDeque::from([(start, 0usize)]);
    while let Some((id, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        out.push(id);
        for child in children_of(tree, id) {
            queue.push_back((child, depth + 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{Node, Tree};

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
    fn pre_order_root_first() {
        let tree = sample();
        let ids = pre_order(&tree);
        assert_eq!(ids[0], tree.root_id());
        assert_eq!(ids.len(), 4);
    }

    #[test]
    fn post_order_root_last() {
        let tree = sample();
        let ids = post_order(&tree);
        assert_eq!(*ids.last().unwrap(), tree.root_id());
        assert_eq!(ids.len(), 4);
    }

    #[test]
    fn bfs_levels() {
        let tree = sample();
        let ids = breadth_first(&tree);
        assert_eq!(ids[0], tree.root_id());
        // children of root appear before grandchild
        let p = tree
            .nodes()
            .iter()
            .find(|n| n.node_type == "paragraph")
            .unwrap()
            .id;
        let t = tree
            .nodes()
            .iter()
            .find(|n| n.node_type == "text")
            .unwrap()
            .id;
        let pi = ids.iter().position(|&x| x == p).unwrap();
        let ti = ids.iter().position(|&x| x == t).unwrap();
        assert!(pi < ti);
    }

    #[test]
    fn max_depth_zero_is_root_only() {
        let tree = sample();
        let ids = collect_ids_max_depth(&tree, tree.root_id(), TraversalOrder::PreOrder, 0);
        assert_eq!(ids, vec![tree.root_id()]);
    }
}

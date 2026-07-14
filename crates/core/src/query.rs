//! Pure AST query helpers (ADR-168 rust_impl for core/ast-traverse-query).

use crate::tree::{Node, NodeId, Tree};

/// Collect all node ids with the given `node_type` (pre-order DFS).
#[must_use]
pub fn find_by_type(tree: &Tree, node_type: &str) -> Vec<NodeId> {
    let mut out = Vec::new();
    walk_collect(tree, tree.root_id(), &mut |node| {
        if node.node_type == node_type {
            out.push(node.id);
        }
    });
    out
}

/// Collect descendant ids of `start` (not including start), pre-order.
#[must_use]
pub fn descendants(tree: &Tree, start: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    let Ok(node) = tree.get_node(start) else {
        return out;
    };
    for child in &node.children {
        collect_subtree(tree, *child, &mut out);
    }
    out
}

/// Depth of a node from the root (root = 0). Returns None if node missing.
#[must_use]
pub fn depth(tree: &Tree, id: NodeId) -> Option<usize> {
    let mut d = 0usize;
    let mut cur = id;
    loop {
        let node = tree.get_node(cur).ok()?;
        match node.parent {
            Some(p) => {
                d += 1;
                cur = p;
            }
            None => return Some(d),
        }
    }
}

fn collect_subtree(tree: &Tree, id: NodeId, out: &mut Vec<NodeId>) {
    out.push(id);
    if let Ok(node) = tree.get_node(id) {
        for child in &node.children.clone() {
            collect_subtree(tree, *child, out);
        }
    }
}

fn walk_collect(tree: &Tree, id: NodeId, f: &mut dyn FnMut(&Node)) {
    if let Ok(node) = tree.get_node(id) {
        // clone children to avoid borrow issues
        let children = node.children.clone();
        f(node);
        for child in children {
            walk_collect(tree, child, f);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{Node, Tree};

    fn sample_tree() -> Tree {
        let mut tree = Tree::new("markdown", "# hi");
        let root = tree.root_id();
        // ensure root typed as document if empty tree uses placeholder
        let h = tree.add_node(Node::new(0, "heading")); // id reassigned by add_node
        let p = tree.add_node(Node::new(0, "paragraph"));
        let t = tree.add_node(Node::new(0, "text"));
        let _ = tree.add_child(root, h);
        let _ = tree.add_child(root, p);
        let _ = tree.add_child(p, t);
        tree
    }

    #[test]
    fn finds_by_type() {
        let tree = sample_tree();
        assert_eq!(find_by_type(&tree, "text").len(), 1);
        assert_eq!(find_by_type(&tree, "heading").len(), 1);
    }

    #[test]
    fn descendants_of_root() {
        let tree = sample_tree();
        let d = descendants(&tree, tree.root_id());
        assert_eq!(d.len(), 3);
    }

    #[test]
    fn depth_values() {
        let tree = sample_tree();
        assert_eq!(depth(&tree, tree.root_id()), Some(0));
        let texts = find_by_type(&tree, "text");
        assert_eq!(depth(&tree, texts[0]), Some(2));
    }
}

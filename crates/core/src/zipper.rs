//! Pure zipper navigation over arena trees (ADR-168 bulk for core/ast-traverse-query).
//!
//! Parity target: `src/core/zipper.ts` navigation helpers (focus / up / down / left / right).
//! Mutating edit operations remain TS until a full WASM edit surface lands.

use crate::tree::{NodeId, Tree};

/// Breadcrumb for one step from root toward focus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crumb {
    pub parent_id: NodeId,
    pub index: usize,
    pub left: Vec<NodeId>,
    pub right: Vec<NodeId>,
}

/// Zipper focused on a single node with a path back to the root.
#[derive(Debug, Clone)]
pub struct Zipper {
    pub focus: NodeId,
    pub path: Vec<Crumb>,
}

/// Create a zipper focused on the tree root.
#[must_use]
pub fn create_zipper(tree: &Tree) -> Zipper {
    Zipper {
        focus: tree.root_id(),
        path: Vec::new(),
    }
}

/// Create a zipper focused on `node_id`, or `None` if the node is missing / disconnected.
#[must_use]
pub fn create_zipper_at(tree: &Tree, node_id: NodeId) -> Option<Zipper> {
    let node = tree.get_node(node_id).ok()?;
    let mut path: Vec<Crumb> = Vec::new();
    let mut current_id = node_id;
    let mut current_parent = node.parent;

    while let Some(parent_id) = current_parent {
        let parent = tree.get_node(parent_id).ok()?;
        let index = parent.children.iter().position(|&c| c == current_id)?;
        path.insert(
            0,
            Crumb {
                parent_id,
                index,
                left: parent.children[..index].to_vec(),
                right: parent.children[index + 1..].to_vec(),
            },
        );
        current_id = parent_id;
        current_parent = parent.parent;
    }

    Some(Zipper {
        focus: node_id,
        path,
    })
}

/// Move focus to the first child, if any.
#[must_use]
pub fn down(tree: &Tree, z: &Zipper) -> Option<Zipper> {
    let node = tree.get_node(z.focus).ok()?;
    let first = *node.children.first()?;
    let mut path = z.path.clone();
    path.push(Crumb {
        parent_id: z.focus,
        index: 0,
        left: Vec::new(),
        right: node.children[1..].to_vec(),
    });
    Some(Zipper { focus: first, path })
}

/// Move focus to the parent, if any.
#[must_use]
pub fn up(z: &Zipper) -> Option<Zipper> {
    let mut path = z.path.clone();
    let crumb = path.pop()?;
    Some(Zipper {
        focus: crumb.parent_id,
        path,
    })
}

/// Move focus to the previous sibling, if any.
#[must_use]
pub fn left(z: &Zipper) -> Option<Zipper> {
    let mut path = z.path.clone();
    let crumb = path.last_mut()?;
    let prev = *crumb.left.last()?;
    crumb.left.pop();
    let old_focus = z.focus;
    crumb.right.insert(0, old_focus);
    crumb.index = crumb.index.saturating_sub(1);
    Some(Zipper {
        focus: prev,
        path,
    })
}

/// Move focus to the next sibling, if any.
#[must_use]
pub fn right(z: &Zipper) -> Option<Zipper> {
    let mut path = z.path.clone();
    let crumb = path.last_mut()?;
    let next = *crumb.right.first()?;
    crumb.right.remove(0);
    let old_focus = z.focus;
    crumb.left.push(old_focus);
    crumb.index += 1;
    Some(Zipper {
        focus: next,
        path,
    })
}

/// Depth of the focus (root = 0).
#[must_use]
pub fn zipper_depth(z: &Zipper) -> usize {
    z.path.len()
}

/// True when focus is the root.
#[must_use]
pub fn is_root(z: &Zipper) -> bool {
    z.path.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{Node, Tree};

    fn sample() -> Tree {
        let mut tree = Tree::new("markdown", "x");
        let root = tree.root_id();
        let a = tree.add_node(Node::new(0, "a"));
        let b = tree.add_node(Node::new(0, "b"));
        let c = tree.add_node(Node::new(0, "c"));
        let _ = tree.add_child(root, a);
        let _ = tree.add_child(root, b);
        let _ = tree.add_child(a, c);
        tree
    }

    #[test]
    fn root_zipper() {
        let tree = sample();
        let z = create_zipper(&tree);
        assert!(is_root(&z));
        assert_eq!(zipper_depth(&z), 0);
    }

    #[test]
    fn down_up_roundtrip() {
        let tree = sample();
        let z = create_zipper(&tree);
        let child = down(&tree, &z).expect("has child");
        assert!(!is_root(&child));
        let back = up(&child).expect("up");
        assert_eq!(back.focus, tree.root_id());
    }

    #[test]
    fn left_right_siblings() {
        let tree = sample();
        let z = create_zipper(&tree);
        let first = down(&tree, &z).unwrap();
        let second = right(&first).unwrap();
        let back = left(&second).unwrap();
        assert_eq!(back.focus, first.focus);
    }

    #[test]
    fn create_at_nested() {
        let tree = sample();
        let c = tree
            .nodes()
            .iter()
            .find(|n| n.node_type == "c")
            .unwrap()
            .id;
        let z = create_zipper_at(&tree, c).unwrap();
        assert_eq!(z.focus, c);
        assert_eq!(zipper_depth(&z), 2);
    }
}

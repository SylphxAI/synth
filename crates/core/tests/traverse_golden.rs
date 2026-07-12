//! Golden fixture: traverse / query pure residual (BW1).

use serde::Deserialize;
use std::path::PathBuf;
use synth_wasm_core::{
    collect_ids, collect_ids_max_depth, depth, descendants, find_by_type, Node, TraversalOrder, Tree,
};

#[derive(Debug, Deserialize)]
struct Fixture {
    language: String,
    #[serde(rename = "sourceText")]
    source_text: String,
    nodes: Vec<NodeSpec>,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct NodeSpec {
    id: u32,
    #[serde(rename = "type")]
    node_type: String,
    parent: Option<u32>,
    children: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    name: String,
    order: Option<String>,
    max_depth: Option<usize>,
    query: Option<String>,
    #[serde(rename = "type")]
    type_name: Option<String>,
    start: Option<u32>,
    expected_ids: Option<Vec<u32>>,
    expected_depth: Option<usize>,
}

fn load_tree(fx: &Fixture) -> Tree {
    // Build tree matching fixture ids by adding nodes in order after root.
    let mut tree = Tree::new(&fx.language, &fx.source_text);
    // Root is always 0 from Tree::new — verify fixture root matches
    assert_eq!(fx.nodes[0].id, 0);
    // Add remaining nodes
    for spec in fx.nodes.iter().skip(1) {
        let id = tree.add_node(Node::new(0, &spec.node_type));
        assert_eq!(id, spec.id, "fixture id order must match arena order");
    }
    // Wire parent/children from fixture (skip root children re-add if already empty)
    for spec in &fx.nodes {
        if let Some(parent) = spec.parent {
            // child already has parent set via add_child below
            let _ = parent;
        }
        for &child in &spec.children {
            tree.add_child(spec.id, child).expect("child link");
        }
    }
    tree
}

#[test]
fn traverse_golden_fixture() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/traverse_golden.json");
    let raw = std::fs::read_to_string(&path).expect("read fixture");
    let fx: Fixture = serde_json::from_str(&raw).expect("parse fixture");
    let tree = load_tree(&fx);

    for case in &fx.cases {
        if let Some(query) = &case.query {
            match query.as_str() {
                "find_by_type" => {
                    let ids = find_by_type(&tree, case.type_name.as_deref().unwrap());
                    assert_eq!(ids, case.expected_ids.clone().unwrap(), "{}", case.name);
                }
                "descendants" => {
                    let ids = descendants(&tree, case.start.unwrap());
                    assert_eq!(ids, case.expected_ids.clone().unwrap(), "{}", case.name);
                }
                "depth" => {
                    let d = depth(&tree, case.start.unwrap());
                    assert_eq!(d, case.expected_depth, "{}", case.name);
                }
                other => panic!("unknown query {other}"),
            }
            continue;
        }

        let order = match case.order.as_deref() {
            Some("pre") => TraversalOrder::PreOrder,
            Some("post") => TraversalOrder::PostOrder,
            Some("bfs") => TraversalOrder::BreadthFirst,
            other => panic!("unknown order {other:?}"),
        };
        let ids = if let Some(md) = case.max_depth {
            collect_ids_max_depth(&tree, tree.root_id(), order, md)
        } else {
            collect_ids(&tree, tree.root_id(), order)
        };
        assert_eq!(ids, case.expected_ids.clone().unwrap(), "{}", case.name);
    }
}

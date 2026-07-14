//! Golden fixture: incremental + batch pure residual (BW1 deepen).

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;
use synth_wasm_core::{
    batch_preorder_ids, calculate_affected_range, chunk_ids, find_affected_nodes,
    find_node_at_offset, group_node_ids_by_type, normalize_simple_edit, offset_to_position,
    plan_batches, plan_edits, ranges_overlap, set_node_span, BatchProcessingOptions, Node,
    Position, SimpleEdit, Span, Tree,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    language: String,
    source_text: String,
    nodes: Vec<NodeSpec>,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct NodeSpec {
    id: u32,
    #[serde(rename = "type")]
    node_type: String,
    #[allow(dead_code)]
    parent: Option<u32>,
    children: Vec<u32>,
    span: Option<SpanSpec>,
}

#[derive(Debug, Deserialize)]
struct SpanSpec {
    start: PosSpec,
    end: PosSpec,
}

#[derive(Debug, Deserialize)]
struct PosSpec {
    line: u32,
    column: u32,
    offset: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    name: String,
    kind: String,
    a: Option<[u32; 2]>,
    b: Option<[u32; 2]>,
    expected: Option<bool>,
    offset: Option<u32>,
    expected_position: Option<PosSpec>,
    simple_edits: Option<Vec<SimpleEditSpec>>,
    expected_affected_ids: Option<Vec<u32>>,
    expected_range: Option<RangeSpec>,
    expected_id: Option<u32>,
    expected_ids: Option<Vec<u32>>,
    #[serde(rename = "type")]
    type_name: Option<String>,
    batch_size: Option<usize>,
    expected_batches: Option<Vec<Vec<u32>>>,
    group_by_type: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SimpleEditSpec {
    start: u32,
    old_length: u32,
    new_length: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RangeSpec {
    start_byte: u32,
    end_byte: u32,
}

fn load_tree(fx: &Fixture) -> Tree {
    let mut tree = Tree::new(&fx.language, &fx.source_text);
    assert_eq!(fx.nodes[0].id, 0);
    for spec in fx.nodes.iter().skip(1) {
        let id = tree.add_node(Node::new(0, &spec.node_type));
        assert_eq!(id, spec.id);
    }
    for spec in &fx.nodes {
        for &child in &spec.children {
            tree.add_child(spec.id, child).expect("child");
        }
        if let Some(span) = &spec.span {
            set_node_span(
                &mut tree,
                spec.id,
                Span::new(
                    Position::new(span.start.line, span.start.column, span.start.offset),
                    Position::new(span.end.line, span.end.column, span.end.offset),
                ),
            );
        }
    }
    tree
}

#[test]
fn incremental_batch_golden_fixture() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/incremental_batch_golden.json");
    let raw = std::fs::read_to_string(&path).expect("read fixture");
    let fx: Fixture = serde_json::from_str(&raw).expect("parse fixture");
    let tree = load_tree(&fx);

    for case in &fx.cases {
        match case.kind.as_str() {
            "ranges_overlap" => {
                let a = case.a.unwrap();
                let b = case.b.unwrap();
                let got = ranges_overlap(a[0], a[1], b[0], b[1]);
                assert_eq!(got, case.expected.unwrap(), "{}", case.name);
            }
            "offset_to_position" => {
                let p = offset_to_position(&fx.source_text, case.offset.unwrap());
                let exp = case.expected_position.as_ref().unwrap();
                assert_eq!(p.line, exp.line, "{}", case.name);
                assert_eq!(p.column, exp.column, "{}", case.name);
                assert_eq!(p.offset, exp.offset, "{}", case.name);
            }
            "plan_edits" => {
                let simple: Vec<SimpleEdit> = case
                    .simple_edits
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| SimpleEdit {
                        start: s.start,
                        old_length: s.old_length,
                        new_length: s.new_length,
                    })
                    .collect();
                let (_edits, affected, range) = plan_edits(&tree, &simple);
                assert_eq!(
                    affected,
                    case.expected_affected_ids.clone().unwrap(),
                    "{}",
                    case.name
                );
                let er = case.expected_range.as_ref().unwrap();
                assert_eq!(range.start_byte, er.start_byte, "{}", case.name);
                assert_eq!(range.end_byte, er.end_byte, "{}", case.name);
                // also exercise discrete helpers
                let edits: Vec<_> = simple
                    .iter()
                    .copied()
                    .map(|s| normalize_simple_edit(&fx.source_text, s))
                    .collect();
                assert_eq!(find_affected_nodes(&tree, &edits), affected);
                let r2 = calculate_affected_range(&edits);
                assert_eq!(r2.start_byte, range.start_byte);
            }
            "find_node_at_offset" => {
                let id = find_node_at_offset(&tree, case.offset.unwrap());
                assert_eq!(id, case.expected_id, "{}", case.name);
            }
            "batch_preorder" => {
                let ids = batch_preorder_ids(&tree);
                assert_eq!(ids, case.expected_ids.clone().unwrap(), "{}", case.name);
            }
            "group_by_type" => {
                let ids = batch_preorder_ids(&tree);
                let grouped: BTreeMap<_, _> = group_node_ids_by_type(&tree, &ids);
                let got = grouped
                    .get(case.type_name.as_deref().unwrap())
                    .cloned()
                    .unwrap_or_default();
                assert_eq!(got, case.expected_ids.clone().unwrap(), "{}", case.name);
            }
            "chunk" => {
                let ids = batch_preorder_ids(&tree);
                let batches = chunk_ids(&ids, case.batch_size.unwrap());
                assert_eq!(
                    batches,
                    case.expected_batches.clone().unwrap(),
                    "{}",
                    case.name
                );
            }
            "find_overlapping_only" => {
                let simple: Vec<SimpleEdit> = case
                    .simple_edits
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| SimpleEdit {
                        start: s.start,
                        old_length: s.old_length,
                        new_length: s.new_length,
                    })
                    .collect();
                let (_edits, affected, _range) = plan_edits(&tree, &simple);
                assert_eq!(
                    affected,
                    case.expected_affected_ids.clone().unwrap(),
                    "{}",
                    case.name
                );
            }
            "plan_batches" => {
                let ids = batch_preorder_ids(&tree);
                let plans = plan_batches(
                    &tree,
                    &ids,
                    BatchProcessingOptions {
                        batch_size: case.batch_size.unwrap(),
                        group_by_type: case.group_by_type.unwrap_or(true),
                    },
                );
                let batches: Vec<Vec<u32>> = plans.into_iter().map(|(_, b)| b).collect();
                assert_eq!(
                    batches,
                    case.expected_batches.clone().unwrap(),
                    "{}",
                    case.name
                );
            }
            other => panic!("unknown kind {other}"),
        }
    }
}

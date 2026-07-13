//! Synth WASM Core
//!
//! Core types and utilities shared across all Synth WASM parsers.
//! Provides Tree structure compatible with the TypeScript @sylphx/synth package.

mod tree;
mod query;
mod error;
mod position;
mod traverse;
mod zipper;
mod incremental;
mod batch;

pub use tree::*;
pub use query::{depth, descendants, find_by_type};
pub use error::*;
pub use position::*;
pub use traverse::{
    breadth_first, collect_ids, collect_ids_max_depth, post_order, pre_order, TraversalOrder,
};
pub use zipper::{
    create_zipper, create_zipper_at, down, is_root, left, right, up, zipper_depth, Crumb, Zipper,
};
pub use incremental::{
    calculate_affected_range, find_affected_nodes, find_node_at_offset, find_overlapping_nodes,
    mark_parents_as_affected, normalize_simple_edit, offset_to_position, plan_edits, ranges_overlap,
    set_node_span, AffectedRange, Edit, SimpleEdit,
};
pub use batch::{
    batch_preorder_ids, batch_select_by_type, chunk_ids, group_node_ids_by_type, plan_batches,
    BatchProcessingOptions, DEFAULT_BATCH_SIZE,
};

use wasm_bindgen::prelude::*;

/// Initialize the WASM module (called automatically)
#[wasm_bindgen(start)]
pub fn init() {
    // Panic hook can be added later if needed
}

/// Get the version of the WASM core
#[wasm_bindgen(js_name = coreVersion)]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

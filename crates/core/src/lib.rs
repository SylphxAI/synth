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
mod metrics_basic;
mod minify_savings;
mod mangle;
mod halstead_math;
mod complexity_types;
mod detect_language;
mod token_span;
mod lint_max_depth;
mod format_indent;
mod function_types;
mod lint_block_types;
mod lint_console;
mod lint_severity;
mod format_style;
mod metrics_ratio;
mod halstead_classify;
mod lint_rule_active;
mod function_meta;
mod format_emit;
mod stmt_emit;
mod literal_emit;
mod assign_sep;
mod quote_props;
mod arrow_emit;
mod fn_sig_emit;
mod node_fallback;
mod binding_id;

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
pub use metrics_basic::{analyze_basic_loc, classify_line, BasicLocMetrics, LineKind};
pub use minify_savings::{compression_ratio, savings, Savings};
pub use mangle::{generate_mangled_name, NameMangler};
pub use halstead_math::{
    calculate_maintainability, compute_halstead, HalsteadMetrics, MaintainabilityLevel,
    MaintainabilityMetrics,
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

pub use detect_language::{common_prefix_len, common_suffix_len, detect_language};
pub use token_span::{find_token_index_at_offset, format_token_stats, is_position_in_token, token_ranges_overlap, TokenizerStats};
pub use complexity_types::{cognitive_decision_weight, cyclomatic_from_decisions, is_decision_count_type, is_decision_node_type, is_nesting_node_type, next_nesting};
pub use lint_max_depth::{compute_depth_from_parents, depth_exceeds, max_depth_message, should_report_default, DEFAULT_MAX_DEPTH};
pub use format_indent::{end_of_line, indent_string, quote_string_literal};
pub use function_types::{avg_depth_of, is_function_node_type, max_depth_of};
pub use lint_block_types::{empty_block_message, is_block_node, is_empty_block};
pub use lint_console::{console_message, is_console_call, is_console_call_source, is_console_member_object};
pub use lint_severity::{
    count_by_severity, filter_by_severity, lint_success, passes_min_severity, severity_rank,
    SeverityCounts, SEVERITY_ORDER,
};
pub use format_style::{
    arrow_needs_parens, binary_op_needs_space, bracket_close, bracket_open, format_binary_operator,
    list_joiner, semi_suffix, statement_separator, wants_trailing_comma,
};
pub use metrics_ratio::{
    avg_nodes_per_function, blank_density, comment_density, complexity_density,
    max_function_complexity, source_density, sum_function_complexity,
};
pub use halstead_classify::{is_operand_node_type, is_operator_node_type, operand_value};
pub use lint_rule_active::{
    config_language_allows, is_rule_enabled, language_applies, node_type_applies,
    resolve_diagnostic_severity, rule_is_active,
};
pub use function_meta::{
    function_loc, function_loc_from_source, function_name, function_param_count,
    function_source_slice,
};
pub use format_emit::{
    async_prefix, exceeds_print_width, export_prefix, format_unary_operator, generator_suffix,
    member_access_close, member_access_open, method_kind_prefix, unary_needs_trailing_space,
    var_kind_token,
};
pub use stmt_emit::{
    await_prefix, class_name_suffix, class_prefix, else_prefix, if_close, if_open, import_prefix,
    new_prefix, return_token,
};
pub use literal_emit::{
    emit_literal, emit_primitive_keyword, emit_raw_or_fallback, emit_string_literal,
};
pub use assign_sep::{
    array_close, array_open, assign_op, body_brace_open, call_close, call_open, empty_braces,
    function_keyword, function_params_glue, prop_sep,
};
pub use quote_props::{
    any_key_needs_quote, emit_prop_key, is_bare_identifier_key, needs_quote_as_needed,
    should_quote_prop,
};
pub use arrow_emit::{
    arrow_after_params, arrow_params_need_parens, arrow_params_open, arrow_token,
};
pub use fn_sig_emit::{emit_named_id, method_params, named_id_prefix, params_close_before_body};
pub use node_fallback::{
    emit_import_stub, from_keyword, import_stub_comment, unsupported_node_comment,
};
pub use binding_id::{classify_declarator_child, is_binding_id_type, is_pattern_type};

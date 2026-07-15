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
mod child_select;
mod block_emit;
mod export_kind;
mod method_kind;
mod member_access_kind;
mod class_emit_kind;
mod unary_binary_emit;
mod conditional_spread_emit;
mod loop_template_emit;
mod call_member_emit;
mod object_emit;
mod function_decl_emit;
mod var_stmt_emit;
mod method_array_import_emit;
mod export_class_emit;
mod printer_engine;
mod compressor_engine;
mod linter_engine;
mod metrics_analyzer_engine;

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
pub use child_select::{
    arrow_body_index, binary_has_left, binary_has_right, body_index, call_arg_count,
    class_body_index, find_first_index, has_callee, identifier_index, is_block_statement_type,
    is_class_body_type, is_function_expression_type, is_identifier_type, is_program_type,
    method_value_index,
};
pub use block_emit::{
    block_close, block_item_sep, block_open, object_close, object_open, trailing_comma_suffix,
    unary_is_prefix,
};
pub use export_kind::{
    classify_export_kind, export_default_token, is_export_default_type, is_export_named_type,
    is_export_type, wants_export_default_token,
};
pub use method_kind::{
    method_after_key, method_has_value, normalize_method_kind, should_emit_method_kind,
};
pub use member_access_kind::{
    is_computed_member, member_brackets, member_close, member_open, optional_chain_close,
    optional_chain_open,
};
pub use class_emit_kind::{
    class_body_close, class_body_open, class_body_token, class_keyword, class_name_trailing_space,
    empty_class_body, is_class_body_type as is_class_emit_body_type, is_class_type,
    should_emit_class_name,
};
pub use unary_binary_emit::{
    binary_needs_spaces, binary_operator_token, is_assignment_operator, is_logical_operator,
    unary_is_always_prefix, unary_is_word_operator, unary_operator_token, update_prefix_token,
};
pub use conditional_spread_emit::{
    catch_close, catch_no_param, catch_open, finally_prefix, is_spread_element_type,
    nullish_coalesce_token, optional_chain_dot, sequence_sep, spread_prefix, ternary_colon,
    ternary_pair, ternary_q, throw_token, try_token, yield_token,
};
pub use loop_template_emit::{
    break_token, case_colon, case_prefix, continue_token, default_label, do_token, do_while_open,
    for_close, for_in_token, for_of_token, for_open, is_loop_statement_type, is_switch_related_type,
    is_template_type, switch_close, switch_open, template_expr_close, template_expr_open,
    template_tick, while_close, while_open,
};
// Pure residual continue8 — unique call/member/collection skeletons only (no name collisions).
pub use call_member_emit::{
    array_skeleton, call_args_close, call_args_open, call_skeleton, declarator_assign,
    is_call_related_type, is_collection_type, is_member_related_type, is_module_declaration_type,
    list_element_sep, member_access_close as call_member_access_close,
    member_access_open as call_member_access_open, member_computed_close, member_computed_open,
    member_dot, member_skeleton, optional_chain as call_optional_chain, property_colon,
    property_skeleton,
};
// Pure residual continue9 — object expression / program separators.
pub use object_emit::{
    block_statement_sep, empty_object, is_object_related_type, object_close_pad, object_open_pad,
    object_prop_sep, object_property_colon, object_property_skeleton, object_skeleton,
    object_wants_trailing_comma, program_statement_sep, semi_if, shorthand_property,
    var_kind_with_space,
};
// Pure residual continue10 — FunctionDeclaration head/skeleton (unique names only).
pub use function_decl_emit::{
    function_declaration_head, function_declaration_skeleton, function_name_fragment,
    function_param_sep, function_params_close, function_params_open, generator_star,
    is_function_declaration_type, is_function_like_type,
};
// Pure residual continue11 — VariableDeclaration / expr / return / if skeletons.
pub use var_stmt_emit::{
    expression_statement_skeleton, if_else_prefix, if_statement_skeleton, if_stmt_close,
    if_stmt_open, is_expression_statement_type, is_if_statement_type, is_return_statement_type,
    is_var_stmt_related_type, is_variable_declaration_type, is_variable_declarator_type,
    return_statement_skeleton, var_decl_kind_prefix, var_declarator_assign, var_declarator_sep,
    var_kind_token_normalized, var_stmt_semi, variable_declaration_skeleton,
    variable_declarator_skeleton,
};
// Pure residual continue12 — MethodDefinition / ArrayExpression / Import / Program.
pub use method_array_import_emit::{
    array_element_sep, array_expression_skeleton, array_trailing_comma,
    import_declaration_skeleton, import_declaration_skeleton_minify,
    import_declaration_skeleton_pretty, is_method_array_import_related_type,
    method_definition_skeleton, method_kind_prefix as method_def_kind_prefix,
    method_params_body_glue, program_statement_sep as method_program_statement_sep,
    program_statements_skeleton,
};
// Pure residual continue13 — ExportDeclaration / ClassDeclaration skeletons.
pub use export_class_emit::{
    class_body_interior, class_body_leading_newline, class_body_skeleton, class_declaration_skeleton,
    class_id_fragment, class_keyword_space, class_method_sep, export_declaration_skeleton,
    export_declaration_skeleton_for_type, export_default_fragment, export_keyword,
    is_export_class_related_type, is_export_default_declaration, is_export_named_declaration,
};

// Product tooling engines (printer / compressor / linter)
pub use printer_engine::{
    check_formatted, format_tree, ArrowParens, FormatOptions, Printer, TrailingComma,
};
pub use compressor_engine::{
    compress_tree, minify_savings as product_minify_savings, Compressor, MinifyOptions,
};
pub use linter_engine::{
    lint_tree, lint_tree_with, BuiltinRule, Diagnostic, DiagnosticSeverity, LintResult, Linter,
    LinterConfig, SeverityCountsDto,
};
// Product metrics analyzer engine
pub use metrics_analyzer_engine::{
    analyze_tree, report_tree, BasicMetrics as ProductBasicMetrics, CodeMetrics,
    ComplexityMetrics as ProductComplexityMetrics, FunctionLocation, FunctionMetrics,
    HalsteadMetricsDto, LocationPoint, MaintainabilityDto, MaintainabilityLevelDto,
    MetricsAnalyzer, MetricsReport,
};

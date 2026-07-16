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
mod ident_literal_emit;
mod assign_logical_update_emit;
mod new_await_chain_emit;
mod try_import_emit;
mod loop_switch_full_emit;
mod pattern_sequence_emit;
mod yield_meta_emit;
mod property_static_emit;
mod literal_widen_emit;
mod class_export_from_emit;
mod arrow_method_default_emit;
mod optional_chain_emit;
mod object_method_conditional_emit;
mod template_import_options_emit;
mod binary_unary_logical_emit;
mod call_member_new_emit;
mod assign_sequence_object_emit;
mod var_return_if_emit;
mod loop_switch_stmt_emit;
mod try_throw_import_stmt_emit;
mod property_static_stmt_emit;
mod yield_meta_stmt_emit;
mod pattern_rest_for_await_emit;
mod ident_block_program_emit;
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
// Pure residual continue14 — Identifier / Literal dual-oracle + named specifier lists.
pub use ident_literal_emit::{
    identifier_token, identifier_token_minify, identifier_token_pretty,
    is_ident_literal_related_type, is_identifier_node_type, is_literal_node_type, literal_token,
    named_specifier_braces, named_specifier_list_interior, non_string_literal_token,
    specifier_alias_fragment, specifier_list_sep, string_literal_token,
};
// Pure residual continue15 — Assignment/Logical/Update/This/Empty/Debugger skeletons.
pub use assign_logical_update_emit::{
    assignment_expression_skeleton, debugger_statement_token, empty_statement_token,
    is_assign_logical_update_related_type, is_assignment_expression_type,
    is_debugger_statement_type, is_empty_statement_type, is_logical_expression_type,
    is_sequence_expression_type, is_super_type, is_this_expression_type,
    is_update_expression_type, logical_expression_skeleton, meta_property_skeleton, paren_group,
    sequence_expression_skeleton, super_token, this_token, update_expression_skeleton,
};
// Pure residual continue16 — New/Await/Yield/Chain/ImportExpression skeletons.
pub use new_await_chain_emit::{
    await_expression_skeleton, chain_expression_skeleton, conditional_expression_skeleton,
    import_expression_skeleton, is_await_expression_type, is_chain_expression_type,
    is_conditional_expression_type, is_import_expression_type, is_new_await_chain_related_type,
    is_new_expression_type, is_tagged_template_type, is_yield_expression_type, new_expression_skeleton,
    new_keyword, tagged_template_skeleton, yield_expression_skeleton,
};
// Pure residual continue17 — Try/Catch/Throw/Labeled/Break/Continue/With + full Import.
pub use try_import_emit::{
    break_continue_skeleton, catch_param_close, finally_token, import_declaration_full_skeleton,
    is_break_statement_type, is_catch_clause_type, is_continue_statement_type,
    is_labeled_statement_type, is_throw_statement_type, is_try_import_related_type,
    is_try_statement_type, is_with_statement_type, labeled_statement_skeleton,
    throw_statement_skeleton, try_statement_skeleton, with_close, with_open,
};
// Pure residual continue18 — full for/while/do/switch/template + pattern skeletons.
pub use loop_switch_full_emit::{
    array_pattern_skeleton, assignment_pattern_skeleton, do_while_statement_skeleton,
    for_in_statement_skeleton, for_of_statement_skeleton, for_statement_skeleton,
    is_array_pattern_type, is_assignment_pattern_type, is_loop_switch_full_related_type,
    is_private_identifier_type, private_identifier_token, switch_case_skeleton,
    switch_statement_skeleton, template_literal_skeleton, while_statement_skeleton,
};
// Pure residual continue19 — ObjectPattern/Rest/for-await/tagged-template.
pub use pattern_sequence_emit::{
    for_await_of_statement_skeleton, is_object_pattern_type, is_pattern_sequence_related_type,
    is_rest_element_type, is_tagged_template_expression_type, object_pattern_assign_property,
    object_pattern_shorthand, object_pattern_skeleton, rest_element_token,
    sequence_expression_paren_skeleton, spread_element_token, tagged_template_expression_skeleton,
    tagged_template_simple,
};
// Pure residual continue20 — PropertyDefinition/StaticBlock/ClassExpression.
pub use property_static_emit::{
    accessor_property_skeleton, class_expression_skeleton, class_method_like_field_skeleton,
    computed_property_key, is_accessor_property_type, is_class_expression_type,
    is_property_definition_type, is_property_static_related_type, is_static_block_type,
    property_definition_skeleton, property_key_token, static_block_skeleton, static_prefix,
};

// Pure residual continue21 — BigIntLiteral/RegExpLiteral/DirectiveLiteral.
// Pure residual continue22 — Null/Boolean/Numeric/String literals.
pub use literal_widen_emit::{
    continue21_bigint_literal_skeleton, continue21_directive_literal_skeleton,
    continue21_regexp_literal_skeleton, is_continue21_bigint_literal_type,
    is_continue21_directive_literal_type, is_continue21_regexp_literal_type,
    is_literal_widen_related_type,
    continue22_boolean_literal_skeleton, continue22_null_literal_skeleton,
    continue22_numeric_literal_skeleton, continue22_string_literal_skeleton,
    is_continue22_boolean_literal_type, is_continue22_null_literal_type,
    is_continue22_numeric_literal_type, is_continue22_primitive_literal_type,
    is_continue22_string_literal_type,
};

// catch_open / try_token already re-exported from conditional_spread_emit dual-oracle.

// Pure residual continue71 — Class-extends heritage + Export-from dual-oracle dens.
// New surface (not continue64–70 shell re-wrap). dens ≠ flip; PreferRust OFF.
pub use class_export_from_emit::{
    class_extends_fragment, class_extends_skeleton, class_extends_token,
    export_all_from_skeleton, export_from_source_token, export_named_from_skeleton,
    export_namespace_from_skeleton, is_class_export_from_related_type,
    is_continue71_class_declaration_type, is_continue71_class_expression_type,
    is_continue71_export_all_type, is_continue71_export_named_type,
    is_continue71_export_namespace_specifier_type, is_continue71_export_specifier_type,
    is_continue71_super_type, super_call_skeleton, super_computed_member_skeleton,
    super_member_skeleton, super_static_member_skeleton, wants_class_extends,
    CONTINUE71_RELATED_TYPES,
};

// Pure residual continue72 — Arrow / Method(static·async·params·computed) /
// ExportDefault dual-oracle dens. New surface (not continue64–71 re-wrap).
// dens ≠ flip; PreferRust OFF.
pub use arrow_method_default_emit::{
    arrow_dual_needs_parens, arrow_empty_skeleton, arrow_function_minify,
    arrow_function_pretty_always, arrow_function_skeleton, export_default_class_skeleton,
    export_default_declaration_skeleton, export_default_expression_skeleton,
    export_default_function_skeleton, is_arrow_method_default_related_type,
    is_continue72_arrow_type, is_continue72_class_declaration_type,
    is_continue72_export_default_type, is_continue72_function_declaration_type,
    is_continue72_function_expression_type, is_continue72_method_definition_type,
    method_definition_full_skeleton, method_key_fragment, method_params_fragment,
    param_list_interior, CONTINUE72_RELATED_TYPES,
};

// Pure residual continue73 — OptionalMember / OptionalCall / ChainExpression
// dual-oracle dens. New surface (not continue64–72 re-wrap).
// dens ≠ flip; PreferRust OFF.
pub use optional_chain_emit::{
    arg_list_interior, chain_optional_call, chain_optional_member,
    continue73_chain_expression_skeleton, is_continue73_call_type,
    is_continue73_chain_expression_type, is_continue73_member_type,
    is_continue73_optional_call_type, is_continue73_optional_member_type,
    is_optional_chain_related_type, optional_call_minify, optional_call_open,
    optional_call_pretty, optional_call_skeleton, optional_member_computed,
    optional_member_skeleton, optional_member_static, optional_member_then_call,
    CONTINUE73_RELATED_TYPES,
};

// Pure residual continue74 — ObjectMethod / ConditionalExpression / ForOf(await)
// dual-oracle dens. New surface (not continue64–73 re-wrap).
// dens ≠ flip; PreferRust OFF.
pub use object_method_conditional_emit::{
    conditional_expression_minify, conditional_expression_pretty,
    continue74_conditional_expression_skeleton, for_await_of_minify, for_of_await_skeleton,
    for_of_pretty, is_continue74_conditional_type, is_continue74_for_in_type,
    is_continue74_for_of_type, is_continue74_object_method_type,
    is_continue74_object_property_type, is_object_method_conditional_related_type,
    object_getter_skeleton, object_method_minify, object_method_pretty,
    object_method_skeleton, object_setter_skeleton, CONTINUE74_RELATED_TYPES,
};

// Pure residual continue75 — TemplateLiteral / TaggedTemplate / ImportExpression(options)
// dual-oracle dens. New surface (not continue64–74 re-wrap).
// dens ≠ flip; PreferRust OFF.
pub use template_import_options_emit::{
    continue75_import_expression_options_skeleton, continue75_tagged_template_skeleton,
    continue75_template_literal_skeleton, import_expression_bare,
    import_expression_with_options_minify, import_expression_with_options_pretty,
    is_continue75_import_expression_type, is_continue75_tagged_template_type,
    is_continue75_template_element_type, is_continue75_template_literal_type,
    is_template_import_options_related_type, tagged_template_cooked, tagged_template_single,
    template_element_cooked, template_interpolation, template_literal_cooked,
    template_literal_empty, template_literal_single, CONTINUE75_RELATED_TYPES,
};

// Pure residual continue76 — Binary / Unary / Logical / Update dual-oracle dens.
// New surface (not continue64–75 re-wrap). dens ≠ flip; PreferRust OFF.
pub use binary_unary_logical_emit::{
    binary_expression_minify, binary_expression_pretty, binary_is_word_operator,
    continue76_binary_expression_skeleton, continue76_binary_operator_token,
    continue76_logical_expression_skeleton, continue76_unary_expression_skeleton,
    continue76_update_expression_skeleton, is_binary_unary_logical_related_type,
    is_continue76_binary_type, is_continue76_logical_type, is_continue76_unary_type,
    is_continue76_update_type, logical_expression_minify, logical_expression_pretty,
    unary_expression_minify, unary_expression_pretty, update_postfix_dec, update_postfix_inc,
    update_prefix_dec, update_prefix_inc, CONTINUE76_RELATED_TYPES,
};

// Pure residual continue77 — Call / Member / New / Array dual-oracle dens.
// New surface (not continue64–76 re-wrap). dens ≠ flip; PreferRust OFF.
pub use call_member_new_emit::{
    array_expression_minify, array_expression_pretty, array_expression_pretty_trailing,
    call_expression_minify, call_expression_pretty, continue77_array_expression_skeleton,
    continue77_call_expression_skeleton, continue77_join_list, continue77_list_sep,
    continue77_member_expression_skeleton, continue77_member_then_call,
    continue77_new_expression_skeleton, continue77_new_keyword, is_call_member_new_related_type,
    is_continue77_array_type, is_continue77_call_type, is_continue77_member_type,
    is_continue77_new_type, member_expression_computed, member_expression_static,
    new_expression_minify, new_expression_pretty, CONTINUE77_RELATED_TYPES,
};

// Pure residual continue78 — Assignment / Sequence / Object / Parenthesized
// dual-oracle dens. New surface (not continue64–77 re-wrap).
// dens ≠ flip; PreferRust OFF.
pub use assign_sequence_object_emit::{
    assignment_expression_minify, assignment_expression_pretty,
    continue78_assignment_expression_skeleton, continue78_object_expression_skeleton,
    continue78_object_property_skeleton, continue78_parenthesized_expression_skeleton,
    continue78_parenthesized_sequence, continue78_sequence_expression_skeleton,
    is_assign_sequence_object_related_type, is_continue78_assignment_type,
    is_continue78_object_type, is_continue78_parenthesized_type, is_continue78_sequence_type,
    object_expression_minify, object_expression_pretty, object_expression_pretty_trailing,
    parenthesized_expression, sequence_expression_minify, sequence_expression_pretty,
    CONTINUE78_RELATED_TYPES,
};

// Pure residual continue79 — Variable / Expression / Return / If dual-oracle dens.
// New surface (not continue64–78 re-wrap). dens ≠ flip; PreferRust OFF.
pub use var_return_if_emit::{
    continue79_expression_statement_skeleton, continue79_if_return,
    continue79_if_statement_skeleton, continue79_return_statement_skeleton,
    continue79_variable_declaration_skeleton, continue79_variable_declarator_skeleton,
    expression_statement_minify, expression_statement_pretty, if_else_statement_minify,
    if_else_statement_pretty, if_statement_minify, if_statement_pretty,
    is_continue79_expression_statement_type, is_continue79_if_type, is_continue79_return_type,
    is_continue79_variable_declaration_type, is_continue79_variable_declarator_type,
    is_var_return_if_related_type, return_statement_minify, return_statement_pretty,
    variable_declaration_minify, variable_declaration_pretty, variable_declarator_minify,
    variable_declarator_pretty, CONTINUE79_RELATED_TYPES,
};

// Pure residual continue80 — For / While / DoWhile / Switch dual-oracle dens.
// New surface (not continue64–79 re-wrap). dens ≠ flip; PreferRust OFF.
pub use loop_switch_stmt_emit::{
    continue80_do_while_statement_skeleton, continue80_for_in_statement_skeleton,
    continue80_for_of_statement_skeleton, continue80_for_statement_skeleton,
    continue80_for_then_while, continue80_switch_case_skeleton,
    continue80_switch_statement_skeleton, continue80_while_statement_skeleton,
    do_while_statement_minify, do_while_statement_pretty, for_in_statement_minify,
    for_in_statement_pretty, for_of_statement_minify, for_of_statement_pretty,
    for_statement_minify, for_statement_pretty, is_continue80_do_while_type,
    is_continue80_for_in_type, is_continue80_for_of_type, is_continue80_for_type,
    is_continue80_switch_case_type, is_continue80_switch_type, is_continue80_while_type,
    is_loop_switch_stmt_related_type, switch_case_minify, switch_case_pretty,
    switch_default_minify, switch_default_pretty, switch_statement_minify,
    switch_statement_pretty, while_statement_minify, while_statement_pretty,
    CONTINUE80_RELATED_TYPES,
};

// Pure residual continue81 — Try / Throw / Labeled / Break / Continue / With /
// Import dual-oracle dens. New surface (not continue64–80 re-wrap).
// dens ≠ flip; PreferRust OFF.
pub use try_throw_import_stmt_emit::{
    break_statement_semi, continue81_break_statement_skeleton,
    continue81_continue_statement_skeleton, continue81_import_declaration_skeleton,
    continue81_labeled_statement_skeleton, continue81_throw_statement_skeleton,
    continue81_try_statement_skeleton, continue81_try_then_throw,
    continue81_with_statement_skeleton, continue_statement_labeled_semi,
    import_named_minify, import_named_pretty, import_side_effect_pretty,
    is_continue81_break_type, is_continue81_catch_type, is_continue81_continue_type,
    is_continue81_import_declaration_type, is_continue81_labeled_type,
    is_continue81_throw_type, is_continue81_try_type, is_continue81_with_type,
    is_try_throw_import_stmt_related_type, labeled_statement_minify,
    labeled_statement_pretty, throw_statement_bare, throw_statement_semi,
    try_catch_minify, try_catch_pretty, try_finally_minify, try_finally_pretty,
    with_statement_minify, with_statement_pretty, CONTINUE81_RELATED_TYPES,
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

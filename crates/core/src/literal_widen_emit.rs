//! Pure residual continue21–27: widen literals through paren/template/private/sequence/yield/this-super-conditional emit.
//! Intentional ts_only plugins retained. NO authority_rust / ts_deleted invent.

#![allow(dead_code)]
/// Type guards for continue21 related AST node types.
#[must_use]
pub fn is_literal_widen_related_type(t: &str) -> bool {
    matches!(
        t,
        "BigIntLiteral" | "RegExpLiteral" | "DirectiveLiteral" | "Directive"
    )
}

#[must_use]
pub fn is_continue21_bigint_literal_type(t: &str) -> bool {
    t == "BigIntLiteral"
}

#[must_use]
pub fn is_continue21_regexp_literal_type(t: &str) -> bool {
    t == "RegExpLiteral"
}

#[must_use]
pub fn is_continue21_directive_literal_type(t: &str) -> bool {
    t == "DirectiveLiteral" || t == "Directive"
}

/// Dual-oracle BigIntLiteral emit residual (`1n`).
#[must_use]
pub fn continue21_bigint_literal_skeleton(value: &str) -> String {
    let v = value.trim();
    if v.ends_with('n') {
        v.to_string()
    } else {
        format!("{v}n")
    }
}

/// Dual-oracle RegExpLiteral emit residual (`/foo/gi`).
#[must_use]
pub fn continue21_regexp_literal_skeleton(pattern: &str, flags: &str) -> String {
    format!("/{pattern}/{flags}")
}

/// Dual-oracle DirectiveLiteral residual (`"use strict";`).
#[must_use]
pub fn continue21_directive_literal_skeleton(value: &str, pretty: bool) -> String {
    let body = format!("\"{value}\"");
    let _ = pretty;
    format!("{body};")
}


// ── continue22 pure residual: Null/Boolean/Numeric/String literal emit ──

/// Type guards for continue22 primitive literal AST node types.
#[must_use]
pub fn is_continue22_primitive_literal_type(t: &str) -> bool {
    matches!(
        t,
        "NullLiteral" | "BooleanLiteral" | "NumericLiteral" | "StringLiteral"
    )
}

#[must_use]
pub fn is_continue22_null_literal_type(t: &str) -> bool {
    t == "NullLiteral"
}

#[must_use]
pub fn is_continue22_boolean_literal_type(t: &str) -> bool {
    t == "BooleanLiteral"
}

#[must_use]
pub fn is_continue22_numeric_literal_type(t: &str) -> bool {
    t == "NumericLiteral"
}

#[must_use]
pub fn is_continue22_string_literal_type(t: &str) -> bool {
    t == "StringLiteral"
}

/// Dual-oracle NullLiteral emit residual.
#[must_use]
pub fn continue22_null_literal_skeleton() -> &'static str {
    "null"
}

/// Dual-oracle BooleanLiteral emit residual.
#[must_use]
pub fn continue22_boolean_literal_skeleton(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

/// Dual-oracle NumericLiteral emit residual (raw value string).
#[must_use]
pub fn continue22_numeric_literal_skeleton(raw: &str) -> String {
    raw.trim().to_string()
}

/// Dual-oracle StringLiteral emit residual with quote style.
#[must_use]
pub fn continue22_string_literal_skeleton(value: &str, double_quote: bool) -> String {
    if double_quote {
        format!("\"{value}\"")
    } else {
        format!("'{value}'")
    }
}


// ── continue23 pure residual: TemplateLiteral / TaggedTemplateExpression emit ──

/// Type guards for continue23 template-related AST node types.
#[must_use]
pub fn is_continue23_template_related_type(t: &str) -> bool {
    matches!(t, "TemplateLiteral" | "TaggedTemplateExpression" | "TemplateElement")
}

#[must_use]
pub fn is_continue23_template_literal_type(t: &str) -> bool {
    t == "TemplateLiteral"
}

#[must_use]
pub fn is_continue23_tagged_template_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

#[must_use]
pub fn is_continue23_template_element_type(t: &str) -> bool {
    t == "TemplateElement"
}

/// Dual-oracle residual: empty template literal skeleton.
#[must_use]
pub fn continue23_empty_template_literal_skeleton() -> &'static str {
    "``"
}

/// Dual-oracle residual: cooked template element fragment emit (no expressions).
#[must_use]
pub fn continue23_template_element_cooked(cooked: &str) -> String {
    cooked.to_string()
}

/// Dual-oracle residual: template literal with single cooked quasi and no expressions.
#[must_use]
pub fn continue23_template_literal_no_expr_skeleton(cooked: &str) -> String {
    format!("`{cooked}`")
}

/// Dual-oracle residual: tagged template skeleton tag + empty template.
#[must_use]
pub fn continue23_tagged_template_skeleton(tag: &str) -> String {
    format!("{tag}``")
}




// ── continue24 pure residual: ArrayExpression / ObjectExpression / ArrayPattern emit ──

/// Type guards for continue24 container expression AST node types.
#[must_use]
pub fn is_continue24_container_type(t: &str) -> bool {
    matches!(
        t,
        "ArrayExpression" | "ObjectExpression" | "ArrayPattern" | "ObjectPattern" | "Property"
    )
}

#[must_use]
pub fn is_continue24_array_expression_type(t: &str) -> bool {
    t == "ArrayExpression"
}

#[must_use]
pub fn is_continue24_object_expression_type(t: &str) -> bool {
    t == "ObjectExpression"
}

#[must_use]
pub fn is_continue24_array_pattern_type(t: &str) -> bool {
    t == "ArrayPattern"
}

#[must_use]
pub fn is_continue24_object_pattern_type(t: &str) -> bool {
    t == "ObjectPattern"
}

/// Dual-oracle residual: empty array expression skeleton.
#[must_use]
pub fn continue24_empty_array_skeleton() -> &'static str {
    "[]"
}

/// Dual-oracle residual: empty object expression skeleton.
#[must_use]
pub fn continue24_empty_object_skeleton() -> &'static str {
    "{}"
}

/// Dual-oracle residual: array of string literal elements (pretty=false, comma-join).
#[must_use]
pub fn continue24_array_string_literals_skeleton(elems: &[&str]) -> String {
    let inner = elems
        .iter()
        .map(|e| format!("\"{e}\""))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{inner}]")
}

/// Dual-oracle residual: object with single identifier property key=value skeleton.
#[must_use]
pub fn continue24_object_prop_skeleton(key: &str, value_raw: &str) -> String {
    format!("{{{key}:{value_raw}}}")
}

/// Dual-oracle residual: array pattern empty skeleton.
#[must_use]
pub fn continue24_empty_array_pattern_skeleton() -> &'static str {
    "[]"
}



// ── continue25 pure residual: ParenthesizedExpression + template-with-expr + PrivateName ──

/// Type guards for continue25 residual AST node types.
#[must_use]
pub fn is_continue25_related_type(t: &str) -> bool {
    matches!(
        t,
        "ParenthesizedExpression"
            | "TemplateLiteral"
            | "TemplateElement"
            | "TaggedTemplateExpression"
            | "PrivateName"
            | "PrivateIdentifier"
    )
}

#[must_use]
pub fn is_continue25_parenthesized_expression_type(t: &str) -> bool {
    t == "ParenthesizedExpression"
}

/// Dual-oracle residual: explicit parentheses around an expression.
#[must_use]
pub fn continue25_parenthesized_expression_skeleton(inner: &str) -> String {
    format!("({inner})")
}

/// Dual-oracle residual: template literal with one expression between cooked quasis.
#[must_use]
pub fn continue25_template_literal_one_expr_skeleton(
    cooked_head: &str,
    expr: &str,
    cooked_tail: &str,
) -> String {
    format!("`{cooked_head}${{{expr}}}{cooked_tail}`")
}

/// Dual-oracle residual: tagged template with one expression.
#[must_use]
pub fn continue25_tagged_template_one_expr_skeleton(
    tag: &str,
    cooked_head: &str,
    expr: &str,
    cooked_tail: &str,
) -> String {
    format!(
        "{tag}{}",
        continue25_template_literal_one_expr_skeleton(cooked_head, expr, cooked_tail)
    )
}

/// Dual-oracle residual: PrivateName / private identifier type guard.
#[must_use]
pub fn is_continue25_private_name_type(t: &str) -> bool {
    t == "PrivateName" || t == "PrivateIdentifier"
}

/// Dual-oracle residual: `#field` token skeleton.
#[must_use]
pub fn continue25_private_name_skeleton(name: &str) -> String {
    let n = name.trim_start_matches('#');
    format!("#{n}")
}


// ── continue26 pure residual: SequenceExpression / UpdateExpression / YieldExpression emit ──

/// Dual-oracle residual: sequence / update / yield related types.
#[must_use]
pub fn is_continue26_related_type(t: &str) -> bool {
    matches!(
        t,
        "SequenceExpression" | "UpdateExpression" | "YieldExpression" | "AwaitExpression"
    )
}

#[must_use]
pub fn is_continue26_sequence_expression_type(t: &str) -> bool {
    t == "SequenceExpression"
}

#[must_use]
pub fn is_continue26_update_expression_type(t: &str) -> bool {
    t == "UpdateExpression"
}

#[must_use]
pub fn is_continue26_yield_expression_type(t: &str) -> bool {
    t == "YieldExpression"
}

/// Dual-oracle residual: sequence of two expressions `a, b`.
#[must_use]
pub fn continue26_sequence_two_skeleton(left: &str, right: &str) -> String {
    format!("{left}, {right}")
}

/// Dual-oracle residual: prefix update `++x` / `--x`.
#[must_use]
pub fn continue26_update_prefix_skeleton(op: &str, arg: &str) -> String {
    format!("{op}{arg}")
}

/// Dual-oracle residual: postfix update `x++` / `x--`.
#[must_use]
pub fn continue26_update_postfix_skeleton(arg: &str, op: &str) -> String {
    format!("{arg}{op}")
}

/// Dual-oracle residual: bare `yield` / `yield expr` / `yield* expr`.
#[must_use]
pub fn continue26_yield_skeleton(delegate: bool, arg: Option<&str>) -> String {
    match (delegate, arg) {
        (true, Some(a)) => format!("yield* {a}"),
        (false, Some(a)) => format!("yield {a}"),
        (_, None) => "yield".to_string(),
    }
}

/// Dual-oracle residual: `await expr`.
#[must_use]
pub fn continue26_await_skeleton(arg: &str) -> String {
    format!("await {arg}")
}


// ── continue27 pure residual: This/Super/Conditional/Logical/DoWhile/ForIn/ForOf emit ──

/// Dual-oracle residual: this / super / conditional / logical / loop residual types.
#[must_use]
pub fn is_continue27_related_type(t: &str) -> bool {
    matches!(
        t,
        "ThisExpression"
            | "Super"
            | "ConditionalExpression"
            | "LogicalExpression"
            | "DoWhileStatement"
            | "ForInStatement"
            | "ForOfStatement"
            | "BreakStatement"
            | "ContinueStatement"
            | "LabeledStatement"
    )
}

#[must_use]
pub fn is_continue27_this_expression_type(t: &str) -> bool {
    t == "ThisExpression"
}

#[must_use]
pub fn is_continue27_super_type(t: &str) -> bool {
    t == "Super"
}

#[must_use]
pub fn is_continue27_conditional_expression_type(t: &str) -> bool {
    t == "ConditionalExpression"
}

#[must_use]
pub fn is_continue27_logical_expression_type(t: &str) -> bool {
    t == "LogicalExpression"
}

#[must_use]
pub fn is_continue27_do_while_statement_type(t: &str) -> bool {
    t == "DoWhileStatement"
}

#[must_use]
pub fn is_continue27_for_in_statement_type(t: &str) -> bool {
    t == "ForInStatement"
}

#[must_use]
pub fn is_continue27_for_of_statement_type(t: &str) -> bool {
    t == "ForOfStatement"
}

/// Dual-oracle residual: `this` keyword skeleton.
#[must_use]
pub fn continue27_this_skeleton() -> &'static str {
    "this"
}

/// Dual-oracle residual: `super` keyword skeleton.
#[must_use]
pub fn continue27_super_skeleton() -> &'static str {
    "super"
}

/// Dual-oracle residual: ternary `test ? consequent : alternate`.
#[must_use]
pub fn continue27_conditional_skeleton(test: &str, consequent: &str, alternate: &str) -> String {
    format!("{test} ? {consequent} : {alternate}")
}

/// Dual-oracle residual: logical binary `left op right` (&&, ||, ??).
#[must_use]
pub fn continue27_logical_skeleton(left: &str, op: &str, right: &str) -> String {
    format!("{left} {op} {right}")
}

/// Dual-oracle residual: do-while skeleton.
#[must_use]
pub fn continue27_do_while_skeleton(body: &str, test: &str) -> String {
    format!("do {body} while ({test})")
}

/// Dual-oracle residual: for-in skeleton `for (left in right) body`.
#[must_use]
pub fn continue27_for_in_skeleton(left: &str, right: &str, body: &str) -> String {
    format!("for ({left} in {right}) {body}")
}

/// Dual-oracle residual: for-of skeleton `for (left of right) body` (+ optional await).
#[must_use]
pub fn continue27_for_of_skeleton(left: &str, right: &str, body: &str, awaited: bool) -> String {
    if awaited {
        format!("for await ({left} of {right}) {body}")
    } else {
        format!("for ({left} of {right}) {body}")
    }
}

/// Dual-oracle residual: bare `break` / `break label`.
#[must_use]
pub fn continue27_break_skeleton(label: Option<&str>) -> String {
    match label {
        Some(l) => format!("break {l}"),
        None => "break".to_string(),
    }
}

/// Dual-oracle residual: bare `continue` / `continue label`.
#[must_use]
pub fn continue27_continue_skeleton(label: Option<&str>) -> String {
    match label {
        Some(l) => format!("continue {l}"),
        None => "continue".to_string(),
    }
}

/// Dual-oracle residual: labeled statement `label: body`.
#[must_use]
pub fn continue27_labeled_skeleton(label: &str, body: &str) -> String {
    format!("{label}: {body}")
}


// ── continue28 pure residual: If/While/Return/Throw/Try/Catch/Switch/Empty emit ──
// Dual-oracle residual of intentional ts_only AST emit surface for control-flow
// statement skeletons. Intentional ts_only plugins retained. NO authority invent.

/// Dual-oracle residual: continue28 related Babel node types.
#[must_use]
pub fn is_continue28_related_type(t: &str) -> bool {
    matches!(
        t,
        "IfStatement"
            | "WhileStatement"
            | "ReturnStatement"
            | "ThrowStatement"
            | "TryStatement"
            | "CatchClause"
            | "SwitchStatement"
            | "SwitchCase"
            | "EmptyStatement"
            | "DebuggerStatement"
    )
}

#[must_use]
pub fn is_continue28_if_statement_type(t: &str) -> bool {
    t == "IfStatement"
}
#[must_use]
pub fn is_continue28_while_statement_type(t: &str) -> bool {
    t == "WhileStatement"
}
#[must_use]
pub fn is_continue28_return_statement_type(t: &str) -> bool {
    t == "ReturnStatement"
}
#[must_use]
pub fn is_continue28_throw_statement_type(t: &str) -> bool {
    t == "ThrowStatement"
}
#[must_use]
pub fn is_continue28_try_statement_type(t: &str) -> bool {
    t == "TryStatement"
}
#[must_use]
pub fn is_continue28_switch_statement_type(t: &str) -> bool {
    t == "SwitchStatement"
}

/// Dual-oracle residual: if skeleton `if (test) consec else alt`.
#[must_use]
pub fn continue28_if_skeleton(test: &str, consec: &str, alt: Option<&str>) -> String {
    match alt {
        Some(a) => format!("if ({test}) {consec} else {a}"),
        None => format!("if ({test}) {consec}"),
    }
}

/// Dual-oracle residual: while skeleton.
#[must_use]
pub fn continue28_while_skeleton(test: &str, body: &str) -> String {
    format!("while ({test}) {body}")
}

/// Dual-oracle residual: return skeleton.
#[must_use]
pub fn continue28_return_skeleton(arg: Option<&str>) -> String {
    match arg {
        Some(a) => format!("return {a}"),
        None => "return".to_string(),
    }
}

/// Dual-oracle residual: throw skeleton.
#[must_use]
pub fn continue28_throw_skeleton(arg: &str) -> String {
    format!("throw {arg}")
}

/// Dual-oracle residual: try/catch skeleton (no finally).
#[must_use]
pub fn continue28_try_catch_skeleton(body: &str, param: &str, handler: &str) -> String {
    format!("try {body} catch ({param}) {handler}")
}

/// Dual-oracle residual: switch skeleton single case.
#[must_use]
pub fn continue28_switch_case_skeleton(disc: &str, case_test: &str, case_body: &str) -> String {
    format!("switch ({disc}) {{ case {case_test}: {case_body} }}")
}

/// Dual-oracle residual: empty / debugger skeletons.
#[must_use]
pub fn continue28_empty_skeleton() -> &'static str {
    ";"
}
#[must_use]
pub fn continue28_debugger_skeleton() -> &'static str {
    "debugger"
}


// ── continue29 pure residual: class/function/import/export emit dual-oracle dens ──
// Dual-oracle residual control-flow / declaration emit skeletons.
// Intentional ts_only plugins retained. dens ≠ flip; no authority invent.

/// Dual-oracle residual: continue29 related AST type catalog.
#[must_use]
pub fn is_continue29_related_type(t: &str) -> bool {
    matches!(
        t,
        "ClassDeclaration"
            | "ClassExpression"
            | "ClassBody"
            | "MethodDefinition"
            | "FunctionDeclaration"
            | "FunctionExpression"
            | "ArrowFunctionExpression"
            | "ImportDeclaration"
            | "ImportSpecifier"
            | "ExportNamedDeclaration"
            | "ExportDefaultDeclaration"
            | "ExportAllDeclaration"
    )
}

#[must_use]
pub fn is_continue29_class_declaration_type(t: &str) -> bool {
    t == "ClassDeclaration"
}
#[must_use]
pub fn is_continue29_function_declaration_type(t: &str) -> bool {
    t == "FunctionDeclaration"
}
#[must_use]
pub fn is_continue29_import_declaration_type(t: &str) -> bool {
    t == "ImportDeclaration"
}
#[must_use]
pub fn is_continue29_export_named_type(t: &str) -> bool {
    t == "ExportNamedDeclaration"
}

/// Dual-oracle residual: class declaration skeleton.
#[must_use]
pub fn continue29_class_skeleton(name: &str, body: &str) -> String {
    format!("class {name} {body}")
}

/// Dual-oracle residual: function declaration skeleton.
#[must_use]
pub fn continue29_function_skeleton(name: &str, params: &str, body: &str) -> String {
    format!("function {name}({params}) {body}")
}

/// Dual-oracle residual: arrow function skeleton.
#[must_use]
pub fn continue29_arrow_skeleton(params: &str, body: &str) -> String {
    format!("({params}) => {body}")
}

/// Dual-oracle residual: import named skeleton.
#[must_use]
pub fn continue29_import_named_skeleton(names: &str, source: &str) -> String {
    format!("import {{ {names} }} from \"{source}\"")
}

/// Dual-oracle residual: export named skeleton.
#[must_use]
pub fn continue29_export_named_skeleton(names: &str) -> String {
    format!("export {{ {names} }}")
}

/// Dual-oracle residual: export default skeleton.
#[must_use]
pub fn continue29_export_default_skeleton(expr: &str) -> String {
    format!("export default {expr}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue21_literal_widen_emit() {
        assert!(is_literal_widen_related_type("BigIntLiteral"));
        assert!(is_literal_widen_related_type("RegExpLiteral"));
        assert!(is_literal_widen_related_type("DirectiveLiteral"));
        assert!(is_continue21_bigint_literal_type("BigIntLiteral"));
        assert!(is_continue21_regexp_literal_type("RegExpLiteral"));
        assert!(is_continue21_directive_literal_type("Directive"));
        assert!(!is_literal_widen_related_type("NumericLiteral"));
        assert_eq!(continue21_bigint_literal_skeleton("42"), "42n");
        assert_eq!(continue21_bigint_literal_skeleton("7n"), "7n");
        assert_eq!(continue21_regexp_literal_skeleton("ab+", "gi"), "/ab+/gi");
        assert_eq!(
            continue21_directive_literal_skeleton("use strict", true),
            "\"use strict\";"
        );
    }

    #[test]
    fn continue22_primitive_literal_emit() {
        assert!(is_continue22_primitive_literal_type("NullLiteral"));
        assert!(is_continue22_primitive_literal_type("BooleanLiteral"));
        assert!(is_continue22_primitive_literal_type("NumericLiteral"));
        assert!(is_continue22_primitive_literal_type("StringLiteral"));
        assert!(!is_continue22_primitive_literal_type("BigIntLiteral"));
        assert!(is_continue22_null_literal_type("NullLiteral"));
        assert!(is_continue22_boolean_literal_type("BooleanLiteral"));
        assert!(is_continue22_numeric_literal_type("NumericLiteral"));
        assert!(is_continue22_string_literal_type("StringLiteral"));
        assert_eq!(continue22_null_literal_skeleton(), "null");
        assert_eq!(continue22_boolean_literal_skeleton(true), "true");
        assert_eq!(continue22_boolean_literal_skeleton(false), "false");
        assert_eq!(continue22_numeric_literal_skeleton(" 3.14 "), "3.14");
        assert_eq!(continue22_string_literal_skeleton("hi", true), "\"hi\"");
        assert_eq!(continue22_string_literal_skeleton("hi", false), "'hi'");
    }

    #[test]
    fn continue23_template_literal_emit() {
        assert!(is_continue23_template_related_type("TemplateLiteral"));
        assert!(is_continue23_template_related_type("TaggedTemplateExpression"));
        assert!(is_continue23_template_related_type("TemplateElement"));
        assert!(!is_continue23_template_related_type("StringLiteral"));
        assert!(is_continue23_template_literal_type("TemplateLiteral"));
        assert!(is_continue23_tagged_template_type("TaggedTemplateExpression"));
        assert!(is_continue23_template_element_type("TemplateElement"));
        assert_eq!(continue23_empty_template_literal_skeleton(), "``");
        assert_eq!(continue23_template_element_cooked("hi"), "hi");
        assert_eq!(continue23_template_literal_no_expr_skeleton("hello"), "`hello`");
        assert_eq!(continue23_tagged_template_skeleton("String.raw"), "String.raw``");
    }

    #[test]
    fn continue24_array_object_emit() {
        assert!(is_continue24_container_type("ArrayExpression"));
        assert!(is_continue24_container_type("ObjectExpression"));
        assert!(is_continue24_container_type("ArrayPattern"));
        assert!(is_continue24_container_type("ObjectPattern"));
        assert!(is_continue24_container_type("Property"));
        assert!(!is_continue24_container_type("StringLiteral"));
        assert!(is_continue24_array_expression_type("ArrayExpression"));
        assert!(is_continue24_object_expression_type("ObjectExpression"));
        assert!(is_continue24_array_pattern_type("ArrayPattern"));
        assert!(is_continue24_object_pattern_type("ObjectPattern"));
        assert_eq!(continue24_empty_array_skeleton(), "[]");
        assert_eq!(continue24_empty_object_skeleton(), "{}");
        assert_eq!(
            continue24_array_string_literals_skeleton(&["a", "b"]),
            "[\"a\",\"b\"]"
        );
        assert_eq!(continue24_object_prop_skeleton("x", "1"), "{x:1}");
        assert_eq!(continue24_empty_array_pattern_skeleton(), "[]");
    }

    #[test]
    fn continue25_paren_template_expr_emit() {
        assert!(is_continue25_related_type("ParenthesizedExpression"));
        assert!(is_continue25_related_type("TemplateLiteral"));
        assert!(!is_continue25_related_type("StringLiteral"));
        assert!(is_continue25_parenthesized_expression_type("ParenthesizedExpression"));
        assert_eq!(continue25_parenthesized_expression_skeleton("a + b"), "(a + b)");
        assert_eq!(
            continue25_template_literal_one_expr_skeleton("a", "x", "b"),
            "`a${x}b`"
        );
        assert_eq!(
            continue25_tagged_template_one_expr_skeleton("tag", "", "id", ""),
            "tag`${id}`"
        );
        assert!(is_continue25_private_name_type("PrivateName"));
        assert!(is_continue25_private_name_type("PrivateIdentifier"));
        assert_eq!(continue25_private_name_skeleton("field"), "#field");
        assert_eq!(continue25_private_name_skeleton("#already"), "#already");
    }

    #[test]
    fn continue26_sequence_update_yield_emit() {
        assert!(is_continue26_related_type("SequenceExpression"));
        assert!(is_continue26_related_type("UpdateExpression"));
        assert!(is_continue26_related_type("YieldExpression"));
        assert!(is_continue26_related_type("AwaitExpression"));
        assert!(!is_continue26_related_type("StringLiteral"));
        assert!(is_continue26_sequence_expression_type("SequenceExpression"));
        assert!(is_continue26_update_expression_type("UpdateExpression"));
        assert!(is_continue26_yield_expression_type("YieldExpression"));
        assert_eq!(continue26_sequence_two_skeleton("a", "b"), "a, b");
        assert_eq!(continue26_update_prefix_skeleton("++", "x"), "++x");
        assert_eq!(continue26_update_postfix_skeleton("x", "--"), "x--");
        assert_eq!(continue26_yield_skeleton(false, None), "yield");
        assert_eq!(continue26_yield_skeleton(false, Some("v")), "yield v");
        assert_eq!(continue26_yield_skeleton(true, Some("v")), "yield* v");
        assert_eq!(continue26_await_skeleton("p"), "await p");
    }

    #[test]
    fn continue27_this_super_conditional_emit() {
        assert!(is_continue27_related_type("ThisExpression"));
        assert!(is_continue27_related_type("Super"));
        assert!(is_continue27_related_type("ConditionalExpression"));
        assert!(is_continue27_related_type("LogicalExpression"));
        assert!(is_continue27_related_type("DoWhileStatement"));
        assert!(is_continue27_related_type("ForInStatement"));
        assert!(is_continue27_related_type("ForOfStatement"));
        assert!(is_continue27_related_type("BreakStatement"));
        assert!(is_continue27_related_type("ContinueStatement"));
        assert!(is_continue27_related_type("LabeledStatement"));
        assert!(!is_continue27_related_type("StringLiteral"));
        assert!(is_continue27_this_expression_type("ThisExpression"));
        assert!(is_continue27_super_type("Super"));
        assert!(is_continue27_conditional_expression_type("ConditionalExpression"));
        assert!(is_continue27_logical_expression_type("LogicalExpression"));
        assert!(is_continue27_do_while_statement_type("DoWhileStatement"));
        assert!(is_continue27_for_in_statement_type("ForInStatement"));
        assert!(is_continue27_for_of_statement_type("ForOfStatement"));
        assert_eq!(continue27_this_skeleton(), "this");
        assert_eq!(continue27_super_skeleton(), "super");
        assert_eq!(
            continue27_conditional_skeleton("a", "b", "c"),
            "a ? b : c"
        );
        assert_eq!(continue27_logical_skeleton("a", "&&", "b"), "a && b");
        assert_eq!(continue27_logical_skeleton("a", "??", "b"), "a ?? b");
        assert_eq!(
            continue27_do_while_skeleton("{}", "x"),
            "do {} while (x)"
        );
        assert_eq!(
            continue27_for_in_skeleton("k", "obj", "{}"),
            "for (k in obj) {}"
        );
        assert_eq!(
            continue27_for_of_skeleton("x", "xs", "{}", false),
            "for (x of xs) {}"
        );
        assert_eq!(
            continue27_for_of_skeleton("x", "xs", "{}", true),
            "for await (x of xs) {}"
        );
        assert_eq!(continue27_break_skeleton(None), "break");
        assert_eq!(continue27_break_skeleton(Some("loop")), "break loop");
        assert_eq!(continue27_continue_skeleton(None), "continue");
        assert_eq!(continue27_continue_skeleton(Some("loop")), "continue loop");
        assert_eq!(continue27_labeled_skeleton("outer", "break"), "outer: break");
    }

    #[test]
    fn continue28_if_while_return_throw_try_switch_emit() {
        assert!(is_continue28_related_type("IfStatement"));
        assert!(is_continue28_related_type("WhileStatement"));
        assert!(is_continue28_related_type("ReturnStatement"));
        assert!(is_continue28_related_type("ThrowStatement"));
        assert!(is_continue28_related_type("TryStatement"));
        assert!(is_continue28_related_type("CatchClause"));
        assert!(is_continue28_related_type("SwitchStatement"));
        assert!(is_continue28_related_type("SwitchCase"));
        assert!(is_continue28_related_type("EmptyStatement"));
        assert!(is_continue28_related_type("DebuggerStatement"));
        assert!(!is_continue28_related_type("StringLiteral"));
        assert!(is_continue28_if_statement_type("IfStatement"));
        assert!(is_continue28_while_statement_type("WhileStatement"));
        assert!(is_continue28_return_statement_type("ReturnStatement"));
        assert!(is_continue28_throw_statement_type("ThrowStatement"));
        assert!(is_continue28_try_statement_type("TryStatement"));
        assert!(is_continue28_switch_statement_type("SwitchStatement"));
        assert_eq!(continue28_if_skeleton("a", "{}", None), "if (a) {}");
        assert_eq!(
            continue28_if_skeleton("a", "{x}", Some("{y}")),
            "if (a) {x} else {y}"
        );
        assert_eq!(continue28_while_skeleton("c", "{}"), "while (c) {}");
        assert_eq!(continue28_return_skeleton(None), "return");
        assert_eq!(continue28_return_skeleton(Some("1")), "return 1");
        assert_eq!(continue28_throw_skeleton("e"), "throw e");
        assert_eq!(
            continue28_try_catch_skeleton("{}", "e", "{}"),
            "try {} catch (e) {}"
        );
        assert_eq!(
            continue28_switch_case_skeleton("x", "1", "break;"),
            "switch (x) { case 1: break; }"
        );
        assert_eq!(continue28_empty_skeleton(), ";");
        assert_eq!(continue28_debugger_skeleton(), "debugger");
    }




    #[test]
    fn continue29_class_function_import_export_emit() {
        assert!(is_continue29_related_type("ClassDeclaration"));
        assert!(is_continue29_related_type("FunctionDeclaration"));
        assert!(is_continue29_related_type("ImportDeclaration"));
        assert!(is_continue29_related_type("ExportNamedDeclaration"));
        assert!(is_continue29_related_type("ArrowFunctionExpression"));
        assert!(!is_continue29_related_type("IfStatement"));
        assert!(is_continue29_class_declaration_type("ClassDeclaration"));
        assert!(is_continue29_function_declaration_type("FunctionDeclaration"));
        assert!(is_continue29_import_declaration_type("ImportDeclaration"));
        assert!(is_continue29_export_named_type("ExportNamedDeclaration"));
        assert_eq!(continue29_class_skeleton("A", "{}"), "class A {}");
        assert_eq!(
            continue29_function_skeleton("f", "x", "{ return x; }"),
            "function f(x) { return x; }"
        );
        assert_eq!(continue29_arrow_skeleton("x", "x + 1"), "(x) => x + 1");
        assert_eq!(
            continue29_import_named_skeleton("a, b", "./m"),
            "import { a, b } from \"./m\""
        );
        assert_eq!(continue29_export_named_skeleton("a"), "export { a }");
        assert_eq!(
            continue29_export_default_skeleton("A"),
            "export default A"
        );
    }

}

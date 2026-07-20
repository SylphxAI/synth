//! Pure VariableDeclaration / ExpressionStatement / Return / If emission helpers —
//! residual pure continue11 for tooling/format-minify-lint fragment.
//! Mirrors `printVariableDeclaration` / `compressVariableDeclaration`,
//! `printExpressionStatement` / `compressExpressionStatement`,
//! `printReturnStatement` / `compressReturnStatement`, and
//! `printIfStatement` / `compressIfStatement` in
//! `packages/synth-js-format` / `synth-js-minify`.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//!
//! Dual-oracle surface keeps small pure skeletons even when sibling token helpers
//! exist in stmt_emit / object_emit / assign_sep — intentional residual unit.
#![allow(dead_code)]

/// Normalize variable kind token (`const` / `let` / `var`; default `const`).
#[must_use]
pub fn var_kind_token_normalized(kind: Option<&str>) -> &'static str {
    match kind {
        Some("let") => "let",
        Some("var") => "var",
        _ => "const",
    }
}

/// Variable kind with trailing space (`const ` / `let ` / `var `).
#[must_use]
pub fn var_decl_kind_prefix(kind: Option<&str>) -> String {
    format!("{} ", var_kind_token_normalized(kind))
}

/// Declarator list separator: pretty `, ` vs minify `,`.
#[must_use]
pub fn var_declarator_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Declarator init assign: pretty ` = ` vs minify `=` (only written when init present).
#[must_use]
pub fn var_declarator_assign(pretty: bool) -> &'static str {
    if pretty {
        " = "
    } else {
        "="
    }
}

/// Statement terminator: format option `semi` (pretty path) vs minify always `;`.
/// When `pretty` is false, always returns `;`. When pretty, honors `semi`.
#[must_use]
pub fn var_stmt_semi(pretty: bool, semi: bool) -> &'static str {
    if !pretty || semi {
        ";"
    } else {
        ""
    }
}

/// Build VariableDeclarator skeleton: `id` or `id = init` / `id=init`.
#[must_use]
pub fn variable_declarator_skeleton(id: &str, init: Option<&str>, pretty: bool) -> String {
    match init {
        Some(v) if !v.is_empty() => format!("{id}{}{v}", var_declarator_assign(pretty)),
        _ => id.to_string(),
    }
}

/// Build VariableDeclaration skeleton: `kind d1, d2;` (pretty/minify seps + semi policy).
/// `declarators` are already-rendered declarator fragments.
#[must_use]
pub fn variable_declaration_skeleton(
    kind: Option<&str>,
    declarators: &[&str],
    pretty: bool,
    semi: bool,
) -> String {
    let mut out = String::with_capacity(
        8 + declarators.iter().map(|d| d.len()).sum::<usize>() + declarators.len() * 2,
    );
    out.push_str(&var_decl_kind_prefix(kind));
    for (i, d) in declarators.iter().enumerate() {
        if i > 0 {
            out.push_str(var_declarator_sep(pretty));
        }
        out.push_str(d);
    }
    out.push_str(var_stmt_semi(pretty, semi));
    out
}

/// ExpressionStatement skeleton: optional expression + semi policy.
#[must_use]
pub fn expression_statement_skeleton(expr: Option<&str>, pretty: bool, semi: bool) -> String {
    let mut out = String::with_capacity(expr.map_or(1, str::len) + 1);
    if let Some(e) = expr {
        out.push_str(e);
    }
    out.push_str(var_stmt_semi(pretty, semi));
    out
}

/// ReturnStatement skeleton: `return` / `return arg` + semi policy.
#[must_use]
pub fn return_statement_skeleton(arg: Option<&str>, pretty: bool, semi: bool) -> String {
    let mut out = String::with_capacity(arg.map_or(6, |a| 8 + a.len()));
    out.push_str("return");
    if let Some(a) = arg.filter(|a| !a.is_empty()) {
        out.push(' ');
        out.push_str(a);
    }
    out.push_str(var_stmt_semi(pretty, semi));
    out
}

/// IfStatement open: pretty `if (` vs minify `if(`.
#[must_use]
pub fn if_stmt_open(pretty: bool) -> &'static str {
    if pretty {
        "if ("
    } else {
        "if("
    }
}

/// IfStatement close of test before body: pretty `) ` vs minify `)`.
#[must_use]
pub fn if_stmt_close(pretty: bool) -> &'static str {
    if pretty {
        ") "
    } else {
        ")"
    }
}

/// Else prefix between consequent and alternate: pretty ` else ` vs minify `else `.
#[must_use]
pub fn if_else_prefix(pretty: bool) -> &'static str {
    if pretty {
        " else "
    } else {
        "else "
    }
}

/// Full IfStatement skeleton from already-rendered test/consequent/alternate.
#[must_use]
pub fn if_statement_skeleton(
    test: &str,
    consequent: &str,
    alternate: Option<&str>,
    pretty: bool,
) -> String {
    let mut out = String::with_capacity(
        test.len() + consequent.len() + alternate.map_or(0, str::len) + 16,
    );
    out.push_str(if_stmt_open(pretty));
    out.push_str(test);
    out.push_str(if_stmt_close(pretty));
    out.push_str(consequent);
    if let Some(alt) = alternate {
        out.push_str(if_else_prefix(pretty));
        out.push_str(alt);
    }
    out
}

/// Type guard: VariableDeclaration.
#[must_use]
pub fn is_variable_declaration_type(t: &str) -> bool {
    t == "VariableDeclaration"
}

/// Type guard: VariableDeclarator.
#[must_use]
pub fn is_variable_declarator_type(t: &str) -> bool {
    t == "VariableDeclarator"
}

/// Type guard: ExpressionStatement.
#[must_use]
pub fn is_expression_statement_type(t: &str) -> bool {
    t == "ExpressionStatement"
}

/// Type guard: ReturnStatement.
#[must_use]
pub fn is_return_statement_type(t: &str) -> bool {
    t == "ReturnStatement"
}

/// Type guard: IfStatement.
#[must_use]
pub fn is_if_statement_type(t: &str) -> bool {
    t == "IfStatement"
}

/// Whether a node type is covered by this var/stmt residual unit surface.
#[must_use]
pub fn is_var_stmt_related_type(t: &str) -> bool {
    matches!(
        t,
        "VariableDeclaration"
            | "VariableDeclarator"
            | "ExpressionStatement"
            | "ReturnStatement"
            | "IfStatement"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_and_separators() {
        assert_eq!(var_kind_token_normalized(Some("let")), "let");
        assert_eq!(var_kind_token_normalized(Some("var")), "var");
        assert_eq!(var_kind_token_normalized(Some("const")), "const");
        assert_eq!(var_kind_token_normalized(None), "const");
        assert_eq!(var_kind_token_normalized(Some("weird")), "const");
        assert_eq!(var_decl_kind_prefix(Some("let")), "let ");
        assert_eq!(var_declarator_sep(true), ", ");
        assert_eq!(var_declarator_sep(false), ",");
        assert_eq!(var_declarator_assign(true), " = ");
        assert_eq!(var_declarator_assign(false), "=");
        assert_eq!(var_stmt_semi(true, true), ";");
        assert_eq!(var_stmt_semi(true, false), "");
        assert_eq!(var_stmt_semi(false, false), ";");
        assert_eq!(var_stmt_semi(false, true), ";");
    }

    #[test]
    fn declarator_and_declaration_skeletons() {
        assert_eq!(
            variable_declarator_skeleton("x", Some("1"), true),
            "x = 1"
        );
        assert_eq!(
            variable_declarator_skeleton("x", Some("1"), false),
            "x=1"
        );
        assert_eq!(variable_declarator_skeleton("x", None, true), "x");
        assert_eq!(
            variable_declaration_skeleton(Some("const"), &["x = 1", "y = 2"], true, true),
            "const x = 1, y = 2;"
        );
        assert_eq!(
            variable_declaration_skeleton(Some("let"), &["x=1", "y=2"], false, false),
            "let x=1,y=2;"
        );
        assert_eq!(
            variable_declaration_skeleton(None, &["a"], true, false),
            "const a"
        );
    }

    #[test]
    fn expression_and_return_skeletons() {
        assert_eq!(
            expression_statement_skeleton(Some("foo()"), true, true),
            "foo();"
        );
        assert_eq!(
            expression_statement_skeleton(Some("foo()"), true, false),
            "foo()"
        );
        assert_eq!(
            expression_statement_skeleton(Some("foo()"), false, false),
            "foo();"
        );
        assert_eq!(expression_statement_skeleton(None, false, true), ";");
        assert_eq!(
            return_statement_skeleton(Some("x"), true, true),
            "return x;"
        );
        assert_eq!(return_statement_skeleton(None, true, true), "return;");
        assert_eq!(return_statement_skeleton(None, true, false), "return");
        assert_eq!(
            return_statement_skeleton(Some("x"), false, false),
            "return x;"
        );
    }

    #[test]
    fn if_statement_skeletons() {
        assert_eq!(if_stmt_open(true), "if (");
        assert_eq!(if_stmt_open(false), "if(");
        assert_eq!(if_stmt_close(true), ") ");
        assert_eq!(if_stmt_close(false), ")");
        assert_eq!(if_else_prefix(true), " else ");
        assert_eq!(if_else_prefix(false), "else ");
        assert_eq!(
            if_statement_skeleton("x", "{ return 1; }", None, true),
            "if (x) { return 1; }"
        );
        assert_eq!(
            if_statement_skeleton("x", "{return 1;}", Some("{return 0;}"), false),
            "if(x){return 1;}else {return 0;}"
        );
        assert_eq!(
            if_statement_skeleton("ok", "a()", Some("b()"), true),
            "if (ok) a() else b()"
        );
    }

    #[test]
    fn type_guards() {
        assert!(is_variable_declaration_type("VariableDeclaration"));
        assert!(!is_variable_declaration_type("VariableDeclarator"));
        assert!(is_variable_declarator_type("VariableDeclarator"));
        assert!(is_expression_statement_type("ExpressionStatement"));
        assert!(is_return_statement_type("ReturnStatement"));
        assert!(is_if_statement_type("IfStatement"));
        assert!(is_var_stmt_related_type("IfStatement"));
        assert!(is_var_stmt_related_type("VariableDeclaration"));
        assert!(!is_var_stmt_related_type("FunctionDeclaration"));
    }
}

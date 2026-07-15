//! Pure MethodDefinition / ArrayExpression / ImportDeclaration emission helpers —
//! residual pure continue12 for tooling/format-minify-lint fragment.
//! Mirrors `printMethodDefinition` / `compressMethodDefinition`,
//! `printArrayExpression` / `compressArrayExpression`, and
//! `printImportDeclaration` / `compressImportDeclaration` in
//! `packages/synth-js-format` / `synth-js-minify`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//!
//! Dual-oracle surface keeps pure skeletons even when sibling token helpers
//! exist in method_kind / call_member_emit — intentional residual dens.
#![allow(dead_code)]

/// Method kind prefix: `get ` / `set ` / `constructor ` when kind is not
/// empty and not `"method"`; otherwise empty (parity with
/// `kind && kind !== 'method'` branch).
#[must_use]
pub fn method_kind_prefix(kind: Option<&str>) -> String {
    match kind {
        Some(k) if !k.is_empty() && k != "method" => format!("{k} "),
        _ => String::new(),
    }
}

/// After-key glue for MethodDefinition: pretty `() ` vs minify `()`.
#[must_use]
pub fn method_params_body_glue(pretty: bool) -> &'static str {
    if pretty {
        "() "
    } else {
        "()"
    }
}

/// Build MethodDefinition skeleton from already-rendered key + body fragments.
/// `body` may be empty when FunctionExpression/BlockStatement is absent.
#[must_use]
pub fn method_definition_skeleton(
    kind: Option<&str>,
    key: &str,
    body: Option<&str>,
    pretty: bool,
) -> String {
    let mut out = String::with_capacity(
        8 + key.len() + body.map_or(0, str::len) + method_kind_prefix(kind).len(),
    );
    out.push_str(&method_kind_prefix(kind));
    out.push_str(key);
    out.push_str(method_params_body_glue(pretty));
    if let Some(b) = body {
        out.push_str(b);
    }
    out
}

/// Array element separator: pretty `, ` vs minify `,`.
#[must_use]
pub fn array_element_sep(pretty: bool) -> &'static str {
    if pretty {
        ", "
    } else {
        ","
    }
}

/// Whether pretty printer should emit a trailing comma after the last element.
/// TS format: `trailingComma === 'all' || trailingComma === 'es5'` AND non-empty.
/// Minify never emits trailing commas in the simplified compressor.
#[must_use]
pub fn array_trailing_comma(pretty: bool, trailing_comma: &str, element_count: usize) -> bool {
    if !pretty || element_count == 0 {
        return false;
    }
    trailing_comma == "all" || trailing_comma == "es5"
}

/// Build ArrayExpression skeleton from already-rendered element fragments.
#[must_use]
pub fn array_expression_skeleton(elements: &[&str], pretty: bool, trailing_comma: &str) -> String {
    let mut out = String::with_capacity(
        2 + elements.iter().map(|e| e.len()).sum::<usize>() + elements.len() * 2,
    );
    out.push('[');
    for (i, e) in elements.iter().enumerate() {
        if i > 0 {
            out.push_str(array_element_sep(pretty));
        }
        out.push_str(e);
    }
    if array_trailing_comma(pretty, trailing_comma, elements.len()) {
        out.push(',');
    }
    out.push(']');
    out
}

/// ImportDeclaration pretty skeleton: `import /* import */` + optional semi.
/// Mirrors simplified TS printer (specifiers not yet densed).
#[must_use]
pub fn import_declaration_skeleton_pretty(semi: bool) -> String {
    let mut out = String::from("import /* import */");
    if semi {
        out.push(';');
    }
    out
}

/// ImportDeclaration minify skeleton: always `import;` (TS compressor).
#[must_use]
pub fn import_declaration_skeleton_minify() -> &'static str {
    "import;"
}

/// ImportDeclaration dual-path skeleton.
#[must_use]
pub fn import_declaration_skeleton(pretty: bool, semi: bool) -> String {
    if pretty {
        import_declaration_skeleton_pretty(semi)
    } else {
        import_declaration_skeleton_minify().to_string()
    }
}

/// Program-level statement separator: pretty `\n\n` vs minify empty join
/// (compressor concatenates nodes without separator).
#[must_use]
pub fn program_statement_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n\n"
    } else {
        ""
    }
}

/// Join already-rendered top-level statement fragments with program separator.
#[must_use]
pub fn program_statements_skeleton(stmts: &[&str], pretty: bool) -> String {
    let sep = program_statement_sep(pretty);
    let mut out = String::with_capacity(
        stmts.iter().map(|s| s.len()).sum::<usize>() + sep.len() * stmts.len().saturating_sub(1),
    );
    for (i, s) in stmts.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(s);
    }
    out
}

/// Whether a node type is covered by this residual dens surface.
#[must_use]
pub fn is_method_array_import_related_type(t: &str) -> bool {
    matches!(
        t,
        "MethodDefinition"
            | "ArrayExpression"
            | "ImportDeclaration"
            | "Program"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_kind_and_skeleton() {
        assert_eq!(method_kind_prefix(None), "");
        assert_eq!(method_kind_prefix(Some("")), "");
        assert_eq!(method_kind_prefix(Some("method")), "");
        assert_eq!(method_kind_prefix(Some("get")), "get ");
        assert_eq!(method_kind_prefix(Some("set")), "set ");
        assert_eq!(method_kind_prefix(Some("constructor")), "constructor ");
        assert_eq!(method_params_body_glue(true), "() ");
        assert_eq!(method_params_body_glue(false), "()");
        assert_eq!(
            method_definition_skeleton(Some("get"), "name", Some("{}"), true),
            "get name() {}"
        );
        assert_eq!(
            method_definition_skeleton(None, "run", Some("{return 1;}"), false),
            "run(){return 1;}"
        );
        assert_eq!(
            method_definition_skeleton(Some("method"), "x", None, true),
            "x() "
        );
        assert_eq!(
            method_definition_skeleton(Some("constructor"), "constructor", Some("{}"), true),
            "constructor constructor() {}"
        );
    }

    #[test]
    fn array_expression_skeletons() {
        assert_eq!(array_element_sep(true), ", ");
        assert_eq!(array_element_sep(false), ",");
        assert!(!array_trailing_comma(false, "all", 2));
        assert!(!array_trailing_comma(true, "none", 2));
        assert!(!array_trailing_comma(true, "es5", 0));
        assert!(array_trailing_comma(true, "es5", 1));
        assert!(array_trailing_comma(true, "all", 3));
        assert_eq!(
            array_expression_skeleton(&["1", "2"], true, "none"),
            "[1, 2]"
        );
        assert_eq!(
            array_expression_skeleton(&["1", "2"], true, "es5"),
            "[1, 2,]"
        );
        assert_eq!(
            array_expression_skeleton(&["1", "2"], false, "all"),
            "[1,2]"
        );
        assert_eq!(array_expression_skeleton(&[], true, "all"), "[]");
        assert_eq!(
            array_expression_skeleton(&["a"], true, "all"),
            "[a,]"
        );
    }

    #[test]
    fn import_and_program_skeletons() {
        assert_eq!(
            import_declaration_skeleton_pretty(true),
            "import /* import */;"
        );
        assert_eq!(
            import_declaration_skeleton_pretty(false),
            "import /* import */"
        );
        assert_eq!(import_declaration_skeleton_minify(), "import;");
        assert_eq!(
            import_declaration_skeleton(true, true),
            "import /* import */;"
        );
        assert_eq!(import_declaration_skeleton(false, true), "import;");
        assert_eq!(import_declaration_skeleton(false, false), "import;");
        assert_eq!(program_statement_sep(true), "\n\n");
        assert_eq!(program_statement_sep(false), "");
        assert_eq!(
            program_statements_skeleton(&["const a = 1;", "const b = 2;"], true),
            "const a = 1;\n\nconst b = 2;"
        );
        assert_eq!(
            program_statements_skeleton(&["a;", "b;"], false),
            "a;b;"
        );
        assert_eq!(program_statements_skeleton(&["only"], true), "only");
        assert_eq!(program_statements_skeleton(&[], true), "");
    }

    #[test]
    fn type_guards() {
        assert!(is_method_array_import_related_type("MethodDefinition"));
        assert!(is_method_array_import_related_type("ArrayExpression"));
        assert!(is_method_array_import_related_type("ImportDeclaration"));
        assert!(is_method_array_import_related_type("Program"));
        assert!(!is_method_array_import_related_type("ObjectExpression"));
        assert!(!is_method_array_import_related_type("FunctionDeclaration"));
    }
}

//! Pure unsupported-node comment fallback emission —
//! mirrors printer default branch `/* ${type} */` and compressor `/*${type}*/`.
//! Residual pure deepen for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// Emit a comment placeholder for an unsupported AST node type.
/// Pretty (format) adds spaces inside the comment delimiters; minify is compact.
#[must_use]
pub fn unsupported_node_comment(node_type: &str, pretty: bool) -> String {
    if pretty {
        format!("/* {node_type} */")
    } else {
        format!("/*{node_type}*/")
    }
}

/// Import declaration stub body used by the simplified TS import printers.
/// Format path writes `/* import */` after `import `; minify collapses to bare
/// `import` + semi (no stub comment).
#[must_use]
pub fn import_stub_comment(pretty: bool) -> &'static str {
    if pretty {
        "/* import */"
    } else {
        ""
    }
}

/// Full simplified import declaration emission (keyword + optional stub + semi).
/// Mirrors printImportDeclaration / compressImportDeclaration.
#[must_use]
pub fn emit_import_stub(pretty: bool, semi: bool) -> String {
    let mut out = String::from("import");
    if pretty {
        out.push(' ');
        out.push_str(import_stub_comment(true));
        if semi {
            out.push(';');
        }
    } else {
        // minify: always terminates with `;` (compressImportDeclaration)
        out.push(';');
    }
    out
}

/// `from` keyword between import/export clause and module specifier.
/// Pretty → ` from `; minify → `from` (no surrounding spaces — caller joins
/// adjacent tokens carefully when full import emission is wired).
#[must_use]
pub fn from_keyword(pretty: bool) -> &'static str {
    if pretty {
        " from "
    } else {
        "from"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_comments() {
        assert_eq!(
            unsupported_node_comment("ConditionalExpression", true),
            "/* ConditionalExpression */"
        );
        assert_eq!(
            unsupported_node_comment("ConditionalExpression", false),
            "/*ConditionalExpression*/"
        );
        assert_eq!(unsupported_node_comment("X", true), "/* X */");
        assert_eq!(unsupported_node_comment("X", false), "/*X*/");
    }

    #[test]
    fn import_stubs() {
        assert_eq!(import_stub_comment(true), "/* import */");
        assert_eq!(import_stub_comment(false), "");
        assert_eq!(emit_import_stub(true, true), "import /* import */;");
        assert_eq!(emit_import_stub(true, false), "import /* import */");
        assert_eq!(emit_import_stub(false, true), "import;");
        assert_eq!(emit_import_stub(false, false), "import;"); // minify always `;`
        assert_eq!(from_keyword(true), " from ");
        assert_eq!(from_keyword(false), "from");
    }
}

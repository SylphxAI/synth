//! Pure export-declaration kind classification —
//! mirrors `printExportDeclaration` / `compressExportDeclaration`
//! (`ExportDefaultDeclaration` vs named) in format/minify tooling.
//! Residual pure continue for tooling/format-minify-lint fragment.
//! NO full printer/compressor engine, NO authority_rust / ts_deleted.

/// True when the node type is an export-default declaration.
/// TS: `node.type === 'ExportDefaultDeclaration'`.
#[must_use]
pub fn is_export_default_type(node_type: &str) -> bool {
    node_type == "ExportDefaultDeclaration"
}

/// True when the node type is a named export declaration.
#[must_use]
pub fn is_export_named_type(node_type: &str) -> bool {
    node_type == "ExportNamedDeclaration"
}

/// True when the node type is any export declaration handled by the printer.
#[must_use]
pub fn is_export_type(node_type: &str) -> bool {
    is_export_default_type(node_type) || is_export_named_type(node_type)
}

/// The `default ` token written after `export ` for default exports.
/// Full combined prefix is `export_prefix(true)` → `export default `.
#[must_use]
pub fn export_default_token() -> &'static str {
    "default "
}

/// Whether to emit the default keyword for a given export node type.
#[must_use]
pub fn wants_export_default_token(node_type: &str) -> bool {
    is_export_default_type(node_type)
}

/// Classify export kind as `"default"`, `"named"`, or `"none"`.
#[must_use]
pub fn classify_export_kind(node_type: &str) -> &'static str {
    if is_export_default_type(node_type) {
        "default"
    } else if is_export_named_type(node_type) {
        "named"
    } else {
        "none"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_type_predicates() {
        assert!(is_export_default_type("ExportDefaultDeclaration"));
        assert!(!is_export_default_type("ExportNamedDeclaration"));
        assert!(is_export_named_type("ExportNamedDeclaration"));
        assert!(is_export_type("ExportDefaultDeclaration"));
        assert!(is_export_type("ExportNamedDeclaration"));
        assert!(!is_export_type("ImportDeclaration"));
        assert!(!is_export_type("ClassDeclaration"));
    }

    #[test]
    fn default_token_and_classify() {
        assert_eq!(export_default_token(), "default ");
        assert!(wants_export_default_token("ExportDefaultDeclaration"));
        assert!(!wants_export_default_token("ExportNamedDeclaration"));
        assert_eq!(classify_export_kind("ExportDefaultDeclaration"), "default");
        assert_eq!(classify_export_kind("ExportNamedDeclaration"), "named");
        assert_eq!(classify_export_kind("VariableDeclaration"), "none");
    }
}

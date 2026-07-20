//! Pure call / member / property / array / import-export emission helpers —
//! residual pure continue8 for tooling/format-minify-lint fragment.
//! Mirrors printer/compressor branches in `packages/synth-js-format` /
//! `synth-js-minify` not covered by stmt/loop/conditional/object block emits.
//! Full engines remain product residual. NO authority_rust / ts_deleted.
//!
//! Dual-oracle surface keeps small pure kernels even when siblings exist in
//! other emit modules — intentional residual unit (not dead product paths).
#![allow(dead_code)]

/// Array expression open `[`.
#[must_use]
pub fn array_open() -> &'static str {
    "["
}

/// Array expression close `]`.
#[must_use]
pub fn array_close() -> &'static str {
    "]"
}

/// Element separator inside arrays/args (pretty & minify both use `, ` in current TS printer).
#[must_use]
pub fn list_element_sep() -> &'static str {
    ", "
}

/// Call / new argument list open `(`.
#[must_use]
pub fn call_args_open() -> &'static str {
    "("
}

/// Call / new argument list close `)`.
#[must_use]
pub fn call_args_close() -> &'static str {
    ")"
}

/// `new ` prefix for NewExpression.
#[must_use]
pub fn new_prefix() -> &'static str {
    "new "
}

/// `await ` prefix for AwaitExpression.
#[must_use]
pub fn await_prefix() -> &'static str {
    "await "
}

/// Non-computed member access dot.
#[must_use]
pub fn member_dot() -> &'static str {
    "."
}

/// Computed member open `[`.
#[must_use]
pub fn member_computed_open() -> &'static str {
    "["
}

/// Computed member close `]`.
#[must_use]
pub fn member_computed_close() -> &'static str {
    "]"
}

/// Member access fragment from `computed` flag (TS: `data.computed`).
#[must_use]
pub fn member_access_open(computed: bool) -> &'static str {
    if computed {
        "["
    } else {
        "."
    }
}

/// Member access close (computed → `]`; non-computed → empty).
#[must_use]
pub fn member_access_close(computed: bool) -> &'static str {
    if computed {
        "]"
    } else {
        ""
    }
}

/// Object property key/value separator (pretty TS printer always `: `).
#[must_use]
pub fn property_colon() -> &'static str {
    ": "
}

/// Variable declarator init assign (pretty: ` = `; only written when init present).
#[must_use]
pub fn declarator_assign() -> &'static str {
    " = "
}

/// `import ` keyword prefix.
#[must_use]
pub fn import_prefix() -> &'static str {
    "import "
}

/// `export ` keyword prefix.
#[must_use]
pub fn export_prefix() -> &'static str {
    "export "
}

/// `default ` after export for ExportDefaultDeclaration.
#[must_use]
pub fn export_default_token() -> &'static str {
    "default "
}

/// Export declaration prefix for named vs default.
#[must_use]
pub fn export_declaration_prefix(is_default: bool) -> String {
    if is_default {
        format!("{}{}", export_prefix(), export_default_token())
    } else {
        export_prefix().to_string()
    }
}

/// Optional chain operator (`?.`) for optional member / call.
#[must_use]
pub fn optional_chain() -> &'static str {
    "?."
}

/// Whether a node type is call/new related.
#[must_use]
pub fn is_call_related_type(ty: &str) -> bool {
    matches!(
        ty,
        "CallExpression" | "NewExpression" | "OptionalCallExpression"
    )
}

/// Whether a node type is member access related.
#[must_use]
pub fn is_member_related_type(ty: &str) -> bool {
    matches!(
        ty,
        "MemberExpression" | "OptionalMemberExpression" | "ChainExpression"
    )
}

/// Whether a node type is array/object property container.
#[must_use]
pub fn is_collection_type(ty: &str) -> bool {
    matches!(
        ty,
        "ArrayExpression" | "ObjectExpression" | "Property" | "SpreadElement"
    )
}

/// Whether a node type is import/export declaration.
#[must_use]
pub fn is_module_declaration_type(ty: &str) -> bool {
    matches!(
        ty,
        "ImportDeclaration"
            | "ExportNamedDeclaration"
            | "ExportDefaultDeclaration"
            | "ExportAllDeclaration"
    )
}

/// Build call-expression emission skeleton: `callee(arg1, arg2)` without
/// printing nested nodes (caller substitutes pieces).
#[must_use]
pub fn call_skeleton(callee: &str, args: &[&str]) -> String {
    let mut out = String::with_capacity(callee.len() + args.len() * 8 + 2);
    out.push_str(callee);
    out.push_str(call_args_open());
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            out.push_str(list_element_sep());
        }
        out.push_str(a);
    }
    out.push_str(call_args_close());
    out
}

/// Build member-expression emission skeleton.
#[must_use]
pub fn member_skeleton(object: &str, property: &str, computed: bool) -> String {
    let mut out = String::with_capacity(object.len() + property.len() + 2);
    out.push_str(object);
    out.push_str(member_access_open(computed));
    out.push_str(property);
    out.push_str(member_access_close(computed));
    out
}

/// Build array-expression emission skeleton with optional trailing comma.
#[must_use]
pub fn array_skeleton(elems: &[&str], trailing_comma: bool) -> String {
    let mut out = String::with_capacity(elems.len() * 8 + 2);
    out.push_str(array_open());
    for (i, e) in elems.iter().enumerate() {
        if i > 0 {
            out.push_str(list_element_sep());
        }
        out.push_str(e);
    }
    if trailing_comma && !elems.is_empty() {
        out.push(',');
    }
    out.push_str(array_close());
    out
}

/// Build object property emission: `key: value`.
#[must_use]
pub fn property_skeleton(key: &str, value: &str) -> String {
    format!("{key}{}{value}", property_colon())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delimiters_and_prefixes() {
        assert_eq!(array_open(), "[");
        assert_eq!(array_close(), "]");
        assert_eq!(list_element_sep(), ", ");
        assert_eq!(call_args_open(), "(");
        assert_eq!(call_args_close(), ")");
        assert_eq!(new_prefix(), "new ");
        assert_eq!(await_prefix(), "await ");
        assert_eq!(member_dot(), ".");
        assert_eq!(member_computed_open(), "[");
        assert_eq!(member_computed_close(), "]");
        assert_eq!(property_colon(), ": ");
        assert_eq!(declarator_assign(), " = ");
        assert_eq!(import_prefix(), "import ");
        assert_eq!(export_prefix(), "export ");
        assert_eq!(export_default_token(), "default ");
        assert_eq!(optional_chain(), "?.");
    }

    #[test]
    fn member_access_flags() {
        assert_eq!(member_access_open(false), ".");
        assert_eq!(member_access_close(false), "");
        assert_eq!(member_access_open(true), "[");
        assert_eq!(member_access_close(true), "]");
        assert_eq!(member_skeleton("obj", "prop", false), "obj.prop");
        assert_eq!(member_skeleton("obj", "k", true), "obj[k]");
    }

    #[test]
    fn call_and_array_skeletons() {
        assert_eq!(call_skeleton("foo", &[]), "foo()");
        assert_eq!(call_skeleton("foo", &["1", "2"]), "foo(1, 2)");
        assert_eq!(array_skeleton(&["a", "b"], false), "[a, b]");
        assert_eq!(array_skeleton(&["a"], true), "[a,]");
        assert_eq!(array_skeleton(&[], true), "[]");
        assert_eq!(property_skeleton("x", "1"), "x: 1");
        assert_eq!(export_declaration_prefix(true), "export default ");
        assert_eq!(export_declaration_prefix(false), "export ");
    }

    #[test]
    fn type_guards() {
        assert!(is_call_related_type("CallExpression"));
        assert!(is_call_related_type("NewExpression"));
        assert!(!is_call_related_type("MemberExpression"));
        assert!(is_member_related_type("OptionalMemberExpression"));
        assert!(is_collection_type("ArrayExpression"));
        assert!(is_collection_type("Property"));
        assert!(is_module_declaration_type("ImportDeclaration"));
        assert!(is_module_declaration_type("ExportDefaultDeclaration"));
        assert!(!is_module_declaration_type("VariableDeclaration"));
    }
}

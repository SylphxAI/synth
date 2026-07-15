//! Pure ExportDeclaration / ClassDeclaration emission helpers —
//! residual pure continue13 for tooling/format-minify-lint fragment.
//! Mirrors `printExportDeclaration` / `compressExportDeclaration` and
//! `printClassDeclaration` / `compressClassDeclaration` in
//! `packages/synth-js-format` / `synth-js-minify`.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//!
//! Dual-oracle surface keeps pure skeletons even when sibling kind helpers
//! exist in export_kind / class_emit_kind — intentional residual dens.
#![allow(dead_code)]

/// Shared `export ` prefix (pretty + minify).
#[must_use]
pub fn export_keyword() -> &'static str {
    "export "
}

/// Optional `default ` after export keyword.
#[must_use]
pub fn export_default_fragment(is_default: bool) -> &'static str {
    if is_default {
        "default "
    } else {
        ""
    }
}

/// Whether node type is ExportDefaultDeclaration.
#[must_use]
pub fn is_export_default_declaration(t: &str) -> bool {
    t == "ExportDefaultDeclaration"
}

/// Whether node type is ExportNamedDeclaration.
#[must_use]
pub fn is_export_named_declaration(t: &str) -> bool {
    t == "ExportNamedDeclaration"
}

/// Build ExportDeclaration skeleton from already-rendered declaration fragment.
/// TS pretty writes indent before `export `; that is printer state, not densed here.
#[must_use]
pub fn export_declaration_skeleton(is_default: bool, declaration: Option<&str>) -> String {
    let mut out = String::with_capacity(
        16 + declaration.map_or(0, str::len) + if is_default { 8 } else { 0 },
    );
    out.push_str(export_keyword());
    out.push_str(export_default_fragment(is_default));
    if let Some(d) = declaration {
        out.push_str(d);
    }
    out
}

/// Dual-path export skeleton from node type string.
#[must_use]
pub fn export_declaration_skeleton_for_type(node_type: &str, declaration: Option<&str>) -> String {
    export_declaration_skeleton(is_export_default_declaration(node_type), declaration)
}

/// Class keyword + trailing space (shared pretty/minify).
#[must_use]
pub fn class_keyword_space() -> &'static str {
    "class "
}

/// Optional class id fragment with trailing space when present (pre-mangle name).
#[must_use]
pub fn class_id_fragment(name: Option<&str>) -> String {
    match name {
        Some(n) if !n.is_empty() => format!("{n} "),
        _ => String::new(),
    }
}

/// Method separator inside ClassBody: pretty `\n` between methods (caller owns
/// indent writes around each method); minify empty join.
#[must_use]
pub fn class_method_sep(pretty: bool) -> &'static str {
    if pretty {
        "\n"
    } else {
        ""
    }
}

/// Whether pretty class body should open with a leading newline after `{`
/// when methods are non-empty (TS printer: `write('{'); write('\n'); …`).
#[must_use]
pub fn class_body_leading_newline(pretty: bool, method_count: usize) -> bool {
    pretty && method_count > 0
}

/// Build ClassBody interior: methods joined with dual-oracle separator,
/// optional leading newline for pretty non-empty bodies.
#[must_use]
pub fn class_body_interior(methods: &[&str], pretty: bool) -> String {
    let mut out = String::with_capacity(
        methods.iter().map(|m| m.len()).sum::<usize>() + methods.len().saturating_mul(2) + 2,
    );
    if class_body_leading_newline(pretty, methods.len()) {
        out.push('\n');
    }
    let sep = class_method_sep(pretty);
    for (i, m) in methods.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(m);
    }
    if class_body_leading_newline(pretty, methods.len()) {
        // Trailing newline before closing `}` so printer can writeIndent then `}`.
        out.push('\n');
    }
    out
}

/// Full class body braces around interior. Empty / no ClassBody → `{}`.
#[must_use]
pub fn class_body_skeleton(methods: &[&str], pretty: bool) -> String {
    if methods.is_empty() {
        return "{}".to_string();
    }
    let interior = class_body_interior(methods, pretty);
    format!("{{{interior}}}")
}

/// Build ClassDeclaration skeleton from optional id + already-rendered methods.
#[must_use]
pub fn class_declaration_skeleton(
    name: Option<&str>,
    methods: &[&str],
    pretty: bool,
) -> String {
    let mut out = String::with_capacity(16 + name.map_or(0, str::len) + methods.len() * 8);
    out.push_str(class_keyword_space());
    out.push_str(&class_id_fragment(name));
    out.push_str(&class_body_skeleton(methods, pretty));
    out
}

/// Whether a node type is covered by this residual dens surface.
#[must_use]
pub fn is_export_class_related_type(t: &str) -> bool {
    matches!(
        t,
        "ExportNamedDeclaration"
            | "ExportDefaultDeclaration"
            | "ClassDeclaration"
            | "ClassExpression"
            | "ClassBody"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_keyword_and_default_fragment() {
        assert_eq!(export_keyword(), "export ");
        assert_eq!(export_default_fragment(true), "default ");
        assert_eq!(export_default_fragment(false), "");
        assert!(is_export_default_declaration("ExportDefaultDeclaration"));
        assert!(!is_export_default_declaration("ExportNamedDeclaration"));
        assert!(is_export_named_declaration("ExportNamedDeclaration"));
    }

    #[test]
    fn export_declaration_skeletons() {
        assert_eq!(
            export_declaration_skeleton(true, Some("class Foo {}")),
            "export default class Foo {}"
        );
        assert_eq!(
            export_declaration_skeleton(false, Some("const a = 1")),
            "export const a = 1"
        );
        assert_eq!(export_declaration_skeleton(true, None), "export default ");
        assert_eq!(export_declaration_skeleton(false, None), "export ");
        assert_eq!(
            export_declaration_skeleton_for_type(
                "ExportDefaultDeclaration",
                Some("function f() {}")
            ),
            "export default function f() {}"
        );
        assert_eq!(
            export_declaration_skeleton_for_type("ExportNamedDeclaration", Some("class A {}")),
            "export class A {}"
        );
    }

    #[test]
    fn class_id_and_method_sep() {
        assert_eq!(class_keyword_space(), "class ");
        assert_eq!(class_id_fragment(Some("Foo")), "Foo ");
        assert_eq!(class_id_fragment(Some("")), "");
        assert_eq!(class_id_fragment(None), "");
        assert_eq!(class_method_sep(true), "\n");
        assert_eq!(class_method_sep(false), "");
        assert!(class_body_leading_newline(true, 1));
        assert!(!class_body_leading_newline(true, 0));
        assert!(!class_body_leading_newline(false, 2));
    }

    #[test]
    fn class_body_and_declaration_skeletons() {
        assert_eq!(class_body_skeleton(&[], true), "{}");
        assert_eq!(class_body_skeleton(&[], false), "{}");
        assert_eq!(
            class_body_skeleton(&["run() {}"], false),
            "{run() {}}"
        );
        assert_eq!(
            class_body_skeleton(&["a() {}", "b() {}"], false),
            "{a() {}b() {}}"
        );
        assert_eq!(
            class_body_skeleton(&["run() {}"], true),
            "{\nrun() {}\n}"
        );
        assert_eq!(
            class_body_skeleton(&["a() {}", "b() {}"], true),
            "{\na() {}\nb() {}\n}"
        );
        assert_eq!(
            class_declaration_skeleton(Some("Foo"), &[], true),
            "class Foo {}"
        );
        assert_eq!(
            class_declaration_skeleton(None, &[], false),
            "class {}"
        );
        assert_eq!(
            class_declaration_skeleton(Some("Bar"), &["m() {}"], false),
            "class Bar {m() {}}"
        );
        assert_eq!(
            class_declaration_skeleton(Some("Bar"), &["m() {}"], true),
            "class Bar {\nm() {}\n}"
        );
    }

    #[test]
    fn type_guards() {
        assert!(is_export_class_related_type("ExportNamedDeclaration"));
        assert!(is_export_class_related_type("ExportDefaultDeclaration"));
        assert!(is_export_class_related_type("ClassDeclaration"));
        assert!(is_export_class_related_type("ClassExpression"));
        assert!(is_export_class_related_type("ClassBody"));
        assert!(!is_export_class_related_type("ImportDeclaration"));
        assert!(!is_export_class_related_type("MethodDefinition"));
        assert!(!is_export_class_related_type("FunctionDeclaration"));
    }
}

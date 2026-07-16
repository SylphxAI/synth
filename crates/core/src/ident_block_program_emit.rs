//! Pure Identifier + Literal + BlockStatement + Program + EmptyStatement +
//! DebuggerStatement dual-oracle emission — residual pure **continue85** for
//! tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–84:
//! - Identifier dual-oracle pretty/minify-mangle composing real
//!   `identifier_token` (ident_literal base)
//! - Literal dual-oracle string/non-string pretty/minify composing real
//!   `literal_token` / `string_literal_token`
//! - BlockStatement dual-oracle empty/body pretty/minify composing real
//!   `block_open` / `block_item_sep` / `block_close`
//! - Program dual-oracle statement join composing real
//!   `program_statements_skeleton`
//! - EmptyStatement dual-oracle `;` composing real
//!   `continue33_empty_statement_skeleton`
//! - DebuggerStatement dual-oracle `debugger;` composing real
//!   `continue32_debugger_skeleton`
//! - Composed program/block residual shells
//!
//! Intentionally does **not** re-wrap continue64–84 partition shells
//! (pattern/rest/for-await continue84 stays separate; yield/meta continue83
//! stays separate; try/throw/import continue81 stays separate). Composes real
//! shipped pure helpers from ident/literal/block/program bases.
//! Full engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::block_emit::{block_close, block_item_sep, block_open};
use crate::ident_literal_emit::{
    identifier_token, literal_token, string_literal_token,
};
use crate::literal_widen_emit::{
    continue32_debugger_skeleton, continue33_empty_statement_skeleton,
};
use crate::method_array_import_emit::program_statements_skeleton;

/// Dual-oracle residual: continue85 related AST type catalog.
pub const CONTINUE85_RELATED_TYPES: &[&str] = &[
    "Identifier",
    "Literal",
    "BlockStatement",
    "Program",
    "EmptyStatement",
    "DebuggerStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_ident_block_program_related_type(t: &str) -> bool {
    CONTINUE85_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue85_identifier_type(t: &str) -> bool {
    t == "Identifier"
}

#[must_use]
pub fn is_continue85_literal_type(t: &str) -> bool {
    t == "Literal"
}

#[must_use]
pub fn is_continue85_block_type(t: &str) -> bool {
    t == "BlockStatement"
}

#[must_use]
pub fn is_continue85_program_type(t: &str) -> bool {
    t == "Program"
}

#[must_use]
pub fn is_continue85_empty_statement_type(t: &str) -> bool {
    t == "EmptyStatement"
}

#[must_use]
pub fn is_continue85_debugger_type(t: &str) -> bool {
    t == "DebuggerStatement"
}

// ── Identifier dual-oracle ──────────────────────────────────────────────────

/// Dual-oracle Identifier skeleton composing real [`identifier_token`].
#[must_use]
pub fn continue85_identifier_skeleton(
    name: Option<&str>,
    pretty: bool,
    mangle_enabled: bool,
    mangled: Option<&str>,
) -> String {
    identifier_token(name, pretty, mangle_enabled, mangled)
}

/// Convenience: pretty identifier (no mangle).
#[must_use]
pub fn identifier_pretty(name: &str) -> String {
    continue85_identifier_skeleton(Some(name), true, false, None)
}

/// Convenience: minify identifier without mangle.
#[must_use]
pub fn identifier_minify(name: &str) -> String {
    continue85_identifier_skeleton(Some(name), false, false, None)
}

/// Convenience: minify + mangle identifier.
#[must_use]
pub fn identifier_mangle(name: &str, mangled: &str) -> String {
    continue85_identifier_skeleton(Some(name), false, true, Some(mangled))
}

// ── Literal dual-oracle ─────────────────────────────────────────────────────

/// Dual-oracle Literal skeleton composing real [`literal_token`].
#[must_use]
pub fn continue85_literal_skeleton(
    is_string: bool,
    value: Option<&str>,
    raw: Option<&str>,
    single_quote: bool,
    pretty: bool,
) -> String {
    literal_token(is_string, value, raw, single_quote, pretty)
}

/// Convenience: pretty string literal (single-quote pole).
#[must_use]
pub fn string_literal_pretty(value: &str) -> String {
    string_literal_token(value, true, true)
}

/// Convenience: minify string literal (always double-quote pole).
#[must_use]
pub fn string_literal_minify(value: &str) -> String {
    string_literal_token(value, true, false)
}

/// Convenience: non-string literal prefers raw.
#[must_use]
pub fn non_string_literal(raw: &str) -> String {
    continue85_literal_skeleton(false, None, Some(raw), false, true)
}

// ── BlockStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle BlockStatement skeleton from already-rendered body stmts.
/// Pretty: `{\n` + `\n`-joined body + `}` when nonempty; empty `{}`.
/// Minify: `{` + tight body join + `}` (no inner newlines).
#[must_use]
pub fn continue85_block_statement_skeleton(stmts: &[&str], pretty: bool) -> String {
    let nonempty = !stmts.is_empty();
    let mut out = String::from(block_open(pretty, nonempty));
    let sep = block_item_sep(pretty);
    for (i, s) in stmts.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(s);
    }
    if pretty && nonempty {
        out.push('\n');
    }
    out.push_str(block_close());
    out
}

/// Convenience: pretty block.
#[must_use]
pub fn block_pretty(stmts: &[&str]) -> String {
    continue85_block_statement_skeleton(stmts, true)
}

/// Convenience: minify block.
#[must_use]
pub fn block_minify(stmts: &[&str]) -> String {
    continue85_block_statement_skeleton(stmts, false)
}

/// Convenience: empty block.
#[must_use]
pub fn block_empty() -> String {
    continue85_block_statement_skeleton(&[], true)
}

// ── Program dual-oracle ─────────────────────────────────────────────────────

/// Dual-oracle Program skeleton composing real [`program_statements_skeleton`].
#[must_use]
pub fn continue85_program_skeleton(stmts: &[&str], pretty: bool) -> String {
    program_statements_skeleton(stmts, pretty)
}

/// Convenience: pretty program.
#[must_use]
pub fn program_pretty(stmts: &[&str]) -> String {
    continue85_program_skeleton(stmts, true)
}

/// Convenience: minify program.
#[must_use]
pub fn program_minify(stmts: &[&str]) -> String {
    continue85_program_skeleton(stmts, false)
}

// ── EmptyStatement / DebuggerStatement dual-oracle ──────────────────────────

/// Dual-oracle EmptyStatement skeleton composing real
/// [`continue33_empty_statement_skeleton`].
#[must_use]
pub fn continue85_empty_statement_skeleton() -> &'static str {
    continue33_empty_statement_skeleton()
}

/// Dual-oracle DebuggerStatement skeleton composing real
/// [`continue32_debugger_skeleton`].
#[must_use]
pub fn continue85_debugger_statement_skeleton() -> &'static str {
    continue32_debugger_skeleton()
}

/// Convenience aliases.
#[must_use]
pub fn empty_statement() -> &'static str {
    continue85_empty_statement_skeleton()
}

#[must_use]
pub fn debugger_statement() -> &'static str {
    continue85_debugger_statement_skeleton()
}

// ── Composed dual-oracle shells ─────────────────────────────────────────────

/// Dual-oracle residual: program of `debugger;` + empty + block with id+literal.
#[must_use]
pub fn continue85_program_shell(pretty: bool) -> String {
    let id = identifier_pretty("x");
    let lit = if pretty {
        string_literal_pretty("hi")
    } else {
        string_literal_minify("hi")
    };
    let assign = if pretty {
        format!("const {id} = {lit};")
    } else {
        format!("const {id}={lit};")
    };
    let body = continue85_block_statement_skeleton(&[assign.as_str()], pretty);
    let dbg = continue85_debugger_statement_skeleton();
    let empty = continue85_empty_statement_skeleton();
    continue85_program_skeleton(&[dbg, empty, body.as_str()], pretty)
}

/// Dual-oracle residual: block holding debugger then empty.
#[must_use]
pub fn continue85_block_debugger_empty(pretty: bool) -> String {
    continue85_block_statement_skeleton(
        &[
            continue85_debugger_statement_skeleton(),
            continue85_empty_statement_skeleton(),
        ],
        pretty,
    )
}

/// Dual-oracle residual: mangled identifier inside minify program stmt.
#[must_use]
pub fn continue85_mangled_program_shell(name: &str, mangled: &str) -> String {
    let id = identifier_mangle(name, mangled);
    let stmt = format!("{id};");
    continue85_program_skeleton(&[stmt.as_str()], false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ident_literal_emit::{identifier_token, literal_token, string_literal_token};
    use crate::literal_widen_emit::{
        continue32_debugger_skeleton, continue33_empty_statement_skeleton,
    };
    use crate::method_array_import_emit::program_statements_skeleton;

    #[test]
    fn continue85_type_catalog() {
        assert_eq!(CONTINUE85_RELATED_TYPES.len(), 6);
        assert!(is_ident_block_program_related_type("Identifier"));
        assert!(is_ident_block_program_related_type("Literal"));
        assert!(is_ident_block_program_related_type("BlockStatement"));
        assert!(is_ident_block_program_related_type("Program"));
        assert!(is_ident_block_program_related_type("EmptyStatement"));
        assert!(is_ident_block_program_related_type("DebuggerStatement"));
        assert!(!is_ident_block_program_related_type("ObjectPattern"));
        assert!(!is_ident_block_program_related_type("ForAwaitStatement"));
        assert!(!is_ident_block_program_related_type("YieldExpression"));
        assert!(!is_ident_block_program_related_type("ImportSpecifier"));
        assert!(is_continue85_identifier_type("Identifier"));
        assert!(is_continue85_literal_type("Literal"));
        assert!(is_continue85_block_type("BlockStatement"));
        assert!(is_continue85_program_type("Program"));
        assert!(is_continue85_empty_statement_type("EmptyStatement"));
        assert!(is_continue85_debugger_type("DebuggerStatement"));
        assert!(!is_continue85_identifier_type("Literal"));
        assert!(!is_continue85_block_type("Program"));
    }

    #[test]
    fn continue85_ident_literal_dual_oracle() {
        assert_eq!(identifier_pretty("foo"), "foo");
        assert_eq!(identifier_minify("foo"), "foo");
        assert_eq!(identifier_mangle("longName", "a"), "a");
        assert_eq!(
            continue85_identifier_skeleton(Some("x"), true, true, Some("a")),
            identifier_token(Some("x"), true, true, Some("a"))
        );
        assert_eq!(
            continue85_identifier_skeleton(Some("x"), false, true, Some("a")),
            identifier_token(Some("x"), false, true, Some("a"))
        );
        assert_eq!(identifier_pretty("x"), identifier_minify("x"));
        assert_ne!(identifier_pretty("long"), identifier_mangle("long", "a"));

        assert_eq!(string_literal_pretty("hi"), "'hi'");
        assert_eq!(string_literal_minify("hi"), "\"hi\"");
        assert_ne!(string_literal_pretty("hi"), string_literal_minify("hi"));
        assert_eq!(
            continue85_literal_skeleton(true, Some("x"), Some("'x'"), true, true),
            literal_token(true, Some("x"), Some("'x'"), true, true)
        );
        assert_eq!(
            continue85_literal_skeleton(true, Some("x"), Some("'x'"), true, false),
            string_literal_token("x", true, false)
        );
        assert_eq!(non_string_literal("0x2a"), "0x2a");
        assert_eq!(
            continue85_literal_skeleton(false, Some("42"), Some("0x2a"), false, true),
            "0x2a"
        );
    }

    #[test]
    fn continue85_block_program_empty_debugger_compose_dual_oracle() {
        assert_eq!(block_empty(), "{}");
        assert_eq!(block_pretty(&["a;"]), "{\na;\n}");
        assert_eq!(block_minify(&["a;"]), "{a;}");
        assert_ne!(block_pretty(&["a;"]), block_minify(&["a;"]));
        assert_eq!(
            block_pretty(&["a;", "b;"]),
            "{\na;\nb;\n}"
        );
        assert_eq!(block_minify(&["a;", "b;"]), "{a;b;}");

        assert_eq!(
            program_pretty(&["const a = 1;", "const b = 2;"]),
            "const a = 1;\n\nconst b = 2;"
        );
        assert_eq!(
            program_minify(&["a;", "b;"]),
            "a;b;"
        );
        assert_eq!(
            continue85_program_skeleton(&["only"], true),
            program_statements_skeleton(&["only"], true)
        );
        assert_ne!(
            program_pretty(&["a;", "b;"]),
            program_minify(&["a;", "b;"])
        );

        assert_eq!(empty_statement(), ";");
        assert_eq!(debugger_statement(), "debugger;");
        assert_eq!(
            continue85_empty_statement_skeleton(),
            continue33_empty_statement_skeleton()
        );
        assert_eq!(
            continue85_debugger_statement_skeleton(),
            continue32_debugger_skeleton()
        );
        assert_ne!(empty_statement(), debugger_statement());

        let shell = continue85_program_shell(true);
        assert!(shell.contains("debugger;"));
        assert!(shell.contains(';'));
        assert!(shell.contains("const x = 'hi';") || shell.contains("const x='hi';"));
        let shell_mini = continue85_program_shell(false);
        assert!(shell_mini.contains("debugger;"));
        assert!(shell_mini.contains("\"hi\""));
        assert_ne!(shell, shell_mini);

        let blk = continue85_block_debugger_empty(true);
        assert!(blk.starts_with("{\n"));
        assert!(blk.contains("debugger;"));
        let blk_mini = continue85_block_debugger_empty(false);
        assert_eq!(blk_mini, "{debugger;;}");
        assert_ne!(blk, blk_mini);

        let mangled = continue85_mangled_program_shell("longName", "a");
        assert_eq!(mangled, "a;");
    }
}

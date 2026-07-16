//! Pure ForStatement + ForInStatement + ForOfStatement + ThrowStatement +
//! LabeledStatement + EmptyStatement dual-oracle emission
//! — residual pure **continue113** for tooling/format-minify-lint.
//!
//! New AST emit skeletons **not** covered by prior dens modules continue71–112:
//! - ForStatement dual-oracle composing real `continue46_for_skeleton`
//! - ForInStatement dual-oracle composing real `continue46_for_in_skeleton`
//! - ForOfStatement dual-oracle composing real `continue46_for_of_skeleton`
//! - ThrowStatement dual-oracle composing real `continue46_throw_skeleton`
//! - LabeledStatement dual-oracle composing real `continue46_label_skeleton`
//! - EmptyStatement dual-oracle composing real `continue46_empty_skeleton`
//! - Labeled continue dual-oracle composing real
//!   `continue46_continue_label_skeleton`
//! - Composed for/throw/label/empty residual shells
//!
//! Intentionally does **not** re-wrap continue112 do-while/switch/break/
//! continue/try poles (continue45 bases), continue111 jsx element/fragment/
//! attr poles (continue44 bases), or continue46 JSX member/namespace/spread/
//! open/close/empty poles already dual-oracled under continue88.
//! Composes real shipped pure helpers from continue46 control bases. Full
//! engines remain product dens. NO authority_rust / ts_deleted.
//! dens ≠ flip; PreferRust OFF.

use crate::literal_widen_emit::{
    continue46_continue_label_skeleton, continue46_empty_skeleton, continue46_for_in_skeleton,
    continue46_for_of_skeleton, continue46_for_skeleton, continue46_label_skeleton,
    continue46_throw_skeleton,
};

/// Dual-oracle residual: continue113 related AST type catalog.
pub const CONTINUE113_RELATED_TYPES: &[&str] = &[
    "ForStatement",
    "ForInStatement",
    "ForOfStatement",
    "ThrowStatement",
    "LabeledStatement",
    "EmptyStatement",
];

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_for_throw_label_empty_related_type(t: &str) -> bool {
    CONTINUE113_RELATED_TYPES.contains(&t)
}

#[must_use]
pub fn is_continue113_for_type(t: &str) -> bool {
    t == "ForStatement"
}

#[must_use]
pub fn is_continue113_for_in_type(t: &str) -> bool {
    t == "ForInStatement"
}

#[must_use]
pub fn is_continue113_for_of_type(t: &str) -> bool {
    t == "ForOfStatement"
}

#[must_use]
pub fn is_continue113_throw_type(t: &str) -> bool {
    t == "ThrowStatement"
}

#[must_use]
pub fn is_continue113_labeled_type(t: &str) -> bool {
    t == "LabeledStatement"
}

#[must_use]
pub fn is_continue113_empty_type(t: &str) -> bool {
    t == "EmptyStatement"
}

#[must_use]
pub fn is_continue113_loop_type(t: &str) -> bool {
    matches!(t, "ForStatement" | "ForInStatement" | "ForOfStatement")
}

#[must_use]
pub fn is_continue113_type(t: &str) -> bool {
    matches!(
        t,
        "ForStatement"
            | "ForInStatement"
            | "ForOfStatement"
            | "ThrowStatement"
            | "LabeledStatement"
            | "EmptyStatement"
    )
}

// ── ForStatement dual-oracle ────────────────────────────────────────────────

/// Dual-oracle ForStatement skeleton composing real
/// [`continue46_for_skeleton`].
#[must_use]
pub fn continue113_for_skeleton(init: &str, test: &str, update: &str, body: &str) -> String {
    continue46_for_skeleton(init, test, update, body)
}

/// Dual-oracle ForStatement pretty alias.
#[must_use]
pub fn continue113_for_pretty(init: &str, test: &str, update: &str, body: &str) -> String {
    continue113_for_skeleton(init, test, update, body)
}

/// Dual-oracle ForStatement minify alias.
#[must_use]
pub fn continue113_for_minify(init: &str, test: &str, update: &str, body: &str) -> String {
    continue113_for_skeleton(init, test, update, body)
}

// ── ForInStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ForInStatement skeleton composing real
/// [`continue46_for_in_skeleton`].
#[must_use]
pub fn continue113_for_in_skeleton(left: &str, right: &str, body: &str) -> String {
    continue46_for_in_skeleton(left, right, body)
}

/// Dual-oracle ForInStatement pretty alias.
#[must_use]
pub fn continue113_for_in_pretty(left: &str, right: &str, body: &str) -> String {
    continue113_for_in_skeleton(left, right, body)
}

/// Dual-oracle ForInStatement minify alias.
#[must_use]
pub fn continue113_for_in_minify(left: &str, right: &str, body: &str) -> String {
    continue113_for_in_skeleton(left, right, body)
}

// ── ForOfStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ForOfStatement skeleton composing real
/// [`continue46_for_of_skeleton`].
#[must_use]
pub fn continue113_for_of_skeleton(left: &str, right: &str, body: &str) -> String {
    continue46_for_of_skeleton(left, right, body)
}

/// Dual-oracle ForOfStatement pretty alias.
#[must_use]
pub fn continue113_for_of_pretty(left: &str, right: &str, body: &str) -> String {
    continue113_for_of_skeleton(left, right, body)
}

/// Dual-oracle ForOfStatement minify alias.
#[must_use]
pub fn continue113_for_of_minify(left: &str, right: &str, body: &str) -> String {
    continue113_for_of_skeleton(left, right, body)
}

// ── ThrowStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle ThrowStatement skeleton composing real
/// [`continue46_throw_skeleton`].
#[must_use]
pub fn continue113_throw_skeleton(arg: &str) -> String {
    continue46_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement pretty alias.
#[must_use]
pub fn continue113_throw_pretty(arg: &str) -> String {
    continue113_throw_skeleton(arg)
}

/// Dual-oracle ThrowStatement minify alias.
#[must_use]
pub fn continue113_throw_minify(arg: &str) -> String {
    continue113_throw_skeleton(arg)
}

// ── LabeledStatement dual-oracle ────────────────────────────────────────────

/// Dual-oracle LabeledStatement skeleton composing real
/// [`continue46_label_skeleton`].
#[must_use]
pub fn continue113_label_skeleton(label: &str, body: &str) -> String {
    continue46_label_skeleton(label, body)
}

/// Dual-oracle LabeledStatement pretty alias.
#[must_use]
pub fn continue113_label_pretty(label: &str, body: &str) -> String {
    continue113_label_skeleton(label, body)
}

/// Dual-oracle LabeledStatement minify alias.
#[must_use]
pub fn continue113_label_minify(label: &str, body: &str) -> String {
    continue113_label_skeleton(label, body)
}

// ── EmptyStatement dual-oracle ──────────────────────────────────────────────

/// Dual-oracle EmptyStatement skeleton composing real
/// [`continue46_empty_skeleton`].
#[must_use]
pub fn continue113_empty_skeleton() -> &'static str {
    continue46_empty_skeleton()
}

/// Dual-oracle EmptyStatement pretty alias.
#[must_use]
pub fn continue113_empty_pretty() -> &'static str {
    continue113_empty_skeleton()
}

/// Dual-oracle EmptyStatement minify alias.
#[must_use]
pub fn continue113_empty_minify() -> &'static str {
    continue113_empty_skeleton()
}

// ── Labeled continue dual-oracle ────────────────────────────────────────────

/// Dual-oracle labeled continue skeleton composing real
/// [`continue46_continue_label_skeleton`].
#[must_use]
pub fn continue113_continue_label_skeleton(label: &str) -> String {
    continue46_continue_label_skeleton(label)
}

/// Dual-oracle labeled continue pretty alias.
#[must_use]
pub fn continue113_continue_label_pretty(label: &str) -> String {
    continue113_continue_label_skeleton(label)
}

/// Dual-oracle labeled continue minify alias.
#[must_use]
pub fn continue113_continue_label_minify(label: &str) -> String {
    continue113_continue_label_skeleton(label)
}

// ── Composed residual shells ────────────────────────────────────────────────

/// Dual-oracle residual: empty-body for (`for (;; ) {}` with given parts).
#[must_use]
pub fn continue113_for_empty(init: &str, test: &str, update: &str) -> String {
    continue113_for_skeleton(init, test, update, "{}")
}

/// Dual-oracle residual: empty-body for-in.
#[must_use]
pub fn continue113_for_in_empty(left: &str, right: &str) -> String {
    continue113_for_in_skeleton(left, right, "{}")
}

/// Dual-oracle residual: empty-body for-of.
#[must_use]
pub fn continue113_for_of_empty(left: &str, right: &str) -> String {
    continue113_for_of_skeleton(left, right, "{}")
}

/// Dual-oracle residual: for body containing a throw seed.
#[must_use]
pub fn continue113_for_throw(init: &str, test: &str, update: &str, arg: &str) -> String {
    let thr = continue113_throw_skeleton(arg);
    continue113_for_skeleton(init, test, update, &format!("{{ {thr} }}"))
}

/// Dual-oracle residual: labeled for shell seed.
#[must_use]
pub fn continue113_label_for(
    label: &str,
    init: &str,
    test: &str,
    update: &str,
    body: &str,
) -> String {
    let inner = continue113_for_skeleton(init, test, update, body);
    continue113_label_skeleton(label, &inner)
}

/// Dual-oracle residual: labeled for body containing labeled continue.
#[must_use]
pub fn continue113_label_for_continue(
    label: &str,
    init: &str,
    test: &str,
    update: &str,
) -> String {
    let c = continue113_continue_label_skeleton(label);
    let body = format!("{{ {c} }}");
    continue113_label_for(label, init, test, update, &body)
}

/// Dual-oracle residual: for-of then throw composition seed.
#[must_use]
pub fn continue113_for_of_then_throw(left: &str, right: &str, arg: &str) -> String {
    format!(
        "{} {}",
        continue113_for_of_empty(left, right),
        continue113_throw_skeleton(arg)
    )
}

/// Dual-oracle residual: empty statement then throw composition seed.
#[must_use]
pub fn continue113_empty_then_throw(arg: &str) -> String {
    format!(
        "{} {}",
        continue113_empty_skeleton(),
        continue113_throw_skeleton(arg)
    )
}

/// Dual-oracle residual: separator pole for compose readability (pretty vs tight).
#[must_use]
pub fn continue113_sep(pretty: bool) -> &'static str {
    if pretty {
        " "
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal_widen_emit::{
        continue46_continue_label_skeleton, continue46_empty_skeleton, continue46_for_in_skeleton,
        continue46_for_of_skeleton, continue46_for_skeleton, continue46_label_skeleton,
        continue46_throw_skeleton,
    };

    #[test]
    fn continue113_type_catalog() {
        assert_eq!(CONTINUE113_RELATED_TYPES.len(), 6);
        assert!(is_for_throw_label_empty_related_type("ForStatement"));
        assert!(is_for_throw_label_empty_related_type("ForInStatement"));
        assert!(is_for_throw_label_empty_related_type("ForOfStatement"));
        assert!(is_for_throw_label_empty_related_type("ThrowStatement"));
        assert!(is_for_throw_label_empty_related_type("LabeledStatement"));
        assert!(is_for_throw_label_empty_related_type("EmptyStatement"));
        assert!(!is_for_throw_label_empty_related_type("DoWhileStatement"));
        assert!(!is_for_throw_label_empty_related_type("JSXElement"));

        assert!(is_continue113_for_type("ForStatement"));
        assert!(!is_continue113_for_type("ForInStatement"));
        assert!(is_continue113_for_in_type("ForInStatement"));
        assert!(is_continue113_for_of_type("ForOfStatement"));
        assert!(is_continue113_throw_type("ThrowStatement"));
        assert!(is_continue113_labeled_type("LabeledStatement"));
        assert!(is_continue113_empty_type("EmptyStatement"));
        assert!(is_continue113_loop_type("ForStatement"));
        assert!(is_continue113_loop_type("ForInStatement"));
        assert!(is_continue113_loop_type("ForOfStatement"));
        assert!(!is_continue113_loop_type("ThrowStatement"));
        assert!(is_continue113_type("ForStatement"));
        assert!(is_continue113_type("EmptyStatement"));
        assert!(!is_continue113_type("WhileStatement"));
    }

    #[test]
    fn continue113_for_family_emit() {
        assert_eq!(
            continue113_for_skeleton("let i = 0", "i < 3", "i++", "{}"),
            "for (let i = 0; i < 3; i++) {}"
        );
        assert_eq!(
            continue113_for_skeleton("let i = 0", "i < 3", "i++", "{}"),
            continue46_for_skeleton("let i = 0", "i < 3", "i++", "{}")
        );
        assert_eq!(
            continue113_for_pretty("a", "b", "c", "{ d; }"),
            continue113_for_minify("a", "b", "c", "{ d; }")
        );

        assert_eq!(
            continue113_for_in_skeleton("const k", "obj", "{}"),
            "for (const k in obj) {}"
        );
        assert_eq!(
            continue113_for_in_skeleton("const k", "obj", "{}"),
            continue46_for_in_skeleton("const k", "obj", "{}")
        );
        assert_eq!(
            continue113_for_in_pretty("k", "o", "{ a; }"),
            continue113_for_in_minify("k", "o", "{ a; }")
        );

        assert_eq!(
            continue113_for_of_skeleton("const x", "xs", "{}"),
            "for (const x of xs) {}"
        );
        assert_eq!(
            continue113_for_of_skeleton("const x", "xs", "{}"),
            continue46_for_of_skeleton("const x", "xs", "{}")
        );
        assert_eq!(
            continue113_for_of_pretty("x", "xs", "{ a; }"),
            continue113_for_of_minify("x", "xs", "{ a; }")
        );
    }

    #[test]
    fn continue113_throw_label_empty_emit() {
        assert_eq!(continue113_throw_skeleton("err"), "throw err;");
        assert_eq!(
            continue113_throw_skeleton("err"),
            continue46_throw_skeleton("err")
        );
        assert_eq!(
            continue113_throw_pretty("e"),
            continue113_throw_minify("e")
        );

        assert_eq!(
            continue113_label_skeleton("loop", "while (1) {}"),
            "loop: while (1) {}"
        );
        assert_eq!(
            continue113_label_skeleton("loop", "while (1) {}"),
            continue46_label_skeleton("loop", "while (1) {}")
        );
        assert_eq!(
            continue113_label_pretty("L", "{}"),
            continue113_label_minify("L", "{}")
        );

        assert_eq!(continue113_empty_skeleton(), ";");
        assert_eq!(continue113_empty_skeleton(), continue46_empty_skeleton());
        assert_eq!(continue113_empty_pretty(), continue113_empty_minify());

        assert_eq!(
            continue113_continue_label_skeleton("loop"),
            "continue loop;"
        );
        assert_eq!(
            continue113_continue_label_skeleton("loop"),
            continue46_continue_label_skeleton("loop")
        );
        assert_eq!(
            continue113_continue_label_pretty("outer"),
            continue113_continue_label_minify("outer")
        );
    }

    #[test]
    fn continue113_composed_residual_shells() {
        assert_eq!(
            continue113_for_empty("let i = 0", "i < 1", "i++"),
            "for (let i = 0; i < 1; i++) {}"
        );
        assert_eq!(
            continue113_for_in_empty("const k", "obj"),
            "for (const k in obj) {}"
        );
        assert_eq!(
            continue113_for_of_empty("const x", "xs"),
            "for (const x of xs) {}"
        );
        assert_eq!(
            continue113_for_throw("let i = 0", "i < 1", "i++", "e"),
            "for (let i = 0; i < 1; i++) { throw e; }"
        );
        assert_eq!(
            continue113_label_for("L", "let i = 0", "i < 1", "i++", "{}"),
            "L: for (let i = 0; i < 1; i++) {}"
        );
        assert_eq!(
            continue113_label_for_continue("L", "let i = 0", "i < 1", "i++"),
            "L: for (let i = 0; i < 1; i++) { continue L; }"
        );
        assert_eq!(
            continue113_for_of_then_throw("const x", "xs", "err"),
            "for (const x of xs) {} throw err;"
        );
        assert_eq!(
            continue113_empty_then_throw("err"),
            "; throw err;"
        );
        assert_eq!(continue113_sep(true), " ");
        assert_eq!(continue113_sep(false), "");
        assert_ne!(continue113_sep(true), continue113_sep(false));
    }
}

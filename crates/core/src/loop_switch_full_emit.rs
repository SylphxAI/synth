//! Pure loop / switch / template full-statement emission —
//! residual pure continue18 for tooling/format-minify-lint fragment.
//! Assembles continue7 token fragments into full statement skeletons
//! (for / for-in / for-of / while / do-while / switch / template).
//! Full engines remain product residual. intentional ts_only×3 retained.
//! NO authority_rust / ts_deleted.

use crate::loop_template_emit::{
    case_colon, case_prefix, default_label, do_token, do_while_open, for_close, for_in_token,
    for_of_token, for_open, is_loop_statement_type, is_switch_related_type, is_template_type,
    switch_close, switch_open, template_expr_close, template_expr_open, template_tick, while_close,
    while_open,
};

/// Whether a type is covered by this residual unit surface.
#[must_use]
pub fn is_loop_switch_full_related_type(t: &str) -> bool {
    is_loop_statement_type(t)
        || is_switch_related_type(t)
        || is_template_type(t)
        || matches!(t, "ArrayPattern" | "AssignmentPattern" | "PrivateIdentifier")
}

/// Whether node type is ArrayPattern.
#[must_use]
pub fn is_array_pattern_type(t: &str) -> bool {
    t == "ArrayPattern"
}

/// Whether node type is AssignmentPattern.
#[must_use]
pub fn is_assignment_pattern_type(t: &str) -> bool {
    t == "AssignmentPattern"
}

/// Whether node type is PrivateIdentifier (`#x`).
#[must_use]
pub fn is_private_identifier_type(t: &str) -> bool {
    t == "PrivateIdentifier"
}

/// ForStatement skeleton: `for (init; test; update) body`.
/// Empty clauses omit text (TS residual often leaves empty slots).
#[must_use]
pub fn for_statement_skeleton(
    init: Option<&str>,
    test: Option<&str>,
    update: Option<&str>,
    body: &str,
    pretty: bool,
) -> String {
    let sep = if pretty { "; " } else { ";" };
    let mut header = String::new();
    if let Some(i) = init {
        header.push_str(i);
    }
    header.push_str(sep);
    if let Some(t) = test {
        header.push_str(t);
    }
    header.push_str(sep);
    if let Some(u) = update {
        header.push_str(u);
    }
    format!(
        "{}{}{}{}",
        for_open(pretty),
        header,
        for_close(pretty).trim_end(),
        body_spacing(body, pretty)
    )
}

/// ForInStatement skeleton: `for (left in right) body`.
#[must_use]
pub fn for_in_statement_skeleton(left: &str, right: &str, body: &str, pretty: bool) -> String {
    format!(
        "{}{}{}{}{}{}",
        for_open(pretty),
        left,
        for_in_token(),
        right,
        for_close(pretty).trim_end(),
        body_spacing(body, pretty)
    )
}

/// ForOfStatement skeleton: `for (left of right) body` (await-of not migrated).
#[must_use]
pub fn for_of_statement_skeleton(left: &str, right: &str, body: &str, pretty: bool) -> String {
    format!(
        "{}{}{}{}{}{}",
        for_open(pretty),
        left,
        for_of_token(),
        right,
        for_close(pretty).trim_end(),
        body_spacing(body, pretty)
    )
}

/// WhileStatement skeleton: `while (test) body`.
#[must_use]
pub fn while_statement_skeleton(test: &str, body: &str, pretty: bool) -> String {
    format!(
        "{}{}{}{}",
        while_open(pretty),
        test,
        while_close(pretty).trim_end(),
        body_spacing(body, pretty)
    )
}

/// DoWhileStatement skeleton: `do body while (test);`
#[must_use]
pub fn do_while_statement_skeleton(body: &str, test: &str, pretty: bool, semi: bool) -> String {
    let mut out = String::from(do_token());
    if pretty {
        // do_token already ends with space
        out.push_str(body.trim_start());
    } else {
        out = format!("do{body}");
    }
    out.push_str(do_while_open(pretty));
    out.push_str(test);
    out.push(')');
    if semi {
        out.push(';');
    }
    out
}

/// One switch case dual-oracle: `case test: consequent` or `default: consequent`.
#[must_use]
pub fn switch_case_skeleton(test: Option<&str>, consequent: &str, pretty: bool) -> String {
    let mut out = String::new();
    match test {
        Some(t) => {
            out.push_str(case_prefix());
            out.push_str(t);
        }
        None => out.push_str(default_label()),
    }
    out.push_str(case_colon(pretty));
    out.push_str(consequent);
    out
}

/// SwitchStatement skeleton: `switch (disc) { cases... }`.
#[must_use]
pub fn switch_statement_skeleton(discriminant: &str, cases: &[&str], pretty: bool) -> String {
    let mut out = String::new();
    out.push_str(switch_open(pretty));
    out.push_str(discriminant);
    out.push_str(switch_close(pretty).trim_end());
    if pretty {
        out.push_str(" {");
        if !cases.is_empty() {
            out.push('\n');
            for c in cases {
                out.push_str("  ");
                out.push_str(c);
                out.push('\n');
            }
        }
        out.push('}');
    } else {
        out.push('{');
        for c in cases {
            out.push_str(c);
        }
        out.push('}');
    }
    out
}

/// TemplateLiteral skeleton from cooked quasis + expressions.
/// `quasis.len() == exprs.len() + 1` (ESTree invariant).
#[must_use]
pub fn template_literal_skeleton(quasis: &[&str], exprs: &[&str]) -> String {
    let mut out = String::from(template_tick());
    for (i, q) in quasis.iter().enumerate() {
        out.push_str(q);
        if let Some(e) = exprs.get(i) {
            out.push_str(template_expr_open());
            out.push_str(e);
            out.push_str(template_expr_close());
        }
    }
    out.push_str(template_tick());
    out
}

/// PrivateIdentifier token: `#name`.
#[must_use]
pub fn private_identifier_token(name: &str) -> String {
    format!("#{name}")
}

/// ArrayPattern skeleton: `[a, b = 1, ...rest]` elements already rendered.
#[must_use]
pub fn array_pattern_skeleton(elements: &[&str], pretty: bool) -> String {
    if pretty {
        format!("[{}]", elements.join(", "))
    } else {
        format!("[{}]", elements.join(","))
    }
}

/// AssignmentPattern skeleton: `left = right`.
#[must_use]
pub fn assignment_pattern_skeleton(left: &str, right: &str, pretty: bool) -> String {
    if pretty {
        format!("{left} = {right}")
    } else {
        format!("{left}={right}")
    }
}

fn body_spacing(body: &str, pretty: bool) -> String {
    // Pretty always inserts a leading space before body (block or statement form).
    if pretty {
        format!(" {body}")
    } else {
        body.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_guards() {
        assert!(is_loop_switch_full_related_type("ForOfStatement"));
        assert!(is_loop_switch_full_related_type("SwitchCase"));
        assert!(is_loop_switch_full_related_type("TemplateLiteral"));
        assert!(is_array_pattern_type("ArrayPattern"));
        assert!(is_assignment_pattern_type("AssignmentPattern"));
        assert!(is_private_identifier_type("PrivateIdentifier"));
        assert!(!is_loop_switch_full_related_type("BinaryExpression"));
    }

    #[test]
    fn for_while_switch_templates() {
        assert_eq!(
            for_statement_skeleton(Some("let i=0"), Some("i<3"), Some("i++"), "{}", true),
            "for (let i=0; i<3; i++) {}"
        );
        assert_eq!(
            for_statement_skeleton(Some("let i=0"), Some("i<3"), Some("i++"), "{}", false),
            "for(let i=0;i<3;i++){}"
        );
        assert_eq!(
            for_in_statement_skeleton("k", "obj", "{}", true),
            "for (k in obj) {}"
        );
        assert_eq!(
            for_of_statement_skeleton("x", "arr", "{}", false),
            "for(x of arr){}"
        );
        assert_eq!(
            while_statement_skeleton("ok", "work()", true),
            "while (ok) work()"
        );
        assert_eq!(
            do_while_statement_skeleton("{}", "x", true, true),
            "do {} while (x);"
        );
        assert_eq!(
            do_while_statement_skeleton("{}", "x", false, true),
            "do{}while(x);"
        );

        let case0 = switch_case_skeleton(Some("1"), "break;", true);
        assert_eq!(case0, "case 1: break;");
        let def = switch_case_skeleton(None, "return;", false);
        assert_eq!(def, "default:return;");
        assert_eq!(
            switch_statement_skeleton("x", &["case 1: break;"], false),
            "switch(x){case 1: break;}"
        );

        assert_eq!(
            template_literal_skeleton(&["a", "b"], &["x"]),
            "`a${x}b`"
        );
        assert_eq!(private_identifier_token("foo"), "#foo");
        assert_eq!(array_pattern_skeleton(&["a", "b"], true), "[a, b]");
        assert_eq!(
            assignment_pattern_skeleton("a", "1", false),
            "a=1"
        );
    }
}

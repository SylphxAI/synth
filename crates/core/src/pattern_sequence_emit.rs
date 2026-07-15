//! Pure ObjectPattern / Rest / for-await / tagged-template emission —
//! residual pure continue19 for tooling/format-minify-lint fragment.
//! Complements continue18 loop/switch/template + pattern skeletons.
//! SequenceExpression dens lives in assign_logical_update (continue15).
//! Full engines remain product dens. intentional ts_only×3 retained.
//! NO authority_rust / ts_deleted.

use crate::assign_logical_update_emit::sequence_expression_skeleton;
use crate::loop_template_emit::{for_of_token, template_tick};

/// Whether a type is covered by this residual dens surface.
#[must_use]
pub fn is_pattern_sequence_related_type(t: &str) -> bool {
    matches!(
        t,
        "ObjectPattern"
            | "RestElement"
            | "ForAwaitStatement"
            | "TaggedTemplateExpression"
            | "SpreadElement"
    )
}

/// Whether node type is ObjectPattern.
#[must_use]
pub fn is_object_pattern_type(t: &str) -> bool {
    t == "ObjectPattern"
}

/// Whether node type is RestElement.
#[must_use]
pub fn is_rest_element_type(t: &str) -> bool {
    t == "RestElement"
}

/// Whether node type is TaggedTemplateExpression.
#[must_use]
pub fn is_tagged_template_expression_type(t: &str) -> bool {
    t == "TaggedTemplateExpression"
}

/// RestElement token: `...arg`.
#[must_use]
pub fn rest_element_token(arg: &str) -> String {
    format!("...{arg}")
}

/// SpreadElement token (expression context): `...arg`.
#[must_use]
pub fn spread_element_token(arg: &str) -> String {
    format!("...{arg}")
}

/// ObjectPattern skeleton: `{a, b = 1, ...rest}` properties already rendered.
#[must_use]
pub fn object_pattern_skeleton(properties: &[&str], pretty: bool) -> String {
    if properties.is_empty() {
        return "{}".to_string();
    }
    if pretty {
        format!("{{ {} }}", properties.join(", "))
    } else {
        format!("{{{}}}", properties.join(","))
    }
}

/// Parenthesized SequenceExpression when explicit: `(a, b)`.
/// Complements continue15 `sequence_expression_skeleton`.
#[must_use]
pub fn sequence_expression_paren_skeleton(exprs: &[&str], pretty: bool) -> String {
    format!("({})", sequence_expression_skeleton(exprs, pretty))
}

/// For-await-of skeleton: `for await (left of right) body`.
#[must_use]
pub fn for_await_of_statement_skeleton(
    left: &str,
    right: &str,
    body: &str,
    pretty: bool,
) -> String {
    if pretty {
        format!(
            "for await ({}{}{}){}",
            left,
            for_of_token(),
            right,
            body_spacing(body, true)
        )
    } else {
        format!(
            "forawait({}{}{}){}",
            left,
            for_of_token().trim(),
            right,
            body_spacing(body, false)
        )
    }
}

/// TaggedTemplateExpression skeleton: `tag\`quasi\${expr}\``.
/// `quasi` is the already-rendered TemplateLiteral body including ticks.
#[must_use]
pub fn tagged_template_expression_skeleton(tag: &str, quasi: &str) -> String {
    format!("{tag}{quasi}")
}

/// Build a minimal tagged template with one cooked quasi and no expressions.
#[must_use]
pub fn tagged_template_simple(tag: &str, cooked: &str) -> String {
    format!("{tag}{}{cooked}{}", template_tick(), template_tick())
}

/// Property assignment pattern inside ObjectPattern: `key = default`.
#[must_use]
pub fn object_pattern_assign_property(key: &str, default: &str, pretty: bool) -> String {
    if pretty {
        format!("{key} = {default}")
    } else {
        format!("{key}={default}")
    }
}

/// Shorthand property inside ObjectPattern: just the binding name.
#[must_use]
pub fn object_pattern_shorthand(name: &str) -> String {
    name.to_owned()
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
        assert!(is_pattern_sequence_related_type("ObjectPattern"));
        assert!(is_pattern_sequence_related_type("RestElement"));
        assert!(is_pattern_sequence_related_type("ForAwaitStatement"));
        assert!(is_pattern_sequence_related_type("TaggedTemplateExpression"));
        assert!(is_object_pattern_type("ObjectPattern"));
        assert!(is_rest_element_type("RestElement"));
        assert!(is_tagged_template_expression_type("TaggedTemplateExpression"));
        assert!(!is_pattern_sequence_related_type("ForStatement"));
        assert!(!is_pattern_sequence_related_type("SequenceExpression"));
    }

    #[test]
    fn object_rest_tagged_for_await() {
        assert_eq!(rest_element_token("rest"), "...rest");
        assert_eq!(spread_element_token("xs"), "...xs");
        assert_eq!(object_pattern_skeleton(&[], true), "{}");
        assert_eq!(
            object_pattern_skeleton(&["a", "b=1", "...rest"], true),
            "{ a, b=1, ...rest }"
        );
        assert_eq!(
            object_pattern_skeleton(&["a", "b"], false),
            "{a,b}"
        );
        assert_eq!(
            object_pattern_assign_property("x", "0", true),
            "x = 0"
        );
        assert_eq!(object_pattern_shorthand("id"), "id");

        assert_eq!(
            sequence_expression_paren_skeleton(&["a", "b"], true),
            "(a, b)"
        );

        assert_eq!(
            for_await_of_statement_skeleton("x", "asyncIter", "{}", true),
            "for await (x of asyncIter) {}"
        );
        assert_eq!(
            for_await_of_statement_skeleton("x", "it", "body()", false),
            "forawait(xofit)body()"
        );

        assert_eq!(
            tagged_template_expression_skeleton("String.raw", "`hi`"),
            "String.raw`hi`"
        );
        assert_eq!(tagged_template_simple("html", "div"), "html`div`");
    }
}

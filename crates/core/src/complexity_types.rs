//! Pure decision/nesting node type classification —
//! mirrors `packages/synth-metrics/src/analyzer.ts` isDecisionNode/isNestingNode.
//! PORTFOLIO-PRODUCTS-WAVE6 pure residual. NO authority_rust / ts_deleted.
//! AST traversal remains TS; this is the pure type-name kernel only.

/// Decision-point type fragments (lowercase substring match).
const DECISION_TYPES: &[&str] = &[
    "if", "switch", "case", "catch", "&&", "||", "?", "for", "while",
];

/// Nesting-increasing type fragments (lowercase substring match).
const NESTING_TYPES: &[&str] = &[
    "if", "for", "while", "do", "switch", "function", "method", "class",
];

/// Expanded decision types used by countDecisionPoints (broader set).
const DECISION_COUNT_TYPES: &[&str] = &[
    "if",
    "else",
    "for",
    "while",
    "do",
    "switch",
    "case",
    "catch",
    "and",
    "or",
    "&&",
    "||",
    "?",
    "conditionalexpression",
    "ifstatement",
    "forstatement",
    "whilestatement",
    "dowhilestatement",
    "switchcase",
    "catchclause",
    "logicalexpression",
];

/// True when node type string is a decision point (isDecisionNode).
#[must_use]
pub fn is_decision_node_type(node_type: &str) -> bool {
    let lower = node_type.to_ascii_lowercase();
    DECISION_TYPES
        .iter()
        .any(|t| lower.contains(t))
}

/// True when node type increases nesting (isNestingNode).
#[must_use]
pub fn is_nesting_node_type(node_type: &str) -> bool {
    let lower = node_type.to_ascii_lowercase();
    NESTING_TYPES
        .iter()
        .any(|t| lower.contains(t))
}

/// True when node type counts toward decision-point tally (countDecisionPoints).
#[must_use]
pub fn is_decision_count_type(node_type: &str) -> bool {
    let lower = node_type.to_ascii_lowercase();
    DECISION_COUNT_TYPES
        .iter()
        .any(|t| lower.contains(t))
}

/// Cyclomatic complexity from decision-point count: decisions + 1.
#[must_use]
pub fn cyclomatic_from_decisions(decision_points: u32) -> u32 {
    decision_points.saturating_add(1)
}

/// Cognitive complexity contribution for a decision at nesting depth.
/// Parity: complexity += 1 + nesting when isDecision.
#[must_use]
pub fn cognitive_decision_weight(nesting: u32) -> u32 {
    1u32.saturating_add(nesting)
}

/// Next nesting depth after visiting a node.
#[must_use]
pub fn next_nesting(current: u32, node_type: &str) -> u32 {
    if is_nesting_node_type(node_type) {
        current.saturating_add(1)
    } else {
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decision_and_nesting_types() {
        // TS isDecisionNode uses short fragment list with includes()
        assert!(is_decision_node_type("IfStatement"));
        assert!(is_decision_node_type("for"));
        assert!(is_decision_node_type("WhileStatement"));
        assert!(is_decision_node_type("SwitchCase"));
        // LogicalExpression is NOT in the short isDecisionNode list
        assert!(!is_decision_node_type("LogicalExpression"));
        // Identifier contains "if" substring — TS includes() true; keep parity
        assert!(is_decision_node_type("Identifier"));
        assert!(is_nesting_node_type("FunctionDeclaration"));
        assert!(is_nesting_node_type("WhileStatement"));
        assert!(!is_nesting_node_type("Literal"));
        assert!(!is_nesting_node_type("StringLiteral"));
    }

    #[test]
    fn count_types_broader() {
        assert!(is_decision_count_type("ElseClause"));
        assert!(is_decision_count_type("CatchClause"));
        assert!(is_decision_count_type("ConditionalExpression"));
        assert!(is_decision_count_type("LogicalExpression"));
    }

    #[test]
    fn cyclomatic_and_cognitive() {
        assert_eq!(cyclomatic_from_decisions(0), 1);
        assert_eq!(cyclomatic_from_decisions(3), 4);
        assert_eq!(cognitive_decision_weight(0), 1);
        assert_eq!(cognitive_decision_weight(2), 3);
        assert_eq!(next_nesting(0, "if"), 1);
        // "Identifier" contains "if" → nesting match (TS includes parity)
        assert_eq!(next_nesting(1, "Identifier"), 2);
        // clean non-nesting type without decision substrings
        assert_eq!(next_nesting(1, "StringLiteral"), 1);
    }
}

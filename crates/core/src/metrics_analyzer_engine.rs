//! Product **metrics analyzer engine** (tooling residual).
//!
//! Ports `packages/synth-metrics/src/analyzer.ts` MetricsAnalyzer onto the Rust
//! `Tree` model: basic LOC/depth, cyclomatic/cognitive complexity, Halstead,
//! maintainability, and per-function metrics.
//!
//! Reuses pure kernels (`metrics_basic`, `halstead_math`, `halstead_classify`,
//! `complexity_types`, `function_types`, `function_meta`). Full product path;
//! TS package remains dual-oracle/npm surface until cutover.
//! NO authority_rust / ts_deleted.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::complexity_types::{
    cognitive_decision_weight, cyclomatic_from_decisions, is_decision_count_type,
    is_decision_node_type, is_nesting_node_type,
};
use crate::function_meta::{function_loc, function_name, function_param_count};
use crate::function_types::{avg_depth_of, is_function_node_type, max_depth_of};
use crate::halstead_classify::{is_operand_node_type, is_operator_node_type, operand_value};
use crate::halstead_math::{
    calculate_maintainability, compute_halstead, HalsteadMetrics, MaintainabilityLevel,
    MaintainabilityMetrics,
};
use crate::metrics_basic::analyze_basic_loc;
use crate::tree::{Node, NodeId, Tree};

/// Basic source + tree shape metrics (parity: BasicMetrics).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicMetrics {
    pub loc: usize,
    pub sloc: usize,
    pub cloc: usize,
    pub blank: usize,
    pub nodes: usize,
    pub max_depth: u32,
    pub avg_depth: f64,
}

/// Cyclomatic / cognitive complexity (parity: ComplexityMetrics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplexityMetrics {
    pub cyclomatic: u32,
    pub decision_points: u32,
    pub cognitive: u32,
}

/// Maintainability level as TS string union.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintainabilityLevelDto {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl From<MaintainabilityLevel> for MaintainabilityLevelDto {
    fn from(level: MaintainabilityLevel) -> Self {
        match level {
            // TS: 'low' means low difficulty (index >= 85)
            MaintainabilityLevel::LowDifficulty => Self::Low,
            MaintainabilityLevel::Moderate => Self::Moderate,
            MaintainabilityLevel::High => Self::High,
            MaintainabilityLevel::VeryHigh => Self::VeryHigh,
        }
    }
}

/// Maintainability index payload (parity: MaintainabilityMetrics).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintainabilityDto {
    pub index: f64,
    pub level: MaintainabilityLevelDto,
    pub maintainable: bool,
}

/// Complete code metrics (parity: CodeMetrics).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeMetrics {
    pub basic: BasicMetrics,
    pub complexity: ComplexityMetrics,
    pub halstead: HalsteadMetricsDto,
    pub maintainability: MaintainabilityDto,
}

/// Halstead payload with TS field names (n1/n2/N1/N2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HalsteadMetricsDto {
    pub n1: f64,
    pub n2: f64,
    #[serde(rename = "N1")]
    pub n1_total: f64,
    #[serde(rename = "N2")]
    pub n2_total: f64,
    pub vocabulary: f64,
    pub length: f64,
    pub calculated_length: f64,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
    pub time: f64,
    pub bugs: f64,
}

impl From<HalsteadMetrics> for HalsteadMetricsDto {
    fn from(h: HalsteadMetrics) -> Self {
        Self {
            n1: h.n1,
            n2: h.n2,
            n1_total: h.n1_total,
            n2_total: h.n2_total,
            vocabulary: h.vocabulary,
            length: h.length,
            calculated_length: h.calculated_length,
            volume: h.volume,
            difficulty: h.difficulty,
            effort: h.effort,
            time: h.time,
            bugs: h.bugs,
        }
    }
}

/// Source location (line/column only; parity FunctionMetrics.location).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationPoint {
    pub line: u32,
    pub column: u32,
}

/// Function span location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionLocation {
    pub start: LocationPoint,
    pub end: LocationPoint,
}

/// Per-function metrics (parity: FunctionMetrics).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionMetrics {
    pub name: String,
    pub location: FunctionLocation,
    pub loc: usize,
    pub complexity: u32,
    pub params: usize,
    pub max_depth: u32,
}

/// File-level report (parity: MetricsReport).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    pub language: String,
    pub metrics: CodeMetrics,
    pub functions: Vec<FunctionMetrics>,
    pub timestamp: u64,
}

/// Product metrics analyzer (ports MetricsAnalyzer class).
#[derive(Debug, Default, Clone)]
pub struct MetricsAnalyzer;

impl MetricsAnalyzer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Analyze a tree and compute all aggregate metrics.
    #[must_use]
    pub fn analyze(&self, tree: &Tree) -> CodeMetrics {
        let basic = self.analyze_basic(tree);
        let complexity = self.analyze_complexity(tree);
        let halstead = self.analyze_halstead(tree);
        let maintainability = Self::maintainability_dto(
            calculate_maintainability(halstead.volume, f64::from(complexity.cyclomatic), basic.loc as f64),
        );
        CodeMetrics {
            basic,
            complexity,
            halstead,
            maintainability,
        }
    }

    /// Full metrics report with per-function breakdown.
    #[must_use]
    pub fn report(&self, tree: &Tree, file: Option<&str>, timestamp: u64) -> MetricsReport {
        MetricsReport {
            file: file.map(str::to_string),
            language: tree.language(),
            metrics: self.analyze(tree),
            functions: self.analyze_functions(tree),
            timestamp,
        }
    }

    fn analyze_basic(&self, tree: &Tree) -> BasicMetrics {
        let loc_m = analyze_basic_loc(&tree.source());
        // TS: empty source still yields lines.length === 1 for split('\n') on "" → [""]
        // Our analyze_basic_loc returns loc=0 for empty; mirror TS for non-empty only.
        // Keep pure kernel; product engine uses tree source length semantics of kernel.
        let depths = self.calculate_depths(tree);
        BasicMetrics {
            loc: loc_m.loc,
            sloc: loc_m.sloc,
            cloc: loc_m.cloc,
            blank: loc_m.blank,
            nodes: tree.node_count(),
            max_depth: max_depth_of(&depths),
            avg_depth: avg_depth_of(&depths),
        }
    }

    fn analyze_complexity(&self, tree: &Tree) -> ComplexityMetrics {
        let decision_points = self.count_decision_points(tree, tree.root_id());
        ComplexityMetrics {
            cyclomatic: cyclomatic_from_decisions(decision_points),
            decision_points,
            cognitive: self.calculate_cognitive(tree, tree.root_id(), 0),
        }
    }

    fn analyze_halstead(&self, tree: &Tree) -> HalsteadMetricsDto {
        let mut operators: HashSet<String> = HashSet::new();
        let mut operands: HashSet<String> = HashSet::new();
        let mut total_operators = 0u32;
        let mut total_operands = 0u32;
        self.traverse_halstead(
            tree,
            tree.root_id(),
            &mut operators,
            &mut operands,
            &mut total_operators,
            &mut total_operands,
        );
        compute_halstead(
            operators.len() as f64,
            operands.len() as f64,
            f64::from(total_operators),
            f64::from(total_operands),
        )
        .into()
    }

    fn analyze_functions(&self, tree: &Tree) -> Vec<FunctionMetrics> {
        let mut out = Vec::new();
        self.traverse(tree, tree.root_id(), &mut |node| {
            if is_function_node_type(&node.node_type)
                && let Some(m) = self.analyze_function_node(tree, node)
            {
                out.push(m);
            }
        });
        out
    }

    fn analyze_function_node(&self, tree: &Tree, node: &Node) -> Option<FunctionMetrics> {
        let (data_name, id_name, params_len, parameters_len) = extract_function_data(node);
        let name = function_name(data_name.as_deref(), id_name.as_deref());
        // TS returns null only when getFunctionName returns falsy — ours always returns a name
        // (including "<anonymous>"), so we always emit (matching TS non-empty name path).
        let loc = if let Some(span) = node.span {
            function_loc(
                &tree.source(),
                span.start.offset as usize,
                span.end.offset as usize,
            )
        } else {
            0
        };
        let complexity = cyclomatic_from_decisions(self.count_decision_points(tree, node.id));
        let params = function_param_count(params_len, parameters_len);
        let max_depth = self.get_max_depth(tree, node.id);
        let (start, end) = match node.span {
            Some(span) => (
                LocationPoint {
                    line: span.start.line,
                    column: span.start.column,
                },
                LocationPoint {
                    line: span.end.line,
                    column: span.end.column,
                },
            ),
            None => (
                LocationPoint { line: 0, column: 0 },
                LocationPoint { line: 0, column: 0 },
            ),
        };
        Some(FunctionMetrics {
            name,
            location: FunctionLocation { start, end },
            loc,
            complexity,
            params,
            max_depth,
        })
    }

    fn count_decision_points(&self, tree: &Tree, node_id: NodeId) -> u32 {
        let Some(node) = tree.nodes().get(node_id as usize) else {
            return 0;
        };
        let mut count = u32::from(is_decision_count_type(&node.node_type));
        for &child in &node.children {
            count = count.saturating_add(self.count_decision_points(tree, child));
        }
        count
    }

    fn calculate_cognitive(&self, tree: &Tree, node_id: NodeId, nesting: u32) -> u32 {
        let Some(node) = tree.nodes().get(node_id as usize) else {
            return 0;
        };
        let mut complexity = 0u32;
        if is_decision_node_type(&node.node_type) {
            complexity = complexity.saturating_add(cognitive_decision_weight(nesting));
        }
        let next = if is_nesting_node_type(&node.node_type) {
            nesting.saturating_add(1)
        } else {
            nesting
        };
        for &child in &node.children {
            complexity = complexity.saturating_add(self.calculate_cognitive(tree, child, next));
        }
        complexity
    }

    fn calculate_depths(&self, tree: &Tree) -> Vec<u32> {
        let mut depths = Vec::new();
        fn walk(tree: &Tree, node_id: NodeId, depth: u32, depths: &mut Vec<u32>) {
            let Some(node) = tree.nodes().get(node_id as usize) else {
                return;
            };
            depths.push(depth);
            for &child in &node.children {
                walk(tree, child, depth.saturating_add(1), depths);
            }
        }
        walk(tree, tree.root_id(), 0, &mut depths);
        depths
    }

    fn get_max_depth(&self, tree: &Tree, node_id: NodeId) -> u32 {
        let Some(node) = tree.nodes().get(node_id as usize) else {
            return 0;
        };
        if node.children.is_empty() {
            return 0;
        }
        let mut max_d = 0u32;
        for &child in &node.children {
            max_d = max_d.max(self.get_max_depth(tree, child));
        }
        max_d.saturating_add(1)
    }

    fn traverse_halstead(
        &self,
        tree: &Tree,
        node_id: NodeId,
        operators: &mut HashSet<String>,
        operands: &mut HashSet<String>,
        total_operators: &mut u32,
        total_operands: &mut u32,
    ) {
        let Some(node) = tree.nodes().get(node_id as usize) else {
            return;
        };
        if is_operator_node_type(&node.node_type) {
            operators.insert(node.node_type.clone());
            *total_operators = total_operators.saturating_add(1);
        } else if is_operand_node_type(&node.node_type) {
            let (value, name) = extract_operand_fields(node);
            let v = operand_value(&node.node_type, value.as_deref(), name.as_deref());
            if !v.is_empty() {
                operands.insert(v);
                *total_operands = total_operands.saturating_add(1);
            }
        }
        for &child in &node.children {
            self.traverse_halstead(
                tree,
                child,
                operators,
                operands,
                total_operators,
                total_operands,
            );
        }
    }

    fn traverse<F: FnMut(&Node)>(&self, tree: &Tree, node_id: NodeId, callback: &mut F) {
        let Some(node) = tree.nodes().get(node_id as usize) else {
            return;
        };
        callback(node);
        for &child in &node.children {
            self.traverse(tree, child, callback);
        }
    }

    fn maintainability_dto(m: MaintainabilityMetrics) -> MaintainabilityDto {
        MaintainabilityDto {
            index: m.index,
            level: m.level.into(),
            maintainable: m.maintainable,
        }
    }
}

/// Convenience: analyze without constructing the analyzer.
#[must_use]
pub fn analyze_tree(tree: &Tree) -> CodeMetrics {
    MetricsAnalyzer::new().analyze(tree)
}

/// Convenience: full report.
#[must_use]
pub fn report_tree(tree: &Tree, file: Option<&str>, timestamp: u64) -> MetricsReport {
    MetricsAnalyzer::new().report(tree, file, timestamp)
}

fn extract_function_data(
    node: &Node,
) -> (
    Option<String>,
    Option<String>,
    Option<usize>,
    Option<usize>,
) {
    let Some(data) = node.data.as_ref() else {
        return (None, None, None, None);
    };
    let data_name = data
        .get("name")
        .and_then(|v| v.as_str().map(str::to_string));
    let id_name = data
        .get("id")
        .and_then(|v| v.as_object())
        .and_then(|o| o.get("name"))
        .and_then(|v| v.as_str().map(str::to_string));
    let params_len = data
        .get("params")
        .and_then(|v| v.as_array())
        .map(std::vec::Vec::len);
    let parameters_len = data
        .get("parameters")
        .and_then(|v| v.as_array())
        .map(std::vec::Vec::len);
    (data_name, id_name, params_len, parameters_len)
}

fn extract_operand_fields(node: &Node) -> (Option<String>, Option<String>) {
    let Some(data) = node.data.as_ref() else {
        return (None, None);
    };
    let value = data.get("value").map(|v| match v {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string().trim_matches('"').to_string(),
    });
    let name = data
        .get("name")
        .and_then(|v| v.as_str().map(str::to_string));
    (value, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::{Position, Span};
    use std::collections::HashMap;

    fn sample_tree() -> Tree {
        // source with blank + comment + code
        let mut tree = Tree::new(
            "javascript",
            "function foo(a) {\n  if (a) {\n    return a + 1;\n  }\n}\n\n// comment\n",
        );
        // root already id 0
        // Program
        let prog = Node::new(0, "Program");
        let prog_id = tree.add_node(prog);
        tree.add_child(tree.root_id(), prog_id).unwrap();

        // FunctionDeclaration with name + params
        let mut data = HashMap::new();
        data.insert("name".into(), serde_json::json!("foo"));
        data.insert("params".into(), serde_json::json!(["a"]));
        let fn_node = Node::new(0, "FunctionDeclaration")
            .with_data(data)
            .with_span(Span::new(
                Position::new(1, 0, 0),
                Position::new(5, 1, 48),
            ));
        let fn_id = tree.add_node(fn_node);
        tree.add_child(prog_id, fn_id).unwrap();

        // if decision
        let if_id = tree.add_node(Node::new(0, "IfStatement"));
        tree.add_child(fn_id, if_id).unwrap();

        // binary + identifiers for Halstead
        let bin_id = tree.add_node(Node::new(0, "BinaryExpression"));
        tree.add_child(if_id, bin_id).unwrap();

        let mut id_data = HashMap::new();
        id_data.insert("name".into(), serde_json::json!("a"));
        let id_id = tree.add_node(Node::new(0, "Identifier").with_data(id_data));
        tree.add_child(bin_id, id_id).unwrap();

        let mut lit_data = HashMap::new();
        lit_data.insert("value".into(), serde_json::json!(1));
        let lit_id = tree.add_node(Node::new(0, "NumericLiteral").with_data(lit_data));
        tree.add_child(bin_id, lit_id).unwrap();

        tree
    }

    #[test]
    fn analyze_basic_loc_and_depth() {
        let tree = sample_tree();
        let m = MetricsAnalyzer::new().analyze(&tree);
        assert!(m.basic.loc >= 5);
        assert!(m.basic.nodes >= 5);
        assert!(m.basic.max_depth >= 3);
        assert!(m.basic.avg_depth > 0.0);
        assert!(m.basic.sloc >= 1);
    }

    #[test]
    fn complexity_counts_if() {
        let tree = sample_tree();
        let m = MetricsAnalyzer::new().analyze(&tree);
        assert!(m.complexity.decision_points >= 1);
        assert_eq!(
            m.complexity.cyclomatic,
            m.complexity.decision_points + 1
        );
        assert!(m.complexity.cognitive >= 1);
    }

    #[test]
    fn halstead_and_maintainability() {
        let tree = sample_tree();
        let m = MetricsAnalyzer::new().analyze(&tree);
        assert!(m.halstead.n1 >= 1.0); // BinaryExpression
        assert!(m.halstead.n2 >= 1.0); // Identifier / NumericLiteral
        assert!(m.halstead.volume >= 0.0);
        assert!((0.0..=100.0).contains(&m.maintainability.index));
        assert!(m.maintainability.maintainable || m.maintainability.index < 20.0);
    }

    #[test]
    fn function_metrics_named() {
        let tree = sample_tree();
        let report = MetricsAnalyzer::new().report(&tree, Some("foo.js"), 1_700_000_000_000);
        assert_eq!(report.file.as_deref(), Some("foo.js"));
        assert_eq!(report.language, "javascript");
        assert_eq!(report.timestamp, 1_700_000_000_000);
        assert!(!report.functions.is_empty());
        let f = &report.functions[0];
        assert_eq!(f.name, "foo");
        assert_eq!(f.params, 1);
        assert!(f.complexity >= 1);
        assert!(f.loc >= 1);
    }

    #[test]
    fn convenience_wrappers() {
        let tree = Tree::new("javascript", "let x = 1;\n");
        let m = analyze_tree(&tree);
        assert_eq!(m.basic.nodes, 1); // root only
        let r = report_tree(&tree, None, 0);
        assert!(r.functions.is_empty());
        assert_eq!(r.language, "javascript");
    }

    #[test]
    fn empty_tree_safe() {
        let tree = Tree::new("javascript", "");
        let m = analyze_tree(&tree);
        assert_eq!(m.basic.loc, 0);
        assert_eq!(m.complexity.decision_points, 0);
        assert_eq!(m.complexity.cyclomatic, 1);
    }
}

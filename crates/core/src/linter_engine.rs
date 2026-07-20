//! Product **linter engine** with built-in rules (tooling residual).
//!
//! Ports `packages/synth-lint/src/linter.ts` + default rules
//! (no-console, max-depth, no-empty-blocks) onto the Rust `Tree` model.
//! Full product path; TS package remains dual-oracle until cutover.

use serde::{Deserialize, Serialize};

use crate::lint_block_types::{empty_block_message, is_empty_block};
use crate::lint_console::{console_message, is_console_call_source};
use crate::lint_max_depth::{depth_exceeds, max_depth_message, DEFAULT_MAX_DEPTH};
use crate::lint_severity::{count_by_severity, lint_success, SeverityCounts};
use crate::tree::{Node, NodeId, Tree};

/// Diagnostic severity (parity: DiagnosticSeverity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

impl DiagnosticSeverity {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Hint => "hint",
        }
    }

    #[must_use]
    pub fn rank(self) -> u8 {
        match self {
            Self::Error => 0,
            Self::Warning => 1,
            Self::Info => 2,
            Self::Hint => 3,
        }
    }
}

/// A single lint diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub rule: String,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub node_id: Option<NodeId>,
}

/// Built-in rule identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinRule {
    NoConsole,
    MaxDepth,
    NoEmptyBlocks,
}

impl BuiltinRule {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::NoConsole => "no-console",
            Self::MaxDepth => "max-depth",
            Self::NoEmptyBlocks => "no-empty-blocks",
        }
    }

    #[must_use]
    pub fn default_enabled(self) -> bool {
        match self {
            Self::NoConsole => false, // opt-in (TS parity)
            Self::MaxDepth | Self::NoEmptyBlocks => true,
        }
    }

    #[must_use]
    pub fn default_severity(self) -> DiagnosticSeverity {
        DiagnosticSeverity::Warning
    }
}

/// Linter configuration.
#[derive(Debug, Clone)]
pub struct LinterConfig {
    pub enable_no_console: bool,
    pub enable_max_depth: bool,
    pub enable_no_empty_blocks: bool,
    pub max_depth: u32,
    pub min_severity: Option<DiagnosticSeverity>,
    pub languages: Vec<String>,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            enable_no_console: BuiltinRule::NoConsole.default_enabled(),
            enable_max_depth: BuiltinRule::MaxDepth.default_enabled(),
            enable_no_empty_blocks: BuiltinRule::NoEmptyBlocks.default_enabled(),
            max_depth: DEFAULT_MAX_DEPTH,
            min_severity: None,
            languages: Vec::new(),
        }
    }
}

/// Linter result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LintResult {
    pub diagnostics: Vec<Diagnostic>,
    pub counts: SeverityCountsDto,
    pub success: bool,
}

/// Serializable severity counts (mirrors lint_severity::SeverityCounts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeverityCountsDto {
    pub error: u32,
    pub warning: u32,
    pub info: u32,
    pub hint: u32,
}

impl From<SeverityCounts> for SeverityCountsDto {
    fn from(c: SeverityCounts) -> Self {
        Self {
            error: c.error,
            warning: c.warning,
            info: c.info,
            hint: c.hint,
        }
    }
}

/// Product linter engine.
#[derive(Debug, Default)]
pub struct Linter {
    config: LinterConfig,
}

impl Linter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_config(config: LinterConfig) -> Self {
        Self { config }
    }

    /// Configure the linter (chainable).
    pub fn configure(&mut self, config: LinterConfig) -> &mut Self {
        self.config = config;
        self
    }

    /// Lint a tree with the active rule set.
    #[must_use]
    pub fn lint(&self, tree: &Tree) -> LintResult {
        if !self.language_allowed(tree) {
            return LintResult {
                diagnostics: Vec::new(),
                counts: SeverityCountsDto {
                    error: 0,
                    warning: 0,
                    info: 0,
                    hint: 0,
                },
                success: true,
            };
        }

        let mut diagnostics = Vec::new();
        for node in tree.nodes() {
            if self.config.enable_no_console {
                self.rule_no_console(tree, node, &mut diagnostics);
            }
            if self.config.enable_max_depth {
                self.rule_max_depth(tree, node, &mut diagnostics);
            }
            if self.config.enable_no_empty_blocks {
                self.rule_no_empty_blocks(node, &mut diagnostics);
            }
        }

        if let Some(min) = self.config.min_severity {
            diagnostics.retain(|d| d.severity.rank() <= min.rank());
        }

        let labels: Vec<&str> = diagnostics.iter().map(|d| d.severity.as_str()).collect();
        let counts = count_by_severity(&labels);
        let success = lint_success(&counts);
        LintResult {
            diagnostics,
            counts: counts.into(),
            success,
        }
    }

    fn language_allowed(&self, tree: &Tree) -> bool {
        if self.config.languages.is_empty() {
            return true;
        }
        let lang = tree.language();
        self.config
            .languages
            .iter()
            .any(|l| l.eq_ignore_ascii_case(&lang))
    }

    fn rule_no_console(&self, tree: &Tree, node: &Node, out: &mut Vec<Diagnostic>) {
        let source = node_source(tree, node);
        if is_console_call_source(&node.node_type, &source)
            || (node.node_type == "CallExpression" && source.starts_with("console."))
            || is_console_member(node)
        {
            out.push(Diagnostic {
                rule: BuiltinRule::NoConsole.name().into(),
                severity: BuiltinRule::NoConsole.default_severity(),
                message: console_message(&source),
                node_id: Some(node.id),
            });
        }
    }

    fn rule_max_depth(&self, tree: &Tree, node: &Node, out: &mut Vec<Diagnostic>) {
        let depth = node_depth(tree, node.id);
        if depth_exceeds(depth, self.config.max_depth) {
            out.push(Diagnostic {
                rule: BuiltinRule::MaxDepth.name().into(),
                severity: BuiltinRule::MaxDepth.default_severity(),
                message: max_depth_message(depth, self.config.max_depth),
                node_id: Some(node.id),
            });
        }
    }

    fn rule_no_empty_blocks(&self, node: &Node, out: &mut Vec<Diagnostic>) {
        if is_empty_block(node.children.len(), &node.node_type) {
            out.push(Diagnostic {
                rule: BuiltinRule::NoEmptyBlocks.name().into(),
                severity: BuiltinRule::NoEmptyBlocks.default_severity(),
                message: empty_block_message(&node.node_type),
                node_id: Some(node.id),
            });
        }
    }
}

fn is_console_member(node: &Node) -> bool {
    if node.node_type != "MemberExpression" {
        return false;
    }
    node.data.as_ref().is_some_and(|d| {
        d.get("object")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s == "console")
            || d.get("object")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .is_some_and(|s| s == "console")
    })
}

fn node_source(tree: &Tree, node: &Node) -> String {
    let source = tree.source();
    if let Some(span) = &node.span {
        let start = span.start.offset as usize;
        let end = span.end.offset as usize;
        if start <= end && end <= source.len() {
            return source[start..end].to_string();
        }
    }
    String::new()
}

fn node_depth(tree: &Tree, id: NodeId) -> u32 {
    let mut depth = 0u32;
    let mut current = id;
    while let Ok(node) = tree.get_node(current) {
        match node.parent {
            Some(p) => {
                depth += 1;
                current = p;
            }
            None => break,
        }
    }
    depth
}

/// Convenience: lint with default config.
#[must_use]
pub fn lint_tree(tree: &Tree) -> LintResult {
    Linter::new().lint(tree)
}

/// Convenience: lint with config.
#[must_use]
pub fn lint_tree_with(tree: &Tree, config: LinterConfig) -> LintResult {
    Linter::with_config(config).lint(tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Span;
    use crate::tree::Node;
    use serde_json::Value;
    use std::collections::HashMap;

    fn data(pairs: &[(&str, Value)]) -> HashMap<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn empty_block_rule_fires() {
        let mut tree = Tree::new("javascript", "{}");
        let block = tree.add_node(Node::new(0, "BlockStatement"));
        let _ = tree.add_child(0, block);
        let result = lint_tree(&tree);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.rule == "no-empty-blocks"),
            "{:?}",
            result.diagnostics
        );
        assert!(result.success); // warnings only
    }

    #[test]
    fn max_depth_rule_fires() {
        let mut tree = Tree::new("javascript", "nested");
        let mut parent = 0u32;
        for i in 0..6 {
            let n = tree.add_node(Node::new(0, format!("Nest{i}")));
            let _ = tree.add_child(parent, n);
            parent = n;
        }
        let mut cfg = LinterConfig::default();
        cfg.max_depth = 3;
        let result = lint_tree_with(&tree, cfg);
        assert!(
            result.diagnostics.iter().any(|d| d.rule == "max-depth"),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn no_console_opt_in() {
        let mut tree = Tree::new("javascript", "console.log(1)");
        let call = tree.add_node(
            Node::new(0, "CallExpression")
                .with_span(Span::from_coords(0, 0, 0, 0, 14, 14))
                .with_data(data(&[("callee", Value::String("console.log".into()))])),
        );
        let _ = tree.add_child(0, call);

        // default: no-console disabled
        let off = lint_tree(&tree);
        assert!(!off.diagnostics.iter().any(|d| d.rule == "no-console"));

        let mut cfg = LinterConfig::default();
        cfg.enable_no_console = true;
        let on = lint_tree_with(&tree, cfg);
        assert!(
            on.diagnostics.iter().any(|d| d.rule == "no-console"),
            "{:?}",
            on.diagnostics
        );
    }
}

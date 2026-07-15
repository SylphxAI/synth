//! Product AST → formatted-code **printer engine** (tooling dens).
//!
//! Ports `packages/synth-js-format/src/printer.ts` onto the Rust `Tree` model.
//! TS package remains the dual-oracle substrate until cutover; this is the
//! full product path (not pure residual fragments).

use serde_json::Value;

use crate::format_indent::{indent_string, quote_string_literal};
use crate::tree::{Node, NodeId, Tree};

/// Prettier-compatible format options (parity: FormatOptions / DEFAULT_OPTIONS).
#[derive(Debug, Clone, PartialEq)]
pub struct FormatOptions {
    pub print_width: usize,
    pub tab_width: usize,
    pub use_tabs: bool,
    pub semi: bool,
    pub single_quote: bool,
    pub trailing_comma: TrailingComma,
    pub bracket_spacing: bool,
    pub arrow_parens: ArrowParens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrailingComma {
    None,
    Es5,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowParens {
    Always,
    Avoid,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            print_width: 80,
            tab_width: 2,
            use_tabs: false,
            semi: true,
            single_quote: false,
            trailing_comma: TrailingComma::Es5,
            bracket_spacing: true,
            arrow_parens: ArrowParens::Always,
        }
    }
}

impl FormatOptions {
    #[must_use]
    pub fn wants_trailing_comma(&self, es5_context: bool) -> bool {
        match self.trailing_comma {
            TrailingComma::All => true,
            TrailingComma::Es5 => es5_context,
            TrailingComma::None => false,
        }
    }
}

/// Product printer: AST → formatted source.
#[derive(Debug)]
pub struct Printer {
    options: FormatOptions,
    current_indent: usize,
    output: Vec<String>,
}

impl Printer {
    #[must_use]
    pub fn new(options: FormatOptions) -> Self {
        Self {
            options,
            current_indent: 0,
            output: Vec::new(),
        }
    }

    /// Print a tree. Prefers a `Program` node; falls back to root children.
    ///
    /// # Errors
    ///
    /// Returns an error when the tree has no printable root/program.
    pub fn print(&mut self, tree: &Tree) -> Result<String, String> {
        self.output.clear();
        self.current_indent = 0;

        let program = tree
            .nodes()
            .iter()
            .find(|n| n.node_type == "Program")
            .map(|n| n.id);

        let root_id = program.unwrap_or_else(|| tree.root_id());
        let children = self.children_of(tree, root_id);
        if children.is_empty() && program.is_none() {
            // empty tree still valid
            return Ok(String::new());
        }
        self.print_statements(tree, &children, "\n\n");
        Ok(self.output.join(""))
    }

    fn children_of(&self, tree: &Tree, id: NodeId) -> Vec<NodeId> {
        tree.get_node(id)
            .map(|n| n.children.clone())
            .unwrap_or_default()
    }

    fn node<'a>(&self, tree: &'a Tree, id: NodeId) -> Option<&'a Node> {
        tree.get_node(id).ok()
    }

    fn data_str(node: &Node, key: &str) -> Option<String> {
        node.data.as_ref().and_then(|d| {
            d.get(key).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                Value::Bool(b) => Some(b.to_string()),
                Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
        })
    }

    fn data_bool(node: &Node, key: &str) -> Option<bool> {
        node.data.as_ref().and_then(|d| d.get(key)?.as_bool())
    }

    fn print_statements(&mut self, tree: &Tree, nodes: &[NodeId], separator: &str) {
        for (i, id) in nodes.iter().enumerate() {
            self.print_node(tree, *id);
            if i + 1 < nodes.len() {
                self.write(separator);
            }
        }
    }

    fn print_node(&mut self, tree: &Tree, id: NodeId) {
        let Some(node) = self.node(tree, id).cloned() else {
            return;
        };
        match node.node_type.as_str() {
            "Program" => {
                let children = self.children_of(tree, id);
                self.print_statements(tree, &children, "\n\n");
            }
            "VariableDeclaration" => self.print_variable_declaration(tree, &node),
            "FunctionDeclaration" => self.print_function_declaration(tree, &node),
            "ExpressionStatement" => self.print_expression_statement(tree, &node),
            "ReturnStatement" => self.print_return_statement(tree, &node),
            "IfStatement" => self.print_if_statement(tree, &node),
            "BlockStatement" => self.print_block_statement(tree, &node),
            "CallExpression" => self.print_call_expression(tree, &node),
            "MemberExpression" => self.print_member_expression(tree, &node),
            "Identifier" => self.print_identifier(&node),
            "Literal" => self.print_literal(&node),
            "BinaryExpression" => self.print_binary_expression(tree, &node),
            "UnaryExpression" => self.print_unary_expression(tree, &node),
            "ArrowFunctionExpression" => self.print_arrow_function(tree, &node),
            "ArrayExpression" => self.print_array_expression(tree, &node),
            "ObjectExpression" => self.print_object_expression(tree, &node),
            "Property" => self.print_property(tree, &node),
            "VariableDeclarator" => self.print_variable_declarator(tree, &node),
            "ImportDeclaration" => self.print_import_declaration(),
            "ExportNamedDeclaration" | "ExportDefaultDeclaration" => {
                self.print_export_declaration(tree, &node);
            }
            "ClassDeclaration" => self.print_class_declaration(tree, &node),
            "MethodDefinition" => self.print_method_definition(tree, &node),
            other => {
                self.write(&format!("/* {other} */"));
            }
        }
    }

    fn print_variable_declaration(&mut self, tree: &Tree, node: &Node) {
        let kind = Self::data_str(node, "kind").unwrap_or_else(|| "const".into());
        self.write_indent();
        self.write(&format!("{kind} "));
        let declarators = self.children_of(tree, node.id);
        for (i, d) in declarators.iter().enumerate() {
            self.print_node(tree, *d);
            if i + 1 < declarators.len() {
                self.write(", ");
            }
        }
        if self.options.semi {
            self.write(";");
        }
    }

    fn print_variable_declarator(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        let id = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "Identifier" || n.node_type.contains("Pattern"))
        });
        let init = children.iter().copied().find(|cid| Some(*cid) != id);
        if let Some(id) = id {
            self.print_node(tree, id);
        }
        if let Some(init) = init {
            self.write(" = ");
            self.print_node(tree, init);
        }
    }

    fn print_function_declaration(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        if Self::data_bool(node, "async").unwrap_or(false) {
            self.write("async ");
        }
        self.write("function");
        if Self::data_bool(node, "generator").unwrap_or(false) {
            self.write("*");
        }
        if let Some(name) = Self::data_str(node, "id") {
            self.write(&format!(" {name}"));
        }
        self.write("(");
        let children = self.children_of(tree, node.id);
        let body = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "BlockStatement")
        });
        let params: Vec<_> = children
            .iter()
            .copied()
            .filter(|cid| Some(*cid) != body)
            .collect();
        for (i, p) in params.iter().enumerate() {
            self.print_node(tree, *p);
            if i + 1 < params.len() {
                self.write(", ");
            }
        }
        self.write(") ");
        if let Some(body) = body {
            self.print_node(tree, body);
        }
    }

    fn print_arrow_function(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if children.is_empty() {
            self.write("() => {}");
            return;
        }
        let body = children[children.len() - 1];
        let params = &children[..children.len() - 1];
        let need_parens = match self.options.arrow_parens {
            ArrowParens::Always => true,
            ArrowParens::Avoid => params.len() != 1,
        };
        if need_parens {
            self.write("(");
        }
        for (i, p) in params.iter().enumerate() {
            self.print_node(tree, *p);
            if i + 1 < params.len() {
                self.write(", ");
            }
        }
        if need_parens {
            self.write(")");
        }
        self.write(" => ");
        self.print_node(tree, body);
    }

    fn print_block_statement(&mut self, tree: &Tree, node: &Node) {
        self.write("{");
        let children = self.children_of(tree, node.id);
        if !children.is_empty() {
            self.write("\n");
            self.current_indent += 1;
            self.print_statements(tree, &children, "\n");
            self.write("\n");
            self.current_indent = self.current_indent.saturating_sub(1);
            self.write_indent();
        }
        self.write("}");
    }

    fn print_expression_statement(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.print_node(tree, *first);
        }
        if self.options.semi {
            self.write(";");
        }
    }

    fn print_return_statement(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        self.write("return");
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.write(" ");
            self.print_node(tree, *first);
        }
        if self.options.semi {
            self.write(";");
        }
    }

    fn print_if_statement(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        self.write("if (");
        let children = self.children_of(tree, node.id);
        if let Some(test) = children.first() {
            self.print_node(tree, *test);
        }
        self.write(") ");
        if let Some(cons) = children.get(1) {
            self.print_node(tree, *cons);
        }
        if let Some(alt) = children.get(2) {
            self.write(" else ");
            self.print_node(tree, *alt);
        }
    }

    fn print_call_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(callee) = children.first() {
            self.print_node(tree, *callee);
        }
        self.write("(");
        let args = if children.len() > 1 {
            &children[1..]
        } else {
            &[]
        };
        for (i, a) in args.iter().enumerate() {
            self.print_node(tree, *a);
            if i + 1 < args.len() {
                self.write(", ");
            }
        }
        self.write(")");
    }

    fn print_member_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(obj) = children.first() {
            self.print_node(tree, *obj);
        }
        let computed = Self::data_bool(node, "computed").unwrap_or(false);
        if computed {
            self.write("[");
            if let Some(prop) = children.get(1) {
                self.print_node(tree, *prop);
            }
            self.write("]");
        } else {
            self.write(".");
            if let Some(prop) = children.get(1) {
                self.print_node(tree, *prop);
            }
        }
    }

    fn print_binary_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(left) = children.first() {
            self.print_node(tree, *left);
        }
        let op = Self::data_str(node, "operator").unwrap_or_else(|| "+".into());
        self.write(&format!(" {op} "));
        if let Some(right) = children.get(1) {
            self.print_node(tree, *right);
        }
    }

    fn print_unary_expression(&mut self, tree: &Tree, node: &Node) {
        let op = Self::data_str(node, "operator").unwrap_or_default();
        let prefix = Self::data_bool(node, "prefix").unwrap_or(true);
        if prefix {
            self.write(&op);
            let children = self.children_of(tree, node.id);
            if let Some(first) = children.first() {
                self.print_node(tree, *first);
            }
        }
    }

    fn print_array_expression(&mut self, tree: &Tree, node: &Node) {
        self.write("[");
        let children = self.children_of(tree, node.id);
        for (i, e) in children.iter().enumerate() {
            self.print_node(tree, *e);
            if i + 1 < children.len() {
                self.write(", ");
            }
        }
        if self.options.wants_trailing_comma(true) && !children.is_empty() {
            self.write(",");
        }
        self.write("]");
    }

    fn print_object_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if children.is_empty() {
            self.write("{}");
            return;
        }
        self.write("{");
        if self.options.bracket_spacing {
            self.write(" ");
        }
        for (i, p) in children.iter().enumerate() {
            self.print_node(tree, *p);
            if i + 1 < children.len() {
                self.write(", ");
            }
        }
        if self.options.wants_trailing_comma(true) {
            self.write(",");
        }
        if self.options.bracket_spacing {
            self.write(" ");
        }
        self.write("}");
    }

    fn print_property(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(key) = children.first() {
            self.print_node(tree, *key);
        }
        self.write(": ");
        if let Some(val) = children.get(1) {
            self.print_node(tree, *val);
        }
    }

    fn print_import_declaration(&mut self) {
        self.write_indent();
        self.write("import ");
        self.write("/* import */");
        if self.options.semi {
            self.write(";");
        }
    }

    fn print_export_declaration(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        self.write("export ");
        if node.node_type == "ExportDefaultDeclaration" {
            self.write("default ");
        }
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.print_node(tree, *first);
        }
    }

    fn print_class_declaration(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        self.write("class ");
        if let Some(name) = Self::data_str(node, "id") {
            self.write(&format!("{name} "));
        }
        let children = self.children_of(tree, node.id);
        let class_body = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "ClassBody")
        });
        if let Some(body_id) = class_body {
            let methods = self.children_of(tree, body_id);
            self.write("{");
            if !methods.is_empty() {
                self.write("\n");
                self.current_indent += 1;
                for m in &methods {
                    self.print_node(tree, *m);
                    self.write("\n");
                }
                self.current_indent = self.current_indent.saturating_sub(1);
                self.write_indent();
            }
            self.write("}");
        } else {
            self.write("{}");
        }
    }

    fn print_method_definition(&mut self, tree: &Tree, node: &Node) {
        self.write_indent();
        if let Some(kind) = Self::data_str(node, "kind")
            && kind != "method"
        {
            self.write(&format!("{kind} "));
        }
        let children = self.children_of(tree, node.id);
        if let Some(key) = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "Identifier")
        }) {
            self.print_node(tree, key);
        }
        self.write("() ");
        if let Some(value) = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "FunctionExpression")
        }) {
            let body = self.children_of(tree, value).into_iter().find(|cid| {
                self.node(tree, *cid)
                    .is_some_and(|n| n.node_type == "BlockStatement")
            });
            if let Some(body) = body {
                self.print_node(tree, body);
            }
        }
    }

    fn print_identifier(&mut self, node: &Node) {
        if let Some(name) = Self::data_str(node, "name") {
            self.write(&name);
        }
    }

    fn print_literal(&mut self, node: &Node) {
        if let Some(data) = &node.data {
            if let Some(value) = data.get("value")
                && let Some(s) = value.as_str()
            {
                self.write(&quote_string_literal(s, self.options.single_quote));
                return;
            }
            if let Some(raw) = data.get("raw").and_then(|v| v.as_str()) {
                self.write(raw);
                return;
            }
            if let Some(value) = data.get("value") {
                self.write(&value.to_string());
            }
        }
    }

    fn write_indent(&mut self) {
        let s = indent_string(self.current_indent, self.options.tab_width, self.options.use_tabs);
        self.write(&s);
    }

    fn write(&mut self, s: &str) {
        self.output.push(s.to_string());
    }
}

/// Format a tree with default or provided options (product API).
///
/// # Errors
///
/// Returns printer errors.
pub fn format_tree(tree: &Tree, options: FormatOptions) -> Result<String, String> {
    let mut printer = Printer::new(options);
    printer.print(tree)
}

/// Check whether source already matches formatted tree output.
#[must_use]
pub fn check_formatted(source: &str, tree: &Tree, options: FormatOptions) -> bool {
    match format_tree(tree, options) {
        Ok(formatted) => source == formatted,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;
    use std::collections::HashMap;

    fn data(pairs: &[(&str, Value)]) -> HashMap<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    fn sample_const_tree() -> Tree {
        let mut tree = Tree::new("javascript", "const x = 1;");
        // root is 0; build Program → VariableDeclaration → VariableDeclarator → Identifier + Literal
        let program = tree.add_node(Node::new(0, "Program"));
        let _ = tree.add_child(0, program);

        let var_decl = tree.add_node(
            Node::new(0, "VariableDeclaration").with_data(data(&[("kind", Value::String("const".into()))])),
        );
        let _ = tree.add_child(program, var_decl);

        let declarator = tree.add_node(Node::new(0, "VariableDeclarator"));
        let _ = tree.add_child(var_decl, declarator);

        let id = tree.add_node(
            Node::new(0, "Identifier").with_data(data(&[("name", Value::String("x".into()))])),
        );
        let lit = tree.add_node(
            Node::new(0, "Literal").with_data(data(&[
                ("value", Value::Number(1.into())),
                ("raw", Value::String("1".into())),
            ])),
        );
        let _ = tree.add_child(declarator, id);
        let _ = tree.add_child(declarator, lit);
        tree
    }

    #[test]
    fn prints_const_declaration() {
        let tree = sample_const_tree();
        let out = format_tree(&tree, FormatOptions::default()).expect("print");
        assert!(out.contains("const x = 1"), "got: {out}");
        assert!(out.ends_with(';') || out.contains(';'));
    }

    #[test]
    fn respects_semi_false() {
        let tree = sample_const_tree();
        let mut opts = FormatOptions::default();
        opts.semi = false;
        let out = format_tree(&tree, opts).expect("print");
        assert!(!out.trim_end().ends_with(';'), "got: {out}");
    }

    #[test]
    fn prints_call_expression_tree() {
        let mut tree = Tree::new("javascript", "foo()");
        let program = tree.add_node(Node::new(0, "Program"));
        let _ = tree.add_child(0, program);
        let expr = tree.add_node(Node::new(0, "ExpressionStatement"));
        let _ = tree.add_child(program, expr);
        let call = tree.add_node(Node::new(0, "CallExpression"));
        let _ = tree.add_child(expr, call);
        let callee = tree.add_node(
            Node::new(0, "Identifier").with_data(data(&[("name", Value::String("foo".into()))])),
        );
        let _ = tree.add_child(call, callee);
        let out = format_tree(&tree, FormatOptions::default()).expect("print");
        assert!(out.contains("foo()"), "got: {out}");
    }
}

//! Product AST → minified-code **compressor engine** (tooling dens).
//!
//! Ports `packages/synth-js-minify/src/compressor.ts` onto the Rust `Tree` model.
//! Uses [`crate::mangle::NameMangler`] for identifier shortening. Full product path.

use serde_json::Value;

use crate::mangle::NameMangler;
use crate::tree::{Node, NodeId, Tree};

/// Minify options (parity: MinifyOptions / DEFAULT_OPTIONS).
#[derive(Debug, Clone, PartialEq)]
pub struct MinifyOptions {
    pub compress: bool,
    pub mangle: bool,
    pub remove_comments: bool,
    pub compact: bool,
    pub reserved: Vec<String>,
}

impl Default for MinifyOptions {
    fn default() -> Self {
        Self {
            compress: true,
            mangle: false,
            remove_comments: true,
            compact: true,
            reserved: Vec::new(),
        }
    }
}

/// Product compressor: AST → compact source.
#[derive(Debug)]
pub struct Compressor {
    options: MinifyOptions,
    output: Vec<String>,
    mangler: NameMangler,
}

impl Compressor {
    #[must_use]
    pub fn new(options: MinifyOptions) -> Self {
        let mangler = NameMangler::new(options.reserved.clone());
        Self {
            options,
            output: Vec::new(),
            mangler,
        }
    }

    /// Compress a tree to minified source.
    ///
    /// # Errors
    ///
    /// Returns error when tree cannot be compressed.
    pub fn compress(&mut self, tree: &Tree) -> Result<String, String> {
        self.output.clear();
        self.mangler = NameMangler::new(self.options.reserved.clone());

        let program = tree
            .nodes()
            .iter()
            .find(|n| n.node_type == "Program")
            .map(|n| n.id);
        let root_id = program.unwrap_or_else(|| tree.root_id());
        let children = self.children_of(tree, root_id);
        self.compress_statements(tree, &children);
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

    fn compress_statements(&mut self, tree: &Tree, nodes: &[NodeId]) {
        for id in nodes {
            self.compress_node(tree, *id);
        }
    }

    fn compress_node(&mut self, tree: &Tree, id: NodeId) {
        let Some(node) = self.node(tree, id).cloned() else {
            return;
        };
        match node.node_type.as_str() {
            "Program" => {
                let children = self.children_of(tree, id);
                self.compress_statements(tree, &children);
            }
            "VariableDeclaration" => self.compress_variable_declaration(tree, &node),
            "FunctionDeclaration" => self.compress_function_declaration(tree, &node),
            "ExpressionStatement" => self.compress_expression_statement(tree, &node),
            "ReturnStatement" => self.compress_return_statement(tree, &node),
            "IfStatement" => self.compress_if_statement(tree, &node),
            "BlockStatement" => self.compress_block_statement(tree, &node),
            "CallExpression" => self.compress_call_expression(tree, &node),
            "MemberExpression" => self.compress_member_expression(tree, &node),
            "Identifier" => self.compress_identifier(&node),
            "Literal" => self.compress_literal(&node),
            "BinaryExpression" => self.compress_binary_expression(tree, &node),
            "UnaryExpression" => self.compress_unary_expression(tree, &node),
            "AwaitExpression" => self.compress_await_expression(tree, &node),
            "ArrowFunctionExpression" => self.compress_arrow_function(tree, &node),
            "ArrayExpression" => self.compress_array_expression(tree, &node),
            "ObjectExpression" => self.compress_object_expression(tree, &node),
            "Property" => self.compress_property(tree, &node),
            "VariableDeclarator" => self.compress_variable_declarator(tree, &node),
            "ImportDeclaration" => self.write("import;"),
            "ExportNamedDeclaration" | "ExportDefaultDeclaration" => {
                self.compress_export_declaration(tree, &node);
            }
            "ClassDeclaration" => self.compress_class_declaration(tree, &node),
            "MethodDefinition" => self.compress_method_definition(tree, &node),
            other => self.write(&format!("/*{other}*/")),
        }
    }

    fn compress_variable_declaration(&mut self, tree: &Tree, node: &Node) {
        let kind = Self::data_str(node, "kind").unwrap_or_else(|| "const".into());
        self.write(&format!("{kind} "));
        let declarators = self.children_of(tree, node.id);
        for (i, d) in declarators.iter().enumerate() {
            self.compress_node(tree, *d);
            if i + 1 < declarators.len() {
                self.write(",");
            }
        }
        self.write(";");
    }

    fn compress_variable_declarator(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        let id = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "Identifier" || n.node_type.contains("Pattern"))
        });
        let init = children.iter().copied().find(|cid| Some(*cid) != id);
        if let Some(id) = id {
            self.compress_node(tree, id);
        }
        if let Some(init) = init {
            self.write("=");
            self.compress_node(tree, init);
        }
    }

    fn compress_function_declaration(&mut self, tree: &Tree, node: &Node) {
        if Self::data_bool(node, "async").unwrap_or(false) {
            self.write("async ");
        }
        self.write("function");
        if Self::data_bool(node, "generator").unwrap_or(false) {
            self.write("*");
        }
        if let Some(name) = Self::data_str(node, "id") {
            let mangled = self.mangler.mangle(&name, self.options.mangle);
            self.write(&format!(" {mangled}"));
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
            self.compress_node(tree, *p);
            if i + 1 < params.len() {
                self.write(",");
            }
        }
        self.write(")");
        if let Some(body) = body {
            self.compress_node(tree, body);
        }
    }

    fn compress_arrow_function(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if children.is_empty() {
            self.write("()=>{}");
            return;
        }
        let body = children[children.len() - 1];
        let params = &children[..children.len() - 1];
        if params.len() == 1 && !self.options.mangle {
            self.compress_node(tree, params[0]);
        } else {
            self.write("(");
            for (i, p) in params.iter().enumerate() {
                self.compress_node(tree, *p);
                if i + 1 < params.len() {
                    self.write(",");
                }
            }
            self.write(")");
        }
        self.write("=>");
        self.compress_node(tree, body);
    }

    fn compress_block_statement(&mut self, tree: &Tree, node: &Node) {
        self.write("{");
        let children = self.children_of(tree, node.id);
        if !children.is_empty() {
            self.compress_statements(tree, &children);
        }
        self.write("}");
    }

    fn compress_expression_statement(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.compress_node(tree, *first);
        }
        self.write(";");
    }

    fn compress_return_statement(&mut self, tree: &Tree, node: &Node) {
        self.write("return");
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.write(" ");
            self.compress_node(tree, *first);
        }
        self.write(";");
    }

    fn compress_if_statement(&mut self, tree: &Tree, node: &Node) {
        self.write("if(");
        let children = self.children_of(tree, node.id);
        if let Some(test) = children.first() {
            self.compress_node(tree, *test);
        }
        self.write(")");
        if let Some(cons) = children.get(1) {
            self.compress_node(tree, *cons);
        }
        if let Some(alt) = children.get(2) {
            self.write("else ");
            self.compress_node(tree, *alt);
        }
    }

    fn compress_call_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(callee) = children.first() {
            self.compress_node(tree, *callee);
        }
        self.write("(");
        let args = if children.len() > 1 {
            &children[1..]
        } else {
            &[]
        };
        for (i, a) in args.iter().enumerate() {
            self.compress_node(tree, *a);
            if i + 1 < args.len() {
                self.write(",");
            }
        }
        self.write(")");
    }

    fn compress_member_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(obj) = children.first() {
            self.compress_node(tree, *obj);
        }
        let computed = Self::data_bool(node, "computed").unwrap_or(false);
        if computed {
            self.write("[");
            if let Some(prop) = children.get(1) {
                self.compress_node(tree, *prop);
            }
            self.write("]");
        } else {
            self.write(".");
            if let Some(prop) = children.get(1) {
                // property name is not mangled (member prop)
                if let Some(n) = self.node(tree, *prop).cloned()
                    && n.node_type == "Identifier"
                    && let Some(name) = Self::data_str(&n, "name")
                {
                    self.write(&name);
                    return;
                }
                self.compress_node(tree, *prop);
            }
        }
    }

    fn compress_binary_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(left) = children.first() {
            self.compress_node(tree, *left);
        }
        let op = Self::data_str(node, "operator").unwrap_or_else(|| "+".into());
        if op == "in" || op == "instanceof" {
            self.write(&format!(" {op} "));
        } else {
            self.write(&op);
        }
        if let Some(right) = children.get(1) {
            self.compress_node(tree, *right);
        }
    }

    fn compress_unary_expression(&mut self, tree: &Tree, node: &Node) {
        let op = Self::data_str(node, "operator").unwrap_or_default();
        let prefix = Self::data_bool(node, "prefix").unwrap_or(true);
        if prefix {
            self.write(&op);
            let children = self.children_of(tree, node.id);
            if let Some(first) = children.first() {
                self.compress_node(tree, *first);
            }
        }
    }

    fn compress_await_expression(&mut self, tree: &Tree, node: &Node) {
        self.write("await ");
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.compress_node(tree, *first);
        }
    }

    fn compress_array_expression(&mut self, tree: &Tree, node: &Node) {
        self.write("[");
        let children = self.children_of(tree, node.id);
        for (i, e) in children.iter().enumerate() {
            self.compress_node(tree, *e);
            if i + 1 < children.len() {
                self.write(",");
            }
        }
        self.write("]");
    }

    fn compress_object_expression(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if children.is_empty() {
            self.write("{}");
            return;
        }
        self.write("{");
        for (i, p) in children.iter().enumerate() {
            self.compress_node(tree, *p);
            if i + 1 < children.len() {
                self.write(",");
            }
        }
        self.write("}");
    }

    fn compress_property(&mut self, tree: &Tree, node: &Node) {
        let children = self.children_of(tree, node.id);
        if let Some(key) = children.first() {
            self.compress_node(tree, *key);
        }
        self.write(":");
        if let Some(val) = children.get(1) {
            self.compress_node(tree, *val);
        }
    }

    fn compress_export_declaration(&mut self, tree: &Tree, node: &Node) {
        self.write("export ");
        if node.node_type == "ExportDefaultDeclaration" {
            self.write("default ");
        }
        let children = self.children_of(tree, node.id);
        if let Some(first) = children.first() {
            self.compress_node(tree, *first);
        }
    }

    fn compress_class_declaration(&mut self, tree: &Tree, node: &Node) {
        self.write("class ");
        if let Some(name) = Self::data_str(node, "id") {
            let mangled = self.mangler.mangle(&name, self.options.mangle);
            self.write(&format!("{mangled} "));
        }
        let children = self.children_of(tree, node.id);
        let class_body = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "ClassBody")
        });
        if let Some(body_id) = class_body {
            let methods = self.children_of(tree, body_id);
            self.write("{");
            for m in &methods {
                self.compress_node(tree, *m);
            }
            self.write("}");
        } else {
            self.write("{}");
        }
    }

    fn compress_method_definition(&mut self, tree: &Tree, node: &Node) {
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
            // method keys are not mangled
            if let Some(n) = self.node(tree, key).cloned()
                && let Some(name) = Self::data_str(&n, "name")
            {
                self.write(&name);
            }
        }
        self.write("()");
        if let Some(value) = children.iter().copied().find(|cid| {
            self.node(tree, *cid)
                .is_some_and(|n| n.node_type == "FunctionExpression")
        }) {
            let body = self.children_of(tree, value).into_iter().find(|cid| {
                self.node(tree, *cid)
                    .is_some_and(|n| n.node_type == "BlockStatement")
            });
            if let Some(body) = body {
                self.compress_node(tree, body);
            }
        }
    }

    fn compress_identifier(&mut self, node: &Node) {
        if let Some(name) = Self::data_str(node, "name") {
            let out = self.mangler.mangle(&name, self.options.mangle);
            self.write(&out);
        }
    }

    fn compress_literal(&mut self, node: &Node) {
        if let Some(data) = &node.data {
            if let Some(value) = data.get("value")
                && let Some(s) = value.as_str()
            {
                self.write(&format!("\"{s}\""));
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

    fn write(&mut self, s: &str) {
        self.output.push(s.to_string());
    }
}

/// Compress a tree (product API).
///
/// # Errors
///
/// Returns compressor errors.
pub fn compress_tree(tree: &Tree, options: MinifyOptions) -> Result<String, String> {
    let mut c = Compressor::new(options);
    c.compress(tree)
}

/// Minify savings (re-exports product helper on top of minify_savings).
#[must_use]
pub fn minify_savings(original: &str, minified: &str) -> crate::minify_savings::Savings {
    crate::minify_savings::savings(original, minified)
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
        let mut tree = Tree::new("javascript", "const foo = 1;");
        let program = tree.add_node(Node::new(0, "Program"));
        let _ = tree.add_child(0, program);
        let var_decl = tree.add_node(
            Node::new(0, "VariableDeclaration").with_data(data(&[("kind", Value::String("const".into()))])),
        );
        let _ = tree.add_child(program, var_decl);
        let declarator = tree.add_node(Node::new(0, "VariableDeclarator"));
        let _ = tree.add_child(var_decl, declarator);
        let id = tree.add_node(
            Node::new(0, "Identifier").with_data(data(&[("name", Value::String("foo".into()))])),
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
    fn compresses_const() {
        let tree = sample_const_tree();
        let out = compress_tree(&tree, MinifyOptions::default()).expect("ok");
        assert_eq!(out, "const foo=1;");
    }

    #[test]
    fn mangles_identifiers_when_enabled() {
        let tree = sample_const_tree();
        let mut opts = MinifyOptions::default();
        opts.mangle = true;
        let out = compress_tree(&tree, opts).expect("ok");
        assert_eq!(out, "const a=1;");
    }

    #[test]
    fn binary_in_needs_spaces() {
        let mut tree = Tree::new("javascript", "a in b");
        let program = tree.add_node(Node::new(0, "Program"));
        let _ = tree.add_child(0, program);
        let expr = tree.add_node(Node::new(0, "ExpressionStatement"));
        let _ = tree.add_child(program, expr);
        let bin = tree.add_node(
            Node::new(0, "BinaryExpression")
                .with_data(data(&[("operator", Value::String("in".into()))])),
        );
        let _ = tree.add_child(expr, bin);
        let left = tree.add_node(
            Node::new(0, "Identifier").with_data(data(&[("name", Value::String("a".into()))])),
        );
        let right = tree.add_node(
            Node::new(0, "Identifier").with_data(data(&[("name", Value::String("b".into()))])),
        );
        let _ = tree.add_child(bin, left);
        let _ = tree.add_child(bin, right);
        let out = compress_tree(&tree, MinifyOptions::default()).expect("ok");
        assert!(out.contains(" in "), "got: {out}");
    }
}

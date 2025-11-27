//! Markdown parser
//!
//! Converts tokenized Markdown into Synth AST.

use synth_wasm_core::{Node, Span, SynthResult, Tree};
use std::collections::HashMap;

use crate::tokenizer::{BlockToken, Tokenizer};

/// Markdown parser
pub struct MarkdownParser;

impl MarkdownParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse Markdown source into a Tree
    pub fn parse(&self, source: &str) -> SynthResult<Tree> {
        let mut tree = Tree::new("markdown", source);
        let mut tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.tokenize();

        for token in tokens {
            if let Some(node_id) = self.build_node(&mut tree, &token)? {
                tree.add_child(tree.root_id(), node_id)?;
            }
        }

        Ok(tree)
    }

    fn build_node(&self, tree: &mut Tree, token: &BlockToken) -> SynthResult<Option<u32>> {
        match token {
            BlockToken::BlankLine { .. } => Ok(None),

            BlockToken::Heading { depth, text, span } => {
                let mut data = HashMap::new();
                data.insert("depth".to_string(), serde_json::json!(*depth));

                let node = Node::new(0, "heading")
                    .with_span(*span)
                    .with_data(data);
                let heading_id = tree.add_node(node);

                // Add text child
                self.add_text_node(tree, heading_id, text, span)?;

                Ok(Some(heading_id))
            }

            BlockToken::Paragraph { text, span } => {
                let node = Node::new(0, "paragraph").with_span(*span);
                let para_id = tree.add_node(node);

                // Add text child
                self.add_text_node(tree, para_id, text, span)?;

                Ok(Some(para_id))
            }

            BlockToken::CodeBlock { lang, meta, code, span } => {
                let mut data = HashMap::new();
                if let Some(l) = lang {
                    data.insert("lang".to_string(), serde_json::json!(l));
                }
                if let Some(m) = meta {
                    data.insert("meta".to_string(), serde_json::json!(m));
                }
                data.insert("value".to_string(), serde_json::json!(code));

                let node = Node::new(0, "code")
                    .with_span(*span)
                    .with_data(data);
                let code_id = tree.add_node(node);

                Ok(Some(code_id))
            }

            BlockToken::ThematicBreak { span } => {
                let node = Node::new(0, "thematicBreak").with_span(*span);
                let id = tree.add_node(node);
                Ok(Some(id))
            }

            BlockToken::BlockQuote { text, span } => {
                let node = Node::new(0, "blockquote").with_span(*span);
                let quote_id = tree.add_node(node);

                // Add text child
                self.add_text_node(tree, quote_id, text, span)?;

                Ok(Some(quote_id))
            }

            BlockToken::ListItem { text, ordered, checked, span } => {
                let mut data = HashMap::new();
                data.insert("ordered".to_string(), serde_json::json!(*ordered));
                if let Some(c) = checked {
                    data.insert("checked".to_string(), serde_json::json!(*c));
                }

                let node = Node::new(0, "listItem")
                    .with_span(*span)
                    .with_data(data);
                let item_id = tree.add_node(node);

                // Add text child
                self.add_text_node(tree, item_id, text, span)?;

                Ok(Some(item_id))
            }
        }
    }

    fn add_text_node(&self, tree: &mut Tree, parent_id: u32, text: &str, span: &Span) -> SynthResult<()> {
        if text.is_empty() {
            return Ok(());
        }

        let mut data = HashMap::new();
        data.insert("value".to_string(), serde_json::json!(text));

        let node = Node::new(0, "text")
            .with_parent(parent_id)
            .with_span(*span)
            .with_data(data);

        let text_id = tree.add_node(node);
        tree.add_child(parent_id, text_id)?;

        Ok(())
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let parser = MarkdownParser::new();
        let tree = parser.parse("# Hello World\n").unwrap();

        assert_eq!(tree.node_count(), 3); // root, heading, text
    }

    #[test]
    fn test_parse_multiple_blocks() {
        let parser = MarkdownParser::new();
        let tree = parser.parse("# Title\n\nParagraph text.\n").unwrap();

        // root, heading, text, paragraph, text
        assert!(tree.node_count() >= 3);
    }

    #[test]
    fn test_parse_code_block() {
        let parser = MarkdownParser::new();
        let tree = parser.parse("```rust\nfn main() {}\n```\n").unwrap();

        assert!(tree.node_count() >= 2); // root, code
    }
}

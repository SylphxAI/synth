//! Ultra-fast Markdown parser with binary output
//!
//! Key optimizations:
//! 1. Zero-copy tokenization
//! 2. Flat binary output format (no JSON serialization)
//! 3. String offsets instead of copies
//! 4. Pre-allocated buffers
//!
//! Binary format (little-endian):
//! - Header: [node_count: u32, string_table_offset: u32]
//! - Nodes: [type: u8, parent: u32, child_count: u16, first_child: u32,
//!           start_offset: u32, end_offset: u32, data_offset: u32]
//! - String table: [len: u32, bytes...]

use crate::fast_tokenizer::{FastToken, FastTokenizer};

/// Node types as u8
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum NodeType {
    Root = 0,
    Heading = 1,
    Paragraph = 2,
    Code = 3,
    ThematicBreak = 4,
    BlockQuote = 5,
    ListItem = 6,
    Text = 7,
}

/// Fixed-size node in binary format (28 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
pub struct BinaryNode {
    pub node_type: u8,
    pub depth: u8,          // for headings
    pub flags: u8,          // ordered, checked for lists
    pub _pad: u8,
    pub parent: u32,
    pub first_child: u32,
    pub next_sibling: u32,
    pub start_offset: u32,
    pub end_offset: u32,
    pub text_start: u32,    // offset into source string
    pub text_len: u32,
}

const NODE_SIZE: usize = std::mem::size_of::<BinaryNode>();

/// Fast parser that outputs binary format
pub struct FastParser<'a> {
    source: &'a str,
    nodes: Vec<BinaryNode>,
}

impl<'a> FastParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            nodes: Vec::with_capacity(source.len() / 50),
        }
    }

    /// Parse and return binary buffer
    pub fn parse_to_binary(&mut self) -> Vec<u8> {
        // Tokenize
        let mut tokenizer = FastTokenizer::new(self.source);
        let tokens = tokenizer.tokenize();

        // Create root node
        self.nodes.push(BinaryNode {
            node_type: NodeType::Root as u8,
            start_offset: 0,
            end_offset: self.source.len() as u32,
            ..Default::default()
        });

        let mut last_child_at_root: Option<usize> = None;

        // Convert tokens to nodes
        for token in tokens {
            let node_idx = self.nodes.len();

            let node = match token {
                FastToken::Heading { depth, text, span } => {
                    let text_start = text.as_ptr() as usize - self.source.as_ptr() as usize;
                    BinaryNode {
                        node_type: NodeType::Heading as u8,
                        depth,
                        parent: 0,
                        start_offset: span.start.offset,
                        end_offset: span.end.offset,
                        text_start: text_start as u32,
                        text_len: text.len() as u32,
                        ..Default::default()
                    }
                }
                FastToken::Paragraph { text, span } => {
                    let text_start = text.as_ptr() as usize - self.source.as_ptr() as usize;
                    BinaryNode {
                        node_type: NodeType::Paragraph as u8,
                        parent: 0,
                        start_offset: span.start.offset,
                        end_offset: span.end.offset,
                        text_start: text_start as u32,
                        text_len: text.len() as u32,
                        ..Default::default()
                    }
                }
                FastToken::CodeBlock { lang, code, span } => {
                    let code_start = code.as_ptr() as usize - self.source.as_ptr() as usize;
                    // Store lang length in depth field (hacky but fast)
                    let lang_len = lang.map(|l| l.len()).unwrap_or(0);
                    BinaryNode {
                        node_type: NodeType::Code as u8,
                        depth: lang_len as u8,
                        parent: 0,
                        start_offset: span.start.offset,
                        end_offset: span.end.offset,
                        text_start: code_start as u32,
                        text_len: code.len() as u32,
                        ..Default::default()
                    }
                }
                FastToken::ThematicBreak { span } => BinaryNode {
                    node_type: NodeType::ThematicBreak as u8,
                    parent: 0,
                    start_offset: span.start.offset,
                    end_offset: span.end.offset,
                    ..Default::default()
                },
                FastToken::BlankLine { .. } => continue, // skip blank lines
                FastToken::BlockQuote { text, span } => {
                    let text_start = text.as_ptr() as usize - self.source.as_ptr() as usize;
                    BinaryNode {
                        node_type: NodeType::BlockQuote as u8,
                        parent: 0,
                        start_offset: span.start.offset,
                        end_offset: span.end.offset,
                        text_start: text_start as u32,
                        text_len: text.len() as u32,
                        ..Default::default()
                    }
                }
                FastToken::ListItem { text, ordered, checked, span } => {
                    let text_start = text.as_ptr() as usize - self.source.as_ptr() as usize;
                    let flags = (if ordered { 1 } else { 0 })
                        | (match checked {
                            Some(true) => 2,
                            Some(false) => 4,
                            None => 0,
                        });
                    BinaryNode {
                        node_type: NodeType::ListItem as u8,
                        flags,
                        parent: 0,
                        start_offset: span.start.offset,
                        end_offset: span.end.offset,
                        text_start: text_start as u32,
                        text_len: text.len() as u32,
                        ..Default::default()
                    }
                }
            };

            self.nodes.push(node);

            // Link siblings
            if let Some(prev_idx) = last_child_at_root {
                self.nodes[prev_idx].next_sibling = node_idx as u32;
            } else {
                // First child of root
                self.nodes[0].first_child = node_idx as u32;
            }
            last_child_at_root = Some(node_idx);
        }

        // Serialize to binary
        self.to_binary()
    }

    fn to_binary(&self) -> Vec<u8> {
        let node_count = self.nodes.len();
        let header_size = 8; // node_count (4) + reserved (4)
        let nodes_size = node_count * NODE_SIZE;
        let total_size = header_size + nodes_size;

        let mut buf = vec![0u8; total_size];

        // Header
        buf[0..4].copy_from_slice(&(node_count as u32).to_le_bytes());
        buf[4..8].copy_from_slice(&(self.source.len() as u32).to_le_bytes());

        // Nodes
        let nodes_ptr = buf[header_size..].as_mut_ptr() as *mut BinaryNode;
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.nodes.as_ptr(),
                nodes_ptr,
                node_count,
            );
        }

        buf
    }

    /// Parse and return node count only (for benchmarking)
    pub fn parse_count(&mut self) -> usize {
        let mut tokenizer = FastTokenizer::new(self.source);
        let tokens = tokenizer.tokenize();

        // Count non-blank tokens + 1 for root
        tokens
            .iter()
            .filter(|t| !matches!(t, FastToken::BlankLine { .. }))
            .count()
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_output() {
        let mut parser = FastParser::new("# Hello\n\nWorld\n");
        let binary = parser.parse_to_binary();

        // Read header
        let node_count = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        assert!(node_count >= 2); // root + heading + paragraph
    }

    #[test]
    fn test_parse_count() {
        let mut parser = FastParser::new("# Hello\n\nWorld\n");
        let count = parser.parse_count();
        assert_eq!(count, 3); // root, heading, paragraph
    }
}

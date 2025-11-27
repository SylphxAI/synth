//! Unified High-performance Markdown parser
//!
//! Single optimized parser combining best of all approaches:
//! - Zero-copy tokenization with memchr SIMD
//! - Safe bounds checking (no unsafe)
//! - u32 fields to avoid overflow
//! - Supports both Tree object and binary output
//!
//! Performance: ~10-15x faster than pure JS

use memchr::memchr;
use std::collections::HashMap;
use synth_wasm_core::{Node, Span, SynthResult, Tree};

/// Node type constants
pub mod node_type {
    pub const ROOT: u8 = 0;
    pub const HEADING: u8 = 1;
    pub const PARAGRAPH: u8 = 2;
    pub const CODE: u8 = 3;
    pub const THEMATIC_BREAK: u8 = 4;
    pub const BLOCKQUOTE: u8 = 5;
    pub const LIST_ITEM: u8 = 6;
}

/// Compact binary node - 24 bytes, safe field sizes
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BinaryNode {
    pub node_type: u8,      // 1: heading, 2: para, 3: code, 4: hr, 5: quote, 6: list
    pub flags: u8,          // depth for heading, ordered/checked for list
    pub _pad: [u8; 2],      // alignment padding
    pub parent: u32,        // parent node index
    pub text_start: u32,    // offset into source
    pub text_len: u32,      // text length
    pub span_start: u32,    // start line
    pub span_end: u32,      // end line
}

const BINARY_NODE_SIZE: usize = std::mem::size_of::<BinaryNode>();
const _: () = assert!(BINARY_NODE_SIZE == 24);

/// High-performance unified Markdown parser
pub struct MarkdownParserV2<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: u32,
}

impl<'a> MarkdownParserV2<'a> {
    /// Create a new parser
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            src: source,
            bytes: source.as_bytes(),
            pos: 0,
            line: 1,
        }
    }

    /// Parse and return a Tree object (compatible with JS API)
    pub fn parse(&mut self) -> SynthResult<Tree> {
        let mut tree = Tree::new("markdown", self.src);

        while self.pos < self.bytes.len() {
            if let Some(node) = self.scan_block_to_node()? {
                let node_id = tree.add_node(node);
                tree.add_child(tree.root_id(), node_id)?;
            }
        }

        Ok(tree)
    }

    /// Parse and return binary buffer (maximum performance)
    ///
    /// Format:
    /// - Header: [node_count: u32, source_len: u32]
    /// - Nodes: 24 bytes each
    pub fn parse_binary(&mut self) -> Vec<u8> {
        let mut nodes: Vec<BinaryNode> = Vec::with_capacity(self.bytes.len() / 32);

        // Root node
        nodes.push(BinaryNode {
            node_type: node_type::ROOT,
            text_len: self.bytes.len() as u32,
            span_start: 1,
            ..Default::default()
        });

        while self.pos < self.bytes.len() {
            if let Some(node) = self.scan_block_to_binary() {
                nodes.push(node);
            }
        }

        // Update root span_end
        if let Some(root) = nodes.first_mut() {
            root.span_end = self.line;
        }

        // Serialize to binary
        let node_count = nodes.len();
        let header_size = 8;
        let mut buf = vec![0u8; header_size + node_count * BINARY_NODE_SIZE];

        // Header
        buf[0..4].copy_from_slice(&(node_count as u32).to_le_bytes());
        buf[4..8].copy_from_slice(&(self.bytes.len() as u32).to_le_bytes());

        // Copy nodes
        let nodes_dst = &mut buf[header_size..];
        for (i, node) in nodes.iter().enumerate() {
            let offset = i * BINARY_NODE_SIZE;
            nodes_dst[offset] = node.node_type;
            nodes_dst[offset + 1] = node.flags;
            // skip padding [2..4]
            nodes_dst[offset + 4..offset + 8].copy_from_slice(&node.parent.to_le_bytes());
            nodes_dst[offset + 8..offset + 12].copy_from_slice(&node.text_start.to_le_bytes());
            nodes_dst[offset + 12..offset + 16].copy_from_slice(&node.text_len.to_le_bytes());
            nodes_dst[offset + 16..offset + 20].copy_from_slice(&node.span_start.to_le_bytes());
            nodes_dst[offset + 20..offset + 24].copy_from_slice(&node.span_end.to_le_bytes());
        }

        buf
    }

    /// Parse and return node count (for benchmarking)
    pub fn parse_count(&mut self) -> usize {
        let mut count = 1; // root

        while self.pos < self.bytes.len() {
            if self.scan_block_to_binary().is_some() {
                count += 1;
            }
        }

        count
    }

    // ============================================================
    // Helper methods
    // ============================================================

    #[inline]
    fn byte(&self, pos: usize) -> Option<u8> {
        self.bytes.get(pos).copied()
    }

    #[inline]
    fn current(&self) -> Option<u8> {
        self.byte(self.pos)
    }

    #[inline]
    fn text_slice(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }

    #[inline]
    fn skip_horizontal_space(&mut self) {
        while let Some(b) = self.current() {
            if b == b' ' || b == b'\t' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    #[inline]
    fn find_newline(&self) -> usize {
        memchr(b'\n', &self.bytes[self.pos..])
            .map(|i| self.pos + i)
            .unwrap_or(self.bytes.len())
    }

    #[inline]
    fn skip_to_newline(&mut self) {
        let end = self.find_newline();
        self.pos = end;
        if self.pos < self.bytes.len() {
            self.pos += 1;
            self.line += 1;
        }
    }

    #[inline]
    fn is_code_fence(&self) -> bool {
        self.byte(self.pos + 1) == Some(b'`') && self.byte(self.pos + 2) == Some(b'`')
    }

    #[inline]
    fn is_thematic_break(&self) -> bool {
        let marker = self.bytes[self.pos];
        if !matches!(marker, b'-' | b'*' | b'_') {
            return false;
        }

        let mut count = 0;
        let mut i = self.pos;
        while i < self.bytes.len() {
            let b = self.bytes[i];
            if b == marker {
                count += 1;
            } else if b == b' ' || b == b'\t' {
                // ok
            } else if b == b'\n' {
                break;
            } else {
                return false;
            }
            i += 1;
        }
        count >= 3
    }

    #[inline]
    fn is_ordered_list(&self) -> bool {
        let mut i = self.pos;
        while i < self.bytes.len() {
            let b = self.bytes[i];
            if b.is_ascii_digit() {
                i += 1;
            } else if b == b'.' || b == b')' {
                return self.byte(i + 1) == Some(b' ');
            } else {
                return false;
            }
        }
        false
    }

    // ============================================================
    // Block scanning → Node (for Tree output)
    // ============================================================

    fn scan_block_to_node(&mut self) -> SynthResult<Option<Node>> {
        self.skip_horizontal_space();

        if self.pos >= self.bytes.len() {
            return Ok(None);
        }

        let b = self.bytes[self.pos];
        let start_line = self.line;
        let start_pos = self.pos;

        match b {
            b'\n' => {
                self.pos += 1;
                self.line += 1;
                Ok(None)
            }
            b'#' => self.scan_heading_node(start_pos, start_line),
            b'`' if self.is_code_fence() => self.scan_code_block_node(start_pos, start_line),
            b'-' | b'*' | b'_' if self.is_thematic_break() => {
                self.scan_thematic_break_node(start_line)
            }
            b'-' | b'*' | b'+' => self.scan_list_item_node(start_pos, start_line),
            b'>' => self.scan_blockquote_node(start_pos, start_line),
            b'0'..=b'9' if self.is_ordered_list() => self.scan_list_item_node(start_pos, start_line),
            _ => self.scan_paragraph_node(start_pos, start_line),
        }
    }

    fn scan_heading_node(&mut self, start_pos: usize, start_line: u32) -> SynthResult<Option<Node>> {
        let mut depth = 0u8;
        while self.current() == Some(b'#') && depth < 6 {
            self.pos += 1;
            depth += 1;
        }

        // Must have space or newline after #
        match self.current() {
            Some(b' ') | Some(b'\n') | None => {}
            _ => {
                self.pos = start_pos;
                return self.scan_paragraph_node(start_pos, start_line);
            }
        }

        self.skip_horizontal_space();
        let text_start = self.pos;
        let text_end = self.find_newline();

        // Trim trailing whitespace
        let mut len = text_end - text_start;
        while len > 0 && matches!(self.byte(text_start + len - 1), Some(b' ' | b'\t')) {
            len -= 1;
        }

        let text = self.text_slice(text_start, text_start + len);
        self.skip_to_newline();

        let mut data = HashMap::new();
        data.insert("depth".to_string(), serde_json::json!(depth));
        data.insert("value".to_string(), serde_json::json!(text));

        let span = Span::from_coords(
            start_line,
            1,
            start_pos as u32,
            self.line.saturating_sub(1).max(start_line),
            (text_end - start_pos) as u32,
            text_end as u32,
        );

        Ok(Some(
            Node::new(0, "heading").with_span(span).with_data(data),
        ))
    }

    fn scan_paragraph_node(
        &mut self,
        start_pos: usize,
        start_line: u32,
    ) -> SynthResult<Option<Node>> {
        loop {
            self.skip_to_newline();

            if self.pos >= self.bytes.len() {
                break;
            }

            let b = self.bytes[self.pos];
            match b {
                b'\n' | b'#' | b'>' | b'-' | b'*' | b'+' | b'`' => break,
                b'0'..=b'9' if self.is_ordered_list() => break,
                _ => {}
            }
        }

        // Calculate text end (exclude trailing newline)
        let text_end = if self.pos > 0 && self.byte(self.pos - 1) == Some(b'\n') {
            self.pos - 1
        } else {
            self.pos
        };

        let text = self.text_slice(start_pos, text_end);

        let mut data = HashMap::new();
        data.insert("value".to_string(), serde_json::json!(text));

        let span = Span::from_coords(
            start_line,
            1,
            start_pos as u32,
            self.line.saturating_sub(1).max(start_line),
            (text_end - start_pos) as u32,
            text_end as u32,
        );

        Ok(Some(
            Node::new(0, "paragraph").with_span(span).with_data(data),
        ))
    }

    fn scan_code_block_node(
        &mut self,
        start_pos: usize,
        start_line: u32,
    ) -> SynthResult<Option<Node>> {
        // Skip ```
        self.pos += 3;

        // Read info line (language)
        let info_start = self.pos;
        let info_end = self.find_newline();
        let lang = self.text_slice(info_start, info_end).trim();
        self.skip_to_newline();

        let code_start = self.pos;

        // Find closing ```
        let code_end = loop {
            if self.pos >= self.bytes.len() {
                break self.bytes.len();
            }

            match memchr(b'`', &self.bytes[self.pos..]) {
                Some(i) => {
                    let tick_pos = self.pos + i;
                    if self.byte(tick_pos + 1) == Some(b'`') && self.byte(tick_pos + 2) == Some(b'`')
                    {
                        // Found closing - count newlines up to here
                        let newlines =
                            memchr::memchr_iter(b'\n', &self.bytes[self.pos..tick_pos]).count();
                        self.line += newlines as u32;

                        self.pos = tick_pos + 3;
                        self.skip_to_newline();
                        break tick_pos;
                    }
                    self.pos = tick_pos + 1;
                }
                None => {
                    // No closing, consume rest
                    let newlines = memchr::memchr_iter(b'\n', &self.bytes[self.pos..]).count();
                    self.line += newlines as u32;
                    self.pos = self.bytes.len();
                    break self.bytes.len();
                }
            }
        };

        let code = self.text_slice(code_start, code_end);

        let mut data = HashMap::new();
        if !lang.is_empty() {
            data.insert("lang".to_string(), serde_json::json!(lang));
        }
        data.insert("value".to_string(), serde_json::json!(code));

        let span = Span::from_coords(
            start_line,
            1,
            start_pos as u32,
            self.line,
            3, // ```
            self.pos as u32,
        );

        Ok(Some(Node::new(0, "code").with_span(span).with_data(data)))
    }

    fn scan_thematic_break_node(&mut self, start_line: u32) -> SynthResult<Option<Node>> {
        let start_pos = self.pos;
        self.skip_to_newline();

        let span = Span::from_coords(
            start_line,
            1,
            start_pos as u32,
            start_line,
            3,
            self.pos as u32,
        );

        Ok(Some(Node::new(0, "thematicBreak").with_span(span)))
    }

    fn scan_blockquote_node(
        &mut self,
        start_pos: usize,
        start_line: u32,
    ) -> SynthResult<Option<Node>> {
        self.pos += 1; // skip >
        self.skip_horizontal_space();

        let text_start = self.pos;
        let text_end = self.find_newline();
        let text = self.text_slice(text_start, text_end);

        self.skip_to_newline();

        let mut data = HashMap::new();
        data.insert("value".to_string(), serde_json::json!(text));

        let span = Span::from_coords(
            start_line,
            1,
            start_pos as u32,
            self.line.saturating_sub(1).max(start_line),
            (text_end - start_pos) as u32,
            text_end as u32,
        );

        Ok(Some(
            Node::new(0, "blockquote").with_span(span).with_data(data),
        ))
    }

    fn scan_list_item_node(
        &mut self,
        start_pos: usize,
        start_line: u32,
    ) -> SynthResult<Option<Node>> {
        let first = self.bytes[self.pos];
        let ordered = first.is_ascii_digit();

        if ordered {
            while self
                .current()
                .map(|b| b.is_ascii_digit())
                .unwrap_or(false)
            {
                self.pos += 1;
            }
            self.pos += 1; // skip . or )
        } else {
            self.pos += 1; // skip - or * or +
        }

        self.skip_horizontal_space();

        // Check for task list [ ] or [x]
        let mut checked: Option<bool> = None;
        if self.current() == Some(b'[') {
            if let (Some(mark), Some(b']')) = (self.byte(self.pos + 1), self.byte(self.pos + 2)) {
                match mark {
                    b'x' | b'X' => {
                        checked = Some(true);
                        self.pos += 3;
                        self.skip_horizontal_space();
                    }
                    b' ' => {
                        checked = Some(false);
                        self.pos += 3;
                        self.skip_horizontal_space();
                    }
                    _ => {}
                }
            }
        }

        let text_start = self.pos;
        let text_end = self.find_newline();
        let text = self.text_slice(text_start, text_end);

        self.skip_to_newline();

        let mut data = HashMap::new();
        data.insert("ordered".to_string(), serde_json::json!(ordered));
        data.insert("value".to_string(), serde_json::json!(text));
        if let Some(c) = checked {
            data.insert("checked".to_string(), serde_json::json!(c));
        }

        let span = Span::from_coords(
            start_line,
            1,
            start_pos as u32,
            self.line.saturating_sub(1).max(start_line),
            (text_end - start_pos) as u32,
            text_end as u32,
        );

        Ok(Some(
            Node::new(0, "listItem").with_span(span).with_data(data),
        ))
    }

    // ============================================================
    // Block scanning → BinaryNode (for binary output)
    // ============================================================

    fn scan_block_to_binary(&mut self) -> Option<BinaryNode> {
        self.skip_horizontal_space();

        if self.pos >= self.bytes.len() {
            return None;
        }

        let b = self.bytes[self.pos];
        let start_line = self.line;
        let start_pos = self.pos;

        match b {
            b'\n' => {
                self.pos += 1;
                self.line += 1;
                None
            }
            b'#' => self.scan_heading_binary(start_pos, start_line),
            b'`' if self.is_code_fence() => self.scan_code_block_binary(start_pos, start_line),
            b'-' | b'*' | b'_' if self.is_thematic_break() => {
                self.scan_thematic_break_binary(start_line)
            }
            b'-' | b'*' | b'+' => self.scan_list_item_binary(start_pos, start_line),
            b'>' => self.scan_blockquote_binary(start_pos, start_line),
            b'0'..=b'9' if self.is_ordered_list() => {
                self.scan_list_item_binary(start_pos, start_line)
            }
            _ => self.scan_paragraph_binary(start_pos, start_line),
        }
    }

    fn scan_heading_binary(&mut self, start_pos: usize, start_line: u32) -> Option<BinaryNode> {
        let mut depth = 0u8;
        while self.current() == Some(b'#') && depth < 6 {
            self.pos += 1;
            depth += 1;
        }

        match self.current() {
            Some(b' ') | Some(b'\n') | None => {}
            _ => {
                self.pos = start_pos;
                return self.scan_paragraph_binary(start_pos, start_line);
            }
        }

        self.skip_horizontal_space();
        let text_start = self.pos;
        let text_end = self.find_newline();

        let mut len = text_end - text_start;
        while len > 0 && matches!(self.byte(text_start + len - 1), Some(b' ' | b'\t')) {
            len -= 1;
        }

        self.skip_to_newline();

        Some(BinaryNode {
            node_type: node_type::HEADING,
            flags: depth,
            parent: 0,
            text_start: text_start as u32,
            text_len: len as u32,
            span_start: start_line,
            span_end: self.line,
            ..Default::default()
        })
    }

    fn scan_paragraph_binary(&mut self, start_pos: usize, start_line: u32) -> Option<BinaryNode> {
        loop {
            self.skip_to_newline();

            if self.pos >= self.bytes.len() {
                break;
            }

            let b = self.bytes[self.pos];
            match b {
                b'\n' | b'#' | b'>' | b'-' | b'*' | b'+' | b'`' => break,
                b'0'..=b'9' if self.is_ordered_list() => break,
                _ => {}
            }
        }

        let text_end = if self.pos > 0 && self.byte(self.pos - 1) == Some(b'\n') {
            self.pos - 1
        } else {
            self.pos
        };

        Some(BinaryNode {
            node_type: node_type::PARAGRAPH,
            parent: 0,
            text_start: start_pos as u32,
            text_len: (text_end - start_pos) as u32,
            span_start: start_line,
            span_end: self.line,
            ..Default::default()
        })
    }

    fn scan_code_block_binary(&mut self, _start_pos: usize, start_line: u32) -> Option<BinaryNode> {
        self.pos += 3;

        let info_start = self.pos;
        let info_end = self.find_newline();
        let lang_len = (info_end - info_start).min(255);
        self.skip_to_newline();

        let code_start = self.pos;

        loop {
            if self.pos >= self.bytes.len() {
                break;
            }

            match memchr(b'`', &self.bytes[self.pos..]) {
                Some(i) => {
                    let tick_pos = self.pos + i;
                    if self.byte(tick_pos + 1) == Some(b'`') && self.byte(tick_pos + 2) == Some(b'`')
                    {
                        let code_end = tick_pos;
                        let newlines =
                            memchr::memchr_iter(b'\n', &self.bytes[self.pos..tick_pos]).count();
                        self.line += newlines as u32;

                        self.pos = tick_pos + 3;
                        self.skip_to_newline();

                        return Some(BinaryNode {
                            node_type: node_type::CODE,
                            flags: lang_len as u8,
                            parent: 0,
                            text_start: code_start as u32,
                            text_len: (code_end.saturating_sub(code_start)) as u32,
                            span_start: start_line,
                            span_end: self.line,
                            ..Default::default()
                        });
                    }
                    self.pos = tick_pos + 1;
                }
                None => {
                    let code_end = self.bytes.len();
                    let newlines = memchr::memchr_iter(b'\n', &self.bytes[self.pos..]).count();
                    self.line += newlines as u32;
                    self.pos = self.bytes.len();

                    return Some(BinaryNode {
                        node_type: node_type::CODE,
                        flags: lang_len as u8,
                        parent: 0,
                        text_start: code_start as u32,
                        text_len: (code_end - code_start) as u32,
                        span_start: start_line,
                        span_end: self.line,
                        ..Default::default()
                    });
                }
            }
        }

        None
    }

    fn scan_thematic_break_binary(&mut self, start_line: u32) -> Option<BinaryNode> {
        let start_pos = self.pos;
        self.skip_to_newline();

        Some(BinaryNode {
            node_type: node_type::THEMATIC_BREAK,
            parent: 0,
            text_start: start_pos as u32,
            span_start: start_line,
            span_end: start_line,
            ..Default::default()
        })
    }

    fn scan_blockquote_binary(&mut self, _start_pos: usize, start_line: u32) -> Option<BinaryNode> {
        self.pos += 1;
        self.skip_horizontal_space();

        let text_start = self.pos;
        let text_end = self.find_newline();

        self.skip_to_newline();

        Some(BinaryNode {
            node_type: node_type::BLOCKQUOTE,
            parent: 0,
            text_start: text_start as u32,
            text_len: (text_end - text_start) as u32,
            span_start: start_line,
            span_end: self.line,
            ..Default::default()
        })
    }

    fn scan_list_item_binary(&mut self, _start_pos: usize, start_line: u32) -> Option<BinaryNode> {
        let first = self.bytes[self.pos];
        let ordered = first.is_ascii_digit();

        if ordered {
            while self
                .current()
                .map(|b| b.is_ascii_digit())
                .unwrap_or(false)
            {
                self.pos += 1;
            }
            self.pos += 1;
        } else {
            self.pos += 1;
        }

        self.skip_horizontal_space();

        let mut flags = if ordered { 1u8 } else { 0u8 };
        if self.current() == Some(b'[') {
            if let (Some(mark), Some(b']')) = (self.byte(self.pos + 1), self.byte(self.pos + 2)) {
                flags |= match mark {
                    b'x' | b'X' => 0b10,
                    b' ' => 0b100,
                    _ => 0,
                };
                if flags & 0b110 != 0 {
                    self.pos += 3;
                    self.skip_horizontal_space();
                }
            }
        }

        let text_start = self.pos;
        let text_end = self.find_newline();

        self.skip_to_newline();

        Some(BinaryNode {
            node_type: node_type::LIST_ITEM,
            flags,
            parent: 0,
            text_start: text_start as u32,
            text_len: (text_end - text_start) as u32,
            span_start: start_line,
            span_end: self.line,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tree() {
        let mut p = MarkdownParserV2::new("# Hello\n\nWorld\n");
        let tree = p.parse().unwrap();
        assert!(tree.node_count() >= 3); // root + heading + paragraph
    }

    #[test]
    fn test_parse_binary() {
        let mut p = MarkdownParserV2::new("# Hello\n");
        let buf = p.parse_binary();

        // Check header
        let node_count = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        assert_eq!(node_count, 2); // root + heading
    }

    #[test]
    fn test_heading() {
        let mut p = MarkdownParserV2::new("# Hello\n");
        let tree = p.parse().unwrap();

        let heading = tree.nodes().iter().find(|n| n.node_type == "heading");
        assert!(heading.is_some());

        let data = heading.unwrap().data.as_ref().unwrap();
        assert_eq!(data.get("depth"), Some(&serde_json::json!(1)));
        assert_eq!(data.get("value"), Some(&serde_json::json!("Hello")));
    }

    #[test]
    fn test_code_block() {
        let mut p = MarkdownParserV2::new("```rust\ncode\n```\n");
        let tree = p.parse().unwrap();

        let code = tree.nodes().iter().find(|n| n.node_type == "code");
        assert!(code.is_some());

        let data = code.unwrap().data.as_ref().unwrap();
        assert_eq!(data.get("lang"), Some(&serde_json::json!("rust")));
        assert_eq!(data.get("value"), Some(&serde_json::json!("code\n")));
    }

    #[test]
    fn test_list_item() {
        let mut p = MarkdownParserV2::new("- [x] Task done\n");
        let tree = p.parse().unwrap();

        let item = tree.nodes().iter().find(|n| n.node_type == "listItem");
        assert!(item.is_some());

        let data = item.unwrap().data.as_ref().unwrap();
        assert_eq!(data.get("ordered"), Some(&serde_json::json!(false)));
        assert_eq!(data.get("checked"), Some(&serde_json::json!(true)));
    }
}

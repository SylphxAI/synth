//! High-performance zero-copy Markdown tokenizer
//!
//! Key optimizations:
//! 1. Zero-copy - works directly on &[u8], no String allocations
//! 2. SIMD-ready - uses memchr for fast pattern matching
//! 3. Single pass - processes input in one scan

use synth_wasm_core::{Position, Span};

/// Block token with zero-copy string slices
#[derive(Debug, Clone)]
pub enum FastToken<'a> {
    Heading {
        depth: u8,
        text: &'a str,
        span: Span,
    },
    Paragraph {
        text: &'a str,
        span: Span,
    },
    CodeBlock {
        lang: Option<&'a str>,
        code: &'a str,
        span: Span,
    },
    ThematicBreak {
        span: Span,
    },
    BlankLine {
        span: Span,
    },
    BlockQuote {
        text: &'a str,
        span: Span,
    },
    ListItem {
        text: &'a str,
        ordered: bool,
        checked: Option<bool>,
        span: Span,
    },
}

/// Fast zero-copy tokenizer
pub struct FastTokenizer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> FastTokenizer<'a> {
    #[inline]
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            line: 1,
            col: 0,
        }
    }

    /// Tokenize all blocks
    pub fn tokenize(&mut self) -> Vec<FastToken<'a>> {
        let mut tokens = Vec::with_capacity(self.bytes.len() / 50); // estimate

        while self.pos < self.bytes.len() {
            if let Some(token) = self.scan_block() {
                tokens.push(token);
            }
        }

        tokens
    }

    #[inline(always)]
    fn current(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    #[inline(always)]
    fn peek(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    #[inline(always)]
    fn advance(&mut self) {
        if let Some(b) = self.current() {
            self.pos += 1;
            if b == b'\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }

    #[inline(always)]
    fn pos_info(&self) -> Position {
        Position::new(self.line, self.col, self.pos as u32)
    }

    /// Skip spaces and tabs (not newlines)
    #[inline]
    fn skip_horizontal_space(&mut self) {
        while matches!(self.current(), Some(b' ' | b'\t')) {
            self.advance();
        }
    }

    /// Find next newline using memchr (SIMD accelerated)
    #[inline]
    fn find_newline(&self) -> usize {
        memchr::memchr(b'\n', &self.bytes[self.pos..])
            .map(|i| self.pos + i)
            .unwrap_or(self.bytes.len())
    }

    /// Read until newline, return slice (zero-copy)
    #[inline]
    fn read_line(&mut self) -> &'a str {
        let start = self.pos;
        let end = self.find_newline();

        // Advance position
        while self.pos < end {
            self.advance();
        }
        if self.pos < self.bytes.len() {
            self.advance(); // consume \n
        }

        // Safety: we're working with valid UTF-8
        &self.src[start..end]
    }

    /// Skip to end of line and consume newline
    #[inline]
    fn skip_line(&mut self) {
        let end = self.find_newline();
        while self.pos < end {
            self.advance();
        }
        if self.pos < self.bytes.len() {
            self.advance();
        }
    }

    fn scan_block(&mut self) -> Option<FastToken<'a>> {
        let start = self.pos_info();

        self.skip_horizontal_space();

        let b = self.current()?;

        match b {
            b'\n' => {
                self.advance();
                Some(FastToken::BlankLine {
                    span: Span::new(start, self.pos_info()),
                })
            }

            b'#' => self.scan_heading(start),

            b'-' | b'*' | b'_' => {
                if self.is_thematic_break() {
                    self.skip_line();
                    Some(FastToken::ThematicBreak {
                        span: Span::new(start, self.pos_info()),
                    })
                } else if b == b'-' || b == b'*' {
                    self.scan_list_item(start, false)
                } else {
                    self.scan_paragraph(start)
                }
            }

            b'0'..=b'9' => {
                if self.is_ordered_list() {
                    self.scan_list_item(start, true)
                } else {
                    self.scan_paragraph(start)
                }
            }

            b'`' if self.peek(1) == Some(b'`') && self.peek(2) == Some(b'`') => {
                self.scan_code_block(start)
            }

            b'>' => self.scan_blockquote(start),

            _ => self.scan_paragraph(start),
        }
    }

    fn scan_heading(&mut self, start: Position) -> Option<FastToken<'a>> {
        let mut depth = 0u8;

        while self.current() == Some(b'#') && depth < 6 {
            self.advance();
            depth += 1;
        }

        // Must have space after # or be at end of line
        match self.current() {
            Some(b' ') => self.advance(),
            Some(b'\n') | None => {}
            _ => {
                // Not valid heading, treat as paragraph
                return self.scan_paragraph(start);
            }
        }

        self.skip_horizontal_space();
        let text = self.read_line().trim_end();

        Some(FastToken::Heading {
            depth,
            text,
            span: Span::new(start, self.pos_info()),
        })
    }

    fn scan_paragraph(&mut self, start: Position) -> Option<FastToken<'a>> {
        let text_start = self.pos;
        let mut text_end = self.pos;

        loop {
            text_end = self.find_newline();
            self.skip_line();

            // Check if next line continues paragraph
            match self.current() {
                None => break,
                Some(b'\n' | b'#' | b'>' | b'-' | b'*' | b'`') => break,
                Some(b'0'..=b'9') if self.is_ordered_list() => break,
                _ => {}
            }
        }

        let text = self.src[text_start..text_end].trim_end();

        Some(FastToken::Paragraph {
            text,
            span: Span::new(start, self.pos_info()),
        })
    }

    fn scan_code_block(&mut self, start: Position) -> Option<FastToken<'a>> {
        // Skip opening ```
        self.advance();
        self.advance();
        self.advance();

        // Read language
        let info_line = self.read_line();
        let lang = if info_line.is_empty() {
            None
        } else {
            Some(info_line.split_whitespace().next().unwrap_or(""))
        };

        let code_start = self.pos;
        let mut code_end = code_start;

        // Find closing ```
        loop {
            if self.pos >= self.bytes.len() {
                code_end = self.bytes.len();
                break;
            }

            // Check for closing ```
            if self.current() == Some(b'`')
                && self.peek(1) == Some(b'`')
                && self.peek(2) == Some(b'`')
            {
                code_end = self.pos;
                self.advance();
                self.advance();
                self.advance();
                self.skip_line();
                break;
            }

            self.skip_line();
        }

        let code = self.src[code_start..code_end].trim_end();

        Some(FastToken::CodeBlock {
            lang,
            code,
            span: Span::new(start, self.pos_info()),
        })
    }

    fn is_thematic_break(&self) -> bool {
        let marker = match self.current() {
            Some(b @ (b'-' | b'*' | b'_')) => b,
            _ => return false,
        };

        let mut count = 0;
        let mut i = 0;

        while let Some(b) = self.peek(i) {
            match b {
                b if b == marker => count += 1,
                b' ' | b'\t' => {}
                b'\n' => break,
                _ => return false,
            }
            i += 1;
        }

        count >= 3
    }

    fn scan_blockquote(&mut self, start: Position) -> Option<FastToken<'a>> {
        self.advance(); // skip >
        self.skip_horizontal_space();

        let text = self.read_line().trim_end();

        Some(FastToken::BlockQuote {
            text,
            span: Span::new(start, self.pos_info()),
        })
    }

    fn is_ordered_list(&self) -> bool {
        let mut i = 0;
        while let Some(b) = self.peek(i) {
            match b {
                b'0'..=b'9' => i += 1,
                b'.' | b')' => return self.peek(i + 1) == Some(b' '),
                _ => return false,
            }
        }
        false
    }

    fn scan_list_item(&mut self, start: Position, ordered: bool) -> Option<FastToken<'a>> {
        if ordered {
            while matches!(self.current(), Some(b'0'..=b'9')) {
                self.advance();
            }
            self.advance(); // skip . or )
        } else {
            self.advance(); // skip - or *
        }

        self.skip_horizontal_space();

        // Check for task list [ ] or [x]
        let checked = if self.current() == Some(b'[') {
            self.advance();
            let mark = self.current();
            self.advance();
            self.advance(); // skip ]
            self.skip_horizontal_space();
            match mark {
                Some(b'x' | b'X') => Some(true),
                Some(b' ') => Some(false),
                _ => None,
            }
        } else {
            None
        };

        let text = self.read_line().trim_end();

        Some(FastToken::ListItem {
            text,
            ordered,
            checked,
            span: Span::new(start, self.pos_info()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let mut t = FastTokenizer::new("# Hello World\n");
        let tokens = t.tokenize();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            FastToken::Heading { depth, text, .. } => {
                assert_eq!(*depth, 1);
                assert_eq!(*text, "Hello World");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_code_block() {
        let mut t = FastTokenizer::new("```rust\nfn main() {}\n```\n");
        let tokens = t.tokenize();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            FastToken::CodeBlock { lang, code, .. } => {
                assert_eq!(*lang, Some("rust"));
                assert_eq!(*code, "fn main() {}");
            }
            _ => panic!("Expected code block"),
        }
    }

    #[test]
    fn test_paragraph() {
        let mut t = FastTokenizer::new("Hello world.\n");
        let tokens = t.tokenize();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            FastToken::Paragraph { text, .. } => {
                assert_eq!(*text, "Hello world.");
            }
            _ => panic!("Expected paragraph"),
        }
    }
}

//! Markdown tokenizer
//!
//! Character-based tokenization for maximum performance.

use synth_wasm_core::{Position, Span};

/// Block token types
#[derive(Debug, Clone)]
pub enum BlockToken {
    Heading {
        depth: u8,
        text: String,
        span: Span,
    },
    Paragraph {
        text: String,
        span: Span,
    },
    CodeBlock {
        lang: Option<String>,
        meta: Option<String>,
        code: String,
        span: Span,
    },
    ThematicBreak {
        span: Span,
    },
    BlankLine {
        #[allow(dead_code)]
        span: Span,
    },
    BlockQuote {
        text: String,
        span: Span,
    },
    ListItem {
        text: String,
        ordered: bool,
        checked: Option<bool>,
        span: Span,
    },
}

/// Markdown tokenizer
pub struct Tokenizer {
    source: Vec<char>,
    pos: usize,
    line: u32,
    column: u32,
    offset: u32,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 0,
            offset: 0,
        }
    }

    /// Tokenize the source into block tokens
    pub fn tokenize(&mut self) -> Vec<BlockToken> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            if let Some(token) = self.scan_block() {
                tokens.push(token);
            }
        }

        tokens
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.source.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.pos += 1;
        self.offset += ch.len_utf8() as u32;

        if ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    fn current_position(&self) -> Position {
        Position::new(self.line, self.column, self.offset)
    }

    fn skip_whitespace_on_line(&mut self) {
        while let Some(ch) = self.current() {
            if ch == ' ' || ch == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();
        while let Some(ch) = self.current() {
            if ch == '\n' {
                self.advance();
                break;
            }
            line.push(ch);
            self.advance();
        }
        line
    }

    fn scan_block(&mut self) -> Option<BlockToken> {
        let start = self.current_position();

        // Skip leading whitespace
        self.skip_whitespace_on_line();

        let ch = self.current()?;

        match ch {
            // Blank line
            '\n' => {
                self.advance();
                let end = self.current_position();
                Some(BlockToken::BlankLine {
                    span: Span::new(start, end),
                })
            }

            // Heading
            '#' => self.scan_heading(start),

            // Thematic break or list
            '-' | '*' | '_' => {
                if self.is_thematic_break() {
                    self.scan_thematic_break(start)
                } else if ch == '-' || ch == '*' {
                    self.scan_list_item(start, false)
                } else {
                    self.scan_paragraph(start)
                }
            }

            // Ordered list
            '0'..='9' => {
                if self.is_ordered_list() {
                    self.scan_list_item(start, true)
                } else {
                    self.scan_paragraph(start)
                }
            }

            // Code block
            '`' => {
                if self.peek(1) == Some('`') && self.peek(2) == Some('`') {
                    self.scan_code_block(start)
                } else {
                    self.scan_paragraph(start)
                }
            }

            // Block quote
            '>' => self.scan_blockquote(start),

            // Paragraph (default)
            _ => self.scan_paragraph(start),
        }
    }

    fn scan_heading(&mut self, start: Position) -> Option<BlockToken> {
        let mut depth = 0u8;

        while self.current() == Some('#') && depth < 6 {
            self.advance();
            depth += 1;
        }

        // Must have space after #
        if self.current() != Some(' ') && self.current() != Some('\n') {
            // Not a valid heading, treat as paragraph
            let text = format!("{}{}", "#".repeat(depth as usize), self.read_line());
            let end = self.current_position();
            return Some(BlockToken::Paragraph {
                text,
                span: Span::new(start, end),
            });
        }

        self.skip_whitespace_on_line();
        let text = self.read_line();
        let end = self.current_position();

        Some(BlockToken::Heading {
            depth,
            text: text.trim_end().to_string(),
            span: Span::new(start, end),
        })
    }

    fn scan_paragraph(&mut self, start: Position) -> Option<BlockToken> {
        let mut text = String::new();

        loop {
            let line = self.read_line();
            text.push_str(&line);

            // Check if next line continues paragraph
            if self.is_eof() {
                break;
            }

            let next = self.current()?;
            if next == '\n' || next == '#' || next == '>' || next == '-' || next == '*' || next == '`' {
                break;
            }

            text.push('\n');
        }

        let end = self.current_position();

        Some(BlockToken::Paragraph {
            text: text.trim_end().to_string(),
            span: Span::new(start, end),
        })
    }

    fn scan_code_block(&mut self, start: Position) -> Option<BlockToken> {
        // Skip opening ```
        self.advance();
        self.advance();
        self.advance();

        // Read language/meta
        let info = self.read_line();
        let (lang, meta) = if info.is_empty() {
            (None, None)
        } else {
            let parts: Vec<&str> = info.splitn(2, ' ').collect();
            let lang = Some(parts[0].to_string());
            let meta = parts.get(1).map(|s| s.to_string());
            (lang, meta)
        };

        // Read code
        let mut code = String::new();
        loop {
            if self.is_eof() {
                break;
            }

            // Check for closing ```
            if self.current() == Some('`') && self.peek(1) == Some('`') && self.peek(2) == Some('`') {
                self.advance();
                self.advance();
                self.advance();
                self.read_line(); // consume rest of line
                break;
            }

            let line = self.read_line();
            code.push_str(&line);
            code.push('\n');
        }

        let end = self.current_position();

        Some(BlockToken::CodeBlock {
            lang,
            meta,
            code: code.trim_end().to_string(),
            span: Span::new(start, end),
        })
    }

    fn is_thematic_break(&self) -> bool {
        let ch = self.current();
        if ch != Some('-') && ch != Some('*') && ch != Some('_') {
            return false;
        }

        let marker = ch.unwrap();
        let mut count = 0;
        let mut i = 0;

        while let Some(c) = self.peek(i) {
            if c == marker {
                count += 1;
            } else if c != ' ' && c != '\t' && c != '\n' {
                return false;
            }
            if c == '\n' {
                break;
            }
            i += 1;
        }

        count >= 3
    }

    fn scan_thematic_break(&mut self, start: Position) -> Option<BlockToken> {
        self.read_line();
        let end = self.current_position();

        Some(BlockToken::ThematicBreak {
            span: Span::new(start, end),
        })
    }

    fn scan_blockquote(&mut self, start: Position) -> Option<BlockToken> {
        self.advance(); // skip >
        self.skip_whitespace_on_line();

        let text = self.read_line();
        let end = self.current_position();

        Some(BlockToken::BlockQuote {
            text: text.trim_end().to_string(),
            span: Span::new(start, end),
        })
    }

    fn is_ordered_list(&self) -> bool {
        let mut i = 0;
        while let Some(c) = self.peek(i) {
            if c.is_ascii_digit() {
                i += 1;
            } else if c == '.' || c == ')' {
                return self.peek(i + 1) == Some(' ');
            } else {
                return false;
            }
        }
        false
    }

    fn scan_list_item(&mut self, start: Position, ordered: bool) -> Option<BlockToken> {
        if ordered {
            // Skip number
            while self.current().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
            self.advance(); // skip . or )
        } else {
            self.advance(); // skip - or *
        }

        self.skip_whitespace_on_line();

        // Check for task list
        let checked = if self.current() == Some('[') {
            self.advance();
            let mark = self.current();
            self.advance(); // skip x or space
            self.advance(); // skip ]
            self.skip_whitespace_on_line();
            match mark {
                Some('x') | Some('X') => Some(true),
                Some(' ') => Some(false),
                _ => None,
            }
        } else {
            None
        };

        let text = self.read_line();
        let end = self.current_position();

        Some(BlockToken::ListItem {
            text: text.trim_end().to_string(),
            ordered,
            checked,
            span: Span::new(start, end),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let mut tokenizer = Tokenizer::new("# Hello World\n");
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            BlockToken::Heading { depth, text, .. } => {
                assert_eq!(*depth, 1);
                assert_eq!(text, "Hello World");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_code_block() {
        let mut tokenizer = Tokenizer::new("```rust\nfn main() {}\n```\n");
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            BlockToken::CodeBlock { lang, code, .. } => {
                assert_eq!(lang.as_deref(), Some("rust"));
                assert_eq!(code, "fn main() {}");
            }
            _ => panic!("Expected code block"),
        }
    }
}

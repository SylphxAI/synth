//! JavaScript Lexer
//!
//! High-performance tokenizer for JavaScript/TypeScript.
//! Supports ES2024 syntax.

use memchr::memchr;

/// Token types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenKind {
    // Literals
    Identifier = 1,
    Number,
    String,
    Template,
    #[allow(dead_code)]
    Regex,
    BigInt,

    // Keywords
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    Let,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,
    // TypeScript keywords
    As,
    Async,
    From,
    Get,
    Of,
    Set,
    Static,
    Type,
    Interface,
    Implements,
    Private,
    Protected,
    Public,
    Readonly,
    Declare,
    Abstract,
    Namespace,
    Module,

    // Punctuation
    LBrace,      // {
    RBrace,      // }
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    Dot,         // .
    DotDotDot,   // ...
    Semicolon,   // ;
    Comma,       // ,
    Colon,       // :
    Question,    // ?
    QuestionDot, // ?.
    QuestionQuestion, // ??
    Arrow,       // =>
    At,          // @

    // Operators
    Plus,        // +
    Minus,       // -
    Star,        // *
    StarStar,    // **
    Slash,       // /
    Percent,     // %
    PlusPlus,    // ++
    MinusMinus,  // --
    LtLt,        // <<
    GtGt,        // >>
    GtGtGt,      // >>>
    Amp,         // &
    Pipe,        // |
    Caret,       // ^
    Tilde,       // ~
    Bang,        // !
    Lt,          // <
    Gt,          // >
    LtEq,        // <=
    GtEq,        // >=
    EqEq,        // ==
    BangEq,      // !=
    EqEqEq,      // ===
    BangEqEq,    // !==
    AmpAmp,      // &&
    PipePipe,    // ||

    // Assignment
    Eq,          // =
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    PercentEq,   // %=
    StarStarEq,  // **=
    LtLtEq,      // <<=
    GtGtEq,      // >>=
    GtGtGtEq,    // >>>=
    AmpEq,       // &=
    PipeEq,      // |=
    CaretEq,     // ^=
    AmpAmpEq,    // &&=
    PipePipeEq,  // ||=
    QuestionQuestionEq, // ??=

    // Comments
    LineComment,
    BlockComment,

    // Special
    Newline,
    Eof,
    Invalid,
}

/// A token with position info
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub start: u32,
    pub end: u32,
}

/// JavaScript Lexer
pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            src: source.as_bytes(),
            pos: 0,
        }
    }

    #[inline]
    fn byte(&self, pos: usize) -> Option<u8> {
        self.src.get(pos).copied()
    }

    #[inline]
    fn current(&self) -> Option<u8> {
        self.byte(self.pos)
    }

    #[inline]
    fn peek(&self) -> Option<u8> {
        self.byte(self.pos + 1)
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(b) = self.current() {
            match b {
                b' ' | b'\t' | b'\r' => self.pos += 1,
                _ => break,
            }
        }
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.pos as u32;

        let Some(b) = self.current() else {
            return Token { kind: TokenKind::Eof, start, end: start };
        };

        let kind = match b {
            b'\n' => {
                self.pos += 1;
                TokenKind::Newline
            }

            // Identifiers and keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'$' => self.scan_identifier(),

            // Numbers
            b'0'..=b'9' => self.scan_number(),

            // Strings
            b'"' | b'\'' => self.scan_string(b),

            // Template literals
            b'`' => self.scan_template(),

            // Punctuation and operators
            b'{' => { self.pos += 1; TokenKind::LBrace }
            b'}' => { self.pos += 1; TokenKind::RBrace }
            b'(' => { self.pos += 1; TokenKind::LParen }
            b')' => { self.pos += 1; TokenKind::RParen }
            b'[' => { self.pos += 1; TokenKind::LBracket }
            b']' => { self.pos += 1; TokenKind::RBracket }
            b';' => { self.pos += 1; TokenKind::Semicolon }
            b',' => { self.pos += 1; TokenKind::Comma }
            b':' => { self.pos += 1; TokenKind::Colon }
            b'~' => { self.pos += 1; TokenKind::Tilde }
            b'@' => { self.pos += 1; TokenKind::At }

            b'.' => self.scan_dot(),
            b'?' => self.scan_question(),
            b'+' => self.scan_plus(),
            b'-' => self.scan_minus(),
            b'*' => self.scan_star(),
            b'/' => self.scan_slash(),
            b'%' => self.scan_percent(),
            b'<' => self.scan_lt(),
            b'>' => self.scan_gt(),
            b'=' => self.scan_eq(),
            b'!' => self.scan_bang(),
            b'&' => self.scan_amp(),
            b'|' => self.scan_pipe(),
            b'^' => self.scan_caret(),

            _ => {
                self.pos += 1;
                TokenKind::Invalid
            }
        };

        Token {
            kind,
            start,
            end: self.pos as u32,
        }
    }

    fn scan_identifier(&mut self) -> TokenKind {
        let start = self.pos;
        while let Some(b) = self.current() {
            match b {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'$' => self.pos += 1,
                _ => break,
            }
        }

        // Check for keywords
        let ident = &self.src[start..self.pos];
        match ident {
            b"await" => TokenKind::Await,
            b"break" => TokenKind::Break,
            b"case" => TokenKind::Case,
            b"catch" => TokenKind::Catch,
            b"class" => TokenKind::Class,
            b"const" => TokenKind::Const,
            b"continue" => TokenKind::Continue,
            b"debugger" => TokenKind::Debugger,
            b"default" => TokenKind::Default,
            b"delete" => TokenKind::Delete,
            b"do" => TokenKind::Do,
            b"else" => TokenKind::Else,
            b"enum" => TokenKind::Enum,
            b"export" => TokenKind::Export,
            b"extends" => TokenKind::Extends,
            b"false" => TokenKind::False,
            b"finally" => TokenKind::Finally,
            b"for" => TokenKind::For,
            b"function" => TokenKind::Function,
            b"if" => TokenKind::If,
            b"import" => TokenKind::Import,
            b"in" => TokenKind::In,
            b"instanceof" => TokenKind::Instanceof,
            b"let" => TokenKind::Let,
            b"new" => TokenKind::New,
            b"null" => TokenKind::Null,
            b"return" => TokenKind::Return,
            b"super" => TokenKind::Super,
            b"switch" => TokenKind::Switch,
            b"this" => TokenKind::This,
            b"throw" => TokenKind::Throw,
            b"true" => TokenKind::True,
            b"try" => TokenKind::Try,
            b"typeof" => TokenKind::Typeof,
            b"var" => TokenKind::Var,
            b"void" => TokenKind::Void,
            b"while" => TokenKind::While,
            b"with" => TokenKind::With,
            b"yield" => TokenKind::Yield,
            // Contextual / TypeScript
            b"as" => TokenKind::As,
            b"async" => TokenKind::Async,
            b"from" => TokenKind::From,
            b"get" => TokenKind::Get,
            b"of" => TokenKind::Of,
            b"set" => TokenKind::Set,
            b"static" => TokenKind::Static,
            b"type" => TokenKind::Type,
            b"interface" => TokenKind::Interface,
            b"implements" => TokenKind::Implements,
            b"private" => TokenKind::Private,
            b"protected" => TokenKind::Protected,
            b"public" => TokenKind::Public,
            b"readonly" => TokenKind::Readonly,
            b"declare" => TokenKind::Declare,
            b"abstract" => TokenKind::Abstract,
            b"namespace" => TokenKind::Namespace,
            b"module" => TokenKind::Module,
            _ => TokenKind::Identifier,
        }
    }

    fn scan_number(&mut self) -> TokenKind {
        // Handle 0x, 0o, 0b prefixes
        if self.current() == Some(b'0') {
            match self.peek() {
                Some(b'x') | Some(b'X') => {
                    self.pos += 2;
                    while let Some(b) = self.current() {
                        match b {
                            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'_' => self.pos += 1,
                            _ => break,
                        }
                    }
                    return self.check_bigint();
                }
                Some(b'o') | Some(b'O') => {
                    self.pos += 2;
                    while let Some(b) = self.current() {
                        match b {
                            b'0'..=b'7' | b'_' => self.pos += 1,
                            _ => break,
                        }
                    }
                    return self.check_bigint();
                }
                Some(b'b') | Some(b'B') => {
                    self.pos += 2;
                    while let Some(b) = self.current() {
                        match b {
                            b'0' | b'1' | b'_' => self.pos += 1,
                            _ => break,
                        }
                    }
                    return self.check_bigint();
                }
                _ => {}
            }
        }

        // Decimal
        while let Some(b) = self.current() {
            match b {
                b'0'..=b'9' | b'_' => self.pos += 1,
                _ => break,
            }
        }

        // Decimal part
        if self.current() == Some(b'.') && self.peek().map(|b| b.is_ascii_digit()).unwrap_or(false) {
            self.pos += 1;
            while let Some(b) = self.current() {
                match b {
                    b'0'..=b'9' | b'_' => self.pos += 1,
                    _ => break,
                }
            }
        }

        // Exponent
        if matches!(self.current(), Some(b'e') | Some(b'E')) {
            self.pos += 1;
            if matches!(self.current(), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            while let Some(b) = self.current() {
                match b {
                    b'0'..=b'9' | b'_' => self.pos += 1,
                    _ => break,
                }
            }
        }

        self.check_bigint()
    }

    #[inline]
    fn check_bigint(&mut self) -> TokenKind {
        if self.current() == Some(b'n') {
            self.pos += 1;
            TokenKind::BigInt
        } else {
            TokenKind::Number
        }
    }

    fn scan_string(&mut self, quote: u8) -> TokenKind {
        self.pos += 1; // skip opening quote

        while let Some(b) = self.current() {
            match b {
                b'\\' => {
                    self.pos += 2; // skip escape
                }
                b'\n' => break, // unterminated
                _ if b == quote => {
                    self.pos += 1;
                    return TokenKind::String;
                }
                _ => self.pos += 1,
            }
        }

        TokenKind::String // unterminated but still return String
    }

    fn scan_template(&mut self) -> TokenKind {
        self.pos += 1; // skip `

        while let Some(b) = self.current() {
            match b {
                b'\\' => self.pos += 2,
                b'`' => {
                    self.pos += 1;
                    return TokenKind::Template;
                }
                b'$' if self.peek() == Some(b'{') => {
                    // Template expression - for now just scan to end
                    // Full implementation would need nesting tracking
                    self.pos += 2;
                }
                _ => self.pos += 1,
            }
        }

        TokenKind::Template
    }

    fn scan_dot(&mut self) -> TokenKind {
        self.pos += 1;
        if self.current() == Some(b'.') && self.peek() == Some(b'.') {
            self.pos += 2;
            TokenKind::DotDotDot
        } else {
            TokenKind::Dot
        }
    }

    fn scan_question(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'.') => {
                // ?. but not ?.digit (which would be ? followed by number)
                if !self.peek().map(|b| b.is_ascii_digit()).unwrap_or(false) {
                    self.pos += 1;
                    TokenKind::QuestionDot
                } else {
                    TokenKind::Question
                }
            }
            Some(b'?') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::QuestionQuestionEq
                } else {
                    TokenKind::QuestionQuestion
                }
            }
            _ => TokenKind::Question,
        }
    }

    fn scan_plus(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'+') => { self.pos += 1; TokenKind::PlusPlus }
            Some(b'=') => { self.pos += 1; TokenKind::PlusEq }
            _ => TokenKind::Plus,
        }
    }

    fn scan_minus(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'-') => { self.pos += 1; TokenKind::MinusMinus }
            Some(b'=') => { self.pos += 1; TokenKind::MinusEq }
            _ => TokenKind::Minus,
        }
    }

    fn scan_star(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'*') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::StarStarEq
                } else {
                    TokenKind::StarStar
                }
            }
            Some(b'=') => { self.pos += 1; TokenKind::StarEq }
            _ => TokenKind::Star,
        }
    }

    fn scan_slash(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'/') => {
                // Line comment
                self.pos += 1;
                if let Some(i) = memchr(b'\n', &self.src[self.pos..]) {
                    self.pos += i;
                } else {
                    self.pos = self.src.len();
                }
                TokenKind::LineComment
            }
            Some(b'*') => {
                // Block comment
                self.pos += 1;
                while self.pos + 1 < self.src.len() {
                    if self.current() == Some(b'*') && self.peek() == Some(b'/') {
                        self.pos += 2;
                        return TokenKind::BlockComment;
                    }
                    self.pos += 1;
                }
                self.pos = self.src.len();
                TokenKind::BlockComment
            }
            Some(b'=') => { self.pos += 1; TokenKind::SlashEq }
            _ => TokenKind::Slash,
        }
    }

    fn scan_percent(&mut self) -> TokenKind {
        self.pos += 1;
        if self.current() == Some(b'=') {
            self.pos += 1;
            TokenKind::PercentEq
        } else {
            TokenKind::Percent
        }
    }

    fn scan_lt(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'<') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::LtLtEq
                } else {
                    TokenKind::LtLt
                }
            }
            Some(b'=') => { self.pos += 1; TokenKind::LtEq }
            _ => TokenKind::Lt,
        }
    }

    fn scan_gt(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'>') => {
                self.pos += 1;
                match self.current() {
                    Some(b'>') => {
                        self.pos += 1;
                        if self.current() == Some(b'=') {
                            self.pos += 1;
                            TokenKind::GtGtGtEq
                        } else {
                            TokenKind::GtGtGt
                        }
                    }
                    Some(b'=') => { self.pos += 1; TokenKind::GtGtEq }
                    _ => TokenKind::GtGt,
                }
            }
            Some(b'=') => { self.pos += 1; TokenKind::GtEq }
            _ => TokenKind::Gt,
        }
    }

    fn scan_eq(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'=') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::EqEqEq
                } else {
                    TokenKind::EqEq
                }
            }
            Some(b'>') => { self.pos += 1; TokenKind::Arrow }
            _ => TokenKind::Eq,
        }
    }

    fn scan_bang(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'=') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::BangEqEq
                } else {
                    TokenKind::BangEq
                }
            }
            _ => TokenKind::Bang,
        }
    }

    fn scan_amp(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'&') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::AmpAmpEq
                } else {
                    TokenKind::AmpAmp
                }
            }
            Some(b'=') => { self.pos += 1; TokenKind::AmpEq }
            _ => TokenKind::Amp,
        }
    }

    fn scan_pipe(&mut self) -> TokenKind {
        self.pos += 1;
        match self.current() {
            Some(b'|') => {
                self.pos += 1;
                if self.current() == Some(b'=') {
                    self.pos += 1;
                    TokenKind::PipePipeEq
                } else {
                    TokenKind::PipePipe
                }
            }
            Some(b'=') => { self.pos += 1; TokenKind::PipeEq }
            _ => TokenKind::Pipe,
        }
    }

    fn scan_caret(&mut self) -> TokenKind {
        self.pos += 1;
        if self.current() == Some(b'=') {
            self.pos += 1;
            TokenKind::CaretEq
        } else {
            TokenKind::Caret
        }
    }

    /// Tokenize the entire source
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.src.len() / 4);
        loop {
            let token = self.next_token();
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("const let var function class");
        assert_eq!(lexer.next_token().kind, TokenKind::Const);
        assert_eq!(lexer.next_token().kind, TokenKind::Let);
        assert_eq!(lexer.next_token().kind, TokenKind::Var);
        assert_eq!(lexer.next_token().kind, TokenKind::Function);
        assert_eq!(lexer.next_token().kind, TokenKind::Class);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("=== !== ?? ?. => ...");
        assert_eq!(lexer.next_token().kind, TokenKind::EqEqEq);
        assert_eq!(lexer.next_token().kind, TokenKind::BangEqEq);
        assert_eq!(lexer.next_token().kind, TokenKind::QuestionQuestion);
        assert_eq!(lexer.next_token().kind, TokenKind::QuestionDot);
        assert_eq!(lexer.next_token().kind, TokenKind::Arrow);
        assert_eq!(lexer.next_token().kind, TokenKind::DotDotDot);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("123 0xff 0o77 0b11 123n 1.5e10");
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
        assert_eq!(lexer.next_token().kind, TokenKind::BigInt);
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" 'world' `template`"#);
        assert_eq!(lexer.next_token().kind, TokenKind::String);
        assert_eq!(lexer.next_token().kind, TokenKind::String);
        assert_eq!(lexer.next_token().kind, TokenKind::Template);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("// line\n/* block */");
        assert_eq!(lexer.next_token().kind, TokenKind::LineComment);
        assert_eq!(lexer.next_token().kind, TokenKind::Newline);
        assert_eq!(lexer.next_token().kind, TokenKind::BlockComment);
    }
}

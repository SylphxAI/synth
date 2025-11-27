//! JavaScript Parser
//!
//! Recursive descent parser for JavaScript/TypeScript.
//! Produces a compact AST suitable for WASM output.

use crate::lexer::{Lexer, Token, TokenKind};

/// AST Node types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeKind {
    // Program
    Program = 1,

    // Declarations
    VariableDeclaration,
    VariableDeclarator,
    FunctionDeclaration,
    ClassDeclaration,
    ImportDeclaration,
    ExportDeclaration,

    // Statements
    BlockStatement,
    ExpressionStatement,
    IfStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    WhileStatement,
    DoWhileStatement,
    SwitchStatement,
    SwitchCase,
    ReturnStatement,
    ThrowStatement,
    TryStatement,
    CatchClause,
    BreakStatement,
    ContinueStatement,
    EmptyStatement,

    // Expressions
    Identifier,
    Literal,
    ArrayExpression,
    ObjectExpression,
    Property,
    FunctionExpression,
    ArrowFunctionExpression,
    ClassExpression,
    CallExpression,
    NewExpression,
    MemberExpression,
    BinaryExpression,
    UnaryExpression,
    UpdateExpression,
    AssignmentExpression,
    LogicalExpression,
    ConditionalExpression,
    #[allow(dead_code)]
    SequenceExpression,
    SpreadElement,
    TemplateLiteral,
    TaggedTemplateExpression,
    ThisExpression,
    Super,
    AwaitExpression,
    YieldExpression,

    // Patterns
    ArrayPattern,
    ObjectPattern,
    #[allow(dead_code)]
    AssignmentPattern,
    RestElement,

    // Import/Export specifiers
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
    ExportSpecifier,

    // Class members
    MethodDefinition,
    PropertyDefinition,

    // Comments
    #[allow(dead_code)]
    Comment,
}

/// Compact AST node - 16 bytes
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Node {
    pub kind: NodeKind,
    pub flags: u8,       // Various flags (e.g., async, generator, computed)
    pub _pad: [u8; 2],
    pub start: u32,
    pub end: u32,
    pub extra: u32,      // Extra data (e.g., operator type, child count)
}

impl Node {
    pub fn new(kind: NodeKind, start: u32, end: u32) -> Self {
        Self {
            kind,
            flags: 0,
            _pad: [0; 2],
            start,
            end,
            extra: 0,
        }
    }

    pub fn with_flags(mut self, flags: u8) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_extra(mut self, extra: u32) -> Self {
        self.extra = extra;
        self
    }
}

/// Flags for nodes
pub mod flags {
    pub const CONST: u8 = 1 << 0;
    pub const LET: u8 = 1 << 1;
    pub const ASYNC: u8 = 1 << 2;
    pub const GENERATOR: u8 = 1 << 3;
    pub const COMPUTED: u8 = 1 << 4;
    pub const SHORTHAND: u8 = 1 << 5;
    pub const STATIC: u8 = 1 << 6;
    pub const EXPORT_DEFAULT: u8 = 1 << 7;
}

/// JavaScript Parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    peek: Token,
    nodes: Vec<Node>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self {
            lexer,
            current,
            peek,
            nodes: Vec::with_capacity(source.len() / 8),
        }
    }

    /// Parse and return node count
    pub fn parse_count(&mut self) -> usize {
        self.parse_program();
        self.nodes.len()
    }

    /// Parse and return binary output
    pub fn parse_binary(&mut self) -> Vec<u8> {
        self.parse_program();

        let node_count = self.nodes.len();
        let node_size = std::mem::size_of::<Node>();
        let mut buf = vec![0u8; 4 + node_count * node_size];

        // Header
        buf[0..4].copy_from_slice(&(node_count as u32).to_le_bytes());

        // Nodes
        for (i, node) in self.nodes.iter().enumerate() {
            let offset = 4 + i * node_size;
            buf[offset] = node.kind as u8;
            buf[offset + 1] = node.flags;
            // skip padding [2..4]
            buf[offset + 4..offset + 8].copy_from_slice(&node.start.to_le_bytes());
            buf[offset + 8..offset + 12].copy_from_slice(&node.end.to_le_bytes());
            buf[offset + 12..offset + 16].copy_from_slice(&node.extra.to_le_bytes());
        }

        buf
    }

    // ========================================
    // Token helpers
    // ========================================

    fn advance(&mut self) {
        self.current = self.peek;
        self.peek = self.lexer.next_token();
    }

    #[allow(dead_code)]
    fn skip_newlines(&mut self) {
        while self.current.kind == TokenKind::Newline {
            self.advance();
        }
    }

    fn skip_comments_and_newlines(&mut self) {
        loop {
            match self.current.kind {
                TokenKind::Newline | TokenKind::LineComment | TokenKind::BlockComment => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) {
        if !self.eat(kind) {
            // In production, would return error
            // For now, just skip
        }
    }

    // ========================================
    // Parsing
    // ========================================

    fn parse_program(&mut self) {
        let start = self.current.start;

        self.skip_comments_and_newlines();

        while !self.at(TokenKind::Eof) {
            self.parse_statement_or_declaration();
            self.skip_comments_and_newlines();
        }

        let end = self.current.end;
        self.nodes.insert(0, Node::new(NodeKind::Program, start, end)
            .with_extra(self.nodes.len() as u32));
    }

    fn parse_statement_or_declaration(&mut self) {
        match self.current.kind {
            // Declarations
            TokenKind::Const | TokenKind::Let | TokenKind::Var => self.parse_variable_declaration(),
            TokenKind::Function => self.parse_function_declaration(),
            TokenKind::Class => self.parse_class_declaration(),
            TokenKind::Import => self.parse_import_declaration(),
            TokenKind::Export => self.parse_export_declaration(),
            TokenKind::Async if self.peek.kind == TokenKind::Function => {
                self.parse_function_declaration()
            }

            // Statements
            TokenKind::LBrace => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::Do => self.parse_do_while_statement(),
            TokenKind::Switch => self.parse_switch_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::Throw => self.parse_throw_statement(),
            TokenKind::Try => self.parse_try_statement(),
            TokenKind::Break => self.parse_break_statement(),
            TokenKind::Continue => self.parse_continue_statement(),
            TokenKind::Semicolon => self.parse_empty_statement(),

            // Expression statement
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration(&mut self) {
        let start = self.current.start;
        let flags = match self.current.kind {
            TokenKind::Const => flags::CONST,
            TokenKind::Let => flags::LET,
            _ => 0,
        };
        self.advance(); // skip const/let/var

        self.skip_comments_and_newlines();

        let mut count = 0;
        loop {
            self.parse_variable_declarator();
            count += 1;

            self.skip_comments_and_newlines();
            if !self.eat(TokenKind::Comma) {
                break;
            }
            self.skip_comments_and_newlines();
        }

        self.eat(TokenKind::Semicolon);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::VariableDeclaration, start, end)
            .with_flags(flags)
            .with_extra(count));
    }

    fn parse_variable_declarator(&mut self) {
        let start = self.current.start;

        // Parse binding pattern or identifier
        self.parse_binding_pattern();

        self.skip_comments_and_newlines();

        // Optional initializer
        if self.eat(TokenKind::Eq) {
            self.skip_comments_and_newlines();
            self.parse_expression();
        }

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::VariableDeclarator, start, end));
    }

    fn parse_binding_pattern(&mut self) {
        match self.current.kind {
            TokenKind::LBracket => self.parse_array_pattern(),
            TokenKind::LBrace => self.parse_object_pattern(),
            _ => self.parse_identifier(),
        }
    }

    fn parse_array_pattern(&mut self) {
        let start = self.current.start;
        self.advance(); // skip [

        self.skip_comments_and_newlines();

        let mut count = 0;
        while !self.at(TokenKind::RBracket) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Comma) {
                // Hole in array pattern
                self.advance();
            } else if self.at(TokenKind::DotDotDot) {
                self.parse_rest_element();
                count += 1;
                break;
            } else {
                self.parse_binding_pattern();
                count += 1;
                self.skip_comments_and_newlines();
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RBracket);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::ArrayPattern, start, end)
            .with_extra(count));
    }

    fn parse_object_pattern(&mut self) {
        let start = self.current.start;
        self.advance(); // skip {

        self.skip_comments_and_newlines();

        let mut count = 0;
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::DotDotDot) {
                self.parse_rest_element();
                count += 1;
                break;
            } else {
                self.parse_property_pattern();
                count += 1;
                self.skip_comments_and_newlines();
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RBrace);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::ObjectPattern, start, end)
            .with_extra(count));
    }

    fn parse_property_pattern(&mut self) {
        let start = self.current.start;

        // Property key
        self.parse_identifier();

        self.skip_comments_and_newlines();

        // Check for : value or = default
        if self.eat(TokenKind::Colon) {
            self.skip_comments_and_newlines();
            self.parse_binding_pattern();
        }

        if self.eat(TokenKind::Eq) {
            self.skip_comments_and_newlines();
            self.parse_expression();
        }

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::Property, start, end));
    }

    fn parse_rest_element(&mut self) {
        let start = self.current.start;
        self.advance(); // skip ...
        self.parse_binding_pattern();
        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::RestElement, start, end));
    }

    fn parse_function_declaration(&mut self) {
        let start = self.current.start;
        let mut flags = 0;

        if self.eat(TokenKind::Async) {
            flags |= flags::ASYNC;
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::Function);
        self.skip_comments_and_newlines();

        if self.eat(TokenKind::Star) {
            flags |= flags::GENERATOR;
            self.skip_comments_and_newlines();
        }

        // Function name (optional for expressions)
        if self.at(TokenKind::Identifier) {
            self.parse_identifier();
        }

        self.skip_comments_and_newlines();
        self.parse_function_params();
        self.skip_comments_and_newlines();
        self.parse_block_statement();

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::FunctionDeclaration, start, end)
            .with_flags(flags));
    }

    fn parse_function_params(&mut self) {
        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();

        while !self.at(TokenKind::RParen) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::DotDotDot) {
                self.parse_rest_element();
                break;
            }
            self.parse_binding_pattern();

            // Default value
            self.skip_comments_and_newlines();
            if self.eat(TokenKind::Eq) {
                self.skip_comments_and_newlines();
                self.parse_expression();
            }

            self.skip_comments_and_newlines();
            if !self.eat(TokenKind::Comma) {
                break;
            }
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RParen);
    }

    fn parse_class_declaration(&mut self) {
        let start = self.current.start;
        self.advance(); // skip class

        self.skip_comments_and_newlines();

        // Class name
        if self.at(TokenKind::Identifier) {
            self.parse_identifier();
        }

        self.skip_comments_and_newlines();

        // Extends
        if self.eat(TokenKind::Extends) {
            self.skip_comments_and_newlines();
            self.parse_expression();
        }

        self.skip_comments_and_newlines();
        self.parse_class_body();

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::ClassDeclaration, start, end));
    }

    fn parse_class_body(&mut self) {
        self.expect(TokenKind::LBrace);
        self.skip_comments_and_newlines();

        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            self.parse_class_member();
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RBrace);
    }

    fn parse_class_member(&mut self) {
        let start = self.current.start;
        let mut flags = 0;

        // Static
        if self.eat(TokenKind::Static) {
            flags |= flags::STATIC;
            self.skip_comments_and_newlines();
        }

        // Async
        if self.eat(TokenKind::Async) {
            flags |= flags::ASYNC;
            self.skip_comments_and_newlines();
        }

        // Generator
        if self.eat(TokenKind::Star) {
            flags |= flags::GENERATOR;
            self.skip_comments_and_newlines();
        }

        // Get/Set
        let is_getter = self.at(TokenKind::Get);
        let is_setter = self.at(TokenKind::Set);
        if is_getter || is_setter {
            self.advance();
            self.skip_comments_and_newlines();
        }

        // Property name
        if self.eat(TokenKind::LBracket) {
            flags |= flags::COMPUTED;
            self.parse_expression();
            self.expect(TokenKind::RBracket);
        } else {
            self.parse_identifier();
        }

        self.skip_comments_and_newlines();

        // Method or property
        if self.at(TokenKind::LParen) {
            self.parse_function_params();
            self.skip_comments_and_newlines();
            self.parse_block_statement();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::MethodDefinition, start, end)
                .with_flags(flags));
        } else {
            // Property
            if self.eat(TokenKind::Eq) {
                self.skip_comments_and_newlines();
                self.parse_expression();
            }
            self.eat(TokenKind::Semicolon);
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::PropertyDefinition, start, end)
                .with_flags(flags));
        }
    }

    fn parse_import_declaration(&mut self) {
        let start = self.current.start;
        self.advance(); // skip import

        self.skip_comments_and_newlines();

        // import "module" (side-effect only)
        if self.at(TokenKind::String) {
            self.advance();
            self.eat(TokenKind::Semicolon);
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::ImportDeclaration, start, end));
            return;
        }

        // import defaultExport from "module"
        // import * as name from "module"
        // import { named } from "module"
        // import defaultExport, { named } from "module"

        if self.at(TokenKind::Identifier) {
            // Default import
            let spec_start = self.current.start;
            self.parse_identifier();
            let spec_end = self.current.start;
            self.nodes.push(Node::new(NodeKind::ImportDefaultSpecifier, spec_start, spec_end));

            self.skip_comments_and_newlines();
            if self.eat(TokenKind::Comma) {
                self.skip_comments_and_newlines();
            }
        }

        if self.eat(TokenKind::Star) {
            // Namespace import
            let spec_start = self.current.start;
            self.skip_comments_and_newlines();
            self.expect(TokenKind::As);
            self.skip_comments_and_newlines();
            self.parse_identifier();
            let spec_end = self.current.start;
            self.nodes.push(Node::new(NodeKind::ImportNamespaceSpecifier, spec_start, spec_end));
        } else if self.at(TokenKind::LBrace) {
            // Named imports
            self.advance();
            self.skip_comments_and_newlines();

            while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
                let spec_start = self.current.start;
                self.parse_identifier();
                self.skip_comments_and_newlines();

                if self.eat(TokenKind::As) {
                    self.skip_comments_and_newlines();
                    self.parse_identifier();
                }

                let spec_end = self.current.start;
                self.nodes.push(Node::new(NodeKind::ImportSpecifier, spec_start, spec_end));

                self.skip_comments_and_newlines();
                if !self.eat(TokenKind::Comma) {
                    break;
                }
                self.skip_comments_and_newlines();
            }

            self.expect(TokenKind::RBrace);
        }

        self.skip_comments_and_newlines();
        self.expect(TokenKind::From);
        self.skip_comments_and_newlines();

        // Module path
        self.advance(); // skip string

        self.eat(TokenKind::Semicolon);
        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::ImportDeclaration, start, end));
    }

    fn parse_export_declaration(&mut self) {
        let start = self.current.start;
        self.advance(); // skip export

        self.skip_comments_and_newlines();

        let mut flags = 0;

        if self.eat(TokenKind::Default) {
            flags |= flags::EXPORT_DEFAULT;
            self.skip_comments_and_newlines();

            // export default expression
            match self.current.kind {
                TokenKind::Function | TokenKind::Async => self.parse_function_declaration(),
                TokenKind::Class => self.parse_class_declaration(),
                _ => {
                    self.parse_expression();
                    self.eat(TokenKind::Semicolon);
                }
            }
        } else if self.at(TokenKind::LBrace) {
            // export { named }
            self.advance();
            self.skip_comments_and_newlines();

            while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
                let spec_start = self.current.start;
                self.parse_identifier();
                self.skip_comments_and_newlines();

                if self.eat(TokenKind::As) {
                    self.skip_comments_and_newlines();
                    self.parse_identifier();
                }

                let spec_end = self.current.start;
                self.nodes.push(Node::new(NodeKind::ExportSpecifier, spec_start, spec_end));

                self.skip_comments_and_newlines();
                if !self.eat(TokenKind::Comma) {
                    break;
                }
                self.skip_comments_and_newlines();
            }

            self.expect(TokenKind::RBrace);

            // Optional: from "module"
            self.skip_comments_and_newlines();
            if self.eat(TokenKind::From) {
                self.skip_comments_and_newlines();
                self.advance(); // skip string
            }

            self.eat(TokenKind::Semicolon);
        } else if self.eat(TokenKind::Star) {
            // export * from "module"
            self.skip_comments_and_newlines();

            if self.eat(TokenKind::As) {
                self.skip_comments_and_newlines();
                self.parse_identifier();
            }

            self.skip_comments_and_newlines();
            self.expect(TokenKind::From);
            self.skip_comments_and_newlines();
            self.advance(); // skip string
            self.eat(TokenKind::Semicolon);
        } else {
            // export declaration
            match self.current.kind {
                TokenKind::Const | TokenKind::Let | TokenKind::Var => self.parse_variable_declaration(),
                TokenKind::Function | TokenKind::Async => self.parse_function_declaration(),
                TokenKind::Class => self.parse_class_declaration(),
                _ => {}
            }
        }

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::ExportDeclaration, start, end)
            .with_flags(flags));
    }

    fn parse_block_statement(&mut self) {
        let start = self.current.start;
        self.expect(TokenKind::LBrace);

        self.skip_comments_and_newlines();

        let mut count = 0;
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            self.parse_statement_or_declaration();
            count += 1;
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RBrace);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::BlockStatement, start, end)
            .with_extra(count));
    }

    fn parse_if_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip if

        self.skip_comments_and_newlines();
        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();
        self.parse_expression();
        self.skip_comments_and_newlines();
        self.expect(TokenKind::RParen);

        self.skip_comments_and_newlines();
        self.parse_statement_or_declaration();

        // else
        self.skip_comments_and_newlines();
        let has_else = self.eat(TokenKind::Else);
        if has_else {
            self.skip_comments_and_newlines();
            self.parse_statement_or_declaration();
        }

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::IfStatement, start, end)
            .with_flags(if has_else { 1 } else { 0 }));
    }

    fn parse_for_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip for

        self.skip_comments_and_newlines();

        // Check for await
        let is_await = self.eat(TokenKind::Await);
        self.skip_comments_and_newlines();

        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();

        // Init
        let has_init = !self.at(TokenKind::Semicolon);
        if has_init {
            if matches!(self.current.kind, TokenKind::Const | TokenKind::Let | TokenKind::Var) {
                self.parse_variable_declaration();
            } else {
                self.parse_expression();
            }
        }

        self.skip_comments_and_newlines();

        // Check for of/in
        let kind = if self.eat(TokenKind::Of) {
            NodeKind::ForOfStatement
        } else if self.eat(TokenKind::In) {
            NodeKind::ForInStatement
        } else {
            // Regular for
            self.eat(TokenKind::Semicolon);
            self.skip_comments_and_newlines();

            // Test
            if !self.at(TokenKind::Semicolon) {
                self.parse_expression();
            }
            self.eat(TokenKind::Semicolon);
            self.skip_comments_and_newlines();

            // Update
            if !self.at(TokenKind::RParen) {
                self.parse_expression();
            }

            NodeKind::ForStatement
        };

        if kind != NodeKind::ForStatement {
            // for-of / for-in: parse right side
            self.skip_comments_and_newlines();
            self.parse_expression();
        }

        self.skip_comments_and_newlines();
        self.expect(TokenKind::RParen);

        self.skip_comments_and_newlines();
        self.parse_statement_or_declaration();

        let end = self.current.start;
        self.nodes.push(Node::new(kind, start, end)
            .with_flags(if is_await { flags::ASYNC } else { 0 }));
    }

    fn parse_while_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip while

        self.skip_comments_and_newlines();
        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();
        self.parse_expression();
        self.skip_comments_and_newlines();
        self.expect(TokenKind::RParen);

        self.skip_comments_and_newlines();
        self.parse_statement_or_declaration();

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::WhileStatement, start, end));
    }

    fn parse_do_while_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip do

        self.skip_comments_and_newlines();
        self.parse_statement_or_declaration();

        self.skip_comments_and_newlines();
        self.expect(TokenKind::While);
        self.skip_comments_and_newlines();
        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();
        self.parse_expression();
        self.skip_comments_and_newlines();
        self.expect(TokenKind::RParen);
        self.eat(TokenKind::Semicolon);

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::DoWhileStatement, start, end));
    }

    fn parse_switch_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip switch

        self.skip_comments_and_newlines();
        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();
        self.parse_expression();
        self.skip_comments_and_newlines();
        self.expect(TokenKind::RParen);

        self.skip_comments_and_newlines();
        self.expect(TokenKind::LBrace);
        self.skip_comments_and_newlines();

        let mut case_count = 0;
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            let case_start = self.current.start;

            let is_default = self.eat(TokenKind::Default);
            if !is_default {
                self.expect(TokenKind::Case);
                self.skip_comments_and_newlines();
                self.parse_expression();
            }

            self.skip_comments_and_newlines();
            self.expect(TokenKind::Colon);
            self.skip_comments_and_newlines();

            let mut stmt_count = 0;
            while !matches!(self.current.kind, TokenKind::Case | TokenKind::Default | TokenKind::RBrace | TokenKind::Eof) {
                self.parse_statement_or_declaration();
                stmt_count += 1;
                self.skip_comments_and_newlines();
            }

            let case_end = self.current.start;
            self.nodes.push(Node::new(NodeKind::SwitchCase, case_start, case_end)
                .with_flags(if is_default { 1 } else { 0 })
                .with_extra(stmt_count));
            case_count += 1;
        }

        self.expect(TokenKind::RBrace);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::SwitchStatement, start, end)
            .with_extra(case_count));
    }

    fn parse_return_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip return

        let has_arg = !self.at(TokenKind::Semicolon) && !self.at(TokenKind::RBrace) && !self.at(TokenKind::Newline);
        if has_arg {
            self.skip_comments_and_newlines();
            self.parse_expression();
        }

        self.eat(TokenKind::Semicolon);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::ReturnStatement, start, end)
            .with_flags(if has_arg { 1 } else { 0 }));
    }

    fn parse_throw_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip throw

        self.skip_comments_and_newlines();
        self.parse_expression();
        self.eat(TokenKind::Semicolon);

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::ThrowStatement, start, end));
    }

    fn parse_try_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip try

        self.skip_comments_and_newlines();
        self.parse_block_statement();

        self.skip_comments_and_newlines();

        // catch
        let has_catch = self.eat(TokenKind::Catch);
        if has_catch {
            let catch_start = self.current.start;

            self.skip_comments_and_newlines();
            if self.eat(TokenKind::LParen) {
                self.skip_comments_and_newlines();
                self.parse_binding_pattern();
                self.skip_comments_and_newlines();
                self.expect(TokenKind::RParen);
            }

            self.skip_comments_and_newlines();
            self.parse_block_statement();

            let catch_end = self.current.start;
            self.nodes.push(Node::new(NodeKind::CatchClause, catch_start, catch_end));
        }

        // finally
        self.skip_comments_and_newlines();
        let has_finally = self.eat(TokenKind::Finally);
        if has_finally {
            self.skip_comments_and_newlines();
            self.parse_block_statement();
        }

        let end = self.current.start;
        let flags = (if has_catch { 1 } else { 0 }) | (if has_finally { 2 } else { 0 });
        self.nodes.push(Node::new(NodeKind::TryStatement, start, end)
            .with_flags(flags));
    }

    fn parse_break_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip break

        // Optional label
        if self.at(TokenKind::Identifier) {
            self.parse_identifier();
        }

        self.eat(TokenKind::Semicolon);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::BreakStatement, start, end));
    }

    fn parse_continue_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip continue

        // Optional label
        if self.at(TokenKind::Identifier) {
            self.parse_identifier();
        }

        self.eat(TokenKind::Semicolon);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::ContinueStatement, start, end));
    }

    fn parse_empty_statement(&mut self) {
        let start = self.current.start;
        self.advance(); // skip ;
        self.nodes.push(Node::new(NodeKind::EmptyStatement, start, start + 1));
    }

    fn parse_expression_statement(&mut self) {
        let start = self.current.start;
        self.parse_expression();
        self.eat(TokenKind::Semicolon);
        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::ExpressionStatement, start, end));
    }

    // ========================================
    // Expressions (simplified)
    // ========================================

    fn parse_expression(&mut self) {
        self.parse_assignment_expression();

        // Sequence expression
        while self.eat(TokenKind::Comma) {
            self.skip_comments_and_newlines();
            self.parse_assignment_expression();
        }
    }

    fn parse_assignment_expression(&mut self) {
        let start = self.current.start;

        // Arrow function: x => body
        if self.at(TokenKind::Identifier) && self.peek.kind == TokenKind::Arrow {
            self.parse_identifier();
            self.advance(); // skip =>
            self.skip_comments_and_newlines();
            self.parse_arrow_body();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::ArrowFunctionExpression, start, end));
            return;
        }

        self.parse_conditional_expression();

        // Assignment
        if matches!(self.current.kind,
            TokenKind::Eq | TokenKind::PlusEq | TokenKind::MinusEq |
            TokenKind::StarEq | TokenKind::SlashEq | TokenKind::PercentEq |
            TokenKind::StarStarEq | TokenKind::LtLtEq | TokenKind::GtGtEq |
            TokenKind::GtGtGtEq | TokenKind::AmpEq | TokenKind::PipeEq |
            TokenKind::CaretEq | TokenKind::AmpAmpEq | TokenKind::PipePipeEq |
            TokenKind::QuestionQuestionEq
        ) {
            self.advance();
            self.skip_comments_and_newlines();
            self.parse_assignment_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::AssignmentExpression, start, end));
        }
    }

    fn parse_arrow_body(&mut self) {
        if self.at(TokenKind::LBrace) {
            self.parse_block_statement();
        } else {
            self.parse_assignment_expression();
        }
    }

    fn parse_conditional_expression(&mut self) {
        let start = self.current.start;
        self.parse_logical_or_expression();

        if self.eat(TokenKind::Question) {
            self.skip_comments_and_newlines();
            self.parse_assignment_expression();
            self.skip_comments_and_newlines();
            self.expect(TokenKind::Colon);
            self.skip_comments_and_newlines();
            self.parse_assignment_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::ConditionalExpression, start, end));
        }
    }

    fn parse_logical_or_expression(&mut self) {
        let start = self.current.start;
        self.parse_logical_and_expression();

        while self.eat(TokenKind::PipePipe) || self.eat(TokenKind::QuestionQuestion) {
            self.skip_comments_and_newlines();
            self.parse_logical_and_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::LogicalExpression, start, end));
        }
    }

    fn parse_logical_and_expression(&mut self) {
        let start = self.current.start;
        self.parse_bitwise_or_expression();

        while self.eat(TokenKind::AmpAmp) {
            self.skip_comments_and_newlines();
            self.parse_bitwise_or_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::LogicalExpression, start, end));
        }
    }

    fn parse_bitwise_or_expression(&mut self) {
        let start = self.current.start;
        self.parse_bitwise_xor_expression();

        while self.eat(TokenKind::Pipe) {
            self.skip_comments_and_newlines();
            self.parse_bitwise_xor_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_bitwise_xor_expression(&mut self) {
        let start = self.current.start;
        self.parse_bitwise_and_expression();

        while self.eat(TokenKind::Caret) {
            self.skip_comments_and_newlines();
            self.parse_bitwise_and_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_bitwise_and_expression(&mut self) {
        let start = self.current.start;
        self.parse_equality_expression();

        while self.eat(TokenKind::Amp) {
            self.skip_comments_and_newlines();
            self.parse_equality_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_equality_expression(&mut self) {
        let start = self.current.start;
        self.parse_relational_expression();

        while matches!(self.current.kind, TokenKind::EqEq | TokenKind::BangEq | TokenKind::EqEqEq | TokenKind::BangEqEq) {
            self.advance();
            self.skip_comments_and_newlines();
            self.parse_relational_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_relational_expression(&mut self) {
        let start = self.current.start;
        self.parse_shift_expression();

        while matches!(self.current.kind, TokenKind::Lt | TokenKind::Gt | TokenKind::LtEq | TokenKind::GtEq | TokenKind::Instanceof | TokenKind::In) {
            self.advance();
            self.skip_comments_and_newlines();
            self.parse_shift_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_shift_expression(&mut self) {
        let start = self.current.start;
        self.parse_additive_expression();

        while matches!(self.current.kind, TokenKind::LtLt | TokenKind::GtGt | TokenKind::GtGtGt) {
            self.advance();
            self.skip_comments_and_newlines();
            self.parse_additive_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_additive_expression(&mut self) {
        let start = self.current.start;
        self.parse_multiplicative_expression();

        while matches!(self.current.kind, TokenKind::Plus | TokenKind::Minus) {
            self.advance();
            self.skip_comments_and_newlines();
            self.parse_multiplicative_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_multiplicative_expression(&mut self) {
        let start = self.current.start;
        self.parse_exponentiation_expression();

        while matches!(self.current.kind, TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            self.advance();
            self.skip_comments_and_newlines();
            self.parse_exponentiation_expression();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_exponentiation_expression(&mut self) {
        let start = self.current.start;
        self.parse_unary_expression();

        if self.eat(TokenKind::StarStar) {
            self.skip_comments_and_newlines();
            self.parse_exponentiation_expression(); // right-associative
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::BinaryExpression, start, end));
        }
    }

    fn parse_unary_expression(&mut self) {
        let start = self.current.start;

        match self.current.kind {
            TokenKind::Bang | TokenKind::Tilde | TokenKind::Plus | TokenKind::Minus |
            TokenKind::Typeof | TokenKind::Void | TokenKind::Delete => {
                self.advance();
                self.skip_comments_and_newlines();
                self.parse_unary_expression();
                let end = self.current.start;
                self.nodes.push(Node::new(NodeKind::UnaryExpression, start, end));
            }
            TokenKind::PlusPlus | TokenKind::MinusMinus => {
                self.advance();
                self.skip_comments_and_newlines();
                self.parse_unary_expression();
                let end = self.current.start;
                self.nodes.push(Node::new(NodeKind::UpdateExpression, start, end));
            }
            TokenKind::Await => {
                self.advance();
                self.skip_comments_and_newlines();
                self.parse_unary_expression();
                let end = self.current.start;
                self.nodes.push(Node::new(NodeKind::AwaitExpression, start, end));
            }
            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) {
        let start = self.current.start;
        self.parse_call_expression();

        if matches!(self.current.kind, TokenKind::PlusPlus | TokenKind::MinusMinus) {
            self.advance();
            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::UpdateExpression, start, end)
                .with_flags(1)); // postfix flag
        }
    }

    fn parse_call_expression(&mut self) {
        let start = self.current.start;
        self.parse_member_expression();

        loop {
            match self.current.kind {
                TokenKind::LParen => {
                    self.parse_arguments();
                    let end = self.current.start;
                    self.nodes.push(Node::new(NodeKind::CallExpression, start, end));
                }
                TokenKind::LBracket => {
                    self.advance();
                    self.skip_comments_and_newlines();
                    self.parse_expression();
                    self.skip_comments_and_newlines();
                    self.expect(TokenKind::RBracket);
                    let end = self.current.start;
                    self.nodes.push(Node::new(NodeKind::MemberExpression, start, end)
                        .with_flags(flags::COMPUTED));
                }
                TokenKind::Dot | TokenKind::QuestionDot => {
                    let optional = self.current.kind == TokenKind::QuestionDot;
                    self.advance();
                    self.skip_comments_and_newlines();
                    self.parse_identifier();
                    let end = self.current.start;
                    self.nodes.push(Node::new(NodeKind::MemberExpression, start, end)
                        .with_flags(if optional { 1 } else { 0 }));
                }
                TokenKind::Template => {
                    // Tagged template
                    self.parse_template_literal();
                    let end = self.current.start;
                    self.nodes.push(Node::new(NodeKind::TaggedTemplateExpression, start, end));
                }
                _ => break,
            }
        }
    }

    fn parse_arguments(&mut self) {
        self.expect(TokenKind::LParen);
        self.skip_comments_and_newlines();

        while !self.at(TokenKind::RParen) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::DotDotDot) {
                self.parse_spread_element();
            } else {
                self.parse_assignment_expression();
            }

            self.skip_comments_and_newlines();
            if !self.eat(TokenKind::Comma) {
                break;
            }
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RParen);
    }

    fn parse_spread_element(&mut self) {
        let start = self.current.start;
        self.advance(); // skip ...
        self.parse_assignment_expression();
        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::SpreadElement, start, end));
    }

    fn parse_member_expression(&mut self) {
        let start = self.current.start;

        if self.eat(TokenKind::New) {
            self.skip_comments_and_newlines();
            self.parse_member_expression();

            if self.at(TokenKind::LParen) {
                self.parse_arguments();
            }

            let end = self.current.start;
            self.nodes.push(Node::new(NodeKind::NewExpression, start, end));
        } else {
            self.parse_primary_expression();
        }
    }

    fn parse_primary_expression(&mut self) {
        let start = self.current.start;

        match self.current.kind {
            TokenKind::Identifier => self.parse_identifier(),
            TokenKind::Number | TokenKind::BigInt | TokenKind::String |
            TokenKind::True | TokenKind::False | TokenKind::Null => {
                self.advance();
                self.nodes.push(Node::new(NodeKind::Literal, start, self.current.start));
            }
            TokenKind::Template => self.parse_template_literal(),
            TokenKind::This => {
                self.advance();
                self.nodes.push(Node::new(NodeKind::ThisExpression, start, self.current.start));
            }
            TokenKind::Super => {
                self.advance();
                self.nodes.push(Node::new(NodeKind::Super, start, self.current.start));
            }
            TokenKind::LParen => {
                self.advance();
                self.skip_comments_and_newlines();
                self.parse_expression();
                self.skip_comments_and_newlines();
                self.expect(TokenKind::RParen);
            }
            TokenKind::LBracket => self.parse_array_expression(),
            TokenKind::LBrace => self.parse_object_expression(),
            TokenKind::Function => self.parse_function_expression(),
            TokenKind::Async => {
                if self.peek.kind == TokenKind::Function {
                    self.parse_function_expression();
                } else {
                    // async arrow function
                    self.advance();
                    self.skip_comments_and_newlines();
                    // Handle as identifier for now
                    self.parse_identifier();
                }
            }
            TokenKind::Class => self.parse_class_expression(),
            TokenKind::Yield => {
                self.advance();
                self.skip_comments_and_newlines();
                if !matches!(self.current.kind, TokenKind::Semicolon | TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket | TokenKind::Comma | TokenKind::Colon) {
                    self.parse_assignment_expression();
                }
                let end = self.current.start;
                self.nodes.push(Node::new(NodeKind::YieldExpression, start, end));
            }
            _ => {
                // Skip unknown token
                self.advance();
            }
        }
    }

    fn parse_identifier(&mut self) {
        let start = self.current.start;
        self.advance();
        self.nodes.push(Node::new(NodeKind::Identifier, start, self.current.start));
    }

    fn parse_template_literal(&mut self) {
        let start = self.current.start;
        self.advance(); // skip template
        self.nodes.push(Node::new(NodeKind::TemplateLiteral, start, self.current.start));
    }

    fn parse_array_expression(&mut self) {
        let start = self.current.start;
        self.advance(); // skip [

        self.skip_comments_and_newlines();

        let mut count = 0;
        while !self.at(TokenKind::RBracket) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Comma) {
                self.advance();
                count += 1;
            } else if self.at(TokenKind::DotDotDot) {
                self.parse_spread_element();
                count += 1;
            } else {
                self.parse_assignment_expression();
                count += 1;
            }

            self.skip_comments_and_newlines();
            if !self.eat(TokenKind::Comma) {
                break;
            }
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RBracket);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::ArrayExpression, start, end)
            .with_extra(count));
    }

    fn parse_object_expression(&mut self) {
        let start = self.current.start;
        self.advance(); // skip {

        self.skip_comments_and_newlines();

        let mut count = 0;
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::DotDotDot) {
                self.parse_spread_element();
            } else {
                self.parse_object_property();
            }
            count += 1;

            self.skip_comments_and_newlines();
            if !self.eat(TokenKind::Comma) {
                break;
            }
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::RBrace);
        let end = self.current.start;

        self.nodes.push(Node::new(NodeKind::ObjectExpression, start, end)
            .with_extra(count));
    }

    fn parse_object_property(&mut self) {
        let start = self.current.start;
        let mut flags = 0;

        // Getter/setter
        let is_getter = self.at(TokenKind::Get);
        let is_setter = self.at(TokenKind::Set);
        if (is_getter || is_setter) && self.peek.kind != TokenKind::Colon && self.peek.kind != TokenKind::LParen {
            self.advance();
            self.skip_comments_and_newlines();
        }

        // Async
        if self.at(TokenKind::Async) && self.peek.kind != TokenKind::Colon && self.peek.kind != TokenKind::LParen {
            flags |= flags::ASYNC;
            self.advance();
            self.skip_comments_and_newlines();
        }

        // Generator
        if self.eat(TokenKind::Star) {
            flags |= flags::GENERATOR;
            self.skip_comments_and_newlines();
        }

        // Computed property
        if self.eat(TokenKind::LBracket) {
            flags |= flags::COMPUTED;
            self.parse_expression();
            self.expect(TokenKind::RBracket);
        } else {
            // Regular key
            self.parse_identifier();
        }

        self.skip_comments_and_newlines();

        // Method shorthand
        if self.at(TokenKind::LParen) {
            self.parse_function_params();
            self.skip_comments_and_newlines();
            self.parse_block_statement();
        } else if self.eat(TokenKind::Colon) {
            // Regular property
            self.skip_comments_and_newlines();
            self.parse_assignment_expression();
        } else {
            // Shorthand property
            flags |= flags::SHORTHAND;
        }

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::Property, start, end)
            .with_flags(flags));
    }

    fn parse_function_expression(&mut self) {
        let start = self.current.start;
        let mut flags = 0;

        if self.eat(TokenKind::Async) {
            flags |= flags::ASYNC;
            self.skip_comments_and_newlines();
        }

        self.expect(TokenKind::Function);
        self.skip_comments_and_newlines();

        if self.eat(TokenKind::Star) {
            flags |= flags::GENERATOR;
            self.skip_comments_and_newlines();
        }

        // Optional name
        if self.at(TokenKind::Identifier) {
            self.parse_identifier();
        }

        self.skip_comments_and_newlines();
        self.parse_function_params();
        self.skip_comments_and_newlines();
        self.parse_block_statement();

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::FunctionExpression, start, end)
            .with_flags(flags));
    }

    fn parse_class_expression(&mut self) {
        let start = self.current.start;
        self.advance(); // skip class

        self.skip_comments_and_newlines();

        // Optional name
        if self.at(TokenKind::Identifier) {
            self.parse_identifier();
        }

        self.skip_comments_and_newlines();

        // Extends
        if self.eat(TokenKind::Extends) {
            self.skip_comments_and_newlines();
            self.parse_expression();
        }

        self.skip_comments_and_newlines();
        self.parse_class_body();

        let end = self.current.start;
        self.nodes.push(Node::new(NodeKind::ClassExpression, start, end));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut parser = Parser::new("const x = 1;");
        let count = parser.parse_count();
        assert!(count >= 3); // Program, VariableDeclaration, VariableDeclarator, etc.
    }

    #[test]
    fn test_function_declaration() {
        let mut parser = Parser::new("function foo(a, b) { return a + b; }");
        let count = parser.parse_count();
        assert!(count >= 5);
    }

    #[test]
    fn test_class_declaration() {
        let mut parser = Parser::new("class Foo extends Bar { constructor() {} }");
        let count = parser.parse_count();
        assert!(count >= 4);
    }

    #[test]
    fn test_import_export() {
        let mut parser = Parser::new("import { foo } from 'bar'; export default x;");
        let count = parser.parse_count();
        assert!(count >= 4);
    }

    #[test]
    fn test_arrow_function() {
        let mut parser = Parser::new("const fn = x => x * 2;");
        let count = parser.parse_count();
        assert!(count >= 4);
    }

    #[test]
    fn test_binary() {
        let mut parser = Parser::new("const x = 1 + 2 * 3;");
        let buf = parser.parse_binary();
        let node_count = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        assert!(node_count >= 5);
    }
}

//! Position and span types for source locations

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A position in the source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Position {
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (0-indexed)
    pub column: u32,
    /// Byte offset from start of source
    pub offset: u32,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(line: u32, column: u32, offset: u32) -> Self {
        Self { line, column, offset }
    }
}

/// A span representing a range in the source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Start position
    pub start: Position,
    /// End position
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a span from line/column/offset values
    pub fn from_coords(
        start_line: u32,
        start_column: u32,
        start_offset: u32,
        end_line: u32,
        end_column: u32,
        end_offset: u32,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_column, start_offset),
            end: Position::new(end_line, end_column, end_offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_web_media_wave4_position_new_and_eq() {
        let p = Position::new(1, 0, 0);
        assert_eq!(p.line, 1);
        assert_eq!(p.column, 0);
        assert_eq!(p.offset, 0);
        assert_eq!(p, Position::new(1, 0, 0));
        assert_ne!(p, Position::new(2, 0, 0));
    }

    #[test]
    fn portfolio_web_media_wave4_span_from_coords() {
        let s = Span::from_coords(1, 0, 0, 2, 5, 20);
        assert_eq!(s.start.line, 1);
        assert_eq!(s.end.line, 2);
        assert_eq!(s.end.column, 5);
        assert_eq!(s.end.offset, 20);
        let s2 = Span::new(Position::new(1, 0, 0), Position::new(1, 10, 10));
        assert_eq!(s2.start.offset, 0);
        assert_eq!(s2.end.column, 10);
    }

    #[test]
    fn portfolio_web_media_wave4_span_serde_roundtrip() {
        let s = Span::from_coords(3, 1, 10, 3, 8, 17);
        let json = serde_json::to_string(&s).expect("ser");
        let back: Span = serde_json::from_str(&json).expect("de");
        assert_eq!(back, s);
    }
}

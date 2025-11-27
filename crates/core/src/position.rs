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

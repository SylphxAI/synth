//! Error types for Synth WASM

use thiserror::Error;
use wasm_bindgen::prelude::*;

/// Synth WASM error types
#[derive(Error, Debug)]
pub enum SynthError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid node ID: {0}")]
    InvalidNodeId(u32),

    #[error("Tree structure error: {0}")]
    TreeStructureError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<SynthError> for JsValue {
    fn from(err: SynthError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

/// Result type for Synth WASM operations
pub type SynthResult<T> = Result<T, SynthError>;

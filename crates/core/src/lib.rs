//! Synth WASM Core
//!
//! Core types and utilities shared across all Synth WASM parsers.
//! Provides Tree structure compatible with the TypeScript @sylphx/synth package.

mod tree;
mod error;
mod position;

pub use tree::*;
pub use error::*;
pub use position::*;

use wasm_bindgen::prelude::*;

/// Initialize the WASM module (called automatically)
#[wasm_bindgen(start)]
pub fn init() {
    // Panic hook can be added later if needed
}

/// Get the version of the WASM core
#[wasm_bindgen(js_name = coreVersion)]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

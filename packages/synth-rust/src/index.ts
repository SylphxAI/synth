/**
 * @module @sylphx/synth-rust
 *
 * Rust parser using Synth's universal AST
 * Conversion layer over tree-sitter-rust
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { RustParseOptions } from './parser.js'
export { createParser, parse, parseAsync, RustParser } from './parser.js'

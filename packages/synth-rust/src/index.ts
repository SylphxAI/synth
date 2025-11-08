/**
 * @sylphx/synth-rust
 *
 * Rust parser using Synth's universal AST
 * Conversion layer over tree-sitter-rust
 */

export { RustParser, createParser, parse, parseAsync } from './parser.js'
export type { RustParseOptions } from './parser.js'

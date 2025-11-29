/**
 * @module @sylphx/synth-go
 *
 * Go parser using Synth's universal AST
 * Conversion layer over tree-sitter-go
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { GoParseOptions } from './parser.js'
export { createParser, GoParser, parse, parseAsync } from './parser.js'

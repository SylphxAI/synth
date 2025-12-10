/**
 * @module @sylphx/synth-php
 *
 * PHP parser using Synth's universal AST
 * Conversion layer over tree-sitter-php
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export { createParser, init, PhpParser, parse, parseAsync } from './parser.js'
export type { PhpParseOptions } from './types.js'

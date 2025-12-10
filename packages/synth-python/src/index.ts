/**
 * @module @sylphx/synth-python
 *
 * Python parser using Synth's universal AST
 * Conversion layer over tree-sitter-python
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { PythonParseOptions } from './parser.js'
export { createParser, init, PythonParser, parse, parseAsync } from './parser.js'

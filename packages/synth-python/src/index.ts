/**
 * @sylphx/synth-python
 *
 * Python parser using Synth's universal AST
 * Conversion layer over tree-sitter-python
 */

export type { PythonParseOptions } from './parser.js'
export { createParser, PythonParser, parse, parseAsync } from './parser.js'

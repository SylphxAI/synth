/**
 * @module @sylphx/synth-jsx
 *
 * JSX/TSX parser using Synth's universal AST
 * Conversion layer over Acorn + acorn-jsx
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { JSXParseOptions } from './parser.js'
export { createParser, JSXParser, parse, parseAsync } from './parser.js'

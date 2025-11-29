/**
 * @module @sylphx/synth-sql
 *
 * SQL parser using Synth's universal AST
 * Conversion layer over node-sql-parser
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { SQLParseOptions } from './parser.js'
export { createParser, parse, parseAsync, SQLParser } from './parser.js'

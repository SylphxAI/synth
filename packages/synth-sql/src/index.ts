/**
 * @sylphx/synth-sql
 *
 * SQL parser using Synth's universal AST
 * Conversion layer over node-sql-parser
 */

export type { SQLParseOptions } from './parser.js'
export { createParser, parse, parseAsync, SQLParser } from './parser.js'

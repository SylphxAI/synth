/**
 * @sylphx/synth-toml
 *
 * TOML parser using Synth's universal AST
 * Hand-written, zero dependencies
 */

export type { TOMLParseOptions } from './parser.js'
export { createParser, parse, parseAsync, TOMLParser } from './parser.js'
export type { Token, TokenType } from './tokenizer.js'
export { TOMLTokenizer } from './tokenizer.js'

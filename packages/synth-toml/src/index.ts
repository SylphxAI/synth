/**
 * @sylphx/synth-toml
 *
 * TOML parser using Synth's universal AST
 * Hand-written, zero dependencies
 */

export { TOMLParser, createParser, parse, parseAsync } from './parser.js'
export type { TOMLParseOptions } from './parser.js'
export { TOMLTokenizer } from './tokenizer.js'
export type { Token, TokenType } from './tokenizer.js'

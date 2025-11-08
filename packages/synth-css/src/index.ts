/**
 * @sylphx/synth-css
 *
 * CSS3 parser using Synth's universal AST
 * Hand-written, zero dependencies
 */

export { CSSParser, createParser, parse, parseAsync } from './parser.js'
export type { CSSParseOptions } from './parser.js'
export { CSSTokenizer } from './tokenizer.js'
export type { Token, TokenType } from './tokenizer.js'

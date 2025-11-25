/**
 * @sylphx/synth-json
 *
 * JSON parser using Synth's universal AST
 */

export { JSONParser, createParser, parse, parseAsync } from './parser.js'
export type { JSONParseOptions } from './parser.js'

// Incremental tokenizer
export { IncrementalJSONTokenizer } from './incremental-tokenizer.js'

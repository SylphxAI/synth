/**
 * @sylphx/synth-json
 *
 * JSON parser using Synth's universal AST
 */

// Incremental tokenizer
export { IncrementalJSONTokenizer } from './incremental-tokenizer.js'
export type { JSONParseOptions } from './parser.js'
export { createParser, JSONParser, parse, parseAsync } from './parser.js'

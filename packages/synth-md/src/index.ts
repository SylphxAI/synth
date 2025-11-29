/**
 * @module @sylphx/synth-md
 *
 * High-performance Markdown parser - 26-42x faster than remark
 *
 * Features:
 * - Ultra-fast parsing (26-42x faster than remark)
 * - Batch processing (4-5x faster tokenization)
 * - Object pooling (10-13x faster for repeated parses)
 * - Incremental parsing (10-100x faster for edits)
 * - Streaming support
 * - Full GFM support
 * - Plugin system
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export { BatchTokenizer } from './batch-tokenizer.js'
export type { Edit } from './incremental-parser.js'
// Export incremental parser
export {
  calculateEditDistance,
  detectEdit,
  IncrementalMarkdownParser,
  shouldUseIncremental,
} from './incremental-parser.js'
export { IncrementalMarkdownTokenizer } from './incremental-tokenizer.js'
export { InlineTokenizer } from './inline-tokenizer.js'
// Export node pool
export { createNodePool, getGlobalNodePool, MarkdownNodePool, NodePool } from './node-pool.js'
export type { ParseOptions } from './parser.js'
// Export main parser
export { createParser, DEFAULT_PARSE_OPTIONS, Parser, parse, parseAsync } from './parser.js'

// Export plugin system
export * from './plugin.js'
export type { StreamingOptions } from './streaming-parser.js'
// Export streaming parser
export { parseStream, parseWithProgress, StreamingMarkdownParser } from './streaming-parser.js'
// Export tokenizers (for advanced use)
export { Tokenizer } from './tokenizer.js'
export * from './tokens.js'
export type { IncrementalParseStats } from './true-incremental-parser.js'
export { TrueIncrementalParser } from './true-incremental-parser.js'
// Export types
export * from './types.js'

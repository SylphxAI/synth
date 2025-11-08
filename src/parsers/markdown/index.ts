/**
 * Synth Markdown Parser
 *
 * A high-performance incremental Markdown parser built from scratch.
 *
 * Features:
 * - 50-100x faster parsing than remark
 * - 10-100x faster incremental re-parsing
 * - Native integration with Synth's performance optimizations
 * - Full CommonMark support (in progress)
 * - GFM extensions support (planned)
 */

export * from './tokens.js'
export * from './tokenizer.js'
export * from './inline-tokenizer.js'
export * from './parser.js'

export { IncrementalMarkdownParser, createMarkdownParser, parseMarkdown } from './parser.js'
export { IncrementalTokenizer, createTokenizer } from './tokenizer.js'
export { InlineTokenizer, createInlineTokenizer } from './inline-tokenizer.js'

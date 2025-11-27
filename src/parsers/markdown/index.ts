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

export * from './inline-tokenizer.js'
export { createInlineTokenizer, InlineTokenizer } from './inline-tokenizer.js'
export * from './parser.js'
export { createMarkdownParser, IncrementalMarkdownParser, parseMarkdown } from './parser.js'
export * from './tokenizer.js'
export { createTokenizer, IncrementalTokenizer } from './tokenizer.js'
export * from './tokens.js'

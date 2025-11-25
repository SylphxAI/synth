/**
 * Incremental Tokenizer
 *
 * Enables efficient token-level incremental parsing by:
 * 1. Tracking token changes
 * 2. Detecting affected token ranges
 * 3. Re-tokenizing only affected regions
 * 4. Reusing unchanged tokens (90%+ reuse target)
 *
 * Performance target: <0.1ms for typical edits
 */

import type { Edit } from './incremental.js'
import type {
  Token,
  TokenStream,
  TokenRange,
  TokenPosition,
} from './types/token.js'
import {
  createTokenStream,
  getTokenRange,
} from './types/token.js'

/**
 * Tokenizer statistics
 */
export interface TokenizerStats {
  /**
   * Total tokens
   */
  totalTokens: number

  /**
   * Tokens reused from previous tokenization
   */
  reusedTokens: number

  /**
   * New tokens created
   */
  newTokens: number

  /**
   * Affected tokens replaced
   */
  affectedTokens: number

  /**
   * Reuse rate (0-1)
   */
  reuseRate: number

  /**
   * Tokenization time (ms)
   */
  timeMs: number
}

/**
 * Base incremental tokenizer
 *
 * Language-specific tokenizers should extend this class
 */
export abstract class IncrementalTokenizer {
  protected tokenStream: TokenStream | null = null
  protected source: string = ''

  /**
   * Language identifier
   */
  protected abstract language: string

  /**
   * Full tokenization (initial parse)
   */
  tokenize(source: string): TokenStream {
    const startTime = performance.now()
    this.source = source

    const tokens = this.tokenizeImpl(source, 0, source.length)
    this.tokenStream = createTokenStream(tokens, source, this.language)

    const timeMs = performance.now() - startTime

    // Log performance
    if (process.env.NODE_ENV !== 'production') {
      console.log(`[Tokenizer] Initial tokenization: ${tokens.length} tokens in ${timeMs.toFixed(2)}ms`)
    }

    return this.tokenStream
  }

  /**
   * Incremental tokenization after edit
   *
   * This is the core optimization:
   * 1. Find affected token range
   * 2. Re-tokenize only affected region
   * 3. Reuse unchanged tokens
   *
   * Expected: 90%+ token reuse for typical edits
   */
  retokenize(newSource: string, edit: Edit): { stream: TokenStream; stats: TokenizerStats } {
    const startTime = performance.now()

    if (!this.tokenStream) {
      // No previous tokens, do full tokenization
      const stream = this.tokenize(newSource)
      return {
        stream,
        stats: {
          totalTokens: stream.tokens.length,
          reusedTokens: 0,
          newTokens: stream.tokens.length,
          affectedTokens: 0,
          reuseRate: 0,
          timeMs: performance.now() - startTime,
        },
      }
    }

    // 1. Find affected token range
    const affectedRange = this.findAffectedTokenRange(edit)

    // 2. Calculate new byte range after edit
    const editDelta = edit.newEndByte - edit.oldEndByte
    const newAffectedEnd = affectedRange.byteRange.end + editDelta

    // 3. Re-tokenize only the affected region
    const newTokens = this.tokenizeImpl(
      newSource,
      affectedRange.byteRange.start,
      newAffectedEnd
    )

    // 4. Reuse unchanged tokens before affected region
    const tokensBeforeEdit = this.tokenStream.tokens.slice(0, affectedRange.startIndex)

    // 5. Reuse unchanged tokens after affected region (with position adjustment)
    const tokensAfterEdit = this.tokenStream.tokens
      .slice(affectedRange.endIndex + 1)
      .map(token => this.adjustTokenPosition(token, editDelta))

    // 6. Merge: before + new + after
    const allTokens = [
      ...tokensBeforeEdit,
      ...newTokens,
      ...tokensAfterEdit,
    ]

    // 7. Re-index tokens
    this.reindexTokens(allTokens)

    // 8. Create new token stream
    this.source = newSource
    this.tokenStream = createTokenStream(allTokens, newSource, this.language)

    const timeMs = performance.now() - startTime
    const reusedCount = tokensBeforeEdit.length + tokensAfterEdit.length

    const stats: TokenizerStats = {
      totalTokens: allTokens.length,
      reusedTokens: reusedCount,
      newTokens: newTokens.length,
      affectedTokens: affectedRange.endIndex - affectedRange.startIndex + 1,
      reuseRate: reusedCount / allTokens.length,
      timeMs,
    }

    // Log performance
    if (process.env.NODE_ENV !== 'production') {
      console.log(
        `[Tokenizer] Incremental: ${stats.totalTokens} tokens ` +
        `(${stats.reusedTokens} reused, ${stats.newTokens} new) ` +
        `${(stats.reuseRate * 100).toFixed(1)}% reuse in ${timeMs.toFixed(2)}ms`
      )
    }

    return { stream: this.tokenStream, stats }
  }

  /**
   * Find the range of tokens affected by an edit
   *
   * Strategy:
   * 1. Find tokens at edit boundaries
   * 2. Expand range to token boundaries (not byte boundaries)
   * 3. Optionally expand to safe boundaries (language-specific)
   */
  protected findAffectedTokenRange(edit: Edit): TokenRange {
    if (!this.tokenStream) {
      throw new Error('No previous token stream')
    }

    // Find tokens at edit start and end
    let tokenRange = getTokenRange(
      this.tokenStream,
      edit.startByte,
      edit.oldEndByte
    )

    if (!tokenRange) {
      // Edit is outside current tokens, re-tokenize everything
      return {
        startIndex: 0,
        endIndex: this.tokenStream.tokens.length - 1,
        byteRange: {
          start: 0,
          end: this.source.length,
        },
      }
    }

    // Expand to safe boundaries (language-specific)
    tokenRange = this.expandToSafeBoundaries(tokenRange, edit)

    return tokenRange
  }

  /**
   * Expand token range to safe re-tokenization boundaries
   *
   * Override in language-specific tokenizers for better efficiency.
   * For example:
   * - Markdown: Expand to blank lines (block boundaries)
   * - JavaScript: Expand to statement boundaries
   * - JSON: Expand to object/array boundaries
   */
  protected expandToSafeBoundaries(range: TokenRange, _edit: Edit): TokenRange {
    // Default: No expansion, use exact token boundaries
    return range
  }

  /**
   * Adjust token position after edit
   */
  protected adjustTokenPosition(token: Token, offsetDelta: number): Token {
    return {
      ...token,
      span: {
        start: {
          ...token.span.start,
          offset: token.span.start.offset + offsetDelta,
        },
        end: {
          ...token.span.end,
          offset: token.span.end.offset + offsetDelta,
        },
      },
    }
  }

  /**
   * Re-index tokens after merging
   */
  protected reindexTokens(tokens: Token[]): void {
    for (let i = 0; i < tokens.length; i++) {
      tokens[i]!.index = i
    }
  }

  /**
   * Get current token stream
   */
  getTokenStream(): TokenStream | null {
    return this.tokenStream
  }

  /**
   * Language-specific tokenization implementation
   *
   * Must be implemented by subclasses
   *
   * @param source - Source text
   * @param startOffset - Byte offset to start tokenizing
   * @param endOffset - Byte offset to end tokenizing
   * @returns Array of tokens
   */
  protected abstract tokenizeImpl(
    source: string,
    startOffset: number,
    endOffset: number
  ): Token[]

  /**
   * Convert byte offset to Position
   */
  protected offsetToPosition(offset: number, source: string): TokenPosition {
    let line = 0
    let column = 0

    for (let i = 0; i < offset && i < source.length; i++) {
      if (source[i] === '\n') {
        line++
        column = 0
      } else {
        column++
      }
    }

    return { line, column, offset }
  }
}

/**
 * Token statistics helper
 */
export function formatTokenStats(stats: TokenizerStats): string {
  return [
    `Total: ${stats.totalTokens} tokens`,
    `Reused: ${stats.reusedTokens} (${(stats.reuseRate * 100).toFixed(1)}%)`,
    `New: ${stats.newTokens}`,
    `Affected: ${stats.affectedTokens}`,
    `Time: ${stats.timeMs.toFixed(2)}ms`,
  ].join(', ')
}

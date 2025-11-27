/**
 * Token Types for Incremental Parsing
 *
 * Tokens are the intermediate representation between raw text and AST nodes.
 * They enable efficient incremental parsing by allowing token-level reuse.
 */

/**
 * Token kind - language-agnostic token types
 */
export enum TokenKind {
  // Whitespace & Separators
  WHITESPACE = 'whitespace',
  NEWLINE = 'newline',
  EOF = 'eof',

  // Text content
  TEXT = 'text',

  // Punctuation
  PUNCTUATION = 'punctuation',

  // Delimiters
  DELIMITER = 'delimiter',

  // Keywords (language-specific)
  KEYWORD = 'keyword',

  // Identifiers
  IDENTIFIER = 'identifier',

  // Literals
  STRING = 'string',
  NUMBER = 'number',
  BOOLEAN = 'boolean',
  NULL = 'null',

  // Operators
  OPERATOR = 'operator',

  // Comments
  COMMENT = 'comment',

  // Markdown-specific
  HEADING_START = 'heading_start', // #, ##, etc.
  EMPHASIS_MARKER = 'emphasis_marker', // *, _
  STRONG_MARKER = 'strong_marker', // **, __
  CODE_FENCE = 'code_fence', // ```
  CODE_MARKER = 'code_marker', // `
  LINK_START = 'link_start', // [
  LINK_END = 'link_end', // ]
  URL_START = 'url_start', // (
  URL_END = 'url_end', // )
  LIST_MARKER = 'list_marker', // -, *, 1.
  BLOCKQUOTE_MARKER = 'blockquote_marker', // >
  HORIZONTAL_RULE = 'horizontal_rule', // ---, ***, ___

  // HTML-specific
  TAG_OPEN = 'tag_open', // <
  TAG_CLOSE = 'tag_close', // >
  TAG_SELF_CLOSE = 'tag_self_close', // />
  ATTRIBUTE = 'attribute',

  // JavaScript-specific
  BRACE_OPEN = 'brace_open', // {
  BRACE_CLOSE = 'brace_close', // }
  PAREN_OPEN = 'paren_open', // (
  PAREN_CLOSE = 'paren_close', // )
  BRACKET_OPEN = 'bracket_open', // [
  BRACKET_CLOSE = 'bracket_close', // ]
  SEMICOLON = 'semicolon', // ;
  COMMA = 'comma', // ,
  DOT = 'dot', // .
  ARROW = 'arrow', // =>

  // Error
  ERROR = 'error',
  UNKNOWN = 'unknown',
}

/**
 * Token flags for additional metadata
 */
export enum TokenFlags {
  NONE = 0,
  LEADING_WHITESPACE = 1 << 0, // Has leading whitespace
  TRAILING_WHITESPACE = 1 << 1, // Has trailing whitespace
  MULTILINE = 1 << 2, // Spans multiple lines
  ESCAPED = 1 << 3, // Contains escaped characters
  INCOMPLETE = 1 << 4, // Incomplete/error token
  SYNTHETIC = 1 << 5, // Synthetic token (not from source)
}

/**
 * Token position with byte-level precision
 */
export interface TokenPosition {
  /**
   * Line number (0-based)
   */
  line: number

  /**
   * Column number (0-based)
   */
  column: number

  /**
   * Byte offset from start of document
   */
  offset: number
}

/**
 * Token span (start and end positions)
 */
export interface TokenSpan {
  start: TokenPosition
  end: TokenPosition
}

/**
 * Base token interface
 */
export interface Token {
  /**
   * Token kind
   */
  kind: TokenKind

  /**
   * Token value (raw text)
   */
  value: string

  /**
   * Position in source
   */
  span: TokenSpan

  /**
   * Token flags
   */
  flags: TokenFlags

  /**
   * Token index in token stream
   */
  index: number

  /**
   * Additional metadata (language-specific)
   */
  metadata?: Record<string, unknown>
}

/**
 * Token stream - ordered sequence of tokens
 */
export interface TokenStream {
  /**
   * All tokens
   */
  tokens: Token[]

  /**
   * Source text
   */
  source: string

  /**
   * Language/syntax
   */
  language: string

  /**
   * Creation timestamp
   */
  created: number

  /**
   * Last modified timestamp
   */
  modified: number
}

/**
 * Token range (for affected region detection)
 */
export interface TokenRange {
  /**
   * Start token index
   */
  startIndex: number

  /**
   * End token index (inclusive)
   */
  endIndex: number

  /**
   * Byte offset range
   */
  byteRange: {
    start: number
    end: number
  }
}

/**
 * Create a token
 */
export function createToken(
  kind: TokenKind,
  value: string,
  span: TokenSpan,
  index: number,
  flags: TokenFlags = TokenFlags.NONE,
  metadata?: Record<string, unknown>
): Token {
  return {
    kind,
    value,
    span,
    flags,
    index,
    metadata,
  }
}

/**
 * Create a token stream
 */
export function createTokenStream(tokens: Token[], source: string, language: string): TokenStream {
  const now = Date.now()
  return {
    tokens,
    source,
    language,
    created: now,
    modified: now,
  }
}

/**
 * Check if position is within token span
 */
export function isPositionInToken(position: number, token: Token): boolean {
  return position >= token.span.start.offset && position <= token.span.end.offset
}

/**
 * Check if token ranges overlap
 */
export function tokenRangesOverlap(
  range1: { start: number; end: number },
  range2: { start: number; end: number }
): boolean {
  return range1.start <= range2.end && range2.start <= range1.end
}

/**
 * Find token at byte offset
 */
export function findTokenAtOffset(stream: TokenStream, offset: number): Token | null {
  // Binary search for efficiency
  let left = 0
  let right = stream.tokens.length - 1

  while (left <= right) {
    const mid = Math.floor((left + right) / 2)
    const token = stream.tokens[mid]!

    if (offset < token.span.start.offset) {
      right = mid - 1
    } else if (offset > token.span.end.offset) {
      left = mid + 1
    } else {
      return token
    }
  }

  return null
}

/**
 * Get token range for byte range
 */
export function getTokenRange(
  stream: TokenStream,
  startByte: number,
  endByte: number
): TokenRange | null {
  const startToken = findTokenAtOffset(stream, startByte)
  const endToken = findTokenAtOffset(stream, endByte)

  if (!startToken || !endToken) {
    return null
  }

  return {
    startIndex: startToken.index,
    endIndex: endToken.index,
    byteRange: {
      start: startToken.span.start.offset,
      end: endToken.span.end.offset,
    },
  }
}

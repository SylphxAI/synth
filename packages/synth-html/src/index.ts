/**
 * @sylphx/synth-html
 *
 * High-performance HTML5 parser for Synth
 */

// Core parser
export { HTMLParser, createParser, parse, parseAsync } from './parser.js'
export type { HTMLParseOptions } from './parser.js'

// Tokenizer
export { HTMLTokenizer } from './tokenizer.js'
export type {
  Token,
  TokenType,
  HTMLToken,
  DoctypeToken,
  StartTagToken,
  EndTagToken,
  SelfClosingTagToken,
  TextToken,
  CommentToken,
  CDATAToken,
} from './tokenizer.js'

// Incremental tokenizer
export { IncrementalHTMLTokenizer } from './incremental-tokenizer.js'

// Types and utilities
export {
  VOID_ELEMENTS,
  isDocumentNode,
  isDoctypeNode,
  isElementNode,
  isTextNode,
  isCommentNode,
  isCDATANode,
  getTagName,
  getAttributes,
  getAttribute,
  isVoidElement,
  isSelfClosing,
  getTextValue,
  getCommentValue,
  getCDATAValue,
  getDoctypeName,
  getDoctypePublicId,
  getDoctypeSystemId,
} from './types.js'
export type { HTMLNodeType } from './types.js'

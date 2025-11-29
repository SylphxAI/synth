/**
 * @module @sylphx/synth-html
 *
 * High-performance HTML5 parser for Synth
 *
 * @packageDocumentation
 */

// Incremental tokenizer
export { IncrementalHTMLTokenizer } from './incremental-tokenizer.js'
export type { HTMLParseOptions } from './parser.js'
// Core parser
export { createParser, HTMLParser, parse, parseAsync } from './parser.js'
export type {
  CDATAToken,
  CommentToken,
  DoctypeToken,
  EndTagToken,
  HTMLToken,
  SelfClosingTagToken,
  StartTagToken,
  TextToken,
  Token,
  TokenType,
} from './tokenizer.js'
// Tokenizer
export { HTMLTokenizer } from './tokenizer.js'
export type { HTMLNodeType } from './types.js'
// Types and utilities
export {
  getAttribute,
  getAttributes,
  getCDATAValue,
  getCommentValue,
  getDoctypeName,
  getDoctypePublicId,
  getDoctypeSystemId,
  getTagName,
  getTextValue,
  isCDATANode,
  isCommentNode,
  isDoctypeNode,
  isDocumentNode,
  isElementNode,
  isSelfClosing,
  isTextNode,
  isVoidElement,
  VOID_ELEMENTS,
} from './types.js'

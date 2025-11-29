/**
 * @module @sylphx/synth-js
 *
 * High-performance JavaScript/TypeScript parser for Synth
 *
 * @packageDocumentation
 */

// Incremental tokenizer
export { IncrementalJavaScriptTokenizer } from './incremental-tokenizer.js'
export type { JSParseOptions } from './parser.js'
// Core parser
export { createParser, JSParser, parse, parseAsync } from './parser.js'
export type { JSNodeType } from './types.js'
// Types and utilities
export {
  findClasses,
  findExports,
  findFunctions,
  findIdentifiersByName,
  findImports,
  getFunctionName,
  getIdentifierName,
  getLiteralRaw,
  getLiteralValue,
  getOperator,
  getSourceType,
  getVariableKind,
  isArrowFunction,
  isAsync,
  isCallExpression,
  isClassDeclaration,
  isExportDeclaration,
  isExpression,
  isFunctionDeclaration,
  isFunctionExpression,
  isGenerator,
  isIdentifier,
  isImportDeclaration,
  isLiteral,
  isMemberExpression,
  isProgramNode,
  isStatement,
  isVariableDeclaration,
} from './types.js'

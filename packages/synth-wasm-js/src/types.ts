/**
 * AST Node types for JavaScript parser
 */

/**
 * Node types in the AST
 */
export enum NodeKind {
  // Program
  Program = 1,

  // Declarations
  VariableDeclaration = 2,
  VariableDeclarator = 3,
  FunctionDeclaration = 4,
  ClassDeclaration = 5,
  ImportDeclaration = 6,
  ExportDeclaration = 7,

  // Statements
  BlockStatement = 8,
  ExpressionStatement = 9,
  IfStatement = 10,
  ForStatement = 11,
  ForInStatement = 12,
  ForOfStatement = 13,
  WhileStatement = 14,
  DoWhileStatement = 15,
  SwitchStatement = 16,
  SwitchCase = 17,
  ReturnStatement = 18,
  ThrowStatement = 19,
  TryStatement = 20,
  CatchClause = 21,
  BreakStatement = 22,
  ContinueStatement = 23,
  EmptyStatement = 24,

  // Expressions
  Identifier = 25,
  Literal = 26,
  ArrayExpression = 27,
  ObjectExpression = 28,
  Property = 29,
  FunctionExpression = 30,
  ArrowFunctionExpression = 31,
  ClassExpression = 32,
  CallExpression = 33,
  NewExpression = 34,
  MemberExpression = 35,
  BinaryExpression = 36,
  UnaryExpression = 37,
  UpdateExpression = 38,
  AssignmentExpression = 39,
  LogicalExpression = 40,
  ConditionalExpression = 41,
  SequenceExpression = 42,
  SpreadElement = 43,
  TemplateLiteral = 44,
  TaggedTemplateExpression = 45,
  ThisExpression = 46,
  Super = 47,
  AwaitExpression = 48,
  YieldExpression = 49,

  // Patterns
  ArrayPattern = 50,
  ObjectPattern = 51,
  AssignmentPattern = 52,
  RestElement = 53,

  // Import/Export specifiers
  ImportSpecifier = 54,
  ImportDefaultSpecifier = 55,
  ImportNamespaceSpecifier = 56,
  ExportSpecifier = 57,

  // Class members
  MethodDefinition = 58,
  PropertyDefinition = 59,

  // Comments
  Comment = 60,
}

/**
 * Flags for AST nodes
 */
export const NodeFlags = {
  CONST: 1 << 0,
  LET: 1 << 1,
  ASYNC: 1 << 2,
  GENERATOR: 1 << 3,
  COMPUTED: 1 << 4,
  SHORTHAND: 1 << 5,
  STATIC: 1 << 6,
  EXPORT_DEFAULT: 1 << 7,
} as const

/**
 * Compact AST node (16 bytes in binary format)
 */
export interface ASTNode {
  kind: NodeKind
  flags: number
  start: number
  end: number
  extra: number
}

/**
 * Parse result with nodes array
 */
export interface ParseResult {
  nodes: ASTNode[]
  source: string
}

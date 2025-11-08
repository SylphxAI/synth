/**
 * Base error class for all Synth errors
 */
export class SynthError extends Error {
  constructor(
    message: string,
    public readonly code?: string
  ) {
    super(message)
    this.name = this.constructor.name
    Error.captureStackTrace?.(this, this.constructor)
  }
}

/**
 * Thrown when an index operation is attempted before building the index
 */
export class IndexNotBuiltError extends SynthError {
  constructor() {
    super('Index not built. Call build() first.', 'INDEX_NOT_BUILT')
  }
}

/**
 * Thrown when an invalid node type is encountered
 */
export class InvalidNodeTypeError extends SynthError {
  constructor(type: string, expected?: string[]) {
    const msg = expected
      ? `Invalid node type '${type}'. Expected one of: ${expected.join(', ')}`
      : `Invalid node type: ${type}`
    super(msg, 'INVALID_NODE_TYPE')
  }
}

/**
 * Thrown when an invalid node ID is referenced
 */
export class InvalidNodeIdError extends SynthError {
  constructor(id: number) {
    super(`Invalid node ID: ${id}`, 'INVALID_NODE_ID')
  }
}

/**
 * Thrown when tree structure constraints are violated
 */
export class TreeStructureError extends SynthError {
  constructor(message: string) {
    super(message, 'TREE_STRUCTURE_ERROR')
  }
}

/**
 * Thrown when a parsing error occurs
 */
export class ParseError extends SynthError {
  constructor(
    message: string,
    public readonly line?: number,
    public readonly column?: number
  ) {
    super(message, 'PARSE_ERROR')
  }
}

/**
 * Type system types and interfaces
 */

import type { NodeId } from '@sylphx/synth'

/**
 * Type kinds
 */
export type TypeKind =
  | 'any'
  | 'unknown'
  | 'never'
  | 'void'
  | 'null'
  | 'undefined'
  | 'boolean'
  | 'number'
  | 'string'
  | 'symbol'
  | 'bigint'
  | 'object'
  | 'array'
  | 'function'
  | 'class'
  | 'interface'
  | 'union'
  | 'intersection'
  | 'tuple'
  | 'literal'
  | 'generic'
  | 'reference'

/**
 * Type representation
 */
export interface Type {
  /** Type kind */
  kind: TypeKind

  /** Type name (for named types) */
  name?: string

  /** Element type (for arrays) */
  elementType?: Type

  /** Parameter types (for functions) */
  parameterTypes?: Type[]

  /** Return type (for functions) */
  returnType?: Type

  /** Property types (for objects) */
  properties?: Map<string, Type>

  /** Union types */
  types?: Type[]

  /** Literal value */
  value?: unknown

  /** Generic type parameters */
  typeParameters?: Type[]

  /** Whether type is nullable */
  nullable?: boolean

  /** Whether type is optional */
  optional?: boolean
}

/**
 * Type error
 */
export interface TypeError {
  /** Error message */
  message: string

  /** Node ID where error occurred */
  nodeId?: NodeId

  /** Expected type */
  expected?: Type

  /** Actual type */
  actual?: Type

  /** Location info */
  location?: {
    line: number
    column: number
  }
}

/**
 * Type check result
 */
export interface TypeCheckResult {
  /** Whether type checking passed */
  success: boolean

  /** Type errors found */
  errors: TypeError[]

  /** Inferred types for nodes */
  types: Map<NodeId, Type>
}

/**
 * Type environment (symbol table)
 */
export interface TypeEnvironment {
  /** Parent environment (for scope chain) */
  parent?: TypeEnvironment

  /** Variable types */
  variables: Map<string, Type>

  /** Function types */
  functions: Map<string, Type>

  /** Class types */
  classes: Map<string, Type>
}

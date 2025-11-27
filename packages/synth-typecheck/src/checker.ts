/**
 * Type checker implementation
 */

import type { BaseNode, NodeId, Tree } from '@sylphx/synth'
import type { Type, TypeCheckResult, TypeEnvironment, TypeError } from './types.js'

/**
 * Type checker
 */
export class TypeChecker {
  private errors: TypeError[] = []
  private types: Map<NodeId, Type> = new Map()
  private env: TypeEnvironment = {
    variables: new Map(),
    functions: new Map(),
    classes: new Map(),
  }

  /**
   * Check types in a tree
   */
  check(tree: Tree): TypeCheckResult {
    this.errors = []
    this.types = new Map()
    this.env = this.createGlobalEnvironment()

    // Traverse and infer types
    this.inferTypes(tree, tree.root)

    return {
      success: this.errors.length === 0,
      errors: this.errors,
      types: this.types,
    }
  }

  /**
   * Get inferred type for a node
   */
  getType(nodeId: NodeId): Type | undefined {
    return this.types.get(nodeId)
  }

  /**
   * Create global type environment with built-in types
   */
  private createGlobalEnvironment(): TypeEnvironment {
    const env: TypeEnvironment = {
      variables: new Map(),
      functions: new Map(),
      classes: new Map(),
    }

    // Built-in types
    env.variables.set('undefined', { kind: 'undefined' })
    env.variables.set('null', { kind: 'null' })

    // Built-in functions (common across languages)
    env.functions.set('console.log', {
      kind: 'function',
      parameterTypes: [{ kind: 'any' }],
      returnType: { kind: 'void' },
    })

    return env
  }

  /**
   * Infer types for a node
   */
  private inferTypes(tree: Tree, nodeId: NodeId): Type {
    const node = tree.nodes[nodeId]
    if (!node) {
      return { kind: 'unknown' }
    }

    // Check if already inferred
    const existing = this.types.get(nodeId)
    if (existing) {
      return existing
    }

    // Infer based on node type
    let type: Type

    switch (node.type) {
      // Literals
      case 'NumericLiteral':
      case 'NumberLiteral':
        type = { kind: 'number', value: node.data?.value }
        break

      case 'StringLiteral':
        type = { kind: 'string', value: node.data?.value }
        break

      case 'BooleanLiteral':
        type = { kind: 'boolean', value: node.data?.value }
        break

      case 'NullLiteral':
        type = { kind: 'null' }
        break

      // Identifiers
      case 'Identifier':
        type = this.inferIdentifier(node)
        break

      // Variable declarations
      case 'VariableDeclarator':
        type = this.inferVariableDeclarator(tree, node)
        break

      // Function declarations
      case 'FunctionDeclaration':
      case 'FunctionExpression':
      case 'ArrowFunctionExpression':
        type = this.inferFunction(tree, node)
        break

      // Binary operations
      case 'BinaryExpression':
        type = this.inferBinaryExpression(tree, node)
        break

      // Unary operations
      case 'UnaryExpression':
        type = this.inferUnaryExpression(tree, node)
        break

      // Logical operations
      case 'LogicalExpression':
        type = this.inferLogicalExpression(tree, node)
        break

      // Call expressions
      case 'CallExpression':
        type = this.inferCallExpression(tree, node)
        break

      // Array expressions
      case 'ArrayExpression':
        type = this.inferArrayExpression(tree, node)
        break

      // Object expressions
      case 'ObjectExpression':
        type = this.inferObjectExpression(tree, node)
        break

      // Member expressions
      case 'MemberExpression':
        type = this.inferMemberExpression(tree, node)
        break

      // Assignment expressions
      case 'AssignmentExpression':
        type = this.inferAssignmentExpression(tree, node)
        break

      default:
        type = { kind: 'unknown' }
    }

    this.types.set(nodeId, type)

    // Recurse through children
    for (const childId of node.children) {
      this.inferTypes(tree, childId)
    }

    return type
  }

  /**
   * Infer type for identifier
   */
  private inferIdentifier(node: BaseNode): Type {
    const name = node.data?.name
    if (!name) {
      return { kind: 'unknown' }
    }

    // Look up in environment
    const varType = this.env.variables.get(String(name))
    if (varType) {
      return varType
    }

    const fnType = this.env.functions.get(String(name))
    if (fnType) {
      return fnType
    }

    return { kind: 'unknown' }
  }

  /**
   * Infer type for variable declarator
   */
  private inferVariableDeclarator(tree: Tree, node: BaseNode): Type {
    const id = node.data?.id as { name?: string } | undefined
    const name = id?.name
    const initId = node.children.find((childId) => {
      const child = tree.nodes[childId]
      return child && child.type !== 'Identifier'
    })

    let type: Type = { kind: 'unknown' }

    if (initId) {
      // Infer from initializer
      type = this.inferTypes(tree, initId)
    }

    // Store in environment
    if (name) {
      this.env.variables.set(String(name), type)
    }

    return type
  }

  /**
   * Infer type for function
   */
  private inferFunction(_tree: Tree, node: BaseNode): Type {
    const id = node.data?.id as { name?: string } | undefined
    const name = node.data?.name || id?.name
    const params = (node.data?.params || []) as unknown[]

    const parameterTypes: Type[] = params.map(() => ({ kind: 'any' as const }))
    const returnType: Type = { kind: 'any' } // Would need return statement analysis

    const fnType: Type = {
      kind: 'function',
      parameterTypes,
      returnType,
    }

    if (name) {
      this.env.functions.set(String(name), fnType)
    }

    return fnType
  }

  /**
   * Infer type for binary expression
   */
  private inferBinaryExpression(tree: Tree, node: BaseNode): Type {
    const operator = node.data?.operator

    // Get operand types
    const [leftId, rightId] = node.children
    const leftType = leftId ? this.inferTypes(tree, leftId) : { kind: 'unknown' }
    const rightType = rightId ? this.inferTypes(tree, rightId) : { kind: 'unknown' }

    // Arithmetic operators
    if (['+', '-', '*', '/', '%', '**'].includes(String(operator))) {
      // String concatenation
      if (operator === '+' && (leftType.kind === 'string' || rightType.kind === 'string')) {
        return { kind: 'string' }
      }
      return { kind: 'number' }
    }

    // Comparison operators
    if (['<', '>', '<=', '>=', '==', '===', '!=', '!=='].includes(String(operator))) {
      return { kind: 'boolean' }
    }

    return { kind: 'unknown' }
  }

  /**
   * Infer type for unary expression
   */
  private inferUnaryExpression(_tree: Tree, node: BaseNode): Type {
    const operator = node.data?.operator

    if (operator === '!') {
      return { kind: 'boolean' }
    }

    if (['+', '-', '~'].includes(String(operator))) {
      return { kind: 'number' }
    }

    if (operator === 'typeof') {
      return { kind: 'string' }
    }

    return { kind: 'unknown' }
  }

  /**
   * Infer type for logical expression
   */
  private inferLogicalExpression(_tree: Tree, node: BaseNode): Type {
    const operator = node.data?.operator

    if (operator === '&&' || operator === '||') {
      return { kind: 'boolean' }
    }

    return { kind: 'unknown' }
  }

  /**
   * Infer type for call expression
   */
  private inferCallExpression(tree: Tree, node: BaseNode): Type {
    // Get callee
    const calleeId = node.children[0]
    if (!calleeId) {
      return { kind: 'unknown' }
    }

    const calleeType = this.inferTypes(tree, calleeId)

    if (calleeType.kind === 'function' && calleeType.returnType) {
      return calleeType.returnType
    }

    return { kind: 'unknown' }
  }

  /**
   * Infer type for array expression
   */
  private inferArrayExpression(tree: Tree, node: BaseNode): Type {
    if (node.children.length === 0) {
      return {
        kind: 'array',
        elementType: { kind: 'unknown' },
      }
    }

    // Infer element type from first element
    const firstType = this.inferTypes(tree, node.children[0]!)

    return {
      kind: 'array',
      elementType: firstType,
    }
  }

  /**
   * Infer type for object expression
   */
  private inferObjectExpression(tree: Tree, node: BaseNode): Type {
    const properties = new Map<string, Type>()

    for (const childId of node.children) {
      const child = tree.nodes[childId]
      if (!child) continue

      if (child.type === 'Property' || child.type === 'ObjectProperty') {
        const keyData = child.data?.key as { name?: string } | string | undefined
        const key = typeof keyData === 'object' ? keyData?.name : keyData
        if (key) {
          const valueId = child.children[0]
          const valueType: Type = valueId ? this.inferTypes(tree, valueId) : { kind: 'unknown' }
          properties.set(String(key), valueType)
        }
      }
    }

    return {
      kind: 'object',
      properties,
    }
  }

  /**
   * Infer type for member expression
   */
  private inferMemberExpression(tree: Tree, node: BaseNode): Type {
    const [objectId, _propertyId] = node.children
    if (!objectId) {
      return { kind: 'unknown' }
    }

    const objectType = this.inferTypes(tree, objectId)

    if (objectType.kind === 'object' && objectType.properties) {
      const propData = node.data?.property as { name?: string } | string | undefined
      const property = typeof propData === 'object' ? propData?.name : propData
      if (property) {
        const propType = objectType.properties.get(String(property))
        if (propType) {
          return propType
        }
      }
    }

    const propName = (node.data?.property as { name?: string } | undefined)?.name
    if (objectType.kind === 'array' && propName === 'length') {
      return { kind: 'number' }
    }

    return { kind: 'unknown' }
  }

  /**
   * Infer type for assignment expression
   */
  private inferAssignmentExpression(tree: Tree, node: BaseNode): Type {
    const [, rightId] = node.children
    if (!rightId) {
      return { kind: 'unknown' }
    }

    return this.inferTypes(tree, rightId)
  }

  /**
   * Check if two types are compatible
   */
  isCompatible(type1: Type, type2: Type): boolean {
    // any is compatible with everything
    if (type1.kind === 'any' || type2.kind === 'any') {
      return true
    }

    // unknown is compatible with everything
    if (type1.kind === 'unknown' || type2.kind === 'unknown') {
      return true
    }

    // Same kind
    if (type1.kind === type2.kind) {
      // For arrays, check element types
      if (type1.kind === 'array' && type2.kind === 'array') {
        if (type1.elementType && type2.elementType) {
          return this.isCompatible(type1.elementType, type2.elementType)
        }
      }

      return true
    }

    return false
  }

  /**
   * Report a type error
   */
  private reportError(message: string, nodeId?: NodeId, expected?: Type, actual?: Type): void {
    this.errors.push({
      message,
      nodeId,
      expected,
      actual,
    })
  }
}

/**
 * Create a new type checker
 */
export function createChecker(): TypeChecker {
  return new TypeChecker()
}

/**
 * Check types in a tree
 */
export function check(tree: Tree): TypeCheckResult {
  const checker = new TypeChecker()
  return checker.check(tree)
}

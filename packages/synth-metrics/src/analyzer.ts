/**
 * Code metrics analyzer
 */

import type { BaseNode, NodeId, Tree } from '@sylphx/synth'
import type {
  BasicMetrics,
  CodeMetrics,
  ComplexityMetrics,
  FunctionMetrics,
  HalsteadMetrics,
  MaintainabilityMetrics,
  MetricsReport,
} from './types.js'

/**
 * Metrics analyzer
 */
export class MetricsAnalyzer {
  /**
   * Analyze a tree and compute all metrics
   */
  analyze(tree: Tree): CodeMetrics {
    const basic = this.analyzeBasic(tree)
    const complexity = this.analyzeComplexity(tree)
    const halstead = this.analyzeHalstead(tree)
    const maintainability = this.calculateMaintainability(basic, complexity, halstead)

    return {
      basic,
      complexity,
      halstead,
      maintainability,
    }
  }

  /**
   * Generate a full metrics report
   */
  report(tree: Tree, file?: string): MetricsReport {
    const metrics = this.analyze(tree)
    const functions = this.analyzeFunctions(tree)

    return {
      file,
      language: tree.meta.language,
      metrics,
      functions,
      timestamp: Date.now(),
    }
  }

  /**
   * Analyze basic metrics
   */
  private analyzeBasic(tree: Tree): BasicMetrics {
    const source = tree.meta.source || ''
    const lines = source.split('\n')

    const loc = lines.length
    let blank = 0
    let cloc = 0

    for (const line of lines) {
      const trimmed = line.trim()
      if (trimmed === '') {
        blank++
      } else if (this.isCommentLine(trimmed)) {
        cloc++
      }
    }

    const sloc = loc - blank - cloc
    const nodes = tree.nodes.length

    const depths = this.calculateDepths(tree)
    const maxDepth = Math.max(...depths, 0)
    const avgDepth = depths.length > 0 ? depths.reduce((a, b) => a + b, 0) / depths.length : 0

    return {
      loc,
      sloc,
      cloc,
      blank,
      nodes,
      maxDepth,
      avgDepth,
    }
  }

  /**
   * Analyze complexity metrics
   */
  private analyzeComplexity(tree: Tree): ComplexityMetrics {
    const decisionPoints = this.countDecisionPoints(tree, tree.root)
    const cyclomatic = decisionPoints + 1
    const cognitive = this.calculateCognitiveComplexity(tree, tree.root, 0)

    return {
      cyclomatic,
      decisionPoints,
      cognitive,
    }
  }

  /**
   * Analyze Halstead metrics
   */
  private analyzeHalstead(tree: Tree): HalsteadMetrics {
    const operators = new Set<string>()
    const operands = new Set<string>()
    let totalOperators = 0
    let totalOperands = 0

    this.traverseForHalstead(tree, tree.root, operators, operands, (isOperator) => {
      if (isOperator) {
        totalOperators++
      } else {
        totalOperands++
      }
    })

    const n1 = operators.size
    const n2 = operands.size
    const N1 = totalOperators
    const N2 = totalOperands

    const vocabulary = n1 + n2
    const length = N1 + N2
    const calculatedLength = n1 * Math.log2(n1 || 1) + n2 * Math.log2(n2 || 1)
    const volume = length * Math.log2(vocabulary || 1)
    const difficulty = (n1 / 2) * (N2 / (n2 || 1))
    const effort = difficulty * volume
    const time = effort / 18 // seconds
    const bugs = effort ** (2 / 3) / 3000

    return {
      n1,
      n2,
      N1,
      N2,
      vocabulary,
      length,
      calculatedLength,
      volume,
      difficulty,
      effort,
      time,
      bugs,
    }
  }

  /**
   * Calculate maintainability index
   */
  private calculateMaintainability(
    basic: BasicMetrics,
    complexity: ComplexityMetrics,
    halstead: HalsteadMetrics
  ): MaintainabilityMetrics {
    // Maintainability Index = 171 - 5.2 * ln(V) - 0.23 * G - 16.2 * ln(LOC)
    // Where V = Halstead Volume, G = Cyclomatic Complexity, LOC = Lines of Code
    const V = halstead.volume || 1
    const G = complexity.cyclomatic
    const LOC = basic.loc || 1

    let index = 171 - 5.2 * Math.log(V) - 0.23 * G - 16.2 * Math.log(LOC)

    // Normalize to 0-100
    index = Math.max(0, Math.min(100, index))

    // Determine level
    let level: 'low' | 'moderate' | 'high' | 'very high'
    if (index >= 85) {
      level = 'low' // Low difficulty
    } else if (index >= 65) {
      level = 'moderate'
    } else if (index >= 20) {
      level = 'high'
    } else {
      level = 'very high'
    }

    const maintainable = index >= 20

    return {
      index,
      level,
      maintainable,
    }
  }

  /**
   * Analyze per-function metrics
   */
  private analyzeFunctions(tree: Tree): FunctionMetrics[] {
    const functions: FunctionMetrics[] = []

    this.traverse(tree, tree.root, (node) => {
      if (this.isFunctionNode(node)) {
        const metrics = this.analyzeFunctionNode(tree, node)
        if (metrics) {
          functions.push(metrics)
        }
      }
    })

    return functions
  }

  /**
   * Analyze a single function node
   */
  private analyzeFunctionNode(tree: Tree, node: BaseNode): FunctionMetrics | null {
    const name = this.getFunctionName(node)
    if (!name) return null

    const loc = this.getFunctionLOC(tree, node)
    const complexity = this.countDecisionPoints(tree, node.id) + 1
    const params = this.getFunctionParams(node)
    const maxDepth = this.getMaxDepth(tree, node.id)

    return {
      name,
      location: {
        start: node.span?.start || { line: 0, column: 0 },
        end: node.span?.end || { line: 0, column: 0 },
      },
      loc,
      complexity,
      params,
      maxDepth,
    }
  }

  /**
   * Check if node is a function
   */
  private isFunctionNode(node: BaseNode): boolean {
    const functionTypes = [
      'FunctionDeclaration',
      'FunctionExpression',
      'ArrowFunctionExpression',
      'MethodDefinition',
      'function_definition',
      'function_declaration',
      'method_declaration',
      'func_decl',
      'FuncDecl',
    ]

    return functionTypes.some((type) => node.type.toLowerCase().includes(type.toLowerCase()))
  }

  /**
   * Get function name
   */
  private getFunctionName(node: BaseNode): string {
    const data = node.data as Record<string, unknown> | undefined
    if (data?.name) return String(data.name)
    const id = data?.id as Record<string, unknown> | undefined
    if (id?.name) return String(id.name)
    return '<anonymous>'
  }

  /**
   * Get function parameters count
   */
  private getFunctionParams(node: BaseNode): number {
    if (node.data?.params && Array.isArray(node.data.params)) {
      return node.data.params.length
    }
    if (node.data?.parameters && Array.isArray(node.data.parameters)) {
      return node.data.parameters.length
    }
    return 0
  }

  /**
   * Get function lines of code
   */
  private getFunctionLOC(tree: Tree, node: BaseNode): number {
    if (!node.span || !tree.meta.source) return 0
    const source = tree.meta.source.slice(node.span.start.offset, node.span.end.offset)
    return source.split('\n').length
  }

  /**
   * Count decision points (for cyclomatic complexity)
   */
  private countDecisionPoints(tree: Tree, nodeId: NodeId): number {
    let count = 0
    const node = tree.nodes[nodeId]
    if (!node) return 0

    // Decision point node types
    const decisionTypes = [
      'if',
      'else',
      'for',
      'while',
      'do',
      'switch',
      'case',
      'catch',
      'and',
      'or',
      '&&',
      '||',
      '?',
      'ConditionalExpression',
      'IfStatement',
      'ForStatement',
      'WhileStatement',
      'DoWhileStatement',
      'SwitchCase',
      'CatchClause',
      'LogicalExpression',
    ]

    if (decisionTypes.some((type) => node.type.toLowerCase().includes(type.toLowerCase()))) {
      count++
    }

    // Recurse through children
    for (const childId of node.children) {
      count += this.countDecisionPoints(tree, childId)
    }

    return count
  }

  /**
   * Calculate cognitive complexity (weighted by nesting)
   */
  private calculateCognitiveComplexity(tree: Tree, nodeId: NodeId, nesting: number): number {
    let complexity = 0
    const node = tree.nodes[nodeId]
    if (!node) return 0

    const isDecision = this.isDecisionNode(node)
    const isNesting = this.isNestingNode(node)

    if (isDecision) {
      complexity += 1 + nesting
    }

    const nextNesting = isNesting ? nesting + 1 : nesting

    for (const childId of node.children) {
      complexity += this.calculateCognitiveComplexity(tree, childId, nextNesting)
    }

    return complexity
  }

  /**
   * Check if node is a decision point
   */
  private isDecisionNode(node: BaseNode): boolean {
    const decisionTypes = ['if', 'switch', 'case', 'catch', '&&', '||', '?', 'for', 'while']
    return decisionTypes.some((type) => node.type.toLowerCase().includes(type.toLowerCase()))
  }

  /**
   * Check if node increases nesting
   */
  private isNestingNode(node: BaseNode): boolean {
    const nestingTypes = ['if', 'for', 'while', 'do', 'switch', 'function', 'method', 'class']
    return nestingTypes.some((type) => node.type.toLowerCase().includes(type.toLowerCase()))
  }

  /**
   * Calculate depths of all nodes
   */
  private calculateDepths(tree: Tree): number[] {
    const depths: number[] = []

    const traverse = (nodeId: NodeId, depth: number) => {
      const node = tree.nodes[nodeId]
      if (!node) return

      depths.push(depth)

      for (const childId of node.children) {
        traverse(childId, depth + 1)
      }
    }

    traverse(tree.root, 0)
    return depths
  }

  /**
   * Get maximum depth from a node
   */
  private getMaxDepth(tree: Tree, nodeId: NodeId): number {
    const node = tree.nodes[nodeId]
    if (!node || node.children.length === 0) return 0

    let maxDepth = 0
    for (const childId of node.children) {
      const childDepth = this.getMaxDepth(tree, childId)
      maxDepth = Math.max(maxDepth, childDepth)
    }

    return maxDepth + 1
  }

  /**
   * Check if a line is a comment
   */
  private isCommentLine(line: string): boolean {
    return (
      line.startsWith('//') ||
      line.startsWith('#') ||
      line.startsWith('/*') ||
      line.startsWith('*') ||
      line.startsWith('<!--')
    )
  }

  /**
   * Traverse tree for Halstead metrics
   */
  private traverseForHalstead(
    tree: Tree,
    nodeId: NodeId,
    operators: Set<string>,
    operands: Set<string>,
    callback: (isOperator: boolean) => void
  ): void {
    const node = tree.nodes[nodeId]
    if (!node) return

    // Classify node as operator or operand
    if (this.isOperatorNode(node)) {
      operators.add(node.type)
      callback(true)
    } else if (this.isOperandNode(node)) {
      const value = this.getNodeValue(node)
      if (value) {
        operands.add(value)
        callback(false)
      }
    }

    // Recurse
    for (const childId of node.children) {
      this.traverseForHalstead(tree, childId, operators, operands, callback)
    }
  }

  /**
   * Check if node is an operator
   */
  private isOperatorNode(node: BaseNode): boolean {
    const operatorTypes = [
      'BinaryExpression',
      'UnaryExpression',
      'LogicalExpression',
      'AssignmentExpression',
      'UpdateExpression',
      'CallExpression',
      'MemberExpression',
    ]

    return operatorTypes.some((type) => node.type.includes(type))
  }

  /**
   * Check if node is an operand
   */
  private isOperandNode(node: BaseNode): boolean {
    const operandTypes = [
      'Identifier',
      'Literal',
      'NumericLiteral',
      'StringLiteral',
      'BooleanLiteral',
    ]

    return operandTypes.some((type) => node.type.includes(type))
  }

  /**
   * Get node value for operands
   */
  private getNodeValue(node: BaseNode): string {
    if (node.data?.value !== undefined) {
      return String(node.data.value)
    }
    if (node.data?.name) {
      return String(node.data.name)
    }
    return node.type
  }

  /**
   * Generic tree traversal
   */
  private traverse(tree: Tree, nodeId: NodeId, callback: (node: BaseNode) => void): void {
    const node = tree.nodes[nodeId]
    if (!node) return

    callback(node)

    for (const childId of node.children) {
      this.traverse(tree, childId, callback)
    }
  }
}

/**
 * Create a new metrics analyzer
 */
export function createAnalyzer(): MetricsAnalyzer {
  return new MetricsAnalyzer()
}

/**
 * Analyze a tree and return metrics
 */
export function analyze(tree: Tree): CodeMetrics {
  const analyzer = new MetricsAnalyzer()
  return analyzer.analyze(tree)
}

/**
 * Generate a metrics report
 */
export function report(tree: Tree, file?: string): MetricsReport {
  const analyzer = new MetricsAnalyzer()
  return analyzer.report(tree, file)
}

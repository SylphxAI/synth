/**
 * Metrics types and interfaces
 */

/**
 * Basic code metrics
 */
export interface BasicMetrics {
  /** Total lines of code */
  loc: number

  /** Source lines of code (non-comment, non-blank) */
  sloc: number

  /** Comment lines */
  cloc: number

  /** Blank lines */
  blank: number

  /** Total nodes in AST */
  nodes: number

  /** Maximum depth of nesting */
  maxDepth: number

  /** Average depth of nodes */
  avgDepth: number
}

/**
 * Cyclomatic complexity metrics
 */
export interface ComplexityMetrics {
  /** Cyclomatic complexity (number of decision points + 1) */
  cyclomatic: number

  /** Number of decision points */
  decisionPoints: number

  /** Cognitive complexity (weighted decision points) */
  cognitive: number
}

/**
 * Halstead complexity metrics
 */
export interface HalsteadMetrics {
  /** Number of distinct operators */
  n1: number

  /** Number of distinct operands */
  n2: number

  /** Total number of operators */
  N1: number

  /** Total number of operands */
  N2: number

  /** Program vocabulary (n1 + n2) */
  vocabulary: number

  /** Program length (N1 + N2) */
  length: number

  /** Calculated program length */
  calculatedLength: number

  /** Volume */
  volume: number

  /** Difficulty */
  difficulty: number

  /** Effort */
  effort: number

  /** Time to program (seconds) */
  time: number

  /** Estimated bugs */
  bugs: number
}

/**
 * Maintainability metrics
 */
export interface MaintainabilityMetrics {
  /** Maintainability index (0-100, higher is better) */
  index: number

  /** Difficulty level based on index */
  level: 'low' | 'moderate' | 'high' | 'very high'

  /** Whether code is considered maintainable (index >= 20) */
  maintainable: boolean
}

/**
 * Complete code metrics
 */
export interface CodeMetrics {
  /** Basic metrics */
  basic: BasicMetrics

  /** Complexity metrics */
  complexity: ComplexityMetrics

  /** Halstead metrics */
  halstead: HalsteadMetrics

  /** Maintainability metrics */
  maintainability: MaintainabilityMetrics
}

/**
 * Function-level metrics
 */
export interface FunctionMetrics {
  /** Function name */
  name: string

  /** Function location */
  location: {
    start: { line: number; column: number }
    end: { line: number; column: number }
  }

  /** Lines of code in function */
  loc: number

  /** Cyclomatic complexity */
  complexity: number

  /** Number of parameters */
  params: number

  /** Maximum nesting depth */
  maxDepth: number
}

/**
 * File-level metrics report
 */
export interface MetricsReport {
  /** File path or name */
  file?: string

  /** Language */
  language: string

  /** Overall metrics */
  metrics: CodeMetrics

  /** Per-function metrics */
  functions: FunctionMetrics[]

  /** Timestamp */
  timestamp: number
}

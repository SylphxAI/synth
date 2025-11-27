import { describe, expect, it } from 'bun:test'
import { addNode, createTree } from '@sylphx/synth'
import { MetricsAnalyzer, analyze, createAnalyzer, report } from './analyzer.js'

describe('MetricsAnalyzer', () => {
  it('should create an analyzer instance', () => {
    const analyzer = new MetricsAnalyzer()
    expect(analyzer).toBeInstanceOf(MetricsAnalyzer)
  })

  it('should create analyzer with factory', () => {
    const analyzer = createAnalyzer()
    expect(analyzer).toBeInstanceOf(MetricsAnalyzer)
  })

  it('should analyze basic metrics for empty tree', () => {
    const tree = createTree('test', '')
    const metrics = analyze(tree)

    expect(metrics.basic).toBeDefined()
    expect(metrics.basic.loc).toBe(1) // Empty string = 1 line
    expect(metrics.basic.nodes).toBeGreaterThan(0) // At least root node
  })

  it('should count lines of code', () => {
    const source = `line 1
line 2
line 3`
    const tree = createTree('test', source)
    const metrics = analyze(tree)

    expect(metrics.basic.loc).toBe(3)
  })

  it('should count blank lines', () => {
    const source = `line 1

line 3

line 5`
    const tree = createTree('test', source)
    const metrics = analyze(tree)

    expect(metrics.basic.blank).toBe(2)
  })

  it('should count comment lines', () => {
    const source = `// comment 1
code line
// comment 2
# comment 3`
    const tree = createTree('javascript', source)
    const metrics = analyze(tree)

    expect(metrics.basic.cloc).toBeGreaterThan(0)
  })

  it('should calculate SLOC', () => {
    const source = `line 1

// comment
line 4`
    const tree = createTree('javascript', source)
    const metrics = analyze(tree)

    expect(metrics.basic.sloc).toBe(2) // Only line 1 and line 4
  })

  it('should count total nodes', () => {
    const tree = createTree('test', 'test')

    // Add some nodes
    const node1 = addNode(tree, {
      type: 'test',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(node1)

    const node2 = addNode(tree, {
      type: 'test',
      parent: node1,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[node1]?.children.push(node2)

    const metrics = analyze(tree)

    expect(metrics.basic.nodes).toBe(3) // root + 2 added
  })

  it('should calculate max depth', () => {
    const tree = createTree('test', '')

    // Create nested structure
    let parentId = tree.root
    for (let i = 0; i < 5; i++) {
      const nodeId = addNode(tree, {
        type: 'block',
        parent: parentId,
        children: [],
        span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
        data: {},
      })
      tree.nodes[parentId]?.children.push(nodeId)
      parentId = nodeId
    }

    const metrics = analyze(tree)

    expect(metrics.basic.maxDepth).toBe(5)
  })

  it('should calculate average depth', () => {
    const tree = createTree('test', '')

    // Add nodes at different depths
    const node1 = addNode(tree, {
      type: 'test',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(node1)

    const node2 = addNode(tree, {
      type: 'test',
      parent: node1,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[node1]?.children.push(node2)

    const metrics = analyze(tree)

    expect(metrics.basic.avgDepth).toBeGreaterThan(0)
  })

  it('should calculate cyclomatic complexity', () => {
    const tree = createTree('javascript', 'if (x) { }')

    // Add if statement (decision point)
    const ifStmt = addNode(tree, {
      type: 'IfStatement',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(ifStmt)

    const metrics = analyze(tree)

    expect(metrics.complexity.cyclomatic).toBeGreaterThan(1)
    expect(metrics.complexity.decisionPoints).toBeGreaterThan(0)
  })

  it('should count decision points', () => {
    const tree = createTree('javascript', 'if (x && y || z) { }')

    // Add decision nodes
    const ifStmt = addNode(tree, {
      type: 'IfStatement',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(ifStmt)

    const andExpr = addNode(tree, {
      type: 'LogicalExpression',
      parent: ifStmt,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: { operator: '&&' },
    })
    tree.nodes[ifStmt]?.children.push(andExpr)

    const orExpr = addNode(tree, {
      type: 'LogicalExpression',
      parent: ifStmt,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: { operator: '||' },
    })
    tree.nodes[ifStmt]?.children.push(orExpr)

    const metrics = analyze(tree)

    expect(metrics.complexity.decisionPoints).toBeGreaterThan(0)
  })

  it('should calculate cognitive complexity', () => {
    const tree = createTree('javascript', '')

    // Nested if statements increase cognitive complexity more
    const if1 = addNode(tree, {
      type: 'IfStatement',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(if1)

    const if2 = addNode(tree, {
      type: 'IfStatement',
      parent: if1,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[if1]?.children.push(if2)

    const metrics = analyze(tree)

    // Cognitive complexity should account for nesting
    expect(metrics.complexity.cognitive).toBeGreaterThanOrEqual(metrics.complexity.cyclomatic)
  })

  it('should calculate Halstead metrics', () => {
    const tree = createTree('javascript', '')
    const metrics = analyze(tree)

    expect(metrics.halstead).toBeDefined()
    expect(metrics.halstead.vocabulary).toBeGreaterThanOrEqual(0)
    expect(metrics.halstead.length).toBeGreaterThanOrEqual(0)
    expect(metrics.halstead.volume).toBeGreaterThanOrEqual(0)
  })

  it('should calculate maintainability index', () => {
    const tree = createTree('javascript', 'function test() { return 42; }')
    const metrics = analyze(tree)

    expect(metrics.maintainability).toBeDefined()
    expect(metrics.maintainability.index).toBeGreaterThanOrEqual(0)
    expect(metrics.maintainability.index).toBeLessThanOrEqual(100)
    expect(metrics.maintainability.level).toBeDefined()
    expect(typeof metrics.maintainability.maintainable).toBe('boolean')
  })

  it('should classify maintainability level correctly', () => {
    // Simple code should have high maintainability
    const simple = createTree('javascript', 'const x = 42;')
    const simpleMetrics = analyze(simple)

    expect(simpleMetrics.maintainability.index).toBeGreaterThan(0)
  })

  it('should analyze function metrics', () => {
    const source = 'function test() { return 42; }'
    const tree = createTree('javascript', source)

    // Add function node
    const fn = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 30, line: 1, column: 30 },
      },
      data: {
        name: 'test',
        params: [],
      },
    })
    tree.nodes[tree.root]?.children.push(fn)

    const metrics = report(tree)

    expect(metrics.functions).toBeDefined()
    expect(metrics.functions.length).toBeGreaterThan(0)
    expect(metrics.functions[0].name).toBe('test')
  })

  it('should count function parameters', () => {
    const tree = createTree('javascript', '')

    const fn = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {
        name: 'test',
        params: ['a', 'b', 'c'],
      },
    })
    tree.nodes[tree.root]?.children.push(fn)

    const metrics = report(tree)

    expect(metrics.functions[0].params).toBe(3)
  })

  it('should calculate function LOC', () => {
    const source = `function test() {
  return 42;
}`
    const tree = createTree('javascript', source)

    const fn = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: source.length, line: 3, column: 1 },
      },
      data: { name: 'test' },
    })
    tree.nodes[tree.root]?.children.push(fn)

    const metrics = report(tree)

    expect(metrics.functions[0].loc).toBe(3)
  })

  it('should calculate function complexity', () => {
    const tree = createTree('javascript', '')

    const fn = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: { name: 'test' },
    })
    tree.nodes[tree.root]?.children.push(fn)

    // Add if statement inside function
    const ifStmt = addNode(tree, {
      type: 'IfStatement',
      parent: fn,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[fn]?.children.push(ifStmt)

    const metrics = report(tree)

    expect(metrics.functions[0].complexity).toBeGreaterThan(1)
  })

  it('should handle anonymous functions', () => {
    const tree = createTree('javascript', '')

    const fn = addNode(tree, {
      type: 'FunctionExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 0, line: 1, column: 0 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(fn)

    const metrics = report(tree)

    expect(metrics.functions[0].name).toBe('<anonymous>')
  })

  it('should use standalone analyze function', () => {
    const tree = createTree('test', 'test')
    const metrics = analyze(tree)

    expect(metrics).toBeDefined()
    expect(metrics.basic).toBeDefined()
    expect(metrics.complexity).toBeDefined()
    expect(metrics.halstead).toBeDefined()
    expect(metrics.maintainability).toBeDefined()
  })

  it('should use standalone report function', () => {
    const tree = createTree('javascript', 'const x = 42;')
    const metrics = report(tree, 'test.js')

    expect(metrics).toBeDefined()
    expect(metrics.file).toBe('test.js')
    expect(metrics.language).toBe('javascript')
    expect(metrics.metrics).toBeDefined()
    expect(metrics.functions).toBeDefined()
    expect(metrics.timestamp).toBeGreaterThan(0)
  })

  it('should handle trees with no source', () => {
    const tree = createTree('test')
    const metrics = analyze(tree)

    expect(metrics.basic.loc).toBe(1)
    expect(metrics.basic.sloc).toBe(0) // Empty source = 0 SLOC (1 blank line)
  })

  it('should calculate all metrics together', () => {
    const source = `function fibonacci(n) {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}`
    const tree = createTree('javascript', source)
    const metrics = analyze(tree)

    expect(metrics.basic.loc).toBe(6)
    expect(metrics.complexity.cyclomatic).toBeGreaterThanOrEqual(1)
    expect(metrics.halstead.volume).toBeGreaterThanOrEqual(0)
    expect(metrics.maintainability.index).toBeGreaterThan(0)
  })

  it('should generate complete report', () => {
    const tree = createTree('javascript', 'function test() { }')
    const metrics = report(tree, 'example.js')

    expect(metrics.file).toBe('example.js')
    expect(metrics.language).toBe('javascript')
    expect(metrics.timestamp).toBeGreaterThan(0)

    // Check all metric categories
    expect(metrics.metrics.basic).toBeDefined()
    expect(metrics.metrics.complexity).toBeDefined()
    expect(metrics.metrics.halstead).toBeDefined()
    expect(metrics.metrics.maintainability).toBeDefined()

    // Check functions array
    expect(Array.isArray(metrics.functions)).toBe(true)
  })
})

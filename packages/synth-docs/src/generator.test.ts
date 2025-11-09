import { describe, it, expect } from 'bun:test'
import { DocGenerator, createGenerator, generate } from './generator.js'
import { createTree, addNode } from '@sylphx/synth'

describe('DocGenerator', () => {
  it('should create a documentation generator', () => {
    const generator = new DocGenerator()
    expect(generator).toBeInstanceOf(DocGenerator)
  })

  it('should create generator with factory', () => {
    const generator = createGenerator()
    expect(generator).toBeInstanceOf(DocGenerator)
  })

  it('should generate docs for empty tree', () => {
    const tree = createTree('javascript', '')
    const result = generate(tree)

    expect(result).toBeDefined()
    expect(result.module).toBeDefined()
    expect(result.output).toBeDefined()
    expect(result.format).toBe('markdown')
  })

  it('should extract function documentation', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'add',
        params: ['a', 'b'],
        comment: '/**\n * Adds two numbers\n * @param {number} a - First number\n * @param {number} b - Second number\n * @returns {number} Sum of a and b\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree)

    expect(result.module.symbols.length).toBe(1)
    expect(result.module.symbols[0].name).toBe('add')
    expect(result.module.symbols[0].kind).toBe('function')
    expect(result.module.symbols[0].description).toContain('Adds two numbers')
    expect(result.module.symbols[0].params).toBeDefined()
    expect(result.module.symbols[0].params!.length).toBe(2)
    expect(result.module.symbols[0].returns).toBeDefined()
  })

  it('should extract class documentation', () => {
    const tree = createTree('javascript', '')

    const classId = addNode(tree, {
      type: 'ClassDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'User',
        comment: '/**\n * User class\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(classId)

    const result = generate(tree)

    expect(result.module.symbols.length).toBe(1)
    expect(result.module.symbols[0].name).toBe('User')
    expect(result.module.symbols[0].kind).toBe('class')
  })

  it('should extract variable documentation', () => {
    const tree = createTree('javascript', '')

    const varId = addNode(tree, {
      type: 'VariableDeclarator',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 20, line: 1, column: 20 },
      },
      data: {
        id: { name: 'count' },
        comment: '/** Counter variable */',
      },
    })
    tree.nodes[tree.root]!.children.push(varId)

    const result = generate(tree)

    expect(result.module.symbols.length).toBe(1)
    expect(result.module.symbols[0].name).toBe('count')
    expect(result.module.symbols[0].kind).toBe('variable')
  })

  it('should extract constant documentation', () => {
    const tree = createTree('javascript', '')

    const constId = addNode(tree, {
      type: 'VariableDeclarator',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 20, line: 1, column: 20 },
      },
      data: {
        id: { name: 'MAX_SIZE' },
        kind: 'const',
        comment: '/** Maximum size */',
      },
    })
    tree.nodes[tree.root]!.children.push(constId)

    const result = generate(tree)

    expect(result.module.symbols[0].kind).toBe('constant')
  })

  it('should parse parameters from comment', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
        comment: '/**\n * @param {string} name - User name\n * @param {number} age - User age\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree)

    const params = result.module.symbols[0].params
    expect(params).toBeDefined()
    expect(params!.length).toBe(2)
    expect(params![0].name).toBe('name')
    expect(params![0].type).toBe('string')
    expect(params![1].name).toBe('age')
    expect(params![1].type).toBe('number')
  })

  it('should parse return value from comment', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
        comment: '/**\n * @returns {boolean} Success status\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree)

    const returns = result.module.symbols[0].returns
    expect(returns).toBeDefined()
    expect(returns!.type).toBe('boolean')
    expect(returns!.description).toContain('Success status')
  })

  it('should parse examples from comment', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
        comment: '/**\n * @example\n * test()\n * // => true\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree)

    const examples = result.module.symbols[0].examples
    expect(examples).toBeDefined()
    expect(examples!.length).toBeGreaterThan(0)
  })

  it('should handle exported symbols', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
        exported: true,
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree)

    expect(result.module.symbols[0].exported).toBe(true)
  })

  it('should handle async functions', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'fetchData',
        async: true,
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree)

    expect(result.module.symbols[0].async).toBe(true)
  })

  it('should generate markdown output', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
        comment: '/** Test function */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree, { format: 'markdown' })

    expect(result.format).toBe('markdown')
    expect(result.output).toContain('# Module')
    expect(result.output).toContain('## test')
  })

  it('should generate JSON output', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree, { format: 'json' })

    expect(result.format).toBe('json')
    const parsed = JSON.parse(result.output)
    expect(parsed.name).toBe('Module')
    expect(parsed.symbols.length).toBe(1)
  })

  it('should generate HTML output', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 50, line: 3, column: 1 },
      },
      data: {
        name: 'test',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree, { format: 'html' })

    expect(result.format).toBe('html')
    expect(result.output).toContain('<!DOCTYPE html>')
    expect(result.output).toContain('<h2>test</h2>')
  })

  it('should use custom title', () => {
    const tree = createTree('javascript', '')
    const result = generate(tree, { title: 'My API' })

    expect(result.output).toContain('# My API')
  })

  it('should include file path', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 10, column: 0 },
        end: { offset: 50, line: 12, column: 1 },
      },
      data: {
        name: 'test',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    const result = generate(tree, { file: 'example.js' })

    expect(result.module.file).toBe('example.js')
    expect(result.module.symbols[0].location?.file).toBe('example.js')
    expect(result.module.symbols[0].location?.line).toBe(10)
  })

  it('should use standalone generate function', () => {
    const tree = createTree('javascript', '')
    const result = generate(tree)

    expect(result).toBeDefined()
    expect(result.module).toBeDefined()
  })

  it('should handle multiple symbols', () => {
    const tree = createTree('javascript', '')

    const fn1 = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 20, line: 1, column: 20 },
      },
      data: { name: 'fn1' },
    })
    tree.nodes[tree.root]!.children.push(fn1)

    const fn2 = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 25, line: 2, column: 0 },
        end: { offset: 45, line: 2, column: 20 },
      },
      data: { name: 'fn2' },
    })
    tree.nodes[tree.root]!.children.push(fn2)

    const result = generate(tree)

    expect(result.module.symbols.length).toBe(2)
    expect(result.module.symbols[0].name).toBe('fn1')
    expect(result.module.symbols[1].name).toBe('fn2')
  })

  it('should filter private symbols', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 20, line: 1, column: 20 },
      },
      data: {
        name: 'privateFunc',
        comment: '/**\n * @access private\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    // Without includePrivate
    const result1 = generate(tree)
    expect(result1.module.symbols.length).toBe(0)

    // With includePrivate
    const result2 = generate(tree, { includePrivate: true })
    expect(result2.module.symbols.length).toBe(1)
  })

  it('should filter internal symbols', () => {
    const tree = createTree('javascript', '')

    const fnId = addNode(tree, {
      type: 'FunctionDeclaration',
      parent: tree.root,
      children: [],
      span: {
        start: { offset: 0, line: 1, column: 0 },
        end: { offset: 20, line: 1, column: 20 },
      },
      data: {
        name: 'internalFunc',
        comment: '/**\n * @internal\n */',
      },
    })
    tree.nodes[tree.root]!.children.push(fnId)

    // Without includeInternal
    const result1 = generate(tree)
    expect(result1.module.symbols.length).toBe(0)

    // With includeInternal
    const result2 = generate(tree, { includeInternal: true })
    expect(result2.module.symbols.length).toBe(1)
  })
})

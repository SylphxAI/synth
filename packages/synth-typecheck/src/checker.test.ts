import { describe, expect, it } from 'bun:test'
import { addNode, createTree } from '@sylphx/synth'
import { check, createChecker, TypeChecker } from './checker.js'

describe('TypeChecker', () => {
  it('should create a type checker', () => {
    const checker = new TypeChecker()
    expect(checker).toBeInstanceOf(TypeChecker)
  })

  it('should create checker with factory', () => {
    const checker = createChecker()
    expect(checker).toBeInstanceOf(TypeChecker)
  })

  it('should check an empty tree', () => {
    const tree = createTree('javascript', '')
    const result = check(tree)

    expect(result).toBeDefined()
    expect(result.success).toBe(true)
    expect(result.errors).toEqual([])
    expect(result.types).toBeInstanceOf(Map)
  })

  it('should infer number literal type', () => {
    const tree = createTree('javascript', '42')

    const numId = addNode(tree, {
      type: 'NumericLiteral',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 2, line: 1, column: 2 } },
      data: { value: 42 },
    })
    tree.nodes[tree.root]?.children.push(numId)

    const result = check(tree)

    expect(result.success).toBe(true)
    const type = result.types.get(numId)
    expect(type).toBeDefined()
    expect(type?.kind).toBe('number')
    expect(type?.value).toBe(42)
  })

  it('should infer string literal type', () => {
    const tree = createTree('javascript', '"hello"')

    const strId = addNode(tree, {
      type: 'StringLiteral',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 7, line: 1, column: 7 } },
      data: { value: 'hello' },
    })
    tree.nodes[tree.root]?.children.push(strId)

    const result = check(tree)

    const type = result.types.get(strId)
    expect(type?.kind).toBe('string')
    expect(type?.value).toBe('hello')
  })

  it('should infer boolean literal type', () => {
    const tree = createTree('javascript', 'true')

    const boolId = addNode(tree, {
      type: 'BooleanLiteral',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 4, line: 1, column: 4 } },
      data: { value: true },
    })
    tree.nodes[tree.root]?.children.push(boolId)

    const result = check(tree)

    const type = result.types.get(boolId)
    expect(type?.kind).toBe('boolean')
    expect(type?.value).toBe(true)
  })

  it('should infer null type', () => {
    const tree = createTree('javascript', 'null')

    const nullId = addNode(tree, {
      type: 'NullLiteral',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 4, line: 1, column: 4 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(nullId)

    const result = check(tree)

    const type = result.types.get(nullId)
    expect(type?.kind).toBe('null')
  })

  it('should infer array type', () => {
    const tree = createTree('javascript', '[1, 2, 3]')

    const arrayId = addNode(tree, {
      type: 'ArrayExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 9, line: 1, column: 9 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(arrayId)

    // Add elements
    const elem1 = addNode(tree, {
      type: 'NumericLiteral',
      parent: arrayId,
      children: [],
      span: { start: { offset: 1, line: 1, column: 1 }, end: { offset: 2, line: 1, column: 2 } },
      data: { value: 1 },
    })
    tree.nodes[arrayId]?.children.push(elem1)

    const result = check(tree)

    const type = result.types.get(arrayId)
    expect(type?.kind).toBe('array')
    expect(type?.elementType).toBeDefined()
    expect(type?.elementType?.kind).toBe('number')
  })

  it('should infer empty array type', () => {
    const tree = createTree('javascript', '[]')

    const arrayId = addNode(tree, {
      type: 'ArrayExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 2, line: 1, column: 2 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(arrayId)

    const result = check(tree)

    const type = result.types.get(arrayId)
    expect(type?.kind).toBe('array')
    expect(type?.elementType?.kind).toBe('unknown')
  })

  it('should infer object type', () => {
    const tree = createTree('javascript', '{ x: 1 }')

    const objId = addNode(tree, {
      type: 'ObjectExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 8, line: 1, column: 8 } },
      data: {},
    })
    tree.nodes[tree.root]?.children.push(objId)

    // Add property
    const propId = addNode(tree, {
      type: 'Property',
      parent: objId,
      children: [],
      span: { start: { offset: 2, line: 1, column: 2 }, end: { offset: 6, line: 1, column: 6 } },
      data: { key: { name: 'x' } },
    })
    tree.nodes[objId]?.children.push(propId)

    const valueId = addNode(tree, {
      type: 'NumericLiteral',
      parent: propId,
      children: [],
      span: { start: { offset: 5, line: 1, column: 5 }, end: { offset: 6, line: 1, column: 6 } },
      data: { value: 1 },
    })
    tree.nodes[propId]?.children.push(valueId)

    const result = check(tree)

    const type = result.types.get(objId)
    expect(type?.kind).toBe('object')
    expect(type?.properties).toBeDefined()
    expect(type?.properties?.has('x')).toBe(true)
    expect(type?.properties?.get('x')?.kind).toBe('number')
  })

  it('should infer binary expression type', () => {
    const tree = createTree('javascript', '1 + 2')

    const binId = addNode(tree, {
      type: 'BinaryExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 5, line: 1, column: 5 } },
      data: { operator: '+' },
    })
    tree.nodes[tree.root]?.children.push(binId)

    const left = addNode(tree, {
      type: 'NumericLiteral',
      parent: binId,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 1, line: 1, column: 1 } },
      data: { value: 1 },
    })
    tree.nodes[binId]?.children.push(left)

    const right = addNode(tree, {
      type: 'NumericLiteral',
      parent: binId,
      children: [],
      span: { start: { offset: 4, line: 1, column: 4 }, end: { offset: 5, line: 1, column: 5 } },
      data: { value: 2 },
    })
    tree.nodes[binId]?.children.push(right)

    const result = check(tree)

    const type = result.types.get(binId)
    expect(type?.kind).toBe('number')
  })

  it('should infer string concatenation type', () => {
    const tree = createTree('javascript', '"a" + "b"')

    const binId = addNode(tree, {
      type: 'BinaryExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 9, line: 1, column: 9 } },
      data: { operator: '+' },
    })
    tree.nodes[tree.root]?.children.push(binId)

    const left = addNode(tree, {
      type: 'StringLiteral',
      parent: binId,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 3, line: 1, column: 3 } },
      data: { value: 'a' },
    })
    tree.nodes[binId]?.children.push(left)

    const right = addNode(tree, {
      type: 'StringLiteral',
      parent: binId,
      children: [],
      span: { start: { offset: 6, line: 1, column: 6 }, end: { offset: 9, line: 1, column: 9 } },
      data: { value: 'b' },
    })
    tree.nodes[binId]?.children.push(right)

    const result = check(tree)

    const type = result.types.get(binId)
    expect(type?.kind).toBe('string')
  })

  it('should infer comparison expression type', () => {
    const tree = createTree('javascript', '1 < 2')

    const binId = addNode(tree, {
      type: 'BinaryExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 5, line: 1, column: 5 } },
      data: { operator: '<' },
    })
    tree.nodes[tree.root]?.children.push(binId)

    const left = addNode(tree, {
      type: 'NumericLiteral',
      parent: binId,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 1, line: 1, column: 1 } },
      data: { value: 1 },
    })
    tree.nodes[binId]?.children.push(left)

    const right = addNode(tree, {
      type: 'NumericLiteral',
      parent: binId,
      children: [],
      span: { start: { offset: 4, line: 1, column: 4 }, end: { offset: 5, line: 1, column: 5 } },
      data: { value: 2 },
    })
    tree.nodes[binId]?.children.push(right)

    const result = check(tree)

    const type = result.types.get(binId)
    expect(type?.kind).toBe('boolean')
  })

  it('should infer unary expression type', () => {
    const tree = createTree('javascript', '!true')

    const unaryId = addNode(tree, {
      type: 'UnaryExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 5, line: 1, column: 5 } },
      data: { operator: '!' },
    })
    tree.nodes[tree.root]?.children.push(unaryId)

    const result = check(tree)

    const type = result.types.get(unaryId)
    expect(type?.kind).toBe('boolean')
  })

  it('should infer typeof expression type', () => {
    const tree = createTree('javascript', 'typeof x')

    const typeofId = addNode(tree, {
      type: 'UnaryExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 8, line: 1, column: 8 } },
      data: { operator: 'typeof' },
    })
    tree.nodes[tree.root]?.children.push(typeofId)

    const result = check(tree)

    const type = result.types.get(typeofId)
    expect(type?.kind).toBe('string')
  })

  it('should infer logical expression type', () => {
    const tree = createTree('javascript', 'true && false')

    const logicalId = addNode(tree, {
      type: 'LogicalExpression',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 13, line: 1, column: 13 } },
      data: { operator: '&&' },
    })
    tree.nodes[tree.root]?.children.push(logicalId)

    const result = check(tree)

    const type = result.types.get(logicalId)
    expect(type?.kind).toBe('boolean')
  })

  it('should check type compatibility', () => {
    const checker = new TypeChecker()

    // Same types
    expect(checker.isCompatible({ kind: 'number' }, { kind: 'number' })).toBe(true)
    expect(checker.isCompatible({ kind: 'string' }, { kind: 'string' })).toBe(true)

    // Different types
    expect(checker.isCompatible({ kind: 'number' }, { kind: 'string' })).toBe(false)

    // Any is compatible with everything
    expect(checker.isCompatible({ kind: 'any' }, { kind: 'number' })).toBe(true)
    expect(checker.isCompatible({ kind: 'number' }, { kind: 'any' })).toBe(true)

    // Unknown is compatible with everything
    expect(checker.isCompatible({ kind: 'unknown' }, { kind: 'number' })).toBe(true)
  })

  it('should use standalone check function', () => {
    const tree = createTree('javascript', '42')
    const result = check(tree)

    expect(result).toBeDefined()
    expect(result.success).toBe(true)
  })

  it('should get inferred type for node', () => {
    const tree = createTree('javascript', '42')

    const numId = addNode(tree, {
      type: 'NumericLiteral',
      parent: tree.root,
      children: [],
      span: { start: { offset: 0, line: 1, column: 0 }, end: { offset: 2, line: 1, column: 2 } },
      data: { value: 42 },
    })
    tree.nodes[tree.root]?.children.push(numId)

    const checker = new TypeChecker()
    checker.check(tree)

    const type = checker.getType(numId)
    expect(type).toBeDefined()
    expect(type?.kind).toBe('number')
  })
})

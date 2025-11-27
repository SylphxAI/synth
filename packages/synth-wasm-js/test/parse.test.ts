import { describe, expect, it } from 'bun:test'
import { NodeKind, parse, parseBinary, parseCount, tokenize, version } from '../src/index.js'

describe('synth-wasm-js', () => {
  it('should return version', async () => {
    const v = await version()
    expect(v).toMatch(/^\d+\.\d+\.\d+$/)
  })

  it('should tokenize JavaScript', async () => {
    const count = await tokenize('const x = 1;')
    expect(count).toBeGreaterThan(0)
  })

  it('should parse and return node count', async () => {
    const count = await parseCount('const x = 1;')
    expect(count).toBeGreaterThanOrEqual(3)
  })

  it('should parse to binary', async () => {
    const binary = await parseBinary('const x = 1;')
    expect(binary).toBeInstanceOf(Uint8Array)
    expect(binary.length).toBeGreaterThan(4)

    // Check header
    const view = new DataView(binary.buffer)
    const nodeCount = view.getUint32(0, true)
    expect(nodeCount).toBeGreaterThanOrEqual(3)
  })

  it('should parse and return structured AST', async () => {
    const result = await parse('const x = 1;')

    expect(result.nodes).toBeInstanceOf(Array)
    expect(result.nodes.length).toBeGreaterThanOrEqual(3)
    expect(result.source).toBe('const x = 1;')

    // First node should be Program
    const program = result.nodes[0]
    expect(program.kind).toBe(NodeKind.Program)
  })

  it('should parse functions', async () => {
    const result = await parse('function add(a, b) { return a + b; }')

    expect(result.nodes.length).toBeGreaterThan(5)

    // Should contain FunctionDeclaration
    const hasFunction = result.nodes.some((n) => n.kind === NodeKind.FunctionDeclaration)
    expect(hasFunction).toBe(true)
  })

  it('should parse classes', async () => {
    const result = await parse('class Foo { constructor() {} }')

    const hasClass = result.nodes.some((n) => n.kind === NodeKind.ClassDeclaration)
    expect(hasClass).toBe(true)
  })

  it('should parse arrow functions', async () => {
    const result = await parse('const fn = x => x * 2;')

    const hasArrow = result.nodes.some((n) => n.kind === NodeKind.ArrowFunctionExpression)
    expect(hasArrow).toBe(true)
  })

  it('should parse imports/exports', async () => {
    const result = await parse("import { foo } from 'bar'; export default x;")

    const hasImport = result.nodes.some((n) => n.kind === NodeKind.ImportDeclaration)
    const hasExport = result.nodes.some((n) => n.kind === NodeKind.ExportDeclaration)
    expect(hasImport).toBe(true)
    expect(hasExport).toBe(true)
  })

  it('should parse complex code', async () => {
    const code = `
      async function fetchData(url) {
        const response = await fetch(url);
        return response.json();
      }

      class EventEmitter {
        constructor() {
          this.listeners = new Map();
        }

        on(event, callback) {
          const list = this.listeners.get(event) ?? [];
          list.push(callback);
          this.listeners.set(event, list);
        }
      }

      export { fetchData, EventEmitter };
    `

    const result = await parse(code)
    expect(result.nodes.length).toBeGreaterThan(20)
  })
})

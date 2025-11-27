import { beforeAll, describe, expect, it } from 'bun:test'
import { initWasm, isWasmInitialized, parse, version } from '../src/index.js'

describe('synth-wasm-md', () => {
  beforeAll(async () => {
    await initWasm()
  })

  it('should initialize WASM', () => {
    expect(isWasmInitialized()).toBe(true)
  })

  it('should parse heading', async () => {
    const tree = await parse('# Hello World')

    expect(tree).toBeDefined()
    expect(tree.meta.language).toBe('markdown')
    expect(tree.nodes.length).toBeGreaterThan(1)

    // Find heading node
    const heading = tree.nodes.find((n) => n.type === 'heading')
    expect(heading).toBeDefined()
  })

  it('should parse paragraph', async () => {
    const tree = await parse('This is a paragraph.')

    const para = tree.nodes.find((n) => n.type === 'paragraph')
    expect(para).toBeDefined()
  })

  it('should parse code block', async () => {
    const tree = await parse('```rust\nfn main() {}\n```')

    const code = tree.nodes.find((n) => n.type === 'code')
    expect(code).toBeDefined()
    expect(code?.data?.lang).toBe('rust')
  })

  it('should parse multiple blocks', async () => {
    const markdown = `# Title

This is a paragraph.

- Item 1
- Item 2
`
    const tree = await parse(markdown)

    expect(tree.nodes.length).toBeGreaterThan(3)
  })

  it('should return version', async () => {
    const v = await version()
    expect(v).toMatch(/^\d+\.\d+\.\d+/)
  })
})

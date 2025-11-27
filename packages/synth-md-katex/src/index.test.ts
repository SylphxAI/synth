import { describe, expect, test } from 'bun:test'
import { parse } from '@sylphx/synth-md'
import { katexPlugin } from './index'

describe('KaTeX Plugin', () => {
  test('should create plugin with default options', () => {
    const plugin = katexPlugin()
    expect(plugin.meta.name).toBe('katex')
    expect(plugin.meta.version).toBe('0.1.0')
  })

  test('should create plugin with custom options', () => {
    const plugin = katexPlugin({ inlineMath: false, blockMath: false })
    expect(plugin.meta.name).toBe('katex')
  })

  test('should be usable with parse', () => {
    const tree = parse('Hello $x$ world', {
      plugins: [katexPlugin()],
    })
    expect(tree).toBeDefined()
    expect(tree.nodes.length).toBeGreaterThan(0)
  })

  test('should add mathExpressions to metadata', () => {
    const tree = parse('Hello world', {
      plugins: [katexPlugin()],
    })
    expect(tree.meta?.data?.mathExpressions).toBeDefined()
  })

  test('should handle block math syntax', () => {
    const tree = parse('$$x^2$$', {
      plugins: [katexPlugin()],
    })
    expect(tree).toBeDefined()
  })
})

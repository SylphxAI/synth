import { describe, expect, test } from 'bun:test'
import { parse } from '@sylphx/synth-md'
import { mermaidPlugin } from './index'

describe('Mermaid Plugin', () => {
  test('should create plugin with default options', () => {
    const plugin = mermaidPlugin()
    expect(plugin.meta.name).toBe('mermaid')
    expect(plugin.meta.version).toBe('0.1.0')
  })

  test('should create plugin with custom options', () => {
    const plugin = mermaidPlugin({ validate: true, theme: 'dark' })
    expect(plugin.meta.name).toBe('mermaid')
  })

  test('should be usable with parse', () => {
    const md = `
\`\`\`mermaid
graph TD
  A --> B
\`\`\`
`
    const tree = parse(md, {
      plugins: [mermaidPlugin()],
    })
    expect(tree).toBeDefined()
    expect(tree.nodes.length).toBeGreaterThan(0)
  })

  test('should handle empty input', () => {
    const tree = parse('', {
      plugins: [mermaidPlugin()],
    })
    expect(tree).toBeDefined()
  })

  test('should process multiple code blocks', () => {
    const md = `
\`\`\`javascript
const x = 1
\`\`\`

\`\`\`mermaid
graph LR
  A --> B
\`\`\`
`
    const tree = parse(md, {
      plugins: [mermaidPlugin()],
    })
    expect(tree.nodes.length).toBeGreaterThan(2)
  })
})

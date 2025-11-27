/**
 * Detailed benchmark: WASM vs JavaScript Markdown Parser
 *
 * Tests:
 * 1. Pure parsing (no serialization)
 * 2. Full parse + JSON serialization
 */

import { bench, group, run } from 'mitata'

// Generate test markdown of various sizes
function generateMarkdown(size: number): string {
  const blocks: string[] = []
  let currentSize = 0

  while (currentSize < size) {
    blocks.push(`# Heading ${blocks.length + 1}`)
    const para = `This is a paragraph with **bold**, *italic*, and \`code\`.
It also has [links](https://example.com) and more text to make it longer.`
    blocks.push(para)
    blocks.push('```javascript\nconst x = 1;\nconst y = 2;\nconsole.log(x + y);\n```')
    blocks.push('- Item 1\n- Item 2\n- Item 3')
    currentSize = blocks.join('\n\n').length
  }

  return blocks.join('\n\n')
}

const large = generateMarkdown(102400) // 100KB

console.log(`Document size: ${large.length} bytes`)
console.log()

// Import parsers
const { parse: wasmParse, initWasm } = await import('../src/index.js')
const { parseAndCount } = await import('../wasm/synth_wasm_md.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

// Initialize WASM
await initWasm()

group('Large Document (100KB) - Full Parse + Serialize', () => {
  bench('WASM (parseToJson + JSON.parse)', async () => {
    await wasmParse(large)
  })

  bench('JavaScript', () => {
    jsParse(large)
  })
})

group('Large Document (100KB) - Pure Parsing Only', () => {
  bench('WASM (parseAndCount - no serialization)', () => {
    parseAndCount(large)
  })

  bench('JavaScript', () => {
    jsParse(large)
  })
})

await run({
  avg: true,
  json: false,
  colors: true,
  min_max: true,
  percentiles: true,
})

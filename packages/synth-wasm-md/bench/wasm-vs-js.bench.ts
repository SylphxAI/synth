/**
 * Benchmark: WASM vs JavaScript Markdown Parser
 */

import { bench, group, run } from 'mitata'

// Generate test markdown of various sizes
function generateMarkdown(size: number): string {
  const blocks: string[] = []
  let currentSize = 0

  while (currentSize < size) {
    // Add heading
    blocks.push(`# Heading ${blocks.length + 1}`)

    // Add paragraph
    const para = `This is a paragraph with **bold**, *italic*, and \`code\`.
It also has [links](https://example.com) and more text to make it longer.`
    blocks.push(para)

    // Add code block
    blocks.push('```javascript\nconst x = 1;\nconst y = 2;\nconsole.log(x + y);\n```')

    // Add list
    blocks.push('- Item 1\n- Item 2\n- Item 3')

    currentSize = blocks.join('\n\n').length
  }

  return blocks.join('\n\n')
}

// Test documents
const small = generateMarkdown(1024) // 1KB
const medium = generateMarkdown(10240) // 10KB
const large = generateMarkdown(102400) // 100KB

console.log('Document sizes:')
console.log(`  Small:  ${small.length} bytes`)
console.log(`  Medium: ${medium.length} bytes`)
console.log(`  Large:  ${large.length} bytes`)
console.log()

// Import parsers
const { parse: wasmParse, initWasm } = await import('../src/index.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

// Initialize WASM
await initWasm()

group('Small Document (1KB)', () => {
  bench('WASM', async () => {
    await wasmParse(small)
  })

  bench('JavaScript', () => {
    jsParse(small)
  })
})

group('Medium Document (10KB)', () => {
  bench('WASM', async () => {
    await wasmParse(medium)
  })

  bench('JavaScript', () => {
    jsParse(medium)
  })
})

group('Large Document (100KB)', () => {
  bench('WASM', async () => {
    await wasmParse(large)
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

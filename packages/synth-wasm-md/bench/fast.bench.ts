/**
 * Benchmark: Fast WASM tokenizer vs JavaScript parser
 */

import { bench, group, run } from 'mitata'

// Generate test markdown
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

const small = generateMarkdown(1024)
const medium = generateMarkdown(10240)
const large = generateMarkdown(102400)

console.log('Document sizes:')
console.log(`  Small:  ${small.length} bytes`)
console.log(`  Medium: ${medium.length} bytes`)
console.log(`  Large:  ${large.length} bytes`)
console.log()

// Import parsers
const { initWasm } = await import('../src/index.js')
const { fastTokenize, parseAndCount } = await import('../wasm/synth_wasm_md.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

await initWasm()

// Verify fast tokenizer works
console.log('Fast tokenize test:', fastTokenize('# Hello\n\nWorld\n'))
console.log()

group('Small Document (1KB)', () => {
  bench('WASM Fast Tokenize (zero-copy)', () => {
    fastTokenize(small)
  })

  bench('WASM Full Parse', () => {
    parseAndCount(small)
  })

  bench('JavaScript', () => {
    jsParse(small)
  })
})

group('Medium Document (10KB)', () => {
  bench('WASM Fast Tokenize (zero-copy)', () => {
    fastTokenize(medium)
  })

  bench('WASM Full Parse', () => {
    parseAndCount(medium)
  })

  bench('JavaScript', () => {
    jsParse(medium)
  })
})

group('Large Document (100KB)', () => {
  bench('WASM Fast Tokenize (zero-copy)', () => {
    fastTokenize(large)
  })

  bench('WASM Full Parse', () => {
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

/**
 * Ultimate Benchmark: All parsing methods compared
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
const xlarge = generateMarkdown(1024000) // 1MB

console.log('Document sizes:')
console.log(`  Small:  ${small.length} bytes`)
console.log(`  Medium: ${medium.length} bytes`)
console.log(`  Large:  ${large.length} bytes`)
console.log(`  XLarge: ${xlarge.length} bytes`)
console.log()

// Import parsers
const { initWasm } = await import('../src/index.js')
const {
  fastTokenize,
  fastParseBinary,
  fastParseCount,
  parseAndCount,
} = await import('../wasm/synth_wasm_md.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

await initWasm()

// Verify
console.log('Verification:')
console.log('  fastTokenize:', fastTokenize('# Hello\n\nWorld\n'))
console.log('  fastParseCount:', fastParseCount('# Hello\n\nWorld\n'))
console.log('  fastParseBinary length:', fastParseBinary('# Hello\n\nWorld\n').length)
console.log()

group('Small Document (1KB)', () => {
  bench('WASM tokenize only', () => {
    fastTokenize(small)
  })

  bench('WASM fast parse (binary)', () => {
    fastParseBinary(small)
  })

  bench('WASM full parse (tree)', () => {
    parseAndCount(small)
  })

  bench('JavaScript', () => {
    jsParse(small)
  })
})

group('Medium Document (10KB)', () => {
  bench('WASM tokenize only', () => {
    fastTokenize(medium)
  })

  bench('WASM fast parse (binary)', () => {
    fastParseBinary(medium)
  })

  bench('WASM full parse (tree)', () => {
    parseAndCount(medium)
  })

  bench('JavaScript', () => {
    jsParse(medium)
  })
})

group('Large Document (100KB)', () => {
  bench('WASM tokenize only', () => {
    fastTokenize(large)
  })

  bench('WASM fast parse (binary)', () => {
    fastParseBinary(large)
  })

  bench('WASM full parse (tree)', () => {
    parseAndCount(large)
  })

  bench('JavaScript', () => {
    jsParse(large)
  })
})

group('XLarge Document (1MB)', () => {
  bench('WASM tokenize only', () => {
    fastTokenize(xlarge)
  })

  bench('WASM fast parse (binary)', () => {
    fastParseBinary(xlarge)
  })

  bench('WASM full parse (tree)', () => {
    parseAndCount(xlarge)
  })

  bench('JavaScript', () => {
    jsParse(xlarge)
  })
})

await run({
  avg: true,
  json: false,
  colors: true,
  min_max: true,
  percentiles: true,
})

// Summary
console.log('\n📊 Summary:')
console.log('WASM tokenize = zero-copy tokenization only')
console.log('WASM fast parse = tokenize + binary tree output')
console.log('WASM full parse = original parser with Tree object')
console.log('JavaScript = @sylphx/synth-md pure JS parser')

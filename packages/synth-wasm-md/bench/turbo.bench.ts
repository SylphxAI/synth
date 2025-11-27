/**
 * TURBO Benchmark - Maximum performance comparison
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
const xlarge = generateMarkdown(1024000)

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
  turboParseBinary,
  turboParseCount,
} = await import('../wasm/synth_wasm_md.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

await initWasm()

// Verify
console.log('Verification:')
console.log('  turboParseCount:', turboParseCount('# Hello\n\nWorld\n'))
console.log('  turboParseBinary length:', turboParseBinary('# Hello\n\nWorld\n').length, 'bytes')
console.log('  (expected: 4 header + 3 nodes * 16 bytes = 52 bytes)')
console.log()

group('Small Document (1KB)', () => {
  bench('🚀 TURBO (16-byte nodes)', () => {
    turboParseBinary(small)
  })

  bench('⚡ Fast (28-byte nodes)', () => {
    fastParseBinary(small)
  })

  bench('📝 JavaScript', () => {
    jsParse(small)
  })
})

group('Medium Document (10KB)', () => {
  bench('🚀 TURBO (16-byte nodes)', () => {
    turboParseBinary(medium)
  })

  bench('⚡ Fast (28-byte nodes)', () => {
    fastParseBinary(medium)
  })

  bench('📝 JavaScript', () => {
    jsParse(medium)
  })
})

group('Large Document (100KB)', () => {
  bench('🚀 TURBO (16-byte nodes)', () => {
    turboParseBinary(large)
  })

  bench('⚡ Fast (28-byte nodes)', () => {
    fastParseBinary(large)
  })

  bench('📝 JavaScript', () => {
    jsParse(large)
  })
})

group('XLarge Document (1MB)', () => {
  bench('🚀 TURBO (16-byte nodes)', () => {
    turboParseBinary(xlarge)
  })

  bench('⚡ Fast (28-byte nodes)', () => {
    fastParseBinary(xlarge)
  })

  bench('📝 JavaScript', () => {
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

// Summary table
console.log('\n' + '='.repeat(60))
console.log('📊 SUMMARY')
console.log('='.repeat(60))
console.log(`
🚀 TURBO = 16-byte compact nodes, maximum inlining, unchecked access
⚡ Fast  = 28-byte nodes, zero-copy tokenizer
📝 JS    = @sylphx/synth-md pure JavaScript parser

Node size comparison:
- TURBO: 16 bytes/node (fits 4 nodes per cache line)
- Fast:  28 bytes/node (fits 2 nodes per cache line)
- JS:    ~200+ bytes/node (object with properties)
`)

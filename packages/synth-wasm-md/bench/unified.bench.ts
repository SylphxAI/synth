/**
 * Unified Parser Benchmark
 *
 * Compares the new unified WASM parser against JavaScript
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
const wasmPkg = await import('../../../crates/wasm-md/pkg/synth_wasm_md.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

// Initialize WASM
await wasmPkg.default()

// Verify
console.log('Verification:')
console.log('  parseCount:', wasmPkg.parseCount('# Hello\n\nWorld\n'))

const tree = wasmPkg.parse('# Hello\n\nWorld\n')
console.log('  parse tree node_count:', tree.node_count)
console.log(`  parse tree JSON: ${JSON.stringify(tree.toJSON()).slice(0, 100)}...`)

const binary = wasmPkg.parseBinary('# Hello\n\nWorld\n')
console.log('  parseBinary length:', binary.length, 'bytes')
console.log()

group('Small Document (1KB)', () => {
  bench('🚀 WASM parse (Tree)', () => {
    wasmPkg.parse(small)
  })

  bench('⚡ WASM parseBinary', () => {
    wasmPkg.parseBinary(small)
  })

  bench('📝 JavaScript', () => {
    jsParse(small)
  })
})

group('Medium Document (10KB)', () => {
  bench('🚀 WASM parse (Tree)', () => {
    wasmPkg.parse(medium)
  })

  bench('⚡ WASM parseBinary', () => {
    wasmPkg.parseBinary(medium)
  })

  bench('📝 JavaScript', () => {
    jsParse(medium)
  })
})

group('Large Document (100KB)', () => {
  bench('🚀 WASM parse (Tree)', () => {
    wasmPkg.parse(large)
  })

  bench('⚡ WASM parseBinary', () => {
    wasmPkg.parseBinary(large)
  })

  bench('📝 JavaScript', () => {
    jsParse(large)
  })
})

group('XLarge Document (1MB)', () => {
  bench('🚀 WASM parse (Tree)', () => {
    wasmPkg.parse(xlarge)
  })

  bench('⚡ WASM parseBinary', () => {
    wasmPkg.parseBinary(xlarge)
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
console.log(`\n${'='.repeat(60)}`)
console.log('📊 SUMMARY')
console.log('='.repeat(60))
console.log(`
🚀 WASM parse (Tree) = Returns Tree object with toJSON()
⚡ WASM parseBinary = Returns compact binary (24 bytes/node)
📝 JavaScript = @sylphx/synth-md pure JavaScript parser

API:
- parse(markdown)      → Tree object (use tree.toJSON() for JS object)
- parseBinary(markdown) → Uint8Array (maximum speed, needs JS decode)
- parseCount(markdown) → number (for benchmarking)
- parseToJson(markdown) → JSON string
`)

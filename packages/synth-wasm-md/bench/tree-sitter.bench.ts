/**
 * Benchmark: tree-sitter WASM vs Custom WASM vs JavaScript
 */

import { bench, group, run } from 'mitata'
import { Language, Parser } from 'web-tree-sitter'

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

// Initialize tree-sitter
await Parser.init()
const treeSitterParser = new Parser()

// Load markdown language
import { resolve } from 'node:path'
const wasmPath = resolve(
  import.meta.dirname,
  '../../../node_modules/tree-sitter-wasm-prebuilt/lib/tree-sitter-markdown.wasm'
)
const Markdown = await Language.load(wasmPath)
treeSitterParser.setLanguage(Markdown)

// Import other parsers
const { parse: customWasmParse, initWasm } = await import('../src/index.js')
const { parse: jsParse } = await import('@sylphx/synth-md')

await initWasm()

// Tree-sitter parse function
function treeSitterParse(text: string) {
  const tree = treeSitterParser.parse(text)
  return tree
}

group('Small Document (1KB)', () => {
  bench('tree-sitter WASM', () => {
    treeSitterParse(small)
  })

  bench('Custom WASM', async () => {
    await customWasmParse(small)
  })

  bench('JavaScript', () => {
    jsParse(small)
  })
})

group('Medium Document (10KB)', () => {
  bench('tree-sitter WASM', () => {
    treeSitterParse(medium)
  })

  bench('Custom WASM', async () => {
    await customWasmParse(medium)
  })

  bench('JavaScript', () => {
    jsParse(medium)
  })
})

group('Large Document (100KB)', () => {
  bench('tree-sitter WASM', () => {
    treeSitterParse(large)
  })

  bench('Custom WASM', async () => {
    await customWasmParse(large)
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

# @sylphx/synth-md

High-performance Markdown parser - 26-42x faster than remark.

## Installation

```bash
npm install @sylphx/synth @sylphx/synth-md
```

## Quick Start

```typescript
import { parse } from '@sylphx/synth-md'

const tree = parse('# Hello **World**')

// Access nodes
for (const node of tree.nodes) {
  console.log(node.type, node.data)
}
```

## Functions

### parse

Parse Markdown synchronously:

```typescript
import { parse } from '@sylphx/synth-md'

const tree = parse(markdown, {
  buildIndex: false,      // Build query index (slower)
  plugins: [],            // Plugins to apply
  useNodePool: true,      // Object pooling (default: true)
  useBatchTokenizer: false, // Batch processing for large docs
  batchSize: 16           // Batch size (1-128)
})
```

### parseAsync

Parse with async plugin support:

```typescript
import { parseAsync } from '@sylphx/synth-md'

const tree = await parseAsync(markdown, {
  plugins: [asyncPlugin]
})
```

### createParser

Create a reusable parser instance:

```typescript
import { createParser } from '@sylphx/synth-md'

const parser = createParser()
  .use(plugin1)
  .use(plugin2)

const tree1 = parser.parse(doc1)
const tree2 = parser.parse(doc2)
```

## Node Types

### Block Elements

| Type | Description | Data Fields |
|------|-------------|-------------|
| `root` | Document root | - |
| `heading` | ATX/Setext heading | `depth: 1-6` |
| `paragraph` | Paragraph | - |
| `codeBlock` | Fenced/indented code | `language?: string`, `meta?: string` |
| `blockquote` | Block quote | - |
| `list` | Ordered/unordered list | `ordered: boolean`, `start?: number` |
| `listItem` | List item | `checked?: boolean` (for task lists) |
| `horizontalRule` | Thematic break | - |
| `html` | HTML block | `value: string` |
| `table` | GFM table | - |
| `tableRow` | Table row | - |
| `tableCell` | Table cell | `align?: 'left'|'center'|'right'` |

### Inline Elements

| Type | Description | Data Fields |
|------|-------------|-------------|
| `text` | Plain text | `value: string` |
| `emphasis` | Italic | - |
| `strong` | Bold | - |
| `code` | Inline code | `value: string` |
| `link` | Link | `url: string`, `title?: string` |
| `image` | Image | `url: string`, `alt?: string`, `title?: string` |
| `strikethrough` | GFM strikethrough | - |
| `autolink` | GFM autolink | `url: string` |
| `hardBreak` | Hard line break | - |
| `softBreak` | Soft line break | - |

## Parser Class

```typescript
import { Parser } from '@sylphx/synth-md'

const parser = new Parser()

// Parse
const tree = parser.parse(markdown, options)

// Async parse
const tree = await parser.parseAsync(markdown, options)

// Register plugins
parser.use(plugin)

// Get query index (if built)
const index = parser.getIndex()
```

## Built-in Plugins

```typescript
import {
  addHeadingIds,
  tableOfContents,
  uppercaseHeadings,
  addCodeLineNumbers,
  removeComments,
  wrapParagraphs
} from '@sylphx/synth-md'

const tree = parse(markdown, {
  plugins: [addHeadingIds, tableOfContents]
})

// Access plugin data
console.log(tree.meta.data?.toc)
```

## Streaming

### parseStream

Parse from a stream:

```typescript
import { parseStream } from '@sylphx/synth-md'
import { createReadStream } from 'fs'

const stream = createReadStream('large.md', { encoding: 'utf8' })
const tree = await parseStream(stream)
```

### parseWithProgress

Parse with progress tracking:

```typescript
import { parseWithProgress } from '@sylphx/synth-md'

const tree = await parseWithProgress(text, (progress) => {
  console.log(`${progress.percent}% complete`)
})
```

### StreamingMarkdownParser

Manual streaming:

```typescript
import { StreamingMarkdownParser } from '@sylphx/synth-md'

const parser = new StreamingMarkdownParser()

parser.on('node', (node) => console.log(node.type))
parser.on('end', (tree) => console.log('Done'))

parser.write('# Hello\n')
parser.write('\nWorld')
await parser.end()
```

## Incremental Parsing

For real-time editing with 10-100x speedup:

```typescript
import { IncrementalMarkdownParser, detectEdit } from '@sylphx/synth-md'

const parser = new IncrementalMarkdownParser()
parser.parse(originalText)

// After edit
const edit = detectEdit(originalText, newText)
const { tree, stats } = parser.update(newText, edit)

console.log(`Token reuse: ${stats.tokenReuseRate * 100}%`)
console.log(`Speedup: ${stats.speedup}x`)
```

## Performance Options

```typescript
// Maximum speed (default)
parse(text)

// With query index (slower build, faster queries)
parse(text, { buildIndex: true })

// For large documents (>10KB)
parse(text, { useBatchTokenizer: true, batchSize: 32 })

// Disable pooling (for debugging)
parse(text, { useNodePool: false })
```

## Examples

### Extract All Headings

```typescript
const tree = parse(markdown)

const headings = tree.nodes
  .filter(n => n.type === 'heading')
  .map(n => ({
    level: n.data?.depth,
    text: getTextContent(tree, n)
  }))
```

### Build Table of Contents

```typescript
import { parse, traverse } from '@sylphx/synth-md'

function buildTOC(markdown: string) {
  const tree = parse(markdown)
  const toc = []

  traverse(tree, {
    heading: (ctx) => {
      toc.push({
        level: ctx.node.data?.depth,
        line: ctx.node.span?.start.line
      })
    }
  })

  return toc
}
```

### Extract Code Blocks

```typescript
const codeBlocks = tree.nodes
  .filter(n => n.type === 'codeBlock')
  .map(n => ({
    language: n.data?.language || 'text',
    code: tree.meta.source.slice(
      n.span!.start.offset,
      n.span!.end.offset
    )
  }))
```

## CommonMark Compliance

Full CommonMark specification support:

- ATX and Setext headings
- Fenced and indented code blocks
- Block quotes (nested)
- Ordered and unordered lists (nested)
- Links and images
- Emphasis and strong
- Inline code
- Hard and soft line breaks
- HTML blocks
- Horizontal rules

## GFM Extensions

- Tables with alignment
- Strikethrough
- Task lists
- Autolinks

## Performance

| Document | Remark | Synth | Speedup |
|----------|--------|-------|---------|
| Small (100B) | 0.084ms | 0.0015ms | **56x** |
| Medium (500B) | 0.448ms | 0.0078ms | **57x** |
| Large (25KB) | 28.4ms | 0.392ms | **72x** |
| Huge (250KB) | 58.8ms | 0.786ms | **75x** |

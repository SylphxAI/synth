# Synth Markdown Parser - Usage Guide

## Installation

```bash
npm install @sylphx/synth
```

## Quick Start

### Basic Parsing

```typescript
import { UltraOptimizedMarkdownParser } from '@sylphx/synth'

const parser = new UltraOptimizedMarkdownParser()

// Parse Markdown (54-75x faster than remark)
const tree = parser.parse(`# Hello World

This is **bold** and *italic* text.`)

console.log(tree)
```

### With Query Index

If you need to query the AST (find all headings, links, etc.):

```typescript
// Enable index building (9-10x faster than remark, but slower than default)
const tree = parser.parse(markdown, { buildIndex: true })

// Get the index
const index = parser.getIndex()

// Query by type
const headings = index.getByType('heading')
const links = index.getByType('link')

// Query by position
const nodesAtLine = index.getByLine(5)
```

### Lazy Index Building (Recommended)

Best of both worlds - parse fast, build index only when needed:

```typescript
// Fast parsing (54-75x)
const tree = parser.parse(markdown)

// ... do fast operations ...

// Build index on demand (only when you need to query)
if (needToQuery) {
  const index = parser.getIndex()  // Builds automatically
  const headings = index.getByType('heading')
}
```

## Supported Features

### CommonMark Elements

#### Block Elements

```markdown
# Heading 1
## Heading 2
### Heading 3

Paragraph text with inline formatting.

```typescript
const code = 'block'
```

- List item 1
- List item 2
  - Nested item

1. Numbered item
2. Another item

> Blockquote text
> Multiple lines

---
```

#### Inline Elements

```markdown
**bold text**
*italic text*
`inline code`
[link text](https://example.com)
![image alt](image.jpg)
```

### GFM Extensions

#### Tables

```markdown
| Header 1 | Header 2 | Header 3 |
|----------|:--------:|---------:|
| Left     | Center   | Right    |
| Cell 1   | Cell 2   | Cell 3   |
```

Alignments:
- `:---` - Left aligned
- `:---:` - Center aligned
- `---:` - Right aligned

#### Strikethrough

```markdown
~~deleted text~~
```

#### Autolinks

```markdown
https://example.com
www.example.com
user@example.com
```

All are automatically converted to clickable links.

#### Task Lists

```markdown
- [x] Completed task
- [ ] Incomplete task
```

## API Reference

### UltraOptimizedMarkdownParser

Main parser class with ultra-optimized performance.

#### Constructor

```typescript
const parser = new UltraOptimizedMarkdownParser()
```

#### Methods

**`parse(text: string, options?: ParseOptions): Tree`**

Parse Markdown text into AST.

```typescript
const tree = parser.parse(markdown, {
  buildIndex: false  // default: false for maximum performance
})
```

**Options:**
- `buildIndex?: boolean` - Build query index (default: `false`)
  - `false`: 54-75x faster than remark (recommended for most use cases)
  - `true`: 9-10x faster than remark (use when you need to query)

**`parseIncremental(text: string, edit: Edit, options?: ParseOptions): Tree`**

Incremental parsing after edits (uses existing infrastructure).

```typescript
const edit: Edit = {
  startOffset: 10,
  oldEndOffset: 15,
  newEndOffset: 20,
  startPosition: { line: 0, column: 10, offset: 10 },
  oldEndPosition: { line: 0, column: 15, offset: 15 },
  newEndPosition: { line: 0, column: 20, offset: 20 }
}

const newTree = parser.parseIncremental(newMarkdown, edit)
```

**`getIndex(): ASTIndex`**

Get query index (builds lazily if not exists).

```typescript
const index = parser.getIndex()
```

**`getTree(): Tree | null`**

Get current AST tree.

```typescript
const tree = parser.getTree()
```

### Tree Structure

```typescript
interface Tree {
  language: string        // 'markdown'
  source: string         // Original source text
  root: NodeId           // Root node ID (0)
  nodes: Record<NodeId, Node | null>
  version: number
  createdAt: number
  updatedAt: number
}
```

### Node Structure

```typescript
interface Node {
  id: NodeId
  type: string           // 'heading', 'paragraph', 'text', etc.
  parent: NodeId | null
  children: NodeId[]
  span: Span             // Position in source
  data?: Record<string, any>  // Node-specific data
}
```

### Query Index

```typescript
interface ASTIndex {
  build(): void
  getByType(type: string): NodeId[]
  getByLine(line: number): NodeId[]
  getByOffset(offset: number): NodeId | null
  getParent(nodeId: NodeId): NodeId | null
  getChildren(nodeId: NodeId): NodeId[]
}
```

## Performance Guide

### When to Use Each Mode

**No Index (Default) - 54-75x faster**
```typescript
parser.parse(markdown)  // Fastest
```

**Use cases:**
- Rendering Markdown to HTML
- Converting Markdown to other formats
- Simple transformations
- Build tools / SSG
- CLI tools

**With Index - 9-10x faster**
```typescript
parser.parse(markdown, { buildIndex: true })
```

**Use cases:**
- Linting / analysis tools
- Finding specific elements
- Complex transformations
- Editor features

**Lazy Index - Best of Both**
```typescript
const tree = parser.parse(markdown)  // Fast parse
// ... fast operations ...
const index = parser.getIndex()  // Build when needed
```

**Use cases:**
- Most applications
- Conditional querying
- Performance-critical paths

### Performance Comparison

| Document Size | Remark | Synth (no index) | Synth (with index) |
|---------------|--------|------------------|--------------------|
| Small (100B) | 0.084ms | 0.0015ms (56x) | 0.010ms (8.4x) |
| Medium (500B) | 0.448ms | 0.0078ms (57x) | 0.045ms (10x) |
| Large (25KB) | 28.4ms | 0.392ms (72x) | 2.74ms (10x) |
| Docs (250KB) | 58.8ms | 0.786ms (75x) | 6.13ms (9.6x) |

## Common Patterns

### Rendering to HTML

```typescript
function renderMarkdown(markdown: string): string {
  const parser = new UltraOptimizedMarkdownParser()
  const tree = parser.parse(markdown)  // No index needed, maximum speed

  return renderTree(tree)
}

function renderTree(tree: Tree): string {
  // Traverse and render nodes
  return renderNode(tree, tree.root)
}
```

### Finding All Links

```typescript
function findAllLinks(markdown: string): string[] {
  const parser = new UltraOptimizedMarkdownParser()
  const tree = parser.parse(markdown, { buildIndex: true })  // Need index
  const index = parser.getIndex()

  const linkIds = index.getByType('link')
  const links: string[] = []

  for (const id of linkIds) {
    const node = tree.nodes[id]
    if (node && node.data?.url) {
      links.push(node.data.url)
    }
  }

  return links
}
```

### Linting

```typescript
function lintMarkdown(markdown: string): string[] {
  const parser = new UltraOptimizedMarkdownParser()
  const tree = parser.parse(markdown, { buildIndex: true })
  const index = parser.getIndex()

  const errors: string[] = []

  // Check all headings are properly nested
  const headings = index.getByType('heading')
  let lastDepth = 0

  for (const id of headings) {
    const node = tree.nodes[id]
    const depth = node?.data?.depth || 1

    if (depth > lastDepth + 1) {
      errors.push(`Heading skips level at line ${node?.span.start.line}`)
    }

    lastDepth = depth
  }

  return errors
}
```

### Live Preview (Editor)

```typescript
let lastTree: Tree | null = null

function updatePreview(markdown: string) {
  const parser = new UltraOptimizedMarkdownParser()

  // Fast parsing without index for real-time rendering
  const tree = parser.parse(markdown)

  // Render immediately (< 1ms for typical documents)
  renderPreview(tree)

  lastTree = tree
}

// On demand - analyze document
function analyzeDocument() {
  if (!lastTree) return

  const parser = new UltraOptimizedMarkdownParser()
  // Lazy build index only when analyzing
  const index = parser.getIndex()

  // Perform analysis
  const stats = {
    headings: index.getByType('heading').length,
    links: index.getByType('link').length,
    images: index.getByType('image').length,
  }

  return stats
}
```

## Migration from Remark

### Before (Remark)

```typescript
import { remark } from 'remark'

const file = remark().parse(markdown)
// Process AST
```

### After (Synth)

```typescript
import { UltraOptimizedMarkdownParser } from '@sylphx/synth'

const parser = new UltraOptimizedMarkdownParser()
const tree = parser.parse(markdown)  // 54-75x faster!
// Process tree
```

### AST Differences

**Remark:**
- Uses `mdast` (Markdown AST) specification
- Nodes have `type`, `children`, `position`
- Deeply nested object structure

**Synth:**
- Flat node structure (SoA pattern)
- Nodes accessed via IDs
- Optimized for performance

**Traversal:**

```typescript
// Remark
function visit(node, callback) {
  callback(node)
  if (node.children) {
    for (const child of node.children) {
      visit(child, callback)
    }
  }
}

// Synth
function visit(tree: Tree, nodeId: NodeId, callback: (node: Node) => void) {
  const node = tree.nodes[nodeId]
  if (!node) return

  callback(node)
  for (const childId of node.children) {
    visit(tree, childId, callback)
  }
}
```

## Troubleshooting

### Parser is slower than expected

1. **Check if index building is enabled**
   ```typescript
   // Slow (with index)
   parser.parse(markdown, { buildIndex: true })

   // Fast (no index)
   parser.parse(markdown)
   ```

2. **Use lazy index building**
   ```typescript
   const tree = parser.parse(markdown)  // Fast
   // ... do work ...
   if (needIndex) {
     const index = parser.getIndex()  // Build when needed
   }
   ```

### Missing GFM features

GFM extensions are fully supported:
- ✅ Tables
- ✅ Strikethrough
- ✅ Autolinks
- ✅ Task lists

If a feature isn't working, please file an issue.

### Type errors

Ensure you're using TypeScript 4.5+ for full type support.

## Best Practices

1. **Default to no index** - Only enable when you need to query
2. **Use lazy index** - Build on demand for best performance
3. **Reuse parser instance** - Create once, parse many times
4. **Profile your code** - Measure before optimizing
5. **Trust the defaults** - They're optimized for 99% of use cases

## Examples

See `/examples` directory for complete examples:
- Static site generator
- Markdown linter
- Live editor with preview
- CLI tool

## License

MIT

## Contributing

See `ROADMAP.md` for planned features and `CONTRIBUTING.md` for guidelines.

## Support

- GitHub Issues: https://github.com/yourorg/synth/issues
- Documentation: https://synth.dev/docs
- Discussions: https://github.com/yourorg/synth/discussions

# Getting Started

Learn how to use Synth to parse, traverse, and transform ASTs at blazing speed.

## Installation

Install the core library and any parsers you need:

```bash
# Core library (required)
npm install @sylphx/synth

# Parsers (install what you need)
npm install @sylphx/synth-md      # Markdown
npm install @sylphx/synth-js      # JavaScript/TypeScript
npm install @sylphx/synth-html    # HTML
npm install @sylphx/synth-json    # JSON
npm install @sylphx/synth-yaml    # YAML
npm install @sylphx/synth-css     # CSS
```

## Basic Usage

### 1. Parse Source Code

```typescript
import { parse } from '@sylphx/synth-md'

const source = `# Hello World

This is a paragraph with **bold** text.
`

const tree = parse(source)
```

### 2. Understand the Tree Structure

Synth uses a flat, arena-based tree structure for maximum performance:

```typescript
// Tree structure
interface Tree {
  meta: {
    language: string    // 'markdown', 'javascript', etc.
    source: string      // Original source code
  }
  root: number          // Root node ID (always 0)
  nodes: BaseNode[]     // All nodes in a flat array
}

// Node structure (same for ALL languages!)
interface BaseNode {
  id: number                      // Unique ID
  type: string                    // 'heading', 'paragraph', etc.
  span?: {                        // Source location
    start: { line, column, offset }
    end: { line, column, offset }
  }
  parent: number | null           // Parent node ID
  children: number[]              // Child node IDs
  data?: Record<string, unknown>  // Language-specific data
}
```

### 3. Traverse the AST

```typescript
import { traverse } from '@sylphx/synth-md'

traverse(tree, {
  // Called for every node
  enter: (ctx) => {
    console.log(ctx.node.type, ctx.depth)
  },

  // Called for specific node types
  heading: (ctx) => {
    console.log('Heading level:', ctx.node.data?.depth)
  },

  paragraph: (ctx) => {
    console.log('Found paragraph')
  }
})
```

### 4. Get Source Text from Nodes

```typescript
// Helper function to extract source text
function getSourceText(tree: Tree, node: BaseNode): string {
  if (!node.span) return ''
  return tree.meta.source.slice(
    node.span.start.offset,
    node.span.end.offset
  )
}

// Use it
for (const node of tree.nodes) {
  if (node.type === 'heading') {
    const text = getSourceText(tree, node)
    console.log(`Heading: "${text}"`)
  }
}
```

### 5. Query Nodes Efficiently

For large documents, use the query index for O(1) lookups:

```typescript
import { parse, createIndex } from '@sylphx/synth-md'

const tree = parse(markdown, { buildIndex: true })
const index = createIndex(tree)

// O(1) queries!
const headings = index.findByType('heading')
const h2s = index.findByData('depth', 2)
const deepNodes = index.findByDepth(3)

// Complex queries
const results = index.query({
  type: ['heading', 'paragraph'],
  depth: { min: 1, max: 3 },
  hasChildren: true
})
```

## Multi-Language Support

Synth uses the **same** `BaseNode` interface for ALL languages:

```typescript
import { parse as parseMD } from '@sylphx/synth-md'
import { parse as parseJS } from '@sylphx/synth-js'
import { parse as parseHTML } from '@sylphx/synth-html'

// All return the same Tree structure!
const mdTree = parseMD('# Hello')
const jsTree = parseJS('const x = 42')
const htmlTree = parseHTML('<div>Hello</div>')

// Same traversal API works for all
import { traverse } from '@sylphx/synth'

traverse(jsTree, {
  FunctionDeclaration: (ctx) => {
    console.log('Function:', ctx.node.data?.id)
  }
})
```

## TypeScript Support

Synth is written in TypeScript and provides full type safety:

```typescript
import type { Tree, BaseNode, Span, NodeId } from '@sylphx/synth'
import { traverse, getNode, getChildren, getParent } from '@sylphx/synth'

// Type-safe tree operations
const node: BaseNode | undefined = getNode(tree, 0)
const children: BaseNode[] = getChildren(tree, 0)
const parent: BaseNode | null = getParent(tree, 1)
```

## Next Steps

- [Core Concepts](/guide/core-concepts) - Deep dive into Tree, Node, and Span
- [Traversal](/guide/traversal) - All traversal methods and options
- [Query Index](/guide/querying) - Fast O(1) node lookups
- [Examples](/examples/ast-chunking) - Real-world usage examples

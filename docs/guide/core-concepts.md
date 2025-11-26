# Core Concepts

Understanding Synth's data structures is key to using it effectively.

## Tree

The `Tree` is the top-level container for an AST:

```typescript
interface Tree {
  meta: TreeMetadata    // Metadata about the source
  root: NodeId          // Root node ID (always 0)
  nodes: BaseNode[]     // All nodes in a flat array
  strings: Map<string, number>  // String interning pool
}

interface TreeMetadata {
  language: string      // 'markdown', 'javascript', 'html', etc.
  source: string        // Original source code
  created: number       // Timestamp
  modified: number      // Timestamp
  data?: Record<string, unknown>  // Custom metadata
}
```

### Why Arena-Based Storage?

Synth stores all nodes in a flat array instead of a traditional tree structure:

```typescript
// Traditional tree (slow)
{
  type: 'root',
  children: [
    { type: 'heading', children: [...] },  // Pointer chasing!
    { type: 'paragraph', children: [...] }
  ]
}

// Synth's arena storage (fast!)
nodes: [
  { id: 0, type: 'root', children: [1, 2] },
  { id: 1, type: 'heading', parent: 0, children: [3] },
  { id: 2, type: 'paragraph', parent: 0, children: [4] },
  { id: 3, type: 'text', parent: 1, children: [] },
  { id: 4, type: 'text', parent: 2, children: [] },
]
```

Benefits:
- **Cache-friendly**: Contiguous memory layout
- **O(1) access**: Direct array indexing by ID
- **No pointer chasing**: 3-10x faster traversal
- **Serializable**: Easy to JSON.stringify/parse

## BaseNode

Every node in Synth, regardless of language, uses the same interface:

```typescript
interface BaseNode {
  /** Unique ID within the tree (array index) */
  id: NodeId

  /** Node type - language-specific but consistent */
  type: string

  /** Source location (optional) */
  span?: Span

  /** Parent node ID (null for root) */
  parent: NodeId | null

  /** Child node IDs */
  children: NodeId[]

  /** Language-specific data */
  data?: Record<string, unknown>
}
```

### The `type` Field

Node types are consistent within each language:

```typescript
// Markdown types
'root' | 'heading' | 'paragraph' | 'text' | 'emphasis' | 'strong' |
'link' | 'image' | 'code' | 'codeBlock' | 'list' | 'listItem' |
'blockquote' | 'horizontalRule' | 'html' | 'table' | ...

// JavaScript types (ESTree-compatible)
'Program' | 'FunctionDeclaration' | 'VariableDeclaration' |
'Identifier' | 'Literal' | 'CallExpression' | 'MemberExpression' | ...

// HTML types
'document' | 'doctype' | 'element' | 'text' | 'comment' | 'cdata'

// JSON types
'Object' | 'Array' | 'Property' | 'String' | 'Number' | 'Boolean' | 'Null'
```

### The `data` Field

Language-specific data goes in `data`:

```typescript
// Markdown heading
{
  id: 1,
  type: 'heading',
  data: {
    depth: 2  // ## = depth 2
  }
}

// HTML element
{
  id: 5,
  type: 'element',
  data: {
    tagName: 'div',
    attributes: { class: 'container', id: 'main' },
    selfClosing: false
  }
}

// JavaScript function
{
  id: 10,
  type: 'FunctionDeclaration',
  data: {
    id: 'myFunction',
    async: true,
    generator: false
  }
}
```

## Span (Source Location)

The `span` tells you where in the source code a node came from:

```typescript
interface Span {
  start: Position
  end: Position
}

interface Position {
  line: number    // 1-based line number
  column: number  // 0-based column number
  offset: number  // 0-based byte offset from start
}
```

### Getting Source Text

Use `span` to extract the original source:

```typescript
function getSourceText(tree: Tree, node: BaseNode): string {
  if (!node.span) return ''
  return tree.meta.source.slice(
    node.span.start.offset,
    node.span.end.offset
  )
}

// Example
const tree = parse('# Hello **World**')
const heading = tree.nodes[1]

console.log(heading.span)
// {
//   start: { line: 1, column: 0, offset: 0 },
//   end: { line: 1, column: 17, offset: 17 }
// }

console.log(getSourceText(tree, heading))
// "# Hello **World**"
```

## NodeId

Node IDs are simple numbers (array indices):

```typescript
type NodeId = number

// Access a node by ID
const node = tree.nodes[nodeId]

// Or use the helper function
import { getNode } from '@sylphx/synth'
const node = getNode(tree, nodeId)
```

### Why Numbers Instead of Objects?

1. **Performance**: No reference counting, no GC pressure
2. **Serialization**: JSON-safe without custom serializers
3. **Memory**: 8 bytes vs 64+ bytes for object references
4. **WASM-ready**: Works with WebAssembly's linear memory

## Tree Operations

### Getting Nodes

```typescript
import { getNode, getRoot, getChildren, getParent } from '@sylphx/synth'

// Get any node by ID
const node = getNode(tree, 5)

// Get the root node
const root = getRoot(tree)

// Get all children as BaseNode[]
const children = getChildren(tree, nodeId)

// Get parent node
const parent = getParent(tree, nodeId)
```

### Modifying Trees

```typescript
import { addNode, updateNode, removeNode } from '@sylphx/synth'

// Add a new node
const newId = addNode(tree, {
  type: 'paragraph',
  parent: 0,
  children: []
})

// Update a node
updateNode(tree, newId, {
  data: { custom: 'value' }
})

// Remove a node
removeNode(tree, newId)
```

## Universal AST Benefits

Because all languages use `BaseNode`, you can:

### 1. Write Generic Tools

```typescript
// Works for ANY language!
function countNodeTypes(tree: Tree): Map<string, number> {
  const counts = new Map<string, number>()
  for (const node of tree.nodes) {
    counts.set(node.type, (counts.get(node.type) || 0) + 1)
  }
  return counts
}
```

### 2. Share Traversal Code

```typescript
// Same traversal for all languages
import { traverse } from '@sylphx/synth'

function findAllTextNodes(tree: Tree): BaseNode[] {
  const results: BaseNode[] = []
  traverse(tree, {
    enter: (ctx) => {
      if (ctx.node.type === 'text' || ctx.node.type === 'Literal') {
        results.push(ctx.node)
      }
    }
  })
  return results
}
```

### 3. Build Cross-Language Tools

```typescript
// Lint any language
import { createRule } from '@sylphx/synth-lint'

const noEmptyBlocks = createRule({
  name: 'no-empty-blocks',
  check: (ctx) => {
    if (ctx.node.children.length === 0 &&
        ['BlockStatement', 'codeBlock', 'element'].includes(ctx.node.type)) {
      return { message: 'Empty block detected' }
    }
  }
})
```

## Next Steps

- [Traversal](/guide/traversal) - All traversal methods
- [Query Index](/guide/querying) - O(1) node lookups
- [Plugins](/guide/plugins) - Transform and visitor plugins

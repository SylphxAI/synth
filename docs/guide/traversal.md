# Traversal

Synth provides multiple ways to traverse ASTs, from simple iteration to powerful visitor patterns.

## Method 1: Direct Array Iteration

The fastest way - iterate the flat `nodes` array:

```typescript
const tree = parse(source)

// Simple and fast!
for (const node of tree.nodes) {
  if (node.type === 'heading') {
    console.log('Heading level:', node.data?.depth)
  }
}

// With filtering
const headings = tree.nodes.filter(n => n.type === 'heading')
const functions = tree.nodes.filter(n => n.type === 'FunctionDeclaration')
```

**Best for**: Simple queries, maximum performance

## Method 2: traverse()

The visitor pattern for structured traversal:

```typescript
import { traverse, TraversalOrder } from '@sylphx/synth'

traverse(tree, {
  // Called when entering any node
  enter: (ctx) => {
    console.log(`Enter: ${ctx.node.type} at depth ${ctx.depth}`)

    // Return false to skip this subtree
    if (ctx.node.type === 'codeBlock') {
      return false
    }
  },

  // Called when leaving any node
  leave: (ctx) => {
    console.log(`Leave: ${ctx.node.type}`)
  },

  // Type-specific visitors
  heading: (ctx) => {
    console.log('Heading:', ctx.node.data?.depth)
  },

  paragraph: (ctx) => {
    console.log('Paragraph at line', ctx.node.span?.start.line)
  },

  FunctionDeclaration: (ctx) => {
    console.log('Function:', ctx.node.data?.id)
  }
})
```

### VisitorContext

Every visitor receives a context object:

```typescript
interface VisitorContext {
  tree: Tree              // The entire tree
  nodeId: NodeId          // Current node ID
  node: BaseNode          // Current node
  parentId: NodeId | null // Parent node ID
  depth: number           // Depth in tree (root = 0)
  index: number           // Index among siblings
  ancestors: NodeId[]     // All ancestor IDs (root → parent)
}
```

### Traversal Options

```typescript
traverse(tree, visitor, {
  // Traversal order
  order: TraversalOrder.PreOrder,    // Parent first (default)
  // order: TraversalOrder.PostOrder,  // Children first
  // order: TraversalOrder.BreadthFirst, // Level by level

  // Maximum depth (-1 for unlimited)
  maxDepth: 5,

  // Filter function
  filter: (ctx) => {
    // Return false to skip this subtree
    return ctx.node.type !== 'comment'
  }
})
```

### Traversal Orders

```typescript
// Pre-order: root → heading → text → paragraph → text
//            (process parent before children)
traverse(tree, visitor, { order: TraversalOrder.PreOrder })

// Post-order: text → heading → text → paragraph → root
//             (process children before parent)
traverse(tree, visitor, { order: TraversalOrder.PostOrder })

// Breadth-first: root → heading, paragraph → text, text
//                (level by level)
traverse(tree, visitor, { order: TraversalOrder.BreadthFirst })
```

## Method 3: select() and find()

Query nodes with predicates:

```typescript
import { select, find, selectByType } from '@sylphx/synth'

// Find all nodes matching a predicate
const deepNodes = select(tree, (ctx) => ctx.depth > 2)

// Find first match
const firstHeading = find(tree, (ctx) => ctx.node.type === 'heading')

// Find by type (shorthand)
const allHeadings = selectByType(tree, 'heading')
const allFunctions = selectByType(tree, 'FunctionDeclaration')
```

## Method 4: Query Index (O(1))

For large documents, build an index for instant lookups:

```typescript
import { createIndex } from '@sylphx/synth'

const index = createIndex(tree)

// O(1) type queries
const headings = index.findByType('heading')
const paragraphs = index.findByType('paragraph')

// O(1) data queries
const h2s = index.findByData('depth', 2)
const asyncFns = index.findByData('async', true)

// O(1) depth queries
const topLevel = index.findByDepth(1)
const nested = index.findByDepthRange(2, 5)

// O(1) relationship queries
const children = index.findChildren(parentId)
const parent = index.findParent(childId)

// Complex queries
const results = index.query({
  type: ['heading', 'paragraph'],
  depth: { min: 1, max: 3 },
  hasChildren: true,
  data: { depth: 2 }
})
```

**When to use**: Documents > 1000 nodes, repeated queries, complex filters

## Method 5: Zipper (Functional Navigation)

Navigate and modify trees functionally:

```typescript
import { createZipper } from '@sylphx/synth'

const zipper = createZipper(tree)

// Navigate
const down = zipper.down()      // First child
const right = down?.right()     // Next sibling
const left = right?.left()      // Previous sibling
const up = left?.up()           // Parent

// Chain navigation
const target = zipper
  .down()     // Into root's children
  ?.right()   // Second child
  ?.down()    // Into that node's children
  ?.node      // Get the node

// Modify (returns new zipper, immutable)
const modified = zipper
  .down()
  ?.replace({ ...zipper.node, data: { custom: true } })
  ?.root()    // Get back to root with changes
```

## Practical Examples

### Extract All Text Content

```typescript
function extractText(tree: Tree): string {
  const texts: string[] = []

  traverse(tree, {
    text: (ctx) => {
      if (ctx.node.data?.value) {
        texts.push(ctx.node.data.value as string)
      }
    }
  })

  return texts.join(' ')
}
```

### Find Functions with Specific Patterns

```typescript
function findAsyncFunctions(tree: Tree): BaseNode[] {
  return tree.nodes.filter(node =>
    node.type === 'FunctionDeclaration' &&
    node.data?.async === true
  )
}
```

### Build Table of Contents

```typescript
interface TOCEntry {
  level: number
  text: string
  line: number
}

function buildTOC(tree: Tree): TOCEntry[] {
  const toc: TOCEntry[] = []

  traverse(tree, {
    heading: (ctx) => {
      const level = ctx.node.data?.depth as number
      const text = getSourceText(tree, ctx.node)
        .replace(/^#+\s*/, '') // Remove # prefix

      toc.push({
        level,
        text,
        line: ctx.node.span?.start.line ?? 0
      })
    }
  })

  return toc
}
```

### Transform AST

```typescript
function uppercaseHeadings(tree: Tree): void {
  traverse(tree, {
    heading: (ctx) => {
      // Find text children
      for (const childId of ctx.node.children) {
        const child = getNode(tree, childId)
        if (child?.type === 'text' && child.data?.value) {
          child.data.value = (child.data.value as string).toUpperCase()
        }
      }
    }
  })
}
```

## Performance Comparison

| Method | Speed | Use Case |
|--------|-------|----------|
| `for...of tree.nodes` | ⚡⚡⚡ Fastest | Simple queries |
| `traverse()` | ⚡⚡ Fast | Structured traversal |
| `select()` / `find()` | ⚡⚡ Fast | Predicate queries |
| `createIndex()` | ⚡ Build once | Repeated queries, large docs |
| `createZipper()` | ⚡ | Functional modification |

## Next Steps

- [Query Index](/guide/querying) - Deep dive into O(1) queries
- [Plugins](/guide/plugins) - Transform and visitor plugins
- [Examples](/examples/ast-chunking) - Real-world examples

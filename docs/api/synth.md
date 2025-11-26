# @sylphx/synth

Core AST infrastructure - types, traversal, query index, and utilities.

## Installation

```bash
npm install @sylphx/synth
```

## Types

### Tree

```typescript
interface Tree {
  meta: TreeMetadata
  root: NodeId
  nodes: BaseNode[]
  strings: Map<string, number>
}

interface TreeMetadata {
  language: string
  source: string
  created: number
  modified: number
  data?: Record<string, unknown>
}
```

### BaseNode

```typescript
interface BaseNode {
  id: NodeId
  type: string
  span?: Span
  parent: NodeId | null
  children: NodeId[]
  data?: Record<string, unknown>
}
```

### Span & Position

```typescript
interface Span {
  start: Position
  end: Position
}

interface Position {
  line: number    // 1-based
  column: number  // 0-based
  offset: number  // 0-based byte offset
}
```

### NodeId

```typescript
type NodeId = number
```

## Tree Operations

### createTree

Create an empty tree:

```typescript
import { createTree } from '@sylphx/synth'

const tree = createTree('markdown', '# Hello')
```

### getNode

Get a node by ID:

```typescript
import { getNode } from '@sylphx/synth'

const node = getNode(tree, 0) // Returns BaseNode | undefined
```

### getRoot

Get the root node:

```typescript
import { getRoot } from '@sylphx/synth'

const root = getRoot(tree) // Returns RootNode
```

### getChildren

Get all children of a node:

```typescript
import { getChildren } from '@sylphx/synth'

const children = getChildren(tree, nodeId) // Returns BaseNode[]
```

### getParent

Get the parent of a node:

```typescript
import { getParent } from '@sylphx/synth'

const parent = getParent(tree, nodeId) // Returns BaseNode | null
```

### addNode

Add a new node to the tree:

```typescript
import { addNode } from '@sylphx/synth'

const newId = addNode(tree, {
  type: 'paragraph',
  parent: 0,
  children: []
})
```

### updateNode

Update a node in place:

```typescript
import { updateNode } from '@sylphx/synth'

updateNode(tree, nodeId, {
  data: { custom: 'value' }
})
```

### removeNode

Remove a node from its parent's children:

```typescript
import { removeNode } from '@sylphx/synth'

removeNode(tree, nodeId)
```

## Traversal

### traverse

Traverse the tree with a visitor:

```typescript
import { traverse, TraversalOrder } from '@sylphx/synth'

traverse(tree, {
  enter: (ctx) => {
    console.log(ctx.node.type)
    // Return false to skip subtree
  },
  leave: (ctx) => {
    console.log('leaving', ctx.node.type)
  },
  // Type-specific visitors
  heading: (ctx) => { ... },
  paragraph: (ctx) => { ... }
}, {
  order: TraversalOrder.PreOrder,
  maxDepth: 10,
  filter: (ctx) => ctx.node.type !== 'comment'
})
```

### VisitorContext

```typescript
interface VisitorContext {
  tree: Tree
  nodeId: NodeId
  node: BaseNode
  parentId: NodeId | null
  depth: number
  index: number
  ancestors: NodeId[]
}
```

### TraversalOrder

```typescript
const TraversalOrder = {
  PreOrder: 'pre-order',
  PostOrder: 'post-order',
  BreadthFirst: 'breadth-first'
}
```

### select

Find all nodes matching a predicate:

```typescript
import { select } from '@sylphx/synth'

const nodes = select(tree, (ctx) => ctx.depth > 2)
```

### find

Find the first node matching a predicate:

```typescript
import { find } from '@sylphx/synth'

const node = find(tree, (ctx) => ctx.node.type === 'heading')
```

### selectByType

Find all nodes of a specific type:

```typescript
import { selectByType } from '@sylphx/synth'

const headings = selectByType(tree, 'heading')
```

## Query Index

### createIndex

Build a query index for O(1) lookups:

```typescript
import { createIndex } from '@sylphx/synth'

const index = createIndex(tree)
```

### ASTIndex Methods

```typescript
// Find by type (O(1))
index.findByType('heading')
index.findByTypes(['heading', 'paragraph'])

// Find by data attribute (O(1))
index.findByData('depth', 2)

// Find by depth (O(1))
index.findByDepth(1)
index.findByDepthRange(1, 3)

// Find relationships (O(1))
index.findChildren(parentId)
index.findParent(childId)

// Complex queries
index.query({
  type: ['heading', 'paragraph'],
  depth: { min: 1, max: 3 },
  hasChildren: true,
  data: { depth: 2 }
})

// Statistics
index.getTypes()
index.getTypeCount('heading')
index.getTypeCounts()
index.getStats()
```

## Zipper

Functional tree navigation:

```typescript
import { createZipper } from '@sylphx/synth'

const zipper = createZipper(tree)

// Navigation
const child = zipper.down()
const sibling = child?.right()
const parent = sibling?.up()

// Get current node
const node = zipper.node

// Modify (immutable)
const modified = zipper.replace({ ...zipper.node, data: { foo: 'bar' } })

// Get back to root with changes
const newRoot = modified?.root()
```

## Plugin System

### createTransformPlugin

Create a plugin that transforms the tree:

```typescript
import { createTransformPlugin } from '@sylphx/synth'

const myPlugin = createTransformPlugin(
  { name: 'my-plugin', version: '1.0.0' },
  (tree) => {
    // Modify tree
    return tree
  }
)
```

### createVisitorPlugin

Create a plugin that visits nodes:

```typescript
import { createVisitorPlugin } from '@sylphx/synth'

const myPlugin = createVisitorPlugin(
  { name: 'counter', version: '1.0.0' },
  {
    enter: (node, context) => {
      context.data.count = (context.data.count || 0) + 1
    }
  }
)
```

## Type Guards

```typescript
import { isTextNode, isParentNode } from '@sylphx/synth'

if (isTextNode(node)) {
  console.log(node.value)
}

if (isParentNode(node)) {
  console.log(node.children.length)
}
```

## Errors

```typescript
import { SynthError, TreeStructureError, IndexNotBuiltError } from '@sylphx/synth'

try {
  // ...
} catch (e) {
  if (e instanceof IndexNotBuiltError) {
    // Index not built yet
  }
}
```

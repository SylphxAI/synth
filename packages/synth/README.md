# @sylphx/synth

Core AST infrastructure - language-agnostic types, utilities, and the world's fastest AST processor.

## Features

- **Tree & Node Types**: Fundamental AST data structures with arena-based storage
- **Query Index**: O(1) node lookup and queries (100-1000x faster than linear scans)
- **Traversal**: Multiple traversal strategies (DFS, BFS, visitor pattern)
- **Zipper**: Functional tree navigation with focus
- **Incremental Processing**: Token-level incremental parsing (10-100x faster re-parsing)
- **Batch Processing**: SIMD-style operations for 1.3-1.4x speedup on large trees
- **Node Pooling**: 70%+ object reuse rate, reduced GC pressure
- **Plugin System**: Extensible transform and visitor plugins

## Installation

```bash
npm install @sylphx/synth
```

## Usage

```typescript
import {
  Tree,
  BaseNode,
  createQueryIndex,
  traverse,
  createZipper,
  createTransformPlugin,
  createVisitorPlugin,
} from '@sylphx/synth'

// Create and query trees
const tree: Tree = { /* ... */ }
const index = createQueryIndex(tree)

// Find nodes by type
const headings = index.getByType('heading')

// Traverse with visitor
traverse(tree, {
  enter: (node) => console.log(node.type),
  exit: (node) => console.log('leaving', node.type)
})

// Functional navigation with zipper
const zipper = createZipper(tree)
const next = zipper.down()?.right()
```

### Plugin System

```typescript
import { createTransformPlugin, createVisitorPlugin } from '@sylphx/synth'

// Transform plugin - modify the tree
const addIds = createTransformPlugin(
  { name: 'add-ids', version: '1.0.0' },
  (tree) => {
    tree.nodes.forEach((node, i) => {
      if (!node.data) node.data = {}
      node.data.id = `node-${i}`
    })
    return tree
  }
)

// Visitor plugin - analyze the tree
const counter = createVisitorPlugin(
  { name: 'counter', version: '1.0.0' },
  {
    enter: (node, context) => {
      context.data.count = (context.data.count || 0) + 1
    }
  }
)
```

### Incremental Parsing

```typescript
import { IncrementalParserManager, detectEdit } from '@sylphx/synth'

const manager = new IncrementalParserManager()

// Parse initial document
manager.parse('file:///doc.md', initialText, 'markdown')

// On document change - <1ms response!
const edit = detectEdit(oldText, newText)
const { tree, tokenReuseRate } = manager.update('file:///doc.md', newText, edit)

console.log(`Token reuse: ${(tokenReuseRate * 100).toFixed(1)}%`)
// Output: Token reuse: 99.3%
```

## API

### Types
- `Tree` - Core tree structure with arena-based node storage
- `BaseNode` - Generic node interface
- `NodeId` - Node identifier type
- `Span` - Source location information

### Query Index
- `createQueryIndex(tree)` - Build index for fast queries
- `index.getByType(type)` - Find nodes by type
- `index.getBySpan(span)` - Find nodes by location

### Traversal
- `traverse(tree, visitor)` - DFS traversal with visitor
- `traverseBFS(tree, visitor)` - BFS traversal
- `createZipper(tree)` - Create zipper for functional navigation

### Incremental
- `IncrementalParserManager` - Multi-language incremental parser
- `detectEdit(oldText, newText)` - Detect edit between versions
- `IncrementalProcessor` - Track and process incremental updates
- `diffTrees(oldTree, newTree)` - Compute tree differences

### Optimizations
- `BatchProcessor` - Process multiple nodes efficiently
- `NodePool` - Object pooling for reduced GC pressure

### Plugin System
- `createTransformPlugin(meta, transform)` - Create transform plugin
- `createVisitorPlugin(meta, visitor)` - Create visitor plugin
- `PluginManager` - Manage and run plugins

## Performance

Built with performance as the #1 priority:

| Feature | Benefit |
|---------|---------|
| Arena-based storage | Cache-friendly, contiguous allocation |
| NodeId system | O(1) access, no pointer chasing |
| Query Index | 100-1000x faster than linear scans |
| Batch Processing | 1.3-1.4x speedup on large trees |
| Node Pooling | 70%+ object reuse, reduced GC |
| Incremental Parsing | 10-100x faster re-parsing |

## License

MIT

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

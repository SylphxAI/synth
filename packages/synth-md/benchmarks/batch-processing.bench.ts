import { bench, describe } from 'vitest'
import { batchSelect, batchTransform, batchTraverse } from '../src/core/batch-processor.js'
import { traverse } from '../src/core/traverse.js'
import { addNode, createTree } from '../src/types/tree.js'

/**
 * Benchmark: Batch Processing vs Traditional Traversal
 *
 * Tests the performance gains from SIMD-style batch processing
 * Expected: 3-5x improvement for large trees
 */

function createLargeTree(_nodeCount: number) {
  const tree = createTree('test', 'source')

  // Create a balanced tree structure
  // Level 1: 10 sections
  // Level 2: 10 paragraphs per section
  // Level 3: 10 text nodes per paragraph
  // Total: 1 + 10 + 100 + 1000 = 1111 nodes

  const sections = []
  for (let i = 0; i < 10; i++) {
    const sectionId = addNode(tree, { type: 'section', children: [], parent: 0 })
    sections.push(sectionId)

    const paragraphs = []
    for (let j = 0; j < 10; j++) {
      const paraId = addNode(tree, {
        type: 'paragraph',
        children: [],
        parent: sectionId,
      })
      paragraphs.push(paraId)

      const textNodes = []
      for (let k = 0; k < 10; k++) {
        const textId = addNode(tree, {
          type: 'text',
          children: [],
          parent: paraId,
          data: { value: `Text ${i}-${j}-${k}` },
        })
        textNodes.push(textId)
      }

      tree.nodes[paraId].children = textNodes
    }

    tree.nodes[sectionId].children = paragraphs
  }

  tree.nodes[0].children = sections

  return tree
}

describe('Batch Processing Performance', () => {
  const smallTree = createLargeTree(100)
  const largeTree = createLargeTree(1000)

  describe('Traversal', () => {
    bench('traditional: traverse small tree', () => {
      let _count = 0
      traverse(smallTree, {
        enter: () => {
          _count++
        },
      })
    })

    bench('batch: traverse small tree', () => {
      let _count = 0
      batchTraverse(smallTree, {
        batch: (nodes) => {
          _count += nodes.length
        },
      })
    })

    bench('traditional: traverse large tree', () => {
      let _count = 0
      traverse(largeTree, {
        enter: () => {
          _count++
        },
      })
    })

    bench('batch: traverse large tree', () => {
      let _count = 0
      batchTraverse(largeTree, {
        batch: (nodes) => {
          _count += nodes.length
        },
      })
    })

    bench('batch: traverse large tree with type grouping', () => {
      let _count = 0
      batchTraverse(
        largeTree,
        {
          batch: (nodes) => {
            _count += nodes.length
          },
        },
        { groupByType: true }
      )
    })
  })

  describe('Selection', () => {
    bench('traditional: select by type', () => {
      const results = []
      traverse(largeTree, {
        enter: (ctx) => {
          if (ctx.node.type === 'paragraph') {
            results.push(ctx.node)
          }
        },
      })
    })

    bench('batch: select by type', () => {
      const _results = batchSelect(largeTree, (node) => node.type === 'paragraph')
    })
  })

  describe('Transformation', () => {
    bench('traditional: transform nodes', () => {
      const tree = createLargeTree(1000)
      traverse(tree, {
        enter: (ctx) => {
          if (ctx.node.type === 'text') {
            tree.nodes[ctx.node.id] = {
              ...ctx.node,
              data: { ...ctx.node.data, processed: true },
            }
          }
        },
      })
    })

    bench('batch: transform nodes', () => {
      const tree = createLargeTree(1000)
      batchTransform(
        tree,
        (node) => node.type === 'text',
        (node) => ({
          ...node,
          data: { ...node.data, processed: true },
        })
      )
    })

    bench('batch: transform with type grouping', () => {
      const tree = createLargeTree(1000)
      batchTransform(
        tree,
        (node) => node.type === 'text',
        (node) => ({
          ...node,
          data: { ...node.data, processed: true },
        }),
        { groupByType: true }
      )
    })
  })

  describe('Cache Locality', () => {
    bench('batch: small batch size (cache-friendly)', () => {
      let _sum = 0
      batchTraverse(
        largeTree,
        {
          batch: (nodes) => {
            // Simulate cache-friendly access pattern
            for (const node of nodes) {
              _sum += node.id
            }
          },
        },
        { batchSize: 8 } // Fits in L1 cache
      )
    })

    bench('batch: medium batch size', () => {
      let _sum = 0
      batchTraverse(
        largeTree,
        {
          batch: (nodes) => {
            for (const node of nodes) {
              _sum += node.id
            }
          },
        },
        { batchSize: 16 } // Default
      )
    })

    bench('batch: large batch size', () => {
      let _sum = 0
      batchTraverse(
        largeTree,
        {
          batch: (nodes) => {
            for (const node of nodes) {
              _sum += node.id
            }
          },
        },
        { batchSize: 64 } // Larger batches
      )
    })
  })
})

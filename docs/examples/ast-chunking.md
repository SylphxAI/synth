# AST-Based Chunking

Learn how to use Synth for semantic document chunking - a common requirement for RAG (Retrieval-Augmented Generation) systems.

## Why AST-Based Chunking?

Traditional chunking methods split text arbitrarily:
- Fixed-size chunks break sentences/paragraphs
- Overlap-based chunking wastes tokens
- No semantic understanding

AST-based chunking respects document structure:
- Preserves complete semantic units
- Includes metadata (heading levels, code languages)
- Perfect source location tracking

## Basic Markdown Chunking

```typescript
import { parse, traverse, getNode, type Tree, type BaseNode } from '@sylphx/synth-md'

interface Chunk {
  id: string
  type: string
  content: string
  startLine: number
  endLine: number
  metadata: Record<string, unknown>
}

function getSourceText(tree: Tree, node: BaseNode): string {
  if (!node.span) return ''
  return tree.meta.source.slice(
    node.span.start.offset,
    node.span.end.offset
  )
}

function chunkMarkdown(source: string): Chunk[] {
  const tree = parse(source)
  const chunks: Chunk[] = []

  // Get root's direct children (top-level blocks)
  const root = tree.nodes[0]!

  for (const childId of root.children) {
    const node = getNode(tree, childId)
    if (!node || !node.span) continue

    const chunk: Chunk = {
      id: `chunk-${node.id}`,
      type: node.type,
      content: getSourceText(tree, node),
      startLine: node.span.start.line,
      endLine: node.span.end.line,
      metadata: {}
    }

    // Add semantic metadata
    switch (node.type) {
      case 'heading':
        chunk.metadata.level = node.data?.depth
        chunk.metadata.isTitle = node.data?.depth === 1
        break
      case 'codeBlock':
        chunk.metadata.language = node.data?.language || 'text'
        break
      case 'list':
        chunk.metadata.ordered = node.data?.ordered ?? false
        chunk.metadata.itemCount = node.children.length
        break
    }

    chunks.push(chunk)
  }

  return chunks
}
```

## Usage

```typescript
const markdown = `# Introduction

This document explains our API.

## Authentication

All requests require an API key.

\`\`\`typescript
const client = new APIClient({ apiKey: 'xxx' })
\`\`\`

## Endpoints

### GET /users

Returns a list of users.

- Pagination supported
- Filter by status
`

const chunks = chunkMarkdown(markdown)
console.log(JSON.stringify(chunks, null, 2))
```

Output:

```json
[
  {
    "id": "chunk-1",
    "type": "heading",
    "content": "# Introduction",
    "startLine": 1,
    "endLine": 1,
    "metadata": { "level": 1, "isTitle": true }
  },
  {
    "id": "chunk-2",
    "type": "paragraph",
    "content": "This document explains our API.",
    "startLine": 3,
    "endLine": 3,
    "metadata": {}
  },
  {
    "id": "chunk-3",
    "type": "heading",
    "content": "## Authentication",
    "startLine": 5,
    "endLine": 5,
    "metadata": { "level": 2, "isTitle": false }
  },
  {
    "id": "chunk-4",
    "type": "codeBlock",
    "content": "```typescript\nconst client = new APIClient({ apiKey: 'xxx' })\n```",
    "startLine": 9,
    "endLine": 11,
    "metadata": { "language": "typescript" }
  }
]
```

## Hierarchical Chunking

Group content under headings for better context:

```typescript
interface HierarchicalChunk {
  id: string
  heading: string
  level: number
  content: string
  children: HierarchicalChunk[]
  startLine: number
  endLine: number
}

function hierarchicalChunk(source: string): HierarchicalChunk[] {
  const tree = parse(source)
  const root = tree.nodes[0]!
  const result: HierarchicalChunk[] = []
  const stack: HierarchicalChunk[] = []

  let currentChunk: HierarchicalChunk | null = null
  let contentParts: string[] = []

  for (const childId of root.children) {
    const node = getNode(tree, childId)!
    const text = getSourceText(tree, node)

    if (node.type === 'heading') {
      // Save previous chunk
      if (currentChunk) {
        currentChunk.content = contentParts.join('\n\n')
        contentParts = []
      }

      const level = node.data?.depth as number

      // Pop stack until we find a lower level
      while (stack.length > 0 && stack[stack.length - 1]!.level >= level) {
        stack.pop()
      }

      // Create new chunk
      currentChunk = {
        id: `section-${node.id}`,
        heading: text.replace(/^#+\s*/, ''),
        level,
        content: '',
        children: [],
        startLine: node.span?.start.line ?? 0,
        endLine: node.span?.end.line ?? 0
      }

      // Add to parent or root
      if (stack.length > 0) {
        stack[stack.length - 1]!.children.push(currentChunk)
      } else {
        result.push(currentChunk)
      }

      stack.push(currentChunk)
    } else if (currentChunk) {
      contentParts.push(text)
      currentChunk.endLine = node.span?.end.line ?? currentChunk.endLine
    }
  }

  // Save last chunk
  if (currentChunk) {
    currentChunk.content = contentParts.join('\n\n')
  }

  return result
}
```

## Multi-Language Chunking

Chunk any supported language:

```typescript
import { parse as parseMD } from '@sylphx/synth-md'
import { parse as parseJS, findFunctions } from '@sylphx/synth-js'
import { parse as parseHTML, isElementNode, getTagName } from '@sylphx/synth-html'

// Chunk JavaScript by functions
function chunkJavaScript(source: string) {
  const tree = parseJS(source)
  const chunks = []

  for (const node of tree.nodes) {
    if (node.type === 'FunctionDeclaration' ||
        node.type === 'ArrowFunctionExpression' ||
        node.type === 'ClassDeclaration') {
      chunks.push({
        type: node.type,
        name: node.data?.id || 'anonymous',
        content: getSourceText(tree, node),
        startLine: node.span?.start.line,
        endLine: node.span?.end.line
      })
    }
  }

  return chunks
}

// Chunk HTML by semantic sections
function chunkHTML(source: string) {
  const tree = parseHTML(source)
  const chunks = []

  const semanticTags = ['header', 'nav', 'main', 'article', 'section', 'aside', 'footer']

  for (const node of tree.nodes) {
    if (isElementNode(node)) {
      const tag = getTagName(node)
      if (tag && semanticTags.includes(tag)) {
        chunks.push({
          type: 'section',
          tag,
          content: getSourceText(tree, node),
          startLine: node.span?.start.line,
          endLine: node.span?.end.line
        })
      }
    }
  }

  return chunks
}
```

## Chunking with Size Limits

Combine AST structure with size constraints:

```typescript
interface SizedChunk {
  content: string
  tokens: number  // Approximate
  nodes: number[]
  startLine: number
  endLine: number
}

function chunkWithSizeLimit(
  source: string,
  maxTokens: number = 500
): SizedChunk[] {
  const tree = parse(source)
  const root = tree.nodes[0]!
  const chunks: SizedChunk[] = []

  let currentChunk: SizedChunk = {
    content: '',
    tokens: 0,
    nodes: [],
    startLine: 0,
    endLine: 0
  }

  // Rough token estimation (4 chars ≈ 1 token)
  const estimateTokens = (text: string) => Math.ceil(text.length / 4)

  for (const childId of root.children) {
    const node = getNode(tree, childId)!
    const text = getSourceText(tree, node)
    const nodeTokens = estimateTokens(text)

    // If this node alone exceeds limit, it becomes its own chunk
    if (nodeTokens > maxTokens) {
      // Save current chunk if not empty
      if (currentChunk.nodes.length > 0) {
        chunks.push(currentChunk)
      }

      chunks.push({
        content: text,
        tokens: nodeTokens,
        nodes: [node.id],
        startLine: node.span?.start.line ?? 0,
        endLine: node.span?.end.line ?? 0
      })

      currentChunk = { content: '', tokens: 0, nodes: [], startLine: 0, endLine: 0 }
      continue
    }

    // Would adding this exceed the limit?
    if (currentChunk.tokens + nodeTokens > maxTokens && currentChunk.nodes.length > 0) {
      chunks.push(currentChunk)
      currentChunk = { content: '', tokens: 0, nodes: [], startLine: 0, endLine: 0 }
    }

    // Add to current chunk
    if (currentChunk.nodes.length === 0) {
      currentChunk.startLine = node.span?.start.line ?? 0
    }
    currentChunk.content += (currentChunk.content ? '\n\n' : '') + text
    currentChunk.tokens += nodeTokens
    currentChunk.nodes.push(node.id)
    currentChunk.endLine = node.span?.end.line ?? 0
  }

  // Don't forget the last chunk
  if (currentChunk.nodes.length > 0) {
    chunks.push(currentChunk)
  }

  return chunks
}
```

## Performance Tips

1. **Use direct iteration** for simple chunking:
   ```typescript
   // Fast!
   for (const node of tree.nodes) { ... }
   ```

2. **Build index** for complex queries on large docs:
   ```typescript
   const tree = parse(source, { buildIndex: true })
   const index = createIndex(tree)
   ```

3. **Reuse parser** for multiple documents:
   ```typescript
   import { Parser } from '@sylphx/synth-md'
   const parser = new Parser()

   for (const doc of documents) {
     const tree = parser.parse(doc)
     // Process...
   }
   ```

## Next Steps

- [Code Analysis](/examples/code-analysis) - Analyze JavaScript/TypeScript
- [Multi-Language](/examples/multi-language) - Work with multiple languages
- [Traversal Guide](/guide/traversal) - Deep dive into traversal methods

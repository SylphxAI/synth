# @sylphx/synth-html

High-performance HTML5 parser for Synth. Converts HTML into a language-agnostic AST that works seamlessly with the Synth ecosystem.

## Features

- **Language-Agnostic AST**: Uses Synth's universal `BaseNode` interface
- **Full HTML5 Support**: DOCTYPE, elements, attributes, void elements, self-closing tags
- **Streaming-Capable**: Ready for future streaming implementation
- **Plugin System**: Compatible with Synth's transform and visitor plugins
- **Async Support**: Both sync and async parsing with automatic async plugin detection
- **TypeScript**: Fully typed with comprehensive type utilities

## Installation

```bash
npm install @sylphx/synth-html
```

## Usage

### Basic Parsing

```typescript
import { parse } from '@sylphx/synth-html'

const tree = parse('<div class="container"><h1>Hello</h1></div>')
```

### Using the Parser Class

```typescript
import { HTMLParser } from '@sylphx/synth-html'

const parser = new HTMLParser()
const tree = parser.parse('<html><body><p>Content</p></body></html>')

// Access the tree
console.log(tree.nodes)
```

### Working with AST Nodes

```typescript
import {
  parse,
  isElementNode,
  getTagName,
  getAttribute,
  isTextNode,
  getTextValue,
} from '@sylphx/synth-html'

const tree = parse('<div id="main" class="container">Hello World</div>')

// Find elements
const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')

// Get attributes
const id = getAttribute(div, 'id') // "main"
const className = getAttribute(div, 'class') // "container"

// Find text nodes
const text = tree.nodes.find(n => isTextNode(n))
const value = getTextValue(text) // "Hello World"
```

### Plugin Support

```typescript
import { parse } from '@sylphx/synth-html'
import { createTransformPlugin } from '@sylphx/synth'

// Create a plugin
const addIdsPlugin = createTransformPlugin(
  { name: 'add-ids', version: '1.0.0' },
  (tree) => {
    // Transform the tree
    return tree
  }
)

// Use the plugin
const tree = parse('<div>Content</div>', {
  plugins: [addIdsPlugin],
})
```

### Async Parsing

```typescript
import { parseAsync } from '@sylphx/synth-html'
import { createTransformPlugin } from '@sylphx/synth'

const asyncPlugin = createTransformPlugin(
  { name: 'async-transform', version: '1.0.0' },
  async (tree) => {
    // Async transformation
    await somethingAsync()
    return tree
  }
)

const tree = await parseAsync('<div>Content</div>', {
  plugins: [asyncPlugin],
})
```

### Registered Plugins

```typescript
import { HTMLParser } from '@sylphx/synth-html'
import { createTransformPlugin } from '@sylphx/synth'

const parser = new HTMLParser()

// Register plugins
parser
  .use(plugin1)
  .use(plugin2)
  .use(plugin3)

// Plugins apply to all parse() calls
const tree = parser.parse('<div>Content</div>')
```

## API Reference

### Parsing Functions

- `parse(html, options?)`: Parse HTML synchronously
- `parseAsync(html, options?)`: Parse HTML asynchronously
- `createParser()`: Create a new parser instance

### Type Guards

- `isDocumentNode(node)`: Check if node is root document
- `isDoctypeNode(node)`: Check if node is DOCTYPE declaration
- `isElementNode(node)`: Check if node is an HTML element
- `isTextNode(node)`: Check if node is a text node
- `isCommentNode(node)`: Check if node is a comment
- `isCDATANode(node)`: Check if node is a CDATA section

### Data Accessors

**Elements:**
- `getTagName(node)`: Get element tag name
- `getAttributes(node)`: Get all attributes
- `getAttribute(node, name)`: Get specific attribute
- `isVoidElement(node)`: Check if element is void (br, img, etc.)
- `isSelfClosing(node)`: Check if element is self-closing

**Text/Comments/CDATA:**
- `getTextValue(node)`: Get text content
- `getCommentValue(node)`: Get comment content
- `getCDATAValue(node)`: Get CDATA content

**DOCTYPE:**
- `getDoctypeName(node)`: Get DOCTYPE name
- `getDoctypePublicId(node)`: Get PUBLIC identifier
- `getDoctypeSystemId(node)`: Get SYSTEM identifier

## Parse Options

```typescript
interface HTMLParseOptions {
  /** Build query index for fast lookups */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]
}
```

## Node Structure

All nodes follow Synth's language-agnostic `BaseNode` interface:

```typescript
interface BaseNode {
  id: number
  type: string  // 'element', 'text', 'comment', etc.
  parent: number | null
  children: number[]
  data?: Record<string, unknown>  // HTML-specific data
}
```

HTML-specific data is stored in the `data` field:

```typescript
// Element node
{
  id: 1,
  type: 'element',
  parent: 0,
  children: [2, 3],
  data: {
    tagName: 'div',
    attributes: { class: 'container', id: 'main' },
    selfClosing: false,
    void: false
  }
}

// Text node
{
  id: 2,
  type: 'text',
  parent: 1,
  children: [],
  data: {
    value: 'Hello World'
  }
}
```

## Performance

Built on the same architecture as `@sylphx/synth-md` (26-42x faster than remark), this parser is designed for:

- **Fast tokenization**: Character-based parsing with minimal allocations
- **Efficient tree building**: Arena-based storage for cache locality
- **Streaming-ready**: Architecture supports future streaming implementation

## Examples

### Parse a Complete HTML Document

```typescript
import { parse, isElementNode, getTagName } from '@sylphx/synth-html'

const html = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>My Page</title>
  </head>
  <body>
    <h1>Welcome</h1>
    <p>Content here</p>
  </body>
</html>`

const tree = parse(html)

// Find all headings
const headings = tree.nodes.filter(n =>
  isElementNode(n) && getTagName(n)?.startsWith('h')
)
```

### Transform HTML

```typescript
import { parse, isElementNode, getTagName } from '@sylphx/synth-html'
import { createTransformPlugin } from '@sylphx/synth'

// Add IDs to all headings
const addHeadingIds = createTransformPlugin(
  { name: 'add-heading-ids', version: '1.0.0' },
  (tree) => {
    tree.nodes.forEach(node => {
      if (isElementNode(node)) {
        const tag = getTagName(node)
        if (tag && /^h[1-6]$/.test(tag)) {
          // Add id attribute
          if (!node.data) node.data = {}
          const attrs = node.data.attributes as Record<string, string>
          if (!attrs.id) {
            attrs.id = `heading-${node.id}`
          }
        }
      }
    })
    return tree
  }
)

const tree = parse('<h1>Title</h1><h2>Subtitle</h2>', {
  plugins: [addHeadingIds],
})
```

## License

MIT

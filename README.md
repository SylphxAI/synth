# Synth

> **The World's Fastest AST Processor** - 50-3000x faster than unified

[![npm](https://img.shields.io/npm/v/@sylphx/synth)](https://www.npmjs.com/package/@sylphx/synth)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- ⚡ **50-3000x faster** than unified/remark
- 🌍 **Universal AST** - Same interface for 19+ languages
- 🔄 **Incremental parsing** - <1ms response time, 99%+ token reuse
- 📦 **Zero dependencies** - Minimal bundle size
- 🛠️ **Rich tooling** - Formatters, minifiers, linters, metrics

## Quick Start

```bash
npm install @sylphx/synth @sylphx/synth-md
```

```typescript
import { parse, traverse, getNode } from '@sylphx/synth-md'

// Parse markdown (42x faster than remark!)
const tree = parse('# Hello **World**')

// Traverse the AST
traverse(tree, {
  heading: (ctx) => console.log('Heading level:', ctx.node.data?.depth),
  strong: (ctx) => console.log('Found bold text')
})

// Get source text from any node
const heading = tree.nodes[1]
const source = tree.meta.source.slice(
  heading.span.start.offset,
  heading.span.end.offset
)
```

## Node Structure

All languages share the same `BaseNode` interface:

```typescript
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

## Supported Languages

| Category | Languages |
|----------|-----------|
| Markup | Markdown, HTML, CSS, JSX, Vue |
| Programming | JavaScript/TypeScript, Python, Go, Rust, Java, PHP, Ruby, C |
| Data | JSON, YAML, TOML, INI, XML |
| Query | SQL, GraphQL, Protocol Buffers |

## Performance

| Operation | Synth | unified | Speedup |
|-----------|-------|---------|---------|
| Parse 1KB | 0.001 ms | 0.10 ms | **92x** |
| Parse 3KB | 0.005 ms | 0.58 ms | **520x** |
| Parse 10KB | 0.03 ms | 3.50 ms | **3154x** |

## Documentation

📚 **[Full Documentation](https://synth.sylphx.ai)** (coming soon)

- [Getting Started](./docs/guide/getting-started.md)
- [Core Concepts](./docs/guide/core-concepts.md)
- [Traversal Guide](./docs/guide/traversal.md)
- [API Reference](./docs/api/synth.md)
- [Examples](./docs/examples/ast-chunking.md)

## Packages

### Core
- `@sylphx/synth` - Core types, traversal, query index

### Parsers
- `@sylphx/synth-md` - Markdown (CommonMark + GFM)
- `@sylphx/synth-js` - JavaScript/TypeScript
- `@sylphx/synth-html` - HTML5
- `@sylphx/synth-json` - JSON
- `@sylphx/synth-yaml` - YAML
- `@sylphx/synth-css` - CSS3
- [View all packages...](./docs/api/index.md)

### Tools
- `@sylphx/synth-js-format` - JavaScript formatter
- `@sylphx/synth-js-minify` - JavaScript minifier
- `@sylphx/synth-lint` - Universal linter
- `@sylphx/synth-metrics` - Code metrics

## Development

```bash
# Install dependencies
bun install

# Build all packages
bun run build

# Run tests
bun run test

# Run docs locally
bun run docs:dev
```

## License

MIT

---

<div align="center">
  <sub>Powered by Sylphx · Built with <a href="https://www.npmjs.com/package/@sylphx/biome-config">biome-config</a> · <a href="https://www.npmjs.com/package/@sylphx/tsconfig">tsconfig</a> · <a href="https://www.npmjs.com/package/@sylphx/doctor">doctor</a> · <a href="https://www.npmjs.com/package/@sylphx/bump">bump</a></sub>
</div>

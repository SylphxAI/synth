---
layout: home

hero:
  name: Synth
  text: The World's Fastest AST Processor
  tagline: 50-3000x faster than unified. Zero dependencies. Universal AST.
  image:
    src: /logo.svg
    alt: Synth
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/SylphxAI/synth

features:
  - icon: ⚡
    title: Blazing Fast
    details: 50-3000x faster than unified/remark. Pure TypeScript, no native dependencies.
  - icon: 🌍
    title: Universal AST
    details: Same BaseNode interface across 19+ languages. Parse once, use everywhere.
  - icon: 🔄
    title: Incremental Parsing
    details: Token-level incremental updates with 99%+ reuse. <1ms response time.
  - icon: 🔌
    title: Plugin System
    details: Transform and visitor plugins. Compose pipelines with ease.
  - icon: 📦
    title: Zero Dependencies
    details: Core library has zero runtime dependencies. Minimal bundle size.
  - icon: 🛠️
    title: Developer Tools
    details: Formatters, minifiers, linters, metrics, and documentation generators.
---

## Quick Example

```typescript
import { parse, traverse, getNode } from '@sylphx/synth-md'

// Parse markdown (42x faster than remark!)
const tree = parse('# Hello **World**')

// Traverse the AST
traverse(tree, {
  heading: (ctx) => {
    console.log('Found heading:', ctx.node.data?.depth)
  },
  strong: (ctx) => {
    console.log('Found bold text')
  }
})

// Get source text from any node
const node = tree.nodes[1] // heading node
const source = tree.meta.source.slice(
  node.span.start.offset,
  node.span.end.offset
)
// → "# Hello **World**"
```

## Supported Languages

<div class="language-grid">

| Web & Markup | Programming | Data & Config | Query & Schema |
|--------------|-------------|---------------|----------------|
| HTML | JavaScript/TypeScript | JSON | SQL |
| Markdown | Python | YAML | GraphQL |
| CSS | Go | TOML | Protocol Buffers |
| JSX/TSX | Rust | INI | MessagePack |
| Vue SFC | Java, PHP, Ruby, C | XML | |

</div>

## Performance

| Operation | Synth | unified | Speedup |
|-----------|-------|---------|---------|
| Parse small (1KB) | 0.001 ms | 0.10 ms | **92x** |
| Parse medium (3KB) | 0.005 ms | 0.58 ms | **520x** |
| Parse large (10KB) | 0.03 ms | 3.50 ms | **3154x** |
| Transform | 0.005 ms | 0.58 ms | **110x** |

## Install

```bash
npm install @sylphx/synth @sylphx/synth-md
```

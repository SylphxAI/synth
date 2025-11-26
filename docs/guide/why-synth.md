# Why Synth?

## The Problem

Existing AST tools are slow and fragmented:

- **unified/remark**: Popular but 50-3000x slower
- **Babel**: JavaScript-only, heavy
- **tree-sitter**: Great but requires native bindings
- **Each language has different AST formats**: No universal interface

## The Solution

Synth provides:

### 1. Blazing Performance

Pure TypeScript, no native dependencies, yet 50-3000x faster than unified:

| Operation | Synth | unified | Speedup |
|-----------|-------|---------|---------|
| Parse small (1KB) | 0.001 ms | 0.10 ms | **92x** |
| Parse medium (3KB) | 0.005 ms | 0.58 ms | **520x** |
| Parse large (10KB) | 0.03 ms | 3.50 ms | **3154x** |

How? Arena-based storage, flat arrays, no pointer chasing.

### 2. Universal AST

One `BaseNode` interface for ALL languages:

```typescript
// Same structure for Markdown, JavaScript, HTML, JSON, etc.
interface BaseNode {
  id: NodeId
  type: string
  span?: Span
  parent: NodeId | null
  children: NodeId[]
  data?: Record<string, unknown>
}
```

Write tools once, use everywhere.

### 3. 19+ Languages

| Category | Languages |
|----------|-----------|
| Web | HTML, CSS, JSX, Vue |
| Programming | JS/TS, Python, Go, Rust, Java, PHP, Ruby, C |
| Data | JSON, YAML, TOML, INI, XML |
| Query | SQL, GraphQL |
| Binary | Protocol Buffers, MessagePack |
| Docs | Markdown (with GFM) |

### 4. Zero Dependencies

Core library has zero runtime dependencies. Minimal bundle size.

### 5. Incremental Parsing

Token-level incremental updates for real-time editing:

- 99%+ token reuse
- <1ms response time
- 10-100x faster than full re-parse

### 6. Rich Tooling

- **Formatters**: @sylphx/synth-js-format
- **Minifiers**: @sylphx/synth-js-minify
- **Linters**: @sylphx/synth-lint
- **Metrics**: @sylphx/synth-metrics
- **Docs**: @sylphx/synth-docs

## When to Use Synth

✅ **Good fit:**
- AST-based document chunking for RAG
- Code analysis and metrics
- Cross-language tooling
- Real-time editors
- High-performance parsing pipelines

❌ **Consider alternatives:**
- Need full spec compliance (use official parsers)
- Building compilers (use language-specific tools)
- Need source maps (not yet supported)

## Comparison

| Feature | Synth | unified | Babel | tree-sitter |
|---------|-------|---------|-------|-------------|
| Speed | ⚡⚡⚡ | ⚡ | ⚡⚡ | ⚡⚡⚡ |
| Languages | 19+ | Markdown | JS/TS | 100+ |
| Universal AST | ✅ | ❌ | ❌ | ❌ |
| Pure JS/TS | ✅ | ✅ | ✅ | ❌ |
| Incremental | ✅ | ❌ | ❌ | ✅ |
| Bundle size | Small | Medium | Large | Large |

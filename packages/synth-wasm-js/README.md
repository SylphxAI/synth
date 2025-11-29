# @sylphx/synth-wasm-js

High-performance JavaScript/TypeScript parser compiled to WebAssembly.

## Features

- **ES2024 syntax** - Full support for modern JavaScript
- **Fast** - 175 MB/s throughput, ~7μs per KB
- **Compact** - 48KB WASM binary
- **Zero dependencies** - Just the WASM module

## Installation

```bash
bun add @sylphx/synth-wasm-js
```

## Usage

```typescript
import { parse, parseCount, parseBinary, NodeKind } from '@sylphx/synth-wasm-js'

// Parse to structured AST
const result = await parse('const x = 1;')
console.log(result.nodes) // Array of ASTNode

// Quick node count (for validation)
const count = await parseCount('function foo() {}')
console.log(count) // 5

// Maximum performance (raw binary)
const binary = await parseBinary('class Foo {}')
```

## API

### `parse(source: string): Promise<ParseResult>`

Parse JavaScript and return a structured AST.

```typescript
const result = await parse('const x = 1 + 2;')

// result.nodes is an array of ASTNode:
// - kind: NodeKind (Program, VariableDeclaration, etc.)
// - flags: node flags (const, let, async, etc.)
// - start: source start offset
// - end: source end offset
// - extra: additional data (e.g., child count)
```

### `parseBinary(source: string): Promise<Uint8Array>`

Parse to compact binary format for maximum performance.

Binary format:
- Header: `[node_count: u32]`
- Nodes: 16 bytes each `(kind: u8, flags: u8, pad: u16, start: u32, end: u32, extra: u32)`

### `parseCount(source: string): Promise<number>`

Returns only the node count. Useful for benchmarking.

### `tokenize(source: string): Promise<number>`

Returns token count. Useful for benchmarking the lexer.

## Supported Syntax

- **Declarations**: `const`, `let`, `var`, `function`, `class`, `import`, `export`
- **Statements**: `if`, `for`, `for-in`, `for-of`, `while`, `do-while`, `switch`, `try/catch/finally`, `return`, `throw`, `break`, `continue`
- **Expressions**: Binary, unary, call, member, new, arrow functions, async/await, yield, template literals, object/array literals, spread, optional chaining, nullish coalescing
- **Patterns**: Array destructuring, object destructuring, rest elements

## Performance

| Metric | Value |
|--------|-------|
| Throughput | 175 MB/s |
| Per 1KB parse | ~7μs |
| WASM size | 48KB |

## Credits

Built with Rust and `wasm-bindgen`.

## License

MIT

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>


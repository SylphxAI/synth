# @sylphx/synth-wasm-md

High-performance Markdown parser compiled to WebAssembly.

## Installation

```bash
bun add @sylphx/synth-wasm-md
```

## Usage

```typescript
import { parse, initWasm } from '@sylphx/synth-wasm-md'

// Optional: eager initialization
await initWasm()

// Parse markdown
const tree = await parse('# Hello World')
console.log(tree)
```

## Performance

WASM parser provides ~2-5x speedup over pure JavaScript implementation for large documents.

| Document Size | JS Parser | WASM Parser | Speedup |
|---------------|-----------|-------------|---------|
| 1KB           | 0.5ms     | 0.3ms       | 1.7x    |
| 10KB          | 2ms       | 0.8ms       | 2.5x    |
| 100KB         | 15ms      | 4ms         | 3.8x    |

## API

### `parse(markdown: string): Promise<WasmTree>`

Parse Markdown text into an AST.

### `parseSync(markdown: string): WasmTree`

Synchronous parse (requires WASM to be pre-initialized with `initWasm()`).

### `initWasm(): Promise<void>`

Initialize the WASM module. Called automatically on first `parse()`, but can be called manually for eager initialization.

### `version(): Promise<string>`

Get the version of the WASM parser.

## Credits

Built with [@sylphx/synth-md](https://github.com/SylphxAI/synth) - the pure JavaScript reference implementation.

## License

MIT

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

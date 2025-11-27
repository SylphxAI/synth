# @sylphx/synth-toml

TOML parser using Synth's universal AST. Hand-written, zero dependencies.

## Features

- ✅ **100% In-House** - Hand-written tokenizer and parser, zero external dependencies
- 🚀 **TOML 1.0 Support** - Full spec compliance
- 🎯 **Universal AST** - Converts TOML to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Lightweight** - No dependencies except @sylphx/synth core

## Installation

```bash
npm install @sylphx/synth-toml
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-toml'

const toml = `
  [package]
  name = "my-package"
  version = "0.1.0"

  [dependencies]
  serde = "1.0"
`

const tree = parse(toml)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { TOMLParser, createParser, parse, parseAsync } from '@sylphx/synth-toml'

// Standalone function (recommended)
const tree = parse('key = "value"')

// Async parsing (for plugins)
const tree = await parseAsync('key = "value"')

// Class instance
const parser = new TOMLParser()
const tree = parser.parse('key = "value"')

// Factory function
const parser = createParser()
const tree = parser.parse('key = "value"')
```

### Plugin System

```typescript
import { parse, type Tree } from '@sylphx/synth-toml'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse('key = "value"', { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync('key = "value"', { plugins: [asyncPlugin] })
```

### Persistent Parser

```typescript
const parser = new TOMLParser()

// Register plugins
parser.use(plugin1).use(plugin2)

// Parse multiple files
const tree1 = parser.parse(toml1)
const tree2 = parser.parse(toml2)

// Get last parsed tree
const lastTree = parser.getTree()
```

## AST Structure

The parser generates a universal Synth AST with these TOML-specific node types:

### Table

```typescript
{
  type: 'Table',
  data: {
    name: 'package'
  },
  children: [/* KeyValue nodes */]
}
```

### TableArray

```typescript
{
  type: 'TableArray',
  data: {
    name: 'products'
  },
  children: [/* KeyValue nodes */]
}
```

### KeyValue

```typescript
{
  type: 'KeyValue',
  data: {
    key: 'name'
  },
  children: [/* Value node */]
}
```

### Value Types

```typescript
{ type: 'String', data: { value: 'text' } }
{ type: 'Integer', data: { value: '42' } }
{ type: 'Float', data: { value: '3.14' } }
{ type: 'Boolean', data: { value: 'true' } }
{ type: 'DateTime', data: { value: '1979-05-27T07:32:00Z' } }
{ type: 'Array', children: [/* Value nodes */] }
{ type: 'InlineTable', children: [/* KeyValue nodes */] }
```

## Supported TOML Features

### Data Types
- ✅ Strings (basic, literal, multi-line)
- ✅ Integers (decimal, underscores)
- ✅ Floats (decimal, scientific notation)
- ✅ Booleans (`true`, `false`)
- ✅ Dates and times (RFC 3339)
- ✅ Arrays (homogeneous, multi-line)
- ✅ Inline tables

### Tables
- ✅ Tables: `[table]`
- ✅ Nested tables: `[parent.child]`
- ✅ Array of tables: `[[array]]`
- ✅ Dotted keys: `parent.child = value`

### Syntax
- ✅ Comments: `# comment`
- ✅ Escape sequences in strings
- ✅ Multi-line strings (basic and literal)
- ✅ Trailing commas in arrays
- ✅ Whitespace flexibility

## Examples

### Cargo.toml (Rust)

```typescript
const toml = `
  [package]
  name = "my-crate"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  serde = "1.0"
  tokio = { version = "1.0", features = ["full"] }

  [[bin]]
  name = "my-binary"
  path = "src/main.rs"
`

const tree = parse(toml)
```

### pyproject.toml (Python)

```typescript
const toml = `
  [tool.poetry]
  name = "my-project"
  version = "0.1.0"
  description = "A Python project"

  [tool.poetry.dependencies]
  python = "^3.9"
  requests = "^2.28.0"

  [tool.poetry.dev-dependencies]
  pytest = "^7.0"
`

const tree = parse(toml)
```

### Configuration File

```typescript
const toml = `
  # Application Configuration
  title = "My App"

  [database]
  server = "192.168.1.1"
  ports = [8000, 8001, 8002]
  connection_max = 5000
  enabled = true

  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"

  [servers.beta]
  ip = "10.0.0.2"
  dc = "eqdc10"
`

const tree = parse(toml)
```

### Arrays

```typescript
const toml = `
  # Arrays
  integers = [1, 2, 3]
  colors = ["red", "yellow", "green"]
  nested_arrays = [[1, 2], [3, 4, 5]]

  # Multi-line arrays
  contributors = [
    "Foo Bar <foo@example.com>",
    "Baz Qux <baz@example.com>"
  ]
`

const tree = parse(toml)
```

### Inline Tables

```typescript
const toml = `
  # Inline tables
  name = { first = "Tom", last = "Preston-Werner" }
  point = { x = 1, y = 2 }
  animal = { type.name = "pug" }
`

const tree = parse(toml)
```

### Date and Time

```typescript
const toml = `
  # Offset Date-Time
  odt1 = 1979-05-27T07:32:00Z
  odt2 = 1979-05-27T00:32:00-07:00

  # Local Date-Time
  ldt1 = 1979-05-27T07:32:00

  # Local Date
  ld1 = 1979-05-27
`

const tree = parse(toml)
```

## Performance

Hand-written parser optimized for:
- Fast tokenization
- Minimal memory allocations
- Arena-based AST storage (O(1) node access)
- Zero external dependencies

## Development Philosophy

This package is **100% in-house** - we own the entire parsing pipeline:

- **Hand-written tokenizer** - Full control over lexical analysis
- **Hand-written parser** - Optimized for TOML grammar
- **Zero dependencies** - Only depends on @sylphx/synth core
- **Simple grammar** - TOML is well-suited for custom implementation

TOML is a configuration format with a simple, well-defined spec (TOML 1.0), making it ideal for in-house implementation.

## Use Cases

- **Rust projects**: Parse Cargo.toml
- **Python projects**: Parse pyproject.toml
- **Config files**: Application configuration
- **Build tools**: Parse tool configuration
- **Static analysis**: Analyze configuration structure
- **Code generation**: Transform config to code

## License

MIT

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

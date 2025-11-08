# @sylphx/synth-json

Hand-written JSON parser using Synth's universal AST.

## Features

- **Spec-Compliant**: Follows RFC 8259 JSON specification
- **Universal AST**: Converts JSON to Synth's language-agnostic BaseNode
- **Position Tracking**: Line, column, and offset information for all nodes
- **Error Reporting**: Clear error messages with line/column locations
- **Plugin Support**: Transform and analyze JSON using Synth plugins
- **Fast**: Direct recursive descent parsing without intermediate steps
- **TypeScript**: Full type safety and IntelliSense support

## Installation

```bash
bun install @sylphx/synth-json
```

## Usage

### Basic Parsing

```typescript
import { parse } from '@sylphx/synth-json'

const json = '{"name": "Alice", "age": 30}'
const tree = parse(json)

// Tree contains Synth AST nodes
console.log(tree.nodes.filter(n => n.type === 'Object'))
console.log(tree.nodes.filter(n => n.type === 'Property'))
console.log(tree.nodes.filter(n => n.type === 'String'))
```

### Parse All JSON Types

```typescript
import { parse } from '@sylphx/synth-json'

// Objects
parse('{"key": "value"}')

// Arrays
parse('[1, 2, 3]')

// Strings
parse('"hello world"')

// Numbers
parse('42')
parse('3.14')
parse('1.5e10')

// Booleans
parse('true')
parse('false')

// Null
parse('null')
```

### With Options

```typescript
import { parse } from '@sylphx/synth-json'

const json = '[1, 2,]'  // Trailing comma

// Allow trailing commas (non-standard JSON5 feature)
const tree = parse(json, {
  allowTrailingCommas: true
})
```

### Using the Parser Class

```typescript
import { JSONParser } from '@sylphx/synth-json'

const parser = new JSONParser()

// Parse multiple documents
const tree1 = parser.parse('{"a": 1}')
const tree2 = parser.parse('[1, 2, 3]')

// Get last parsed tree
const lastTree = parser.getTree()
```

### Async Parsing

```typescript
import { parseAsync } from '@sylphx/synth-json'

const json = '{"name": "Alice"}'
const tree = await parseAsync(json)
```

### Position Information

```typescript
import { parse } from '@sylphx/synth-json'

const json = `{
  "name": "Alice",
  "age": 30
}`

const tree = parse(json)

// Every node has span information
tree.nodes.forEach(node => {
  if (node.span) {
    console.log(`${node.type} at line ${node.span.start.line}, column ${node.span.start.column}`)
  }
})
```

### Extract Data

```typescript
import { parse } from '@sylphx/synth-json'

const json = '{"users": [{"name": "Alice"}, {"name": "Bob"}]}'
const tree = parse(json)

// Find all string values
const strings = tree.nodes
  .filter(n => n.type === 'String')
  .map(n => n.data?.value)

console.log(strings) // ["users", "Alice", "name", "Bob", "name"]

// Find all property keys
const properties = tree.nodes
  .filter(n => n.type === 'Property')
  .map(n => n.data?.key)

console.log(properties) // ["users", "name", "name"]
```

## AST Node Types

The JSON parser generates these node types:

```typescript
type JSONNodeType =
  | 'Object'    // JSON object { ... }
  | 'Array'     // JSON array [ ... ]
  | 'Property'  // Object property "key": value
  | 'String'    // String value "..."
  | 'Number'    // Number value 42, 3.14, 1e10
  | 'Boolean'   // Boolean value true, false
  | 'Null'      // Null value null
```

### Node Structure

**Object Node:**
```typescript
{
  type: 'Object',
  children: [/* Property node IDs */],
  data: {}
}
```

**Array Node:**
```typescript
{
  type: 'Array',
  children: [/* Value node IDs */],
  data: {}
}
```

**Property Node:**
```typescript
{
  type: 'Property',
  children: [/* Value node ID */],
  data: {
    key: string,              // Property key
    keySpan: { start, end }   // Key position info
  }
}
```

**String Node:**
```typescript
{
  type: 'String',
  children: [],
  data: {
    value: string  // Decoded string value
  }
}
```

**Number Node:**
```typescript
{
  type: 'Number',
  children: [],
  data: {
    value: number,  // Parsed number
    raw: string     // Original text
  }
}
```

**Boolean Node:**
```typescript
{
  type: 'Boolean',
  children: [],
  data: {
    value: boolean
  }
}
```

**Null Node:**
```typescript
{
  type: 'Null',
  children: [],
  data: {
    value: null
  }
}
```

## Options

```typescript
interface JSONParseOptions {
  /** Build query index for AST (default: false) */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]

  /** Allow trailing commas (default: false) */
  allowTrailingCommas?: boolean

  /** Allow comments - non-standard (default: false) */
  allowComments?: boolean
}
```

## Examples

### Parse Configuration File

```typescript
import { parse } from '@sylphx/synth-json'

const config = `{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "^4.17.21"
  }
}`

const tree = parse(config)

// Find all dependency names
const deps = tree.nodes
  .filter(n => {
    if (n.type !== 'Property') return false
    const parent = tree.nodes[n.parent!]
    const grandparent = parent ? tree.nodes[parent.parent!] : null
    const key = grandparent?.type === 'Property' ? grandparent.data?.key : null
    return key === 'dependencies'
  })
  .map(n => n.data?.key)

console.log(deps) // ["react", "lodash"]
```

### Validate JSON Structure

```typescript
import { parse } from '@sylphx/synth-json'

function validateJSON(json: string): boolean {
  try {
    parse(json)
    return true
  } catch (error) {
    console.error('Invalid JSON:', error.message)
    return false
  }
}

validateJSON('{"valid": true}')  // true
validateJSON('{invalid}')        // false
```

### Extract Nested Values

```typescript
import { parse } from '@sylphx/synth-json'

const json = `{
  "user": {
    "profile": {
      "name": "Alice",
      "email": "alice@example.com"
    }
  }
}`

const tree = parse(json)

// Find the "name" property value
const nameNode = tree.nodes.find(n =>
  n.type === 'Property' && n.data?.key === 'name'
)

if (nameNode) {
  const valueNode = tree.nodes[nameNode.children[0]!]
  console.log(valueNode?.data?.value) // "Alice"
}
```

### Pretty Error Messages

```typescript
import { parse } from '@sylphx/synth-json'

const invalidJSON = `{
  "name": "Alice",
  "age": 30,
  "active": TRUE
}`

try {
  parse(invalidJSON)
} catch (error) {
  console.error(error.message)
  // Output: Unexpected character: 'T' at line 4, column 13
}
```

## API Reference

### Functions

- `parse(source, options?)`: Parse JSON string synchronously
- `parseAsync(source, options?)`: Parse JSON string asynchronously
- `createParser()`: Create a new JSONParser instance

### Classes

- `JSONParser`: Main parser class
  - `constructor()`: Create parser instance
  - `parse(source, options?)`: Parse JSON string
  - `parseAsync(source, options?)`: Parse JSON string asynchronously
  - `getTree()`: Get last parsed tree
  - `use(plugin)`: Register a plugin

## Supported JSON Features

- ✅ Objects (`{ "key": "value" }`)
- ✅ Arrays (`[1, 2, 3]`)
- ✅ Strings (`"hello"`)
- ✅ Numbers (integers, floats, scientific notation)
- ✅ Booleans (`true`, `false`)
- ✅ Null (`null`)
- ✅ Escape sequences (`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`)
- ✅ Unicode escapes (`\uXXXX`)
- ✅ Nested structures
- ⚠️ Trailing commas (optional, non-standard)
- ⚠️ Comments (planned, non-standard)

## How It Works

1. **Tokenize**: Hand-written recursive descent parser reads source character by character
2. **Build AST**: Creates Synth BaseNode for each JSON value
3. **Track Position**: Records line, column, and offset for every node
4. **Validate**: Ensures JSON follows RFC 8259 specification

The parser works directly with Synth's universal AST:
- **Fast**: Single-pass parsing without intermediate tokens
- **Accurate**: Precise error locations
- **Universal**: Same AST structure works for all languages

## Performance

The parser is designed for correctness and clarity:
- Recursive descent parsing
- Direct node creation
- No intermediate token array
- Minimal allocations

## Limitations

This is a spec-compliant RFC 8259 JSON parser with optional extensions:

- Standard JSON only (no comments by default)
- No streaming/incremental parsing
- No circular reference detection (JSON can't have cycles)
- Large files load entirely into memory

## Comparison with JSON.parse

**JSON.parse (Native):**
- Faster (native C++ implementation)
- Returns plain JavaScript objects
- No position information
- No AST access
- No extensibility

**@sylphx/synth-json:**
- Pure TypeScript
- Returns Synth AST with position info
- Full access to parse tree structure
- Plugin system for transformations
- Educational/introspection use cases

## Use Cases

- **AST Analysis**: Inspect JSON structure programmatically
- **Error Reporting**: Show precise error locations in JSON files
- **Code Generation**: Generate code from JSON schemas
- **Validation**: Custom JSON validation rules
- **Transformation**: Transform JSON using Synth plugins
- **Learning**: Understand how JSON parsers work
- **Linting**: Build JSON linters and formatters

## License

MIT

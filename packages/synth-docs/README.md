# @sylphx/synth-docs

Documentation generator for Synth AST - extract docs from any language.

## Features

- ✅ **Universal Documentation** - Extract docs from any language
- 📝 **JSDoc Support** - Parse JSDoc-style comments
- 🎯 **Multi-Format Output** - Markdown, JSON, HTML
- 🚀 **Language-Agnostic** - Works on Synth's universal AST
- 📦 **Symbol Extraction** - Functions, classes, variables, constants
- 🔍 **Tag Parsing** - @param, @returns, @example, @deprecated, etc.
- ⚡ **Fast** - Leverages Synth's performance-optimized AST

## Installation

```bash
npm install @sylphx/synth-docs
```

## Usage

### Quick Start

```typescript
import { generate } from '@sylphx/synth-docs'
import { parse } from '@sylphx/synth-js'

const code = `
/**
 * Adds two numbers
 * @param {number} a - First number
 * @param {number} b - Second number
 * @returns {number} Sum of a and b
 * @example
 * add(1, 2)
 * // => 3
 */
function add(a, b) {
  return a + b;
}
`

const tree = parse(code)
const result = generate(tree, { format: 'markdown' })

console.log(result.output)
```

### Basic API

```typescript
import { DocGenerator } from '@sylphx/synth-docs'

const generator = new DocGenerator()
const result = generator.generate(tree, {
  format: 'markdown',  // or 'json', 'html'
  title: 'My API',
  file: 'api.js',
  includePrivate: false,
  includeInternal: false
})

console.log(result.module)  // Extracted documentation
console.log(result.output)  // Formatted output
```

## Output Formats

### Markdown (Default)

```typescript
const result = generate(tree, { format: 'markdown' })
console.log(result.output)
```

Generates clean Markdown documentation:

```markdown
# Module

## add

**function** • **exported**

Adds two numbers

### Parameters

- `a`: number - First number
- `b`: number - Second number

### Returns

**number** - Sum of a and b

### Examples

\`\`\`javascript
add(1, 2)
// => 3
\`\`\`
```

### JSON

```typescript
const result = generate(tree, { format: 'json' })
const docs = JSON.parse(result.output)
```

Generates structured JSON:

```json
{
  "name": "Module",
  "symbols": [
    {
      "name": "add",
      "kind": "function",
      "description": "Adds two numbers",
      "params": [
        { "name": "a", "type": "number", "description": "First number" },
        { "name": "b", "type": "number", "description": "Second number" }
      ],
      "returns": {
        "type": "number",
        "description": "Sum of a and b"
      },
      "examples": ["add(1, 2)\n// => 3"]
    }
  ]
}
```

### HTML

```typescript
const result = generate(tree, { format: 'html' })
```

Generates styled HTML documentation with proper formatting.

## Supported Tags

### @param

Document function parameters.

```javascript
/**
 * @param {string} name - User name
 * @param {number} age - User age
 * @param {boolean} active - Whether user is active
 */
function createUser(name, age, active) { }
```

### @returns / @return

Document return values.

```javascript
/**
 * @returns {boolean} Success status
 */
function save() { return true; }
```

### @example

Provide usage examples.

```javascript
/**
 * @example
 * multiply(2, 3)
 * // => 6
 *
 * @example
 * multiply(5, 10)
 * // => 50
 */
function multiply(a, b) { return a * b; }
```

### @access

Control visibility (public, private, protected).

```javascript
/**
 * @access private
 */
function internalHelper() { }
```

Use `includePrivate: true` to include private symbols.

### @internal

Mark internal API.

```javascript
/**
 * @internal
 */
function internalAPI() { }
```

Use `includeInternal: true` to include internal symbols.

### @deprecated

Mark deprecated APIs.

```javascript
/**
 * @deprecated Use newFunction instead
 */
function oldFunction() { }
```

### Custom Tags

Any `@tag` is captured:

```javascript
/**
 * @since 1.0.0
 * @author John Doe
 * @license MIT
 */
function example() { }
```

## Symbol Types

### Functions

```typescript
import { generate } from '@sylphx/synth-docs'

const code = `
/**
 * Calculate sum
 * @param {number} a - First number
 * @param {number} b - Second number
 * @returns {number} Sum
 */
export async function sum(a, b) {
  return a + b;
}
`

const tree = parse(code)
const result = generate(tree)

// result.module.symbols[0]:
// {
//   name: 'sum',
//   kind: 'function',
//   description: 'Calculate sum',
//   params: [...],
//   returns: {...},
//   exported: true,
//   async: true
// }
```

### Classes

```typescript
const code = `
/**
 * User class
 */
export class User {
  constructor(name) {
    this.name = name;
  }
}
`

const tree = parse(code)
const result = generate(tree)

// result.module.symbols[0]:
// {
//   name: 'User',
//   kind: 'class',
//   description: 'User class',
//   exported: true
// }
```

### Variables & Constants

```typescript
const code = `
/**
 * Maximum size
 * @type {number}
 */
export const MAX_SIZE = 100;

/**
 * Counter variable
 */
let count = 0;
`

const tree = parse(code)
const result = generate(tree)

// Extracts both constant and variable
```

## Options

```typescript
interface DocOptions {
  /** Output format */
  format?: 'markdown' | 'json' | 'html'

  /** Include private symbols */
  includePrivate?: boolean

  /** Include internal symbols */
  includeInternal?: boolean

  /** Title for documentation */
  title?: string

  /** File path (for location info) */
  file?: string
}
```

### Examples

```typescript
// Custom title
generate(tree, { title: 'My API Reference' })

// Include private members
generate(tree, { includePrivate: true })

// Include internal APIs
generate(tree, { includeInternal: true })

// Add file path for locations
generate(tree, { file: 'src/api.js' })
```

## Advanced Usage

### Extract Specific Symbol

```typescript
const result = generate(tree)

// Find specific symbol
const addFunc = result.module.symbols.find(s => s.name === 'add')

console.log(addFunc.params)
console.log(addFunc.returns)
console.log(addFunc.examples)
```

### Generate Multiple Formats

```typescript
const markdown = generate(tree, { format: 'markdown' })
const json = generate(tree, { format: 'json' })
const html = generate(tree, { format: 'html' })

// Save to files
fs.writeFileSync('docs.md', markdown.output)
fs.writeFileSync('docs.json', json.output)
fs.writeFileSync('docs.html', html.output)
```

### Batch Documentation

```typescript
import { generate } from '@sylphx/synth-docs'
import { parse } from '@sylphx/synth-js'
import { readdirSync, readFileSync, writeFileSync } from 'fs'

const files = readdirSync('src').filter(f => f.endsWith('.js'))
const allDocs = []

for (const file of files) {
  const source = readFileSync(`src/${file}`, 'utf-8')
  const tree = parse(source)
  const result = generate(tree, {
    file: `src/${file}`,
    format: 'json'
  })
  allDocs.push(result.module)
}

// Combine all documentation
writeFileSync('api-docs.json', JSON.stringify(allDocs, null, 2))
```

## Symbol Documentation Structure

```typescript
interface SymbolDoc {
  name: string
  kind: 'function' | 'class' | 'interface' | 'variable' | 'constant' | 'type' | 'method' | 'property'
  description?: string
  params?: ParamDoc[]
  returns?: ReturnDoc
  examples?: string[]
  tags?: Map<string, string>
  location?: {
    file?: string
    line: number
    column: number
  }
  type?: string
  exported?: boolean
  async?: boolean
  access?: 'public' | 'private' | 'protected'
}
```

## Use Cases

- **API documentation** - Generate docs for libraries
- **Code exploration** - Understand codebases
- **Documentation sites** - Build doc websites
- **IDE integration** - Power autocomplete hints
- **Code review** - Review API changes
- **Multi-language docs** - Consistent docs across languages
- **Markdown generation** - README files, wiki pages

## Language Support

Works with any language parsed by Synth:
- JavaScript
- TypeScript
- Python (with docstrings)
- Go (with godoc comments)
- Rust (with rustdoc comments)
- And more...

The generator adapts to each language's documentation conventions through the universal AST.

## Examples

### Generate README

```typescript
import { generate } from '@sylphx/synth-docs'
import { parse } from '@sylphx/synth-js'
import { readFileSync, writeFileSync } from 'fs'

const source = readFileSync('index.js', 'utf-8')
const tree = parse(source)

const result = generate(tree, {
  format: 'markdown',
  title: 'API Reference',
  file: 'index.js'
})

const readme = `
# My Library

${result.output}

## License

MIT
`

writeFileSync('README.md', readme)
```

### JSON API Docs

```typescript
const result = generate(tree, { format: 'json' })
const api = JSON.parse(result.output)

// Use in API explorer, VS Code extension, etc.
console.log(api.symbols.map(s => s.name))
```

### HTML Documentation Site

```typescript
const result = generate(tree, {
  format: 'html',
  title: 'API Documentation',
  file: 'api.js'
})

writeFileSync('docs/index.html', result.output)
```

## Performance

Leverages Synth's performance-optimized AST:
- Single-pass extraction
- O(1) node access
- Efficient comment parsing
- Minimal memory overhead

## API Reference

### generate(tree, options?)

Generate documentation from a tree.

```typescript
const result = generate(tree, { format: 'markdown' })
```

### DocGenerator

Advanced documentation generator with state.

```typescript
const generator = new DocGenerator()
const result = generator.generate(tree, options)
```

### createGenerator()

Factory function to create a generator.

```typescript
const generator = createGenerator()
```

## License

MIT

---

**Note:** This is a universal documentation generator. Works across all languages supported by Synth parsers.

# @sylphx/synth-js-minify

JavaScript minifier using Synth's universal AST - compress code while preserving functionality.

## Features

- **Compact Output**: Remove whitespace, minimize syntax
- **Name Mangling**: Shorten variable/function names (optional)
- **Safe Compression**: Preserve code semantics
- **Universal AST**: Works on Synth's language-agnostic AST
- **Fast**: Direct AST traversal without intermediate representations
- **Configurable**: Control compression level and mangling behavior

## Installation

```bash
npm install @sylphx/synth-js-minify
```

## Usage

### Basic Minification

```typescript
import { minify } from '@sylphx/synth-js-minify'

const code = `
  function calculate(a, b) {
    const sum = a + b;
    const product = a * b;
    return { sum, product };
  }
`

const minified = minify(code)
console.log(minified)
// Output: function calculate(a,b){const sum=a+b;const product=a*b;return {sum:sum,product:product};}
```

### With Options

```typescript
import { minify } from '@sylphx/synth-js-minify'

const code = `
  function myLongFunctionName(param) {
    const myVariable = param * 2;
    return myVariable;
  }
`

// Enable name mangling for maximum compression
const minified = minify(code, {
  compress: true,
  mangle: true,
})
// Output: function a(b){const c=b*2;return c;}
```

### Protect Reserved Names

```typescript
import { minify } from '@sylphx/synth-js-minify'

const code = `
  function importantAPI(data) {
    return processData(data);
  }
`

const minified = minify(code, {
  mangle: true,
  reserved: ['importantAPI'], // Don't mangle this name
})
// Output: function importantAPI(a){return processData(a);}
```

### Using the Minifier Class

```typescript
import { Minifier } from '@sylphx/synth-js-minify'

const minifier = new Minifier({
  compress: true,
  mangle: false,
})

const code = 'const x = 42;'
const minified = minifier.minify(code)
console.log(minified) // "const x=42;"

// Calculate compression ratio
const ratio = minifier.compressionRatio(code)
console.log(`Reduced by ${(ratio * 100).toFixed(1)}%`)
```

### Minify a Synth Tree Directly

```typescript
import { parse } from '@sylphx/synth-js'
import { Minifier } from '@sylphx/synth-js-minify'

// Parse code to Synth AST
const tree = parse('const x = 42;')

// Minify the AST
const minifier = new Minifier()
const minified = minifier.minifyTree(tree)

console.log(minified) // "const x=42;"
```

### Calculate Size Savings

```typescript
import { minify, savings } from '@sylphx/synth-js-minify'

const original = `
  function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
`

const minified = minify(original)
const stats = savings(original, minified)

console.log(`Original: ${stats.originalSize} bytes`)
console.log(`Minified: ${stats.minifiedSize} bytes`)
console.log(`Saved: ${stats.savedBytes} bytes (${stats.savedPercent.toFixed(1)}%)`)
```

## Options

```typescript
interface MinifyOptions {
  /** Compress code (remove unnecessary whitespace, parentheses) */
  compress?: boolean // default: true

  /** Mangle variable names (shorten identifiers) */
  mangle?: boolean // default: false

  /** Remove comments */
  removeComments?: boolean // default: true

  /** Compact object/array literals on one line */
  compact?: boolean // default: true

  /** Preserve specific identifier names from mangling */
  reserved?: string[] // default: []
}
```

## Examples

### Different Compression Levels

```typescript
import { minify } from '@sylphx/synth-js-minify'

const code = `
  function greet(name) {
    const message = "Hello, " + name + "!";
    console.log(message);
    return message;
  }
`

// Basic compression (default)
minify(code)
// function greet(name){const message="Hello, "+name+"!";console.log(message);return message;}

// With mangling (maximum compression)
minify(code, { mangle: true })
// function a(b){const c="Hello, "+b+"!";console.log(c);return c;}
```

### Before and After

**Before:**
```javascript
function calculateTotal(items) {
  const subtotal = items.reduce((sum, item) => sum + item.price, 0);
  const tax = subtotal * 0.08;
  const total = subtotal + tax;
  return total;
}
```

**After (basic):**
```javascript
function calculateTotal(items){const subtotal=items.reduce((sum,item)=>sum+item.price,0);const tax=subtotal*0.08;const total=subtotal+tax;return total;}
```

**After (mangled):**
```javascript
function a(b){const c=b.reduce((d,e)=>d+e.price,0);const f=c*0.08;const g=c+f;return g;}
```

## API Reference

### Functions

- `minify(code, options?)`: Minify JavaScript code string
- `savings(original, minified)`: Calculate compression statistics

### Classes

- `Minifier`: Main minifier class
  - `constructor(options?)`: Create minifier with options
  - `minify(code, options?)`: Minify code string
  - `minifyTree(tree, options?)`: Minify Synth AST tree
  - `compressionRatio(code, minified?)`: Calculate compression ratio

- `Compressor`: Low-level AST-to-minified-code printer
  - `constructor(options?)`: Create compressor with options
  - `compress(tree)`: Convert Synth tree to minified code

## Supported JavaScript Features

- ✅ Variable declarations (var, let, const)
- ✅ Function declarations and expressions
- ✅ Arrow functions
- ✅ Classes and methods
- ✅ Object and array literals
- ✅ Binary and unary expressions
- ✅ If statements
- ✅ Return statements
- ✅ Block statements
- ✅ Call expressions
- ✅ Member expressions
- ✅ Import/export statements
- ✅ Async/await
- ⚠️ Template literals (basic support)
- ⚠️ For/while loops (basic support)

## How It Works

1. **Parse**: JavaScript code → Synth AST (via `@sylphx/synth-js`)
2. **Compress**: Synth AST → Minified code (via `Compressor`)
3. **Mangle** (optional): Shorten identifier names

The minifier works directly on Synth's universal AST:
- **Fast**: Single-pass compression without copying
- **Safe**: Preserves program semantics
- **Universal**: Same AST structure for all languages

## Compression Strategies

### Whitespace Removal
- Remove all unnecessary spaces, tabs, newlines
- Keep spaces only where syntactically required (e.g., `in` operator)

### Syntax Compaction
- Remove optional semicolons where safe
- Use shortest quote style for strings
- Remove unnecessary parentheses

### Name Mangling (Optional)
- Shorten variable/function names: `myVariable` → `a`
- Preserve reserved/exported names
- Generate collision-free short names: a, b, c, ..., z, aa, ab, ...

## Performance

Typical compression ratios:
- **Basic compression**: 30-50% size reduction
- **With mangling**: 50-70% size reduction

The minifier is designed for speed:
- Direct AST traversal
- Minimal string allocations
- No intermediate representations

## Limitations

This is a foundational implementation focused on core JavaScript features:

- Limited template literal optimization
- Basic comment handling
- No dead code elimination
- No constant folding
- Simple identifier mangling (not scope-aware)

For production use, consider tools like:
- **Terser**: Full-featured JavaScript minifier
- **esbuild**: Extremely fast bundler with minification
- **SWC**: Fast TypeScript/JavaScript compiler

## Use Cases

- **Learning**: Understand how minifiers work
- **Prototyping**: Quick minification in Synth-based tools
- **Integration**: Minify code in Synth AST pipelines
- **Foundation**: Build custom minifiers for other languages
- **Development**: Reduce payload size for web applications

## Comparison with Other Minifiers

This minifier is educational/foundational:

**Similarities to Terser/UglifyJS:**
- Remove whitespace
- Mangle names
- Compact syntax

**Differences:**
- Works on universal AST (can minify multiple languages)
- Simpler implementation
- Smaller feature set
- Focused on correctness over maximum compression

## License

MIT

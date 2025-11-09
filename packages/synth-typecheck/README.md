# @sylphx/synth-typecheck

Type checker for Synth AST - cross-language type inference and validation.

## Features

- ✅ **Type Inference** - Automatically infer types from AST
- 🎯 **Universal Types** - Works across all languages
- 🚀 **Language-Agnostic** - Built on Synth's universal AST
- 📊 **Type Compatibility** - Check if types are compatible
- 🔍 **Error Detection** - Find type errors in code
- ⚡ **Fast** - Leverages Synth's performance-optimized AST

## Installation

```bash
npm install @sylphx/synth-typecheck
```

## Usage

### Quick Start

```typescript
import { check } from '@sylphx/synth-typecheck'
import { parse } from '@sylphx/synth-js'

const tree = parse(`
const x = 42;
const y = "hello";
const z = x + y;  // Type error: number + string
`)

const result = check(tree)

console.log(result.success)  // false
console.log(result.errors)   // [{ message: ..., nodeId: ... }]

// Get inferred types
for (const [nodeId, type] of result.types) {
  console.log(`Node ${nodeId}: ${type.kind}`)
}
```

### Type Checker API

```typescript
import { TypeChecker } from '@sylphx/synth-typecheck'

const checker = new TypeChecker()
const result = checker.check(tree)

// Get type for specific node
const type = checker.getType(nodeId)
console.log(type.kind)  // 'number', 'string', etc.

// Check type compatibility
const compatible = checker.isCompatible(type1, type2)
```

## Type System

### Supported Type Kinds

```typescript
type TypeKind =
  | 'any'          // Any type
  | 'unknown'      // Unknown type
  | 'never'        // Never type
  | 'void'         // Void type
  | 'null'         // Null
  | 'undefined'    // Undefined
  | 'boolean'      // Boolean
  | 'number'       // Number
  | 'string'       // String
  | 'symbol'       // Symbol
  | 'bigint'       // BigInt
  | 'object'       // Object
  | 'array'        // Array
  | 'function'     // Function
  | 'class'        // Class
  | 'interface'    // Interface
  | 'union'        // Union type
  | 'intersection' // Intersection type
  | 'tuple'        // Tuple
  | 'literal'      // Literal type
  | 'generic'      // Generic type
  | 'reference'    // Type reference
```

### Type Structure

```typescript
interface Type {
  kind: TypeKind
  name?: string                  // For named types
  elementType?: Type             // For arrays
  parameterTypes?: Type[]        // For functions
  returnType?: Type              // For functions
  properties?: Map<string, Type> // For objects
  types?: Type[]                 // For unions
  value?: unknown                // For literals
  nullable?: boolean
  optional?: boolean
}
```

## Examples

### Literal Types

```typescript
import { check } from '@sylphx/synth-typecheck'
import { parse } from '@sylphx/synth-js'

// Number literal
const tree1 = parse('42')
const result1 = check(tree1)
// Type: { kind: 'number', value: 42 }

// String literal
const tree2 = parse('"hello"')
const result2 = check(tree2)
// Type: { kind: 'string', value: 'hello' }

// Boolean literal
const tree3 = parse('true')
const result3 = check(tree3)
// Type: { kind: 'boolean', value: true }

// Null literal
const tree4 = parse('null')
const result4 = check(tree4)
// Type: { kind: 'null' }
```

### Array Types

```typescript
const tree = parse('[1, 2, 3]')
const result = check(tree)

// Type: { kind: 'array', elementType: { kind: 'number' } }
```

### Object Types

```typescript
const tree = parse('{ x: 1, y: "hello" }')
const result = check(tree)

// Type: {
//   kind: 'object',
//   properties: Map {
//     'x' => { kind: 'number' },
//     'y' => { kind: 'string' }
//   }
// }
```

### Binary Expressions

```typescript
// Arithmetic
const tree1 = parse('1 + 2')
const result1 = check(tree1)
// Type: { kind: 'number' }

// String concatenation
const tree2 = parse('"a" + "b"')
const result2 = check(tree2)
// Type: { kind: 'string' }

// Comparison
const tree3 = parse('1 < 2')
const result3 = check(tree3)
// Type: { kind: 'boolean' }
```

### Unary Expressions

```typescript
// Negation
const tree1 = parse('!true')
const result1 = check(tree1)
// Type: { kind: 'boolean' }

// Typeof
const tree2 = parse('typeof x')
const result2 = check(tree2)
// Type: { kind: 'string' }

// Numeric
const tree3 = parse('-42')
const result3 = check(tree3)
// Type: { kind: 'number' }
```

### Logical Expressions

```typescript
const tree = parse('true && false')
const result = check(tree)

// Type: { kind: 'boolean' }
```

### Variable Declarations

```typescript
const tree = parse('const x = 42')
const result = check(tree)

// Variable 'x' inferred as: { kind: 'number' }
```

### Function Types

```typescript
const tree = parse('function add(a, b) { return a + b; }')
const result = check(tree)

// Type: {
//   kind: 'function',
//   parameterTypes: [{ kind: 'any' }, { kind: 'any' }],
//   returnType: { kind: 'any' }
// }
```

## Type Compatibility

### Check Compatibility

```typescript
import { TypeChecker } from '@sylphx/synth-typecheck'

const checker = new TypeChecker()

const num = { kind: 'number' }
const str = { kind: 'string' }
const any = { kind: 'any' }

checker.isCompatible(num, num)  // true
checker.isCompatible(num, str)  // false
checker.isCompatible(num, any)  // true (any is compatible with everything)
```

### Array Compatibility

```typescript
const checker = new TypeChecker()

const numArray = {
  kind: 'array',
  elementType: { kind: 'number' }
}

const strArray = {
  kind: 'array',
  elementType: { kind: 'string' }
}

checker.isCompatible(numArray, numArray)  // true
checker.isCompatible(numArray, strArray)  // false
```

## Type Errors

```typescript
interface TypeError {
  message: string
  nodeId?: NodeId
  expected?: Type
  actual?: Type
  location?: {
    line: number
    column: number
  }
}
```

### Example

```typescript
const tree = parse('const x: number = "hello"')
const result = check(tree)

if (!result.success) {
  for (const error of result.errors) {
    console.error(error.message)
    console.error(`Expected: ${error.expected?.kind}`)
    console.error(`Actual: ${error.actual?.kind}`)
  }
}
```

## Advanced Usage

### Custom Type Environment

```typescript
import { TypeChecker } from '@sylphx/synth-typecheck'

const checker = new TypeChecker()

// Type environment is automatically created with built-ins
// You can extend it by checking code that declares types
```

### Get All Inferred Types

```typescript
const result = check(tree)

for (const [nodeId, type] of result.types) {
  const node = tree.nodes[nodeId]
  console.log(`${node.type}: ${type.kind}`)
}
```

### Integration with Linter

```typescript
import { check } from '@sylphx/synth-typecheck'
import { createLinter } from '@sylphx/synth-lint'

const tree = parse(code)

// Type check first
const typeResult = check(tree)
if (!typeResult.success) {
  console.error('Type errors found!')
}

// Then lint
const linter = createLinter()
const lintResult = linter.lint(tree)
```

## Use Cases

- **Type validation** - Ensure code is type-safe
- **Type inference** - Automatically determine types
- **IDE support** - Power autocomplete and type hints
- **Code analysis** - Detect type-related issues
- **Cross-language checking** - Universal type system
- **Documentation** - Generate type docs
- **Refactoring** - Safe type-preserving refactors

## Supported Languages

Works on any language parsed by Synth:
- JavaScript
- TypeScript
- Python
- Go
- Rust
- And more...

The type checker adapts to each language's type system through the universal AST.

## Limitations

This is a basic type inference system. It:
- ✅ Infers literal types
- ✅ Infers array and object types
- ✅ Infers expression result types
- ✅ Handles variable declarations
- ❌ Does not perform advanced type narrowing
- ❌ Does not handle complex generics
- ❌ Does not perform full flow analysis

For production TypeScript type checking, use the official TypeScript compiler. This type checker is designed for cross-language analysis and basic type inference.

## Performance

Leverages Synth's performance-optimized AST:
- Single-pass type inference
- O(1) node access
- Efficient type representation
- Minimal memory overhead

## API Reference

### check(tree)

Check types in a tree and return result.

```typescript
const result = check(tree)
```

### TypeChecker

Advanced type checker with state.

```typescript
const checker = new TypeChecker()
const result = checker.check(tree)
const type = checker.getType(nodeId)
const compatible = checker.isCompatible(type1, type2)
```

### createChecker()

Factory function to create a type checker.

```typescript
const checker = createChecker()
```

## License

MIT

---

**Note:** This is a universal type checker for basic type inference across languages. For advanced TypeScript checking, use the official TypeScript compiler.

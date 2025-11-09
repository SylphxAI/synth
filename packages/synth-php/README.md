# @sylphx/synth-php

PHP parser using Synth's universal AST. Conversion layer over tree-sitter-php.

## Features

- ✅ **Strategic Dependency** - Uses tree-sitter-php (battle-tested PHP parser)
- 🚀 **Full PHP Support** - PHP 7 and PHP 8+ features
- 🎯 **Universal AST** - Converts tree-sitter CST to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Battle-Tested** - tree-sitter powers VS Code, Atom, and many other editors

## Installation

```bash
npm install @sylphx/synth-php
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-php'

const php = `<?php
function greet($name) {
    return "Hello, " . $name . "!";
}

echo greet("World");
?>`

const tree = parse(php)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { PhpParser, createParser, parse, parseAsync } from '@sylphx/synth-php'

// Standalone function (recommended)
const tree = parse('<?php $x = 42; ?>')

// Async parsing (for plugins)
const tree = await parseAsync('<?php $x = 42; ?>')

// Class instance
const parser = new PhpParser()
const tree = parser.parse('<?php $x = 42; ?>')

// Factory function
const parser = createParser()
const tree = parser.parse('<?php $x = 42; ?>')
```

### Plugin System

```typescript
import { parse, parseAsync, type Tree } from '@sylphx/synth-php'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse('<?php $x = 42; ?>', { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync('<?php $x = 42; ?>', { plugins: [asyncPlugin] })
```

## AST Structure

The parser generates a universal Synth AST by converting tree-sitter's concrete syntax tree. Each node includes:

### Node Structure

```typescript
{
  type: 'FunctionDefinition',  // Mapped from tree-sitter type
  parent: NodeId,
  children: [NodeId],
  span: {
    start: { offset, line, column },
    end: { offset, line, column }
  },
  data: {
    text: 'function greet()...',   // Original source text
    isNamed: true,                  // tree-sitter named node
    originalType: 'function_definition'  // Original tree-sitter type
  }
}
```

## Supported PHP Features

### Variables
- ✅ Variables (`$name`, `$value`)
- ✅ Superglobals (`$_GET`, `$_POST`, `$_SESSION`, etc.)
- ✅ Variable variables (`$$name`)
- ✅ Static variables
- ✅ Global variables

### Data Types
- ✅ Strings (single `'...'`, double `"..."`, heredoc, nowdoc)
- ✅ Integers (decimal, hex, octal, binary)
- ✅ Floats (`3.14`, `1.5e10`)
- ✅ Booleans (`true`, `false`)
- ✅ `null`
- ✅ Arrays (`array(...)`, `[...]`)
- ✅ Objects
- ✅ Resources
- ✅ Callable types

### Control Flow
- ✅ `if/elseif/else` statements
- ✅ `for` loops
- ✅ `foreach` loops (with key/value)
- ✅ `while` loops
- ✅ `do-while` loops
- ✅ `switch/case` statements
- ✅ `match` expressions (PHP 8+)
- ✅ `try/catch/finally`
- ✅ `break`, `continue`, `return`
- ✅ `goto` statements

### Functions
- ✅ Function definitions (`function name()`)
- ✅ Parameters and default values
- ✅ Type hints (`int`, `string`, `array`, classes)
- ✅ Return type declarations (`: int`, `: string`)
- ✅ Variadic functions (`...$args`)
- ✅ Anonymous functions (closures)
- ✅ Arrow functions (`fn() => ...`) (PHP 7.4+)
- ✅ `use` keyword for closures

### Classes and Objects
- ✅ Class declarations (`class MyClass`)
- ✅ Constructors (`__construct`)
- ✅ Properties (public, private, protected, static)
- ✅ Methods (public, private, protected, static, abstract, final)
- ✅ Constants (`const NAME = value`)
- ✅ Visibility modifiers
- ✅ Constructor property promotion (PHP 8+)

### Inheritance
- ✅ `extends` keyword
- ✅ `implements` keyword (interfaces)
- ✅ Abstract classes and methods
- ✅ Final classes and methods
- ✅ Interfaces
- ✅ Traits (`trait`, `use`)

### Modern PHP Features (PHP 8+)
- ✅ Named arguments
- ✅ Match expressions
- ✅ Nullsafe operator (`?->`)
- ✅ Constructor property promotion
- ✅ Union types (`int|string`)
- ✅ Mixed type
- ✅ Attributes (`#[...]`)
- ✅ Enumerations (`enum`)
- ✅ Readonly properties

### Operators
- ✅ Arithmetic (`+`, `-`, `*`, `/`, `%`, `**`)
- ✅ Comparison (`==`, `===`, `!=`, `!==`, `<`, `>`, `<=`, `>=`, `<=>`)
- ✅ Logical (`&&`, `||`, `!`, `and`, `or`, `xor`)
- ✅ String (`.` concatenation)
- ✅ Assignment (`=`, `+=`, `-=`, `.=`, etc.)
- ✅ Ternary (`? :`)
- ✅ Null coalescing (`??`) (PHP 7+)
- ✅ Instanceof operator
- ✅ Error suppression (`@`)

### Namespaces and Imports
- ✅ Namespace declarations (`namespace App\Models;`)
- ✅ `use` statements
- ✅ Aliased imports (`use Foo as Bar`)
- ✅ Grouped imports

### Comments
- ✅ Line comments (`//` and `#`)
- ✅ Block comments (`/* ... */`)
- ✅ PHPDoc comments (`/** ... */`)

## Examples

### Parse a Class

```typescript
import { parse } from '@sylphx/synth-php'

const php = `<?php
class Calculator {
    public function add($a, $b) {
        return $a + $b;
    }

    public function subtract($a, $b) {
        return $a - $b;
    }
}
?>`

const tree = parse(php)

// Find class declaration
const classNode = tree.nodes.find(n => n.type === 'ClassDeclaration')
console.log(classNode)

// Find method declarations
const methodNodes = tree.nodes.filter(n => n.type.includes('Method'))
console.log(methodNodes)
```

### Parse with Type Hints

```typescript
import { parse } from '@sylphx/synth-php'

const php = `<?php
function add(int $a, int $b): int {
    return $a + $b;
}
?>`

const tree = parse(php)

// Find function with type declarations
const funcNode = tree.nodes.find(n => n.type === 'FunctionDefinition')
console.log(funcNode)
```

### Parse Arrow Function (PHP 7.4+)

```typescript
import { parse } from '@sylphx/synth-php'

const php = `<?php
$numbers = [1, 2, 3, 4, 5];
$squared = array_map(fn($n) => $n * $n, $numbers);
?>`

const tree = parse(php)

// Find arrow function
const arrowNode = tree.nodes.find(n => n.type.includes('Arrow'))
console.log(arrowNode)
```

### Parse Match Expression (PHP 8+)

```typescript
import { parse } from '@sylphx/synth-php'

const php = `<?php
$result = match($status) {
    'success' => 'Operation successful',
    'error' => 'An error occurred',
    default => 'Unknown status'
};
?>`

const tree = parse(php)

// Find match expression
const matchNode = tree.nodes.find(n => n.type.includes('Match'))
console.log(matchNode)
```

### Parse Enum (PHP 8.1+)

```typescript
import { parse } from '@sylphx/synth-php'

const php = `<?php
enum Status: string {
    case Pending = 'pending';
    case Approved = 'approved';
    case Rejected = 'rejected';
}
?>`

const tree = parse(php)

// Find enum declaration
const enumNode = tree.nodes.find(n => n.type.includes('Enum'))
console.log(enumNode)
```

### Apply Plugin

```typescript
import { parse, type Tree, type Node } from '@sylphx/synth-php'

// Plugin to count functions
const functionCounterPlugin = {
  name: 'function-counter',
  transform(tree: Tree) {
    const functions = tree.nodes.filter(n => n.type === 'FunctionDefinition')
    console.log(`Found ${functions.length} functions`)
    return tree
  }
}

const php = `<?php
function foo() {}
function bar() {}
function baz() {}
?>`

const tree = parse(php, { plugins: [functionCounterPlugin] })
// Output: Found 3 functions
```

## Use Cases

- **Code Analysis** - Analyze PHP codebases for patterns, complexity, dependencies
- **Linting** - Build custom linters for PHP code
- **Documentation** - Generate API docs from PHPDoc comments
- **Refactoring** - Automate code transformations
- **Metrics** - Calculate code metrics (cyclomatic complexity, LOC, etc.)
- **IDE Features** - Power autocomplete, go-to-definition, find references
- **Code Generation** - Generate PHP code from templates
- **Migration Tools** - Automate PHP version upgrades (5 → 7 → 8)
- **Security Analysis** - Detect security vulnerabilities

## Performance

- **Fast Parsing** - tree-sitter is highly optimized
- **Incremental Parsing** - tree-sitter supports incremental re-parsing
- **Low Memory** - Synth's arena-based storage is memory efficient
- **O(1) Node Access** - NodeId-based access is constant time

## Architecture

```
PHP Source Code
      ↓
tree-sitter-php (parse)
      ↓
tree-sitter CST
      ↓
@sylphx/synth-php (convert)
      ↓
Synth Universal AST
      ↓
Plugins (transform)
      ↓
Final AST
```

## Why tree-sitter-php?

- ✅ **Battle-Tested** - Powers VS Code, Atom, Neovim, and GitHub's code navigation
- ✅ **Complete** - Supports PHP 7 and PHP 8+ including latest features
- ✅ **Fast** - Written in C, highly optimized
- ✅ **Incremental** - Supports incremental parsing for editors
- ✅ **Error Recovery** - Handles partial/invalid code gracefully
- ✅ **Maintained** - Actively maintained by the tree-sitter community

**Our Value:** Universal AST format, cross-language tools, plugin system, and TypeScript API.

## API Reference

### `parse(source, options?)`

Parse PHP source code synchronously.

```typescript
const tree = parse('<?php $x = 42; ?>')
```

### `parseAsync(source, options?)`

Parse PHP source code asynchronously (for async plugins).

```typescript
const tree = await parseAsync('<?php $x = 42; ?>')
```

### `createParser()`

Create a new PhpParser instance.

```typescript
const parser = createParser()
```

### `PhpParser`

Main parser class with plugin support.

```typescript
const parser = new PhpParser()
parser.use(plugin)
const tree = parser.parse('<?php $x = 42; ?>')
```

### Options

```typescript
interface PhpParseOptions {
  buildIndex?: boolean    // Build query index (not yet implemented)
  plugins?: Plugin[]      // Plugins to apply
  phpVersion?: 7 | 8      // PHP version (for compatibility)
}
```

## License

MIT

---

**Part of the Synth universal AST ecosystem** - Works seamlessly with all other Synth parsers and tools.

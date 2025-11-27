# @sylphx/synth-python

Python parser using Synth's universal AST. Conversion layer over tree-sitter-python.

## Features

- ✅ **Strategic Dependency** - Uses tree-sitter-python (battle-tested Python 3 parser)
- 🚀 **Python 3 Support** - Full modern Python syntax
- 🎯 **Universal AST** - Converts tree-sitter CST to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Battle-Tested** - tree-sitter powers VS Code, Atom, and many other editors

## Installation

```bash
npm install @sylphx/synth-python
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-python'

const python = `
def greet(name):
    return f"Hello, {name}!"

print(greet("World"))
`

const tree = parse(python)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { PythonParser, createParser, parse, parseAsync } from '@sylphx/synth-python'

// Standalone function (recommended)
const tree = parse('x = 42')

// Async parsing (for plugins)
const tree = await parseAsync('x = 42')

// Class instance
const parser = new PythonParser()
const tree = parser.parse('x = 42')

// Factory function
const parser = createParser()
const tree = parser.parse('x = 42')
```

### Plugin System

```typescript
import { parse, type Tree } from '@sylphx/synth-python'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse('x = 42', { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync('x = 42', { plugins: [asyncPlugin] })
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
    text: 'def greet()...',   // Original source text
    isNamed: true,              // tree-sitter named node
    originalType: 'function_definition'  // Original tree-sitter type
  }
}
```

## Supported Python Features

### Data Types
- ✅ Strings (single, double, triple-quoted)
- ✅ Integers
- ✅ Floats
- ✅ Booleans (`True`, `False`)
- ✅ Lists `[1, 2, 3]`
- ✅ Tuples `(1, 2, 3)`
- ✅ Dictionaries `{"key": "value"}`
- ✅ Sets `{1, 2, 3}`
- ✅ None

### Control Flow
- ✅ `if/elif/else` statements
- ✅ `for` loops
- ✅ `while` loops
- ✅ `try/except/finally`
- ✅ `with` statements
- ✅ `match/case` (Python 3.10+)

### Functions
- ✅ Function definitions `def func():`
- ✅ Parameters and default values
- ✅ `*args` and `**kwargs`
- ✅ Type hints `def func(x: int) -> str:`
- ✅ Lambda functions
- ✅ Decorators `@decorator`
- ✅ Async functions `async def`

### Classes
- ✅ Class definitions `class MyClass:`
- ✅ Inheritance `class Child(Parent):`
- ✅ Methods and `__init__`
- ✅ Class methods `@classmethod`
- ✅ Static methods `@staticmethod`
- ✅ Properties `@property`
- ✅ Data classes (Python 3.7+)

### Imports
- ✅ `import module`
- ✅ `from module import name`
- ✅ Import aliases `import numpy as np`
- ✅ Relative imports `from ..parent import name`

### Modern Python
- ✅ F-strings `f"Hello, {name}!"`
- ✅ Walrus operator `:=` (Python 3.8+)
- ✅ Type hints
- ✅ Async/await
- ✅ Context managers
- ✅ Generators and comprehensions

## Examples

### Flask Web Application

```typescript
const python = `
from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/api/users', methods=['GET', 'POST'])
def users():
    if request.method == 'POST':
        user = request.get_json()
        # Process user
        return jsonify({"status": "created"}), 201

    return jsonify({"users": []}), 200

if __name__ == '__main__':
    app.run(debug=True)
`

const tree = parse(python)
```

### Data Processing

```typescript
const python = `
import pandas as pd
import numpy as np

def process_data(filename):
    # Read CSV
    df = pd.read_csv(filename)

    # Transform
    df['processed'] = df['value'].apply(lambda x: x * 2)
    df['category'] = df['category'].str.lower()

    # Aggregate
    result = df.groupby('category').agg({
        'value': ['mean', 'sum', 'count'],
        'processed': 'max'
    })

    return result

if __name__ == '__main__':
    result = process_data('data.csv')
    print(result.head())
`

const tree = parse(python)
```

### Object-Oriented Programming

```typescript
const python = `
from typing import List, Optional
from dataclasses import dataclass

@dataclass
class Person:
    name: str
    age: int
    email: Optional[str] = None

    def is_adult(self) -> bool:
        return self.age >= 18

    @classmethod
    def from_dict(cls, data: dict) -> 'Person':
        return cls(**data)

class Employee(Person):
    def __init__(self, name: str, age: int, salary: float):
        super().__init__(name, age)
        self.salary = salary

    @property
    def annual_salary(self) -> float:
        return self.salary * 12

# Usage
emp = Employee("Alice", 30, 5000)
print(emp.annual_salary)
`

const tree = parse(python)
```

### Async Programming

```typescript
const python = `
import asyncio
import aiohttp

async def fetch_url(session, url):
    async with session.get(url) as response:
        return await response.text()

async def fetch_all(urls):
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_url(session, url) for url in urls]
        results = await asyncio.gather(*tasks)
        return results

async def main():
    urls = [
        'https://api.example.com/1',
        'https://api.example.com/2',
        'https://api.example.com/3'
    ]
    results = await fetch_all(urls)
    print(f"Fetched {len(results)} URLs")

if __name__ == '__main__':
    asyncio.run(main())
`

const tree = parse(python)
```

### Machine Learning

```typescript
const python = `
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score

def train_model(X, y):
    # Split data
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=0.2, random_state=42
    )

    # Train model
    model = RandomForestClassifier(n_estimators=100)
    model.fit(X_train, y_train)

    # Evaluate
    y_pred = model.predict(X_test)
    accuracy = accuracy_score(y_test, y_pred)

    return model, accuracy

if __name__ == '__main__':
    X = np.random.rand(1000, 10)
    y = np.random.randint(0, 2, 1000)
    model, acc = train_model(X, y)
    print(f"Accuracy: {acc:.2f}")
`

const tree = parse(python)
```

## Performance

Leverages tree-sitter's high-performance parsing:
- Fast incremental parsing
- Error recovery
- Battle-tested in production editors
- Efficient memory usage

## Development Philosophy

This package uses a **strategic dependency** approach:

- **Third-party parser:** tree-sitter-python (used by VS Code, Atom, GitHub)
- **Our conversion layer:** tree-sitter CST → Synth universal AST
- **Our value:** Universal format, cross-language tools, plugin system

### Why tree-sitter?

- ❌ Writing Python parser: 200+ hours, complex grammar, Python 2/3 compatibility
- ✅ Using tree-sitter: Battle-tested, incremental parsing, error recovery
- **Our focus:** Universal AST format, transformations, cross-language operations

## Use Cases

- **Code analysis:** Analyze Python codebases
- **Linting:** Build custom Python linters
- **Documentation:** Extract docstrings and signatures
- **Code generation:** Transform Python AST
- **Migration tools:** Refactor Python code
- **Static analysis:** Type checking, complexity analysis
- **Cross-language tools:** Analyze Python + JavaScript + other languages together

## Comparison with ast module

Unlike Python's built-in `ast` module, `@sylphx/synth-python`:

- Works in JavaScript/TypeScript environments
- Uses universal AST format compatible with other languages
- Provides plugin system for transformations
- Integrates with other Synth parsers
- Suitable for multi-language tooling

## License

MIT

---

**Note:** This package uses tree-sitter-python for parsing. See [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python) for parser details.

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

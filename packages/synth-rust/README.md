# @sylphx/synth-rust

Rust parser using Synth's universal AST. Conversion layer over tree-sitter-rust.

## Features

- ✅ **Strategic Dependency** - Uses tree-sitter-rust (battle-tested Rust parser)
- 🚀 **Full Rust Support** - All Rust language features including async/await, generics, traits
- 🎯 **Universal AST** - Converts tree-sitter CST to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Battle-Tested** - tree-sitter powers VS Code, Atom, and many other editors

## Installation

```bash
npm install @sylphx/synth-rust
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-rust'

const rust = `
fn main() {
    println!("Hello, World!");
}
`

const tree = parse(rust)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { RustParser, createParser, parse, parseAsync } from '@sylphx/synth-rust'

// Standalone function (recommended)
const tree = parse('fn main() { println!("Hello"); }')

// Async parsing (for plugins)
const tree = await parseAsync('fn main() { println!("Hello"); }')

// Class instance
const parser = new RustParser()
const tree = parser.parse('fn main() { println!("Hello"); }')

// Factory function
const parser = createParser()
const tree = parser.parse('fn main() { println!("Hello"); }')
```

### Plugin System

```typescript
import { parse, type Tree } from '@sylphx/synth-rust'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse(rustSource, { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync(rustSource, { plugins: [asyncPlugin] })
```

## AST Structure

The parser generates a universal Synth AST by converting tree-sitter's concrete syntax tree. Each node includes:

### Node Structure

```typescript
{
  type: 'FunctionItem',  // Mapped from tree-sitter type
  parent: NodeId,
  children: [NodeId],
  span: {
    start: { offset, line, column },
    end: { offset, line, column }
  },
  data: {
    text: 'fn main()...',     // Original source text
    isNamed: true,                // tree-sitter named node
    originalType: 'function_item'  // Original tree-sitter type
  }
}
```

## Supported Rust Features

### Data Types
- ✅ Strings (regular, raw, byte strings)
- ✅ Integers (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize)
- ✅ Floats (f32, f64)
- ✅ Booleans (`true`, `false`)
- ✅ Arrays `[1, 2, 3, 4, 5]`
- ✅ Vectors `vec![1, 2, 3]`
- ✅ Tuples `(i32, f64, &str)`
- ✅ Structs (named, tuple, unit)
- ✅ Enums (C-like, with data)
- ✅ References and pointers

### Control Flow
- ✅ `if/else` expressions
- ✅ `match` expressions and pattern matching
- ✅ `for` loops (range, iterators)
- ✅ `while` loops
- ✅ `loop` with `break` and `continue`
- ✅ `if let` and `while let`

### Functions
- ✅ Function declarations
- ✅ Parameters and return types
- ✅ Multiple return values (tuples)
- ✅ Generic functions
- ✅ Closures (|| x + 1)
- ✅ Methods (impl blocks)
- ✅ Associated functions

### Types and Traits
- ✅ Type definitions `type MyInt = i32`
- ✅ Struct definitions (named, tuple, unit)
- ✅ Enum definitions
- ✅ Trait definitions
- ✅ Trait implementations `impl Trait for Type`
- ✅ Generic types `struct Point<T>`
- ✅ Trait bounds `T: Display + Clone`
- ✅ Associated types

### Ownership and Lifetimes
- ✅ References `&T`, `&mut T`
- ✅ Lifetimes `'a`, `'static`
- ✅ Lifetime annotations in functions
- ✅ Lifetime bounds
- ✅ Smart pointers (Box, Rc, Arc, etc.)

### Pattern Matching
- ✅ Match expressions
- ✅ Destructuring (tuples, structs, enums)
- ✅ Pattern guards `if condition`
- ✅ Range patterns `1..=5`
- ✅ Multiple patterns `1 | 2 | 3`

### Modules and Crates
- ✅ Module declarations `mod name`
- ✅ Use statements `use std::collections::HashMap`
- ✅ Path imports `use std::io::{self, Write}`
- ✅ Glob imports `use std::prelude::*`
- ✅ Aliasing `use foo as bar`
- ✅ Visibility modifiers (`pub`, `pub(crate)`, etc.)

### Error Handling
- ✅ Result type `Result<T, E>`
- ✅ Option type `Option<T>`
- ✅ Question mark operator `?`
- ✅ `panic!` and `unwrap`
- ✅ Custom error types

### Advanced Features
- ✅ Generics (functions, structs, enums, traits)
- ✅ Macros (invocations and definitions)
- ✅ Attributes `#[derive(Debug)]`
- ✅ Async/await `async fn`, `.await`
- ✅ Unsafe code blocks
- ✅ FFI declarations

## Examples

### HTTP Server (Actix Web)

```typescript
const rust = `
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
`

const tree = parse(rust)
```

### CLI Application

```typescript
const rust = `
use std::env;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
}
`

const tree = parse(rust)
```

### Async/Await with Tokio

```typescript
const rust = `
use tokio::time::{sleep, Duration};

async fn say_hello() {
    println!("Hello");
    sleep(Duration::from_secs(1)).await;
    println!("World");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}
`

const tree = parse(rust)
```

### Generic Data Structures

```typescript
const rust = `
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: std::fmt::Display> Point<T> {
    fn print(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

fn main() {
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.5);
}
`

const tree = parse(rust)
```

### Traits and Implementations

```typescript
const rust = `
trait Summary {
    fn summarize(&self) -> String;

    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = Article {
        headline: String::from("Rust 2.0 Released"),
        content: String::from("Exciting new features..."),
    };

    notify(&article);
}
`

const tree = parse(rust)
```

### Error Handling with Result

```typescript
const rust = `
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}
`

const tree = parse(rust)
```

## Performance

Leverages tree-sitter's high-performance parsing:
- Fast incremental parsing
- Error recovery
- Battle-tested in production editors
- Efficient memory usage

## Development Philosophy

This package uses a **strategic dependency** approach:

- **Third-party parser:** tree-sitter-rust (used by VS Code, Atom, GitHub)
- **Our conversion layer:** tree-sitter CST → Synth universal AST
- **Our value:** Universal format, cross-language tools, plugin system

### Why tree-sitter?

- ❌ Writing Rust parser: 200+ hours, complex grammar, constant language updates
- ✅ Using tree-sitter: Battle-tested, incremental parsing, error recovery
- **Our focus:** Universal AST format, transformations, cross-language operations

## Use Cases

- **Code analysis:** Analyze Rust codebases
- **Linting:** Build custom Rust linters
- **Documentation:** Extract comments, docs, and signatures
- **Code generation:** Transform Rust AST
- **Migration tools:** Refactor Rust code
- **Static analysis:** Complexity analysis, dependency graphs
- **Cross-language tools:** Analyze Rust + JavaScript + Python together

## Comparison with syn

Unlike Rust's `syn` crate, `@sylphx/synth-rust`:

- Works in JavaScript/TypeScript environments
- Uses universal AST format compatible with other languages
- Provides plugin system for transformations
- Integrates with other Synth parsers
- Suitable for multi-language tooling

## License

MIT

---

**Note:** This package uses tree-sitter-rust for parsing. See [tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust) for parser details.

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

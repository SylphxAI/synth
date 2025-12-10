import { describe, expect, it } from 'bun:test'
import type { Tree } from '@sylphx/synth'
import { createParser, init, parse, parseAsync, RustParser } from './parser.js'

describe('RustParser', () => {
  describe('Basic Parsing', () => {
    it('should parse a simple function', async () => {
      const rust = `fn main() {
    println!("Hello, World!");
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
      expect(tree.meta.language).toBe('rust')
      expect(tree.meta.source).toBe(rust)
      expect(Object.keys(tree.nodes).length).toBeGreaterThan(1)
    })

    it('should parse struct definition', async () => {
      const rust = `struct Point {
    x: i32,
    y: i32,
}`

      const tree = await parseAsync(rust)
      const root = tree.nodes[tree.root]!
      expect(root.children.length).toBeGreaterThan(0)
    })

    it('should parse variable declarations', async () => {
      const rust = `fn main() {
    let x = 5;
    let mut y = 10;
    const MAX: i32 = 100;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse enum definition', async () => {
      const rust = `enum Color {
    Red,
    Green,
    Blue,
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Data Types', () => {
    it('should parse string literals', async () => {
      const rust = `fn main() {
    let s = "Hello, Rust!";
    let raw = r"C:\\Users\\Name";
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse integer types', async () => {
      const rust = `fn main() {
    let a: i8 = 127;
    let b: i16 = 32767;
    let c: i32 = 2147483647;
    let d: i64 = 9223372036854775807;
    let e: u8 = 255;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse float types', async () => {
      const rust = `fn main() {
    let x: f32 = 3.14;
    let y: f64 = 2.718281828;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse boolean values', async () => {
      const rust = `fn main() {
    let t = true;
    let f = false;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse arrays and vectors', async () => {
      const rust = `fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3];
    let typed: Vec<i32> = Vec::new();
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse tuples', async () => {
      const rust = `fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Control Flow', () => {
    it('should parse if/else statements', async () => {
      const rust = `fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else {
        println!("not divisible by 4 or 3");
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse match expressions', async () => {
      const rust = `fn main() {
    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        13..=19 => println!("Teen"),
        _ => println!("Something else"),
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse for loops', async () => {
      const rust = `fn main() {
    for i in 0..5 {
        println!("{}", i);
    }

    for item in &[1, 2, 3] {
        println!("{}", item);
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse while loops', async () => {
      const rust = `fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse loop with break/continue', async () => {
      const rust = `fn main() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            continue;
        }
        if count == 5 {
            break;
        }
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Functions', () => {
    it('should parse function with parameters', async () => {
      const rust = `fn add(x: i32, y: i32) -> i32 {
    x + y
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse function with multiple return values', async () => {
      const rust = `fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse generic functions', async () => {
      const rust = `fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse closures', async () => {
      const rust = `fn main() {
    let add_one = |x| x + 1;
    let sum = |x, y| x + y;
    let multiply = |x: i32, y: i32| -> i32 { x * y };
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Structs and Implementations', () => {
    it('should parse struct with methods', async () => {
      const rust = `struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse associated functions', async () => {
      const rust = `impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse tuple structs', async () => {
      const rust = `struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Traits', () => {
    it('should parse trait definitions', async () => {
      const rust = `trait Summary {
    fn summarize(&self) -> String;

    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse trait implementations', async () => {
      const rust = `trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Ownership and Borrowing', () => {
    it('should parse references', async () => {
      const rust = `fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse mutable references', async () => {
      const rust = `fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse lifetimes', async () => {
      const rust = `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Pattern Matching', () => {
    it('should parse destructuring patterns', async () => {
      const rust = `fn main() {
    let (x, y, z) = (1, 2, 3);

    struct Point { x: i32, y: i32 }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse enum pattern matching', async () => {
      const rust = `enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(text) => println!("{}", text),
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Error Handling', () => {
    it('should parse Result type', async () => {
      const rust = `use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found"),
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse question mark operator', async () => {
      const rust = `use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse Option type', async () => {
      const rust = `fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    match some_number {
        Some(i) => println!("Got: {}", i),
        None => println!("Got nothing"),
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Macros', () => {
    it('should parse macro invocations', async () => {
      const rust = `fn main() {
    println!("Hello, {}!", "world");
    vec![1, 2, 3, 4, 5];
    assert_eq!(2 + 2, 4);
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse macro definitions', async () => {
      const rust = `macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Modules and Crates', () => {
    it('should parse module declarations', async () => {
      const rust = `mod garden {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    pub fn plant() {
        println!("Planting vegetables");
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse use statements', async () => {
      const rust = `use std::collections::HashMap;
use std::io::{self, Write};
use std::fmt::Result as FmtResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Generics', () => {
    it('should parse generic structs', async () => {
      const rust = `struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse generic enums', async () => {
      const rust = `enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Real-World Examples', () => {
    it('should parse a simple web server (Actix)', async () => {
      const rust = `use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse a CLI application', async () => {
      const rust = `use std::env;
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
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse async/await code', async () => {
      const rust = `use tokio::time::{sleep, Duration};

async fn say_hello() {
    println!("Hello");
    sleep(Duration::from_secs(1)).await;
    println!("World");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse JSON serialization with serde', async () => {
      const rust = `use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);

    let parsed: Person = serde_json::from_str(&json).unwrap();
    println!("{:?}", parsed);
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('Edge Cases', () => {
    it('should parse comments', async () => {
      const rust = `// This is a line comment
fn main() {
    /* This is a
       block comment */
    let x = 5; // inline comment
}

/// Documentation comment
fn documented() {}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse attributes', async () => {
      const rust = `#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })

    it('should parse visibility modifiers', async () => {
      const rust = `pub struct Public {
    pub field: i32,
    private_field: String,
}

pub(crate) fn crate_visible() {}
pub(super) fn parent_visible() {}
fn private() {}`

      const tree = await parseAsync(rust)
      expect(tree).toBeDefined()
    })
  })

  describe('API', () => {
    it('should support standalone parse function', async () => {
      const tree = await parseAsync('fn main() {}')
      expect(tree).toBeDefined()
      expect(tree.meta.language).toBe('rust')
    })

    it('should support async parsing', async () => {
      const tree = await parseAsync('fn main() {}')
      expect(tree).toBeDefined()
      expect(tree.meta.language).toBe('rust')
    })

    it('should support createParser factory', async () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(RustParser)

      const tree = await parser.parseAsync('fn main() {}')
      expect(tree).toBeDefined()
    })

    it('should support RustParser class', async () => {
      const parser = new RustParser()
      const tree = await parser.parseAsync('fn main() {}')
      expect(tree).toBeDefined()

      const retrieved = parser.getTree()
      expect(retrieved).toBe(tree)
    })

    it('should support plugins', async () => {
      let transformed = false

      const plugin = {
        name: 'test-plugin',
        transform(tree: Tree) {
          transformed = true
          return tree
        },
      }

      await parseAsync('fn main() {}', { plugins: [plugin] })
      expect(transformed).toBe(true)
    })

    it('should support async plugins', async () => {
      let transformed = false

      const plugin = {
        name: 'async-plugin',
        async transform(tree: Tree) {
          await new Promise((resolve) => setTimeout(resolve, 10))
          transformed = true
          return tree
        },
      }

      await parseAsync('fn main() {}', { plugins: [plugin] })
      expect(transformed).toBe(true)
    })

    it('should throw error for synchronous parse()', () => {
      expect(() => {
        parse('fn main() {}')
      }).toThrow(/WASM/)
    })

    it('should support init() for pre-initialization', async () => {
      // init() should not throw
      await init()
      // Second call should be instant (cached)
      await init()
    })

    it('should support use() method for plugins', async () => {
      let count = 0

      const plugin1 = {
        name: 'plugin1',
        transform(tree: Tree) {
          count++
          return tree
        },
      }

      const plugin2 = {
        name: 'plugin2',
        transform(tree: Tree) {
          count++
          return tree
        },
      }

      const parser = new RustParser()
      parser.use(plugin1).use(plugin2)
      await parser.parseAsync('fn main() {}')

      expect(count).toBe(2)
    })
  })
})

# @sylphx/synth-ini

INI parser using Synth's universal AST. Hand-written, zero dependencies.

## Features

- ✅ **100% In-House** - Hand-written parser, zero external dependencies
- 🚀 **INI Format Support** - Windows INI, .gitconfig, .editorconfig, PHP INI
- 🎯 **Universal AST** - Converts INI to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Lightweight** - No dependencies except @sylphx/synth core
- ⚙️ **Flexible** - Supports both `;` and `#` comments, `=` and `:` separators

## Installation

```bash
npm install @sylphx/synth-ini
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-ini'

const ini = `
  [user]
  name = John Doe
  email = john@example.com

  [core]
  editor = vim
`

const tree = parse(ini)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { INIParser, createParser, parse, parseAsync } from '@sylphx/synth-ini'

// Standalone function (recommended)
const tree = parse('key = value')

// Async parsing (for plugins)
const tree = await parseAsync('key = value')

// Class instance
const parser = new INIParser()
const tree = parser.parse('key = value')

// Factory function
const parser = createParser()
const tree = parser.parse('key = value')
```

### Plugin System

```typescript
import { parse, type Tree } from '@sylphx/synth-ini'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse('key = value', { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync('key = value', { plugins: [asyncPlugin] })
```

### Custom Options

```typescript
// Custom comment characters
const tree = parse(iniSource, {
  commentChars: ['//', '#']
})

// Allow duplicate keys
const tree = parse(iniSource, {
  allowDuplicates: true
})
```

## AST Structure

The parser generates a universal Synth AST with these INI-specific node types:

### Section

```typescript
{
  type: 'Section',
  data: {
    name: 'user'
  },
  children: [/* KeyValue nodes */]
}
```

### KeyValue

```typescript
{
  type: 'KeyValue',
  data: {
    key: 'name',
    value: 'John Doe'
  }
}
```

## Supported INI Features

### Syntax
- ✅ Sections: `[section]`
- ✅ Key-value pairs: `key = value`
- ✅ Alternative separator: `key: value`
- ✅ Comments: `; comment` or `# comment`
- ✅ Inline comments: `key = value ; comment`
- ✅ Quoted values: `key = "quoted"`
- ✅ Empty values: `key =`
- ✅ Nested section names: `[parent.child]`
- ✅ Global keys (before any section)
- ✅ Windows line endings (`\r\n`)

### Comment Styles
- ✅ Semicolon: `; comment`
- ✅ Hash: `# comment`
- ✅ Custom: configurable via `commentChars` option

## Examples

### .gitconfig (Git Configuration)

```typescript
const ini = `
  [user]
  name = John Doe
  email = john@example.com

  [core]
  editor = vim
  autocrlf = input

  [alias]
  st = status
  co = checkout
  br = branch

  [color]
  ui = auto

  [remote "origin"]
  url = https://github.com/user/repo.git
  fetch = +refs/heads/*:refs/remotes/origin/*
`

const tree = parse(ini)
```

### .editorconfig (Editor Configuration)

```typescript
const ini = `
  # EditorConfig is awesome
  root = true

  # Unix-style newlines with a newline ending every file
  [*]
  charset = utf-8
  end_of_line = lf
  insert_final_newline = true
  trim_trailing_whitespace = true

  # Matches multiple files with brace expansion notation
  [*.{js,ts,jsx,tsx}]
  indent_style = space
  indent_size = 2

  # Markdown files
  [*.md]
  trim_trailing_whitespace = false
  max_line_length = off

  # Python files
  [*.py]
  indent_style = space
  indent_size = 4
`

const tree = parse(ini)
```

### Windows INI (Application Settings)

```typescript
const ini = `
  [Settings]
  WindowWidth=1024
  WindowHeight=768
  Fullscreen=false
  Language=English

  [Graphics]
  Quality=High
  VSync=true
  AntiAliasing=4x
  Shadows=Medium

  [Audio]
  MasterVolume=80
  MusicVolume=60
  SFXVolume=70
`

const tree = parse(ini)
```

### PHP Configuration

```typescript
const ini = `
  ; PHP Configuration File

  [PHP]
  engine = On
  short_open_tag = Off
  precision = 14
  output_buffering = 4096
  zlib.output_compression = Off

  [Date]
  date.timezone = America/New_York

  [Session]
  session.save_handler = files
  session.use_cookies = 1
  session.use_only_cookies = 1
  session.name = PHPSESSID
  session.auto_start = 0
  session.cookie_lifetime = 0
  session.cookie_path = /
  session.cookie_domain =
  session.cookie_httponly = 1
`

const tree = parse(ini)
```

### Database Configuration

```typescript
const ini = `
  [database]
  host = localhost
  port = 5432
  name = myapp
  user = admin
  password = secret123

  [cache]
  enabled = true
  driver = redis
  host = 127.0.0.1
  port = 6379
  ttl = 3600

  [logging]
  level = debug
  file = /var/log/app.log
  max_size = 10485760
`

const tree = parse(ini)
```

## Performance

Hand-written parser optimized for:
- Fast line-by-line parsing
- Minimal memory allocations
- Arena-based AST storage (O(1) node access)
- Zero external dependencies

## Development Philosophy

This package is **100% in-house** - we own the entire parsing pipeline:

- **Hand-written parser** - Direct line-by-line parsing
- **Zero dependencies** - Only depends on @sylphx/synth core
- **Simple format** - INI is well-suited for custom implementation

INI is a simple configuration format with straightforward syntax, making it ideal for in-house implementation without a separate tokenization phase.

## Use Cases

- **Git configuration**: Parse .gitconfig files
- **Editor configuration**: Parse .editorconfig files
- **Windows applications**: Parse INI config files
- **PHP configuration**: Parse php.ini files
- **Database configuration**: Parse database config files
- **Static analysis**: Analyze configuration structure
- **Config validation**: Validate configuration files
- **Code generation**: Transform config to code

## Comparison with Other Parsers

Unlike many INI parsers that return plain JavaScript objects, `@sylphx/synth-ini` returns a universal Synth AST that:

- Preserves structural information (sections, key-value pairs)
- Supports powerful transformations via plugins
- Integrates with other Synth parsers
- Enables cross-language AST operations
- Provides consistent API across all formats

## License

MIT

# @sylphx/synth-css

CSS3 parser using Synth's universal AST. Hand-written, zero dependencies.

## Features

- ✅ **100% In-House** - Hand-written tokenizer and parser, zero external dependencies
- 🚀 **CSS3 Support** - Modern CSS including variables, grid, flexbox
- 🎯 **Universal AST** - Converts CSS to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Lightweight** - No dependencies except @sylphx/synth core

## Installation

```bash
npm install @sylphx/synth-css
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-css'

const css = `
  .button {
    background-color: #007bff;
    padding: 10px 20px;
    border-radius: 4px;
  }
`

const tree = parse(css)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { CSSParser, createParser, parse, parseAsync } from '@sylphx/synth-css'

// Standalone function (recommended)
const tree = parse('.class { color: red; }')

// Async parsing (for plugins)
const tree = await parseAsync('.class { color: red; }')

// Class instance
const parser = new CSSParser()
const tree = parser.parse('.class { color: red; }')

// Factory function
const parser = createParser()
const tree = parser.parse('.class { color: red; }')
```

### Plugin System

```typescript
import { parse, type Tree } from '@sylphx/synth-css'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse('.class { color: red; }', { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync('.class { color: red; }', { plugins: [asyncPlugin] })
```

### Persistent Parser

```typescript
const parser = new CSSParser()

// Register plugins
parser.use(plugin1).use(plugin2)

// Parse multiple files
const tree1 = parser.parse(css1)
const tree2 = parser.parse(css2)

// Get last parsed tree
const lastTree = parser.getTree()
```

## AST Structure

The parser generates a universal Synth AST with these CSS-specific node types:

### StyleRule

```typescript
{
  type: 'StyleRule',
  data: {
    selector: '.button'
  },
  children: [/* Declaration nodes */]
}
```

### AtRule

```typescript
{
  type: 'AtRule',
  data: {
    name: 'media',
    params: 'screen and (max-width: 768px)'
  },
  children: [/* Nested rules */]
}
```

### Declaration

```typescript
{
  type: 'Declaration',
  data: {
    property: 'background-color',
    value: '#007bff'
  }
}
```

## Supported CSS Features

### Selectors
- ✅ Element: `div`, `p`, `body`
- ✅ Class: `.button`, `.container`
- ✅ ID: `#header`, `#nav`
- ✅ Universal: `*`
- ✅ Descendant: `div p`
- ✅ Child: `ul > li`
- ✅ Adjacent sibling: `h1 + p`
- ✅ General sibling: `h1 ~ p`
- ✅ Attribute: `[type="text"]`, `[href^="http"]`
- ✅ Pseudo-class: `:hover`, `:first-child`
- ✅ Pseudo-element: `::before`, `::after`
- ✅ Multiple selectors: `h1, h2, h3`

### At-Rules
- ✅ `@media` - Media queries
- ✅ `@keyframes` - Animations
- ✅ `@import` - External stylesheets
- ✅ `@font-face` - Custom fonts
- ✅ Nested at-rules

### Properties
- ✅ Colors: `#fff`, `rgb()`, `rgba()`, `hsl()`, named colors
- ✅ Lengths: `px`, `em`, `rem`, `%`, `vh`, `vw`
- ✅ Functions: `calc()`, `var()`, `url()`, gradients
- ✅ Shorthand: `margin`, `padding`, `border`, etc.
- ✅ `!important` declarations
- ✅ CSS Variables: `--primary-color`

### Modern CSS
- ✅ Flexbox: `display: flex`, `justify-content`, etc.
- ✅ Grid: `display: grid`, `grid-template-columns`, etc.
- ✅ CSS Variables: `:root { --color: red; }`
- ✅ CSS Functions: `var()`, `calc()`, etc.

## Examples

### Flexbox Layout

```typescript
const css = `
  .flex-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }
`

const tree = parse(css)
```

### Media Queries

```typescript
const css = `
  @media screen and (max-width: 768px) {
    .container {
      width: 100%;
    }
  }
`

const tree = parse(css)
```

### CSS Variables

```typescript
const css = `
  :root {
    --primary-color: #007bff;
    --spacing: 1rem;
  }

  .button {
    background-color: var(--primary-color);
    padding: var(--spacing);
  }
`

const tree = parse(css)
```

### Animations

```typescript
const css = `
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .animated {
    animation: fadeIn 0.3s ease-in-out;
  }
`

const tree = parse(css)
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
- **Hand-written parser** - Optimized for CSS grammar
- **Zero dependencies** - Only depends on @sylphx/synth core
- **Simple grammar** - CSS is well-suited for custom implementation

See the main README for the full development strategy.

## License

MIT

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

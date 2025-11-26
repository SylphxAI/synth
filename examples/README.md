# Synth Examples

This directory contains examples demonstrating the various features of Synth.

## Running Examples

```bash
# Build first
pnpm build

# Run an example
node examples/basic.js
node examples/composition.js
```

## Examples

### `basic.ts`
Demonstrates:
- Basic parsing and compilation
- Tree structure
- Simple transformations
- Zipper navigation

### `composition.ts`
Demonstrates:
- Functional composition with `compose()` and `pipe()`
- Transform utilities (`tap`, `timed`)
- Building transformation pipelines
- Metadata manipulation

## Creating Your Own Examples

```typescript
import { synth } from '../src/index.js'
import { markdown } from '../src/adapters/index.js'

const processor = synth()
  .adapter('markdown', markdown())

// Your code here
```

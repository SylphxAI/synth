# @sylphx/synth-md

## 0.2.2 (2025-11-29)

### 🐛 Bug Fixes

- add @module JSDoc tag for proper TypeDoc generation ([9e660b8](https://github.com/SylphxAI/synth/commit/9e660b8ac886db7d0697d70a1b8ecaae1aeb1a10))

### 📚 Documentation

- add @packageDocumentation tag to all packages ([8f3070a](https://github.com/SylphxAI/synth/commit/8f3070a4011661b76632b3ec6ee6c967484f849c))

## 0.2.2 (2025-11-29)

### 🐛 Bug Fixes

- add @module JSDoc tag for proper TypeDoc generation ([9e660b8](https://github.com/SylphxAI/synth/commit/9e660b8ac886db7d0697d70a1b8ecaae1aeb1a10))

### 📚 Documentation

- add @packageDocumentation tag to all packages ([8f3070a](https://github.com/SylphxAI/synth/commit/8f3070a4011661b76632b3ec6ee6c967484f849c))

## 0.2.2 (2025-11-29)

### 🐛 Bug Fixes

- add @module JSDoc tag for proper TypeDoc generation ([9e660b8](https://github.com/SylphxAI/synth/commit/9e660b8ac886db7d0697d70a1b8ecaae1aeb1a10))

### 📚 Documentation

- add @packageDocumentation tag to all packages ([8f3070a](https://github.com/SylphxAI/synth/commit/8f3070a4011661b76632b3ec6ee6c967484f849c))

## 0.2.1 (2025-11-27)

### 📦 Dependencies

- Updated `@sylphx/synth` to 0.3.0

## 0.2.0 (2025-11-27)

### ✨ Features

- upgrade to biome v2 ([33b04fd](https://github.com/SylphxAI/synth/commit/33b04fdb725a6ff103e74cfa9a0011ed7ea0bba1))
- add build and prepack scripts to all packages ([b8e1b54](https://github.com/SylphxAI/synth/commit/b8e1b548753a499acfd96bb0083d8b1339498f7e))

### 🐛 Bug Fixes

- resolve all TypeScript type errors for doctor 1.21+ compliance ([1e9b03f](https://github.com/SylphxAI/synth/commit/1e9b03fd2901db35f89a553a8a650ab3c57e5d3e))

### 🔧 Chores

- use workspace:* for internal dependencies ([289b280](https://github.com/SylphxAI/synth/commit/289b28017951aea84891537e758df86f7bbb6780))
- add changeset for project improvements ([616016b](https://github.com/SylphxAI/synth/commit/616016bea15d088c932bc000133ad3bc039edd36))
- achieve 100% doctor score (v1.18.0) ([454faf8](https://github.com/SylphxAI/synth/commit/454faf882e00f6caccd99e363e765a16a1be196a))
- improve project health score to 100% ([4a46114](https://github.com/SylphxAI/synth/commit/4a461142c8fa8e28c501d46a7579220dd08dbc75))
- version packages - v0.2.0 synth-js, v0.1.3 synth ([c546c55](https://github.com/SylphxAI/synth/commit/c546c5582b77a06e6899a2f12dfc7b76264560a2))

## 0.1.4 (2025-11-27)

### 🐛 Bug Fixes

- TypeScript support improvements ([864099b](https://github.com/SylphxAI/synth/commit/864099bbd7c6daf879b76736c6387d0a1386f3b5))

### 🔧 Chores

- achieve 100% doctor score (v1.18.0) ([454faf8](https://github.com/SylphxAI/synth/commit/454faf882e00f6caccd99e363e765a16a1be196a))
- improve project health score to 100% ([4a46114](https://github.com/SylphxAI/synth/commit/4a461142c8fa8e28c501d46a7579220dd08dbc75))
- version packages - v0.2.0 synth-js, v0.1.3 synth ([c546c55](https://github.com/SylphxAI/synth/commit/c546c5582b77a06e6899a2f12dfc7b76264560a2))

## 0.1.3

### Patch Changes

- 864099b: Fix TypeScript support:

  - **synth-js**: Enable TypeScript parsing by default (`typescript: true`)
  - **All packages**: Fix exports order (`types` before `import`) for proper TypeScript module resolution

- Updated dependencies [864099b]
  - @sylphx/synth@0.1.3

## 0.1.2

### Patch Changes

- be71f47: docs: fix README files with correct package names and install commands

  - Fixed @sylphx/synth README: replaced old @sylphx/ast-core name with correct package name
  - Fixed @sylphx/synth-html README: typo "bpm install" → "npm install"
  - Standardized install commands: changed "bun install" → "npm install" across packages
  - Replaced all "Flux AST" and "flux" references with "Synth" and "synth"
  - Updated benchmarks README files with correct project name

- Updated dependencies [be71f47]
  - @sylphx/synth@0.1.2

## 0.1.1

### Patch Changes

- 2b393c9: fix: resolve workspace:^ dependencies to actual version numbers

  v0.1.0 was published with broken dependencies containing literal "workspace:^"
  instead of actual version numbers. This patch release fixes the dependency
  declarations so packages can be installed correctly.

  The root cause was that changesets uses npm publish internally, which doesn't
  understand the workspace:^ protocol used by bun/pnpm workspaces.

- Updated dependencies [2b393c9]
  - @sylphx/synth@0.1.1

## 0.1.0

### Minor Changes

- ce8dd19: # Initial Release - Synth AST Processor

  ## 🎉 First Public Release

  Complete multi-language AST processing framework with token-level incremental parsing, zero runtime dependencies, and unified architecture.

  ## Core Infrastructure

  **@sylphx/synth** - Universal AST foundation

  - Language-agnostic BaseNode interface
  - Generic plugin system (transform, visitor, parser)
  - Node pooling for memory efficiency (30% less GC pressure)
  - Tree traversal utilities (walk, find, filter, map)
  - Serialization system (JSON, MessagePack)
  - Zero runtime dependencies

  ## Language Parsers

  **Markdown Ecosystem**

  - **@sylphx/synth-md** - Full GFM parser with incremental parsing
  - **@sylphx/synth-md-gfm** - GitHub Flavored Markdown extensions
  - **@sylphx/synth-md-katex** - LaTeX math rendering support
  - **@sylphx/synth-md-mermaid** - Mermaid diagram support

  **Web Languages**

  - **@sylphx/synth-html** - HTML parser with void element handling
  - **@sylphx/synth-css** - CSS parser with selector support
  - **@sylphx/synth-jsx** - JSX/TSX parser
  - **@sylphx/synth-xml** - XML parser

  **Programming Languages**

  - **@sylphx/synth-js** - JavaScript/TypeScript (via acorn)
  - **@sylphx/synth-python** - Python parser
  - **@sylphx/synth-rust** - Rust parser
  - **@sylphx/synth-go** - Go parser
  - **@sylphx/synth-c** - C/C++ parser
  - **@sylphx/synth-java** - Java parser
  - **@sylphx/synth-php** - PHP parser
  - **@sylphx/synth-ruby** - Ruby parser

  **Data Formats**

  - **@sylphx/synth-json** - JSON parser with incremental support
  - **@sylphx/synth-yaml** - YAML parser
  - **@sylphx/synth-toml** - TOML parser
  - **@sylphx/synth-ini** - INI parser
  - **@sylphx/synth-msgpack** - MessagePack serialization
  - **@sylphx/synth-protobuf** - Protocol Buffers parser

  **Specialized**

  - **@sylphx/synth-graphql** - GraphQL schema parser
  - **@sylphx/synth-sql** - SQL query parser
  - **@sylphx/synth-vue** - Vue SFC parser

  ## Analysis & Transformation

  **@sylphx/synth-lint** - Universal linting framework

  - Rule system for any language
  - Built-in rules (no-empty-blocks, no-console, max-depth)
  - Custom rule creation API

  **@sylphx/synth-metrics** - Code metrics & complexity

  - Cyclomatic complexity
  - Maintainability index
  - Code smells detection

  **@sylphx/synth-typecheck** - Type checking utilities

  **@sylphx/synth-js-format** - JavaScript/TypeScript formatter

  **@sylphx/synth-js-minify** - JavaScript minifier

  **@sylphx/synth-docs** - Documentation generator

  ## Performance Features

  **Token-Level Incremental Parsing**

  - 85-99% token reuse for typical edits
  - <1ms response time for document updates
  - 6-100x speedup over full re-parsing
  - Smart boundary expansion strategies
  - Binary search token lookup (O(log n))

  **Memory Optimization**

  - Node pooling (30% less GC pressure)
  - Session-based memory management
  - LRU eviction for multi-document scenarios

  ## Architecture Highlights

  - Universal AST interface across all languages
  - Composable plugin system
  - Zero runtime dependencies (core packages)
  - Strategic dependencies for complex parsers
  - Monorepo with 32 packages
  - Built with Bun + bunup (100ms build time)
  - TypeScript with isolatedDeclarations

  ## Build System

  - **bunup** - Monorepo build tool with workspace support
  - Incremental builds (only changed packages)
  - Automatic .d.ts generation
  - 50-100x faster than traditional tsc
  - Full TypeScript 5.5+ isolatedDeclarations support

  ## Breaking Changes

  None - this is the initial release.

### Patch Changes

- Updated dependencies [ce8dd19]
  - @sylphx/synth@0.1.0

## 0.1.0

### Minor Changes

- Initial release of Synth

  This is the first published version of Synth, a high-performance AST processing toolkit with zero dependencies.

  **Packages:**

  - **@sylphx/synth**: Core AST infrastructure with types, tree traversal, zipper navigation, query index, incremental parsing, optimizations (batch processing, node pooling), and plugin system
  - **@sylphx/synth-md**: High-performance Markdown parser achieving 26-42x faster parsing than remark, with streaming and incremental parsing support

  **Features:**

  - Unified core package with all essential functionality
  - Language-specific parser plugins
  - Tree-shakable packages for minimal bundle size
  - TypeScript with full type safety
  - Comprehensive test coverage
  - Optimized for performance with SIMD-style batch processing
  - Support for incremental and streaming parsing

### Patch Changes

- Updated dependencies
  - @sylphx/synth@0.1.0

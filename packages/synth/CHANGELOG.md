# @sylphx/synth

## 0.1.3

### Patch Changes

- 864099b: Fix TypeScript support:

  - **synth-js**: Enable TypeScript parsing by default (`typescript: true`)
  - **All packages**: Fix exports order (`types` before `import`) for proper TypeScript module resolution

## 0.1.2

### Patch Changes

- be71f47: docs: fix README files with correct package names and install commands

  - Fixed @sylphx/synth README: replaced old @sylphx/ast-core name with correct package name
  - Fixed @sylphx/synth-html README: typo "bpm install" → "npm install"
  - Standardized install commands: changed "bun install" → "npm install" across packages
  - Replaced all "Flux AST" and "flux" references with "Synth" and "synth"
  - Updated benchmarks README files with correct project name

## 0.1.1

### Patch Changes

- 2b393c9: fix: resolve workspace:^ dependencies to actual version numbers

  v0.1.0 was published with broken dependencies containing literal "workspace:^"
  instead of actual version numbers. This patch release fixes the dependency
  declarations so packages can be installed correctly.

  The root cause was that changesets uses npm publish internally, which doesn't
  understand the workspace:^ protocol used by bun/pnpm workspaces.

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

## 0.1.0

### Minor Changes

- Initial release of Synth

  This is the first published version of Synth, a high-performance AST processing toolkit with zero dependencies.

  **Packages:**

  - **@sylphx/synth**: Unified core package containing AST infrastructure (types, tree traversal, zipper navigation, query index, incremental parsing), performance optimizations (batch processing, node pooling), and plugin system
  - **@sylphx/synth-md**: High-performance Markdown parser achieving 26-42x faster parsing than remark, with streaming and incremental parsing support

  **Features:**

  - Unified core package with all essential functionality
  - Language-specific parser plugins (@sylphx/synth-\*)
  - Tree-shakable exports for minimal bundle size
  - TypeScript with full type safety
  - Comprehensive test coverage
  - Optimized for performance with SIMD-style batch processing
  - Support for incremental and streaming parsing
  - Zero runtime dependencies

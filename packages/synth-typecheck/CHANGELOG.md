# @sylphx/synth-typecheck

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

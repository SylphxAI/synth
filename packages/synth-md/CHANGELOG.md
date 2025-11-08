# @sylphx/synth-md

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

# MCP Family AST Foundation Roadmap

Decision record: `docs/adr/ADR-35-mcp-family-ast-foundation.md`

Synth is the `@sylphx/synth*` universal AST substrate for the Sylphx MCP family.
It should make source structure fast, stable, and portable for Rust-native MCP
products without becoming an MCP server itself.

`SylphxAI/ast` owns the separate `@sylphlab/ast-*` ANTLR-backed typed parser
contract line. The two foundations must be evaluated through public package
exports, fixtures, benchmarks, and source-span conformance rather than private
workspace coupling.

## Role In The Family

Synth gives the portfolio a shared structural language:

- repository architecture tools use AST spans, symbols, imports, and hierarchy
  to build evidence graphs;
- code retrieval tools use language-aware chunks, symbol neighborhoods, and
  incremental re-indexing signals;
- reader tools can use structured Markdown, HTML, JSON, YAML, and document-side
  code blocks as citeable evidence;
- filesystem tools can validate safe transformations against parser spans;
- consultation tools can cite structural facts instead of prose-only summaries.

The product MCP servers own runtime behavior. Synth owns the reusable parser and
AST contracts they validate against.

## Architecture Target

- Synth public packages expose stable AST contracts, parser options, traversal
  helpers, query primitives, and fixtures.
- AST public packages expose ANTLR-backed typed parser contracts and grammar
  fixtures for the `@sylphlab/ast-*` package line.
- Generated metadata describes package capability, language coverage, runtime
  requirements, and benchmark status.
- Rust/WASM bridge outputs are treated as public package surfaces when consumed
  by Rust-native MCP products.
- Product repos use official Rust MCP SDK servers and consume Synth only through
  public packages, generated schemas, fixtures, or documented Rust/WASM outputs.
- No product imports Synth workspace internals.

## Roadmap

### Phase 0: Boundary And Decision

- Record Synth's role as MCP-family AST foundation.
- Make explicit that Synth is not an MCP runtime and will not add a TypeScript
  MCP adapter.
- Link the role from project manifests and docs.

Exit gate: ADR and project manifests land with JSON/whitespace validation.

### Phase 1: Generated Capability Catalog

- Generate a catalog from `packages/*/package.json`.
- Include language, package name, runtime requirements, parser mode, WASM usage,
  benchmark group, and public exports.
- Use the generated catalog as the source for family docs and product planning.

Exit gate: catalog generation/check runs in CI and fails drift.

### Phase 2: Contract Fixtures

- Add cross-language AST fixtures for imports, exports, symbols, classes,
  functions, doc comments, markdown headings, tables, links, code blocks, and
  frontmatter.
- Add source-span invariants and expected query results.
- Keep fixtures product-neutral; product-specific labels belong in consuming
  repos.

Exit gate: fixture test suite proves stable parse output and query behavior.

### Phase 3: Rust/WASM Consumption Path

- Define the supported path for Rust-native MCP products to consume Synth
  output: package API, generated fixture files, WASM component, Rust crate, or
  offline generated index.
- Benchmark startup cost, parse latency, memory ceiling, and binary/package
  footprint for each viable path.
- Prefer the simplest path that keeps Rust MCP runtime installation reliable.

Exit gate: at least one Rust-native consumer path is benchmarked and documented
  without private workspace imports.

### Phase 4: Portfolio Conformance

- `architecture-reader-mcp` validates evidence graph extraction against Synth
  and AST fixtures where each substrate is selected.
- `coderag` validates chunking and symbol-neighborhood behavior against Synth
  and AST fixtures where each substrate is selected.
- reader products validate structured text evidence where Synth owns the parser
  contract.
- Mission Control tracks consumption proof as Work Items and links PR/CI
  readback.

Exit gate: each consuming product has its own tests and ADR/reference to the
  Synth contract it uses.

## Performance Bar

- Parser output must be deterministic for unchanged input.
- Source spans must remain stable across minor package releases unless an ADR
  records the migration.
- Incremental parsing must prove token/node reuse where advertised.
- Benchmark claims must be backed by checked benchmark artifacts or generated
  reports.
- Runtime consumption paths must not make MCP server install fragile.

## Non-Goals

- Build an MCP server inside Synth.
- Add a TypeScript MCP adapter for product repos.
- Encode repository-architecture, search-ranking, filesystem policy, or
  consultation semantics inside Synth core.
- Let product repos import private Synth workspace modules.
- Claim ownership of `SylphxAI/ast` `@sylphlab/ast-*` parser contracts.

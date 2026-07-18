# Synth

Synth is the Sylphx universal AST processing and language-tooling monorepo. It
owns the `@sylphx/synth*` package family, language parser packages, AST
traversal and query primitives, WASM parser bridges, and tooling packages for
formatting, minification, linting, metrics, and documentation generation.

Project identity is split by boundary: vendor-neutral project facts live in
[`project.manifest.json`](./project.manifest.json), while Sylphx-specific governance facts live in
[`.doctrine/project.json`](./.doctrine/project.json).

## Lifecycle And Layer

- Lifecycle: `production`
- Layer: `foundation`

## Goals

- Provide a fast, shared universal AST substrate for Sylphx and external users
  through the `@sylphx/synth*` package family.
- Publish the `@sylphx/synth*` npm package family for parsers and AST tooling.
- Keep parser/tooling package boundaries clean so downstream applications can
  consume public package exports without repo-internal knowledge.
- Serve the Rust-native MCP family as a product-neutral AST foundation through
  public packages, generated contracts, fixtures, benchmarks, and Rust/WASM
  bridge outputs.

## Non-Goals

- Application-specific content workflows, UI behavior, billing, auth, or runtime
  deployment belong outside this repository.
- Consumer-specific syntax hacks or product-only AST behavior do not belong in
  Synth core packages.
- ANTLR-backed `@sylphlab/ast-*` parser contracts belong in
  `SylphxAI/ast`, not in Synth.
- MCP server runtime, transport, tool schemas, install packaging, and product
  roadmaps belong in the owning MCP repositories, not in Synth.
- Sylphx Platform CI, preview, deploy, and runner ownership remain outside this
  repository.

## Boundary Summary

Synth owns AST contracts, parser implementations, package exports, docs for the
package family, and benchmark/tooling surfaces directly tied to
`@sylphx/synth*` packages. `SylphxAI/ast` owns the separate
`@sylphlab/ast-*` ANTLR parser-contract line. Consumers use published packages
and documentation; they do not reach into workspace internals or couple product
behavior to private parser modules.

## Public Surfaces

- npm packages under `@sylphx/synth*`, declared in `packages/*/package.json`.
- Documentation under `README.md` and `docs/`.
- MCP family AST foundation roadmap:
  [`docs/roadmap/mcp-family-ast-foundation.md`](./docs/roadmap/mcp-family-ast-foundation.md)
- Vendor-neutral project manifest at `project.manifest.json`.
- Required CI contexts: `risk-classification/pass`, `ci`, and `trunk-admission/pass`.
- Release workflow in `.github/workflows/release.yml` publishing through
  the SylphxAI/.github reusable Changesets release workflow.

## Delivery Proof

Pull requests target `main` and pass ADR-29 admission contexts before merge.
Pull requests and merge groups run `.github/workflows/ci.yml`, including ADR-29 classification/fan-in contexts, isolated Rust setup, `bun run validate`, project-control boundary tests, and GroundAtlas package dogfooding. Main pushes run postsubmit proof and recovery-decision wiring plus `.github/workflows/release.yml` for Changesets release PRs or package publication; the release build bootstraps Rust when needed, installs the Rust WASM target and `wasm-pack`, and then builds Synth WASM packages. Published package changes require Release workflow evidence and npm registry readback. Generated `.groundatlas*` files plus GroundAtlas JSON/Markdown reports are evidence/navigation only, not source of truth.


## GroundAtlas

GroundAtlas package dogfood is **retired** (Control Plane ADR-0014). Do not re-add required groundatlas CI jobs.

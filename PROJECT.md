# Synth

Synth is the Sylphx universal AST processing and language-tooling monorepo. It
owns the `@sylphx/synth` package family, language parser packages, AST traversal
and query primitives, WASM parser bridges, and tooling packages for formatting,
minification, linting, metrics, and documentation generation.

Machine-readable project identity lives in
[.doctrine/project.json](./.doctrine/project.json).

## Lifecycle And Layer

- Lifecycle: `production`
- Layer: `foundation`

## Goals

- Provide a fast, shared universal AST substrate for Sylphx and external users.
- Publish the `@sylphx/synth*` npm package family for parsers and AST tooling.
- Keep parser/tooling package boundaries clean so downstream applications can
  consume public package exports without repo-internal knowledge.

## Non-Goals

- Application-specific content workflows, UI behavior, billing, auth, or runtime
  deployment belong outside this repository.
- Consumer-specific syntax hacks or product-only AST behavior do not belong in
  Synth core packages.
- Sylphx Platform CI, preview, deploy, and runner ownership remain outside this
  repository.

## Boundary Summary

Synth owns AST contracts, parser implementations, package exports, docs for the
package family, and benchmark/tooling surfaces directly tied to Synth packages.
Consumers use the published packages and documentation; they do not reach into
workspace internals or couple product behavior to private parser modules.

## Public Surfaces

- npm packages under `@sylphx/synth*`, declared in `packages/*/package.json`.
- Documentation under `README.md` and `docs/`.
- Required CI contexts: `risk-classification/pass` and `trunk-admission/pass`.
- Release workflow in `.github/workflows/release.yml` publishing through
  `Changesets`.

## Delivery Proof

Pull requests target `main` and pass ADR-29 admission contexts before merge.
Main-branch pushes run postsubmit proof and recovery-decision wiring through
`.github/workflows/ci.yml`. Package publication is performed by the release
workflow after `main` changes.

# Agent Instructions - Synth

This repository follows the central Sylphx doctrine:
<https://github.com/SylphxAI/doctrine>.

Before changing behavior, read:

- [PROJECT.md](./PROJECT.md), [project.manifest.json](./project.manifest.json), and [.doctrine/project.json](./.doctrine/project.json) for this repository's goal, boundary, public
  surfaces, delivery proof, vendor-neutral manifest, and Sylphx Doctrine adapter facts.

Local validation ladder:

```bash
bun install --frozen-lockfile
bun run validate
node --test test/project-control.node-test.mjs
npm exec --yes --package groundatlas@0.1.2 -- ga fleet . --out .groundatlas-pilot --require-atlas --strict --json
```

Rust/WASM parser changes also need the Rust toolchain with
`wasm32-unknown-unknown` and should pass the relevant package build before PR.

CI is wired through ADR-29 fan-in contexts:
`risk-classification/pass`, `ci`, and `trunk-admission/pass`.

Generated `.groundatlas*` reports are evidence/navigation only. Do not treat them as source of truth.

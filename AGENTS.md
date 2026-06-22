# Agent Instructions - Synth

This repository follows the central Sylphx doctrine:
<https://github.com/SylphxAI/doctrine>.

Before changing behavior, read:

- [PROJECT.md](./PROJECT.md) for this repository's goal, boundary, public
  surfaces, and delivery proof.
- [.doctrine/project.json](./.doctrine/project.json) for the machine-readable
  project manifest consumed by central audits.

Local validation ladder:

```bash
bun install
bun run lint
bun run typecheck
bun test
bun run build
```

Rust/WASM parser changes also need the Rust toolchain with
`wasm32-unknown-unknown` and should pass the relevant package build before PR.

CI is wired through ADR-29 fan-in contexts:
`risk-classification/pass` and `trunk-admission/pass`.

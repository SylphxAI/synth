# Project Control Gate Spec

## Purpose

Make Synth dogfood GroundAtlas as a real open-source package consumer while
keeping project truth split cleanly:

- `project.manifest.json` is the vendor-neutral per-project control file.
- `.doctrine/project.json` is the Sylphx Doctrine adapter and governance catalog.
- `.groundatlas*` files plus GroundAtlas JSON/Markdown reports are generated evidence/navigation only, not SSOT.

## Required Read Path

Before changing Synth, read the smallest relevant set:

1. `AGENTS.md`, `PROJECT.md`, `project.manifest.json`, and `.doctrine/project.json`.
2. `README.md`, `docs/guide/`, `docs/api/`, and package-level docs for public behavior changes.
3. `packages/*/package.json`, `crates/`, `src/`, `types/`, and relevant tests for package/source changes.
4. `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and release evidence for validation or publication changes.
5. `SECURITY.md` for vulnerability-reporting and public trust boundaries.

## CI Contract

The CI workflow must:

- install with `bun install --frozen-lockfile`;
- run `bun run validate` for the current lint, single-concurrency build, typecheck, and test baseline;
- run `node --test test/project-control.node-test.mjs`;
- run `SylphxAI/groundatlas@v0.1.3` with `package-spec: groundatlas@0.1.3`;
- require generated atlas evidence and strict fleet status;
- assert the Markdown fleet scorecard title and adopted summary;
- upload GroundAtlas JSON and Markdown reports as CI artifacts.

The Release workflow must bootstrap Rust when `rustup` is absent, install the
Rust `wasm32-unknown-unknown` target, and install `wasm-pack` before running the
shared release build command, because Synth publishes WASM-backed packages as
part of the package family.

## Done Evidence

A Synth project-control change is not done until PR CI, merge-group CI, main CI,
Release workflow status, and artifact/readback evidence are captured. Changed
packages additionally require npm registry readback for every published package.

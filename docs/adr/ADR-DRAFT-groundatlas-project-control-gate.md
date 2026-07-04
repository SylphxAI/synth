---
status: draft
slug: groundatlas-project-control-gate
---

# ADR-DRAFT: GroundAtlas Project Control Gate

## Context

Synth already owns public AST, parser, tooling, WASM, documentation, and release
surfaces. The repository also carries a Sylphx-specific `.doctrine/project.json`
adapter. Fleet dogfooding needs a vendor-neutral project manifest and CI proof
that the released GroundAtlas package/action can discover the repository without
turning generated reports into source of truth.

## Decision

Adopt `project.manifest.json` as Synth's vendor-neutral project control file.
Keep `.doctrine/project.json` as the Sylphx Doctrine adapter and org-local
governance catalog. CI MUST run `groundatlas@0.1.2` through the released
`SylphxAI/groundatlas@v0.1.2` action and assert that generated `.groundatlas*`
outputs are evidence/navigation only.

## Consequences

- Agents, contributors, and automation read `PROJECT.md`, `project.manifest.json`,
  `.doctrine/project.json`, `README.md`, `docs/`, workflows, source, tests, and
  release evidence before durable changes.
- Package publication remains owned by Synth's release workflow and requires CI,
  release evidence, and npm registry readback for changed packages.
- Organization-wide mandatory rulesets and scorecards remain outside this repo;
  Synth only provides repo-local manifest and dogfood evidence.

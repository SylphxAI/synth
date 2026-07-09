---
status: proposed
slug: mcp-family-ast-foundation
---

# ADR-35: MCP Family AST Foundation

## Context

Synth owns the public AST substrate for Sylphx parser, traversal, query,
tooling, and WASM parser surfaces. The MCP family now needs a consistent way for
repository architecture, code retrieval, reader evidence, filesystem, and
consultation tools to reason about source structure without each product
inventing its own parser contracts.

The MCP server runtime target for product repositories is Rust using the
official Rust MCP SDK. Synth is not an MCP server repository and should not add a
TypeScript MCP adapter to serve downstream products. Its leverage is the
foundation layer: stable AST contracts, parser fixtures, query semantics,
language coverage, benchmarks, and generated package metadata that Rust-native
MCP products can consume through public, versioned surfaces.

## Decision

Synth will serve the MCP family as the AST foundation, not as an MCP runtime.

Synth owns:

- language-neutral AST node contracts and source-span invariants;
- parser fixtures and conformance cases for language families;
- query and traversal semantics that downstream products can validate against;
- benchmark suites for parser latency, memory, incremental parsing, and package
  size;
- generated package capability metadata from the public package manifests;
- Rust/WASM parser bridge boundaries where they are part of Synth package
  delivery.

Product repositories own their MCP server behavior, tool schemas, security
policy, transport, install packaging, runtime telemetry, and customer-facing
roadmaps. Rust-native MCP servers may use Synth packages, generated schemas,
fixtures, or Rust/WASM bridge outputs, but must not import Synth workspace
internals or make Synth aware of product-specific workflows.

TypeScript remains acceptable inside Synth for existing package implementation
and public JavaScript package delivery. It is not a target MCP adapter layer for
the Sylphx MCP family.

## Consequences

- Architecture and code-intelligence products get one shared AST vocabulary
  without coupling Synth to their runtime decisions.
- Rust-native MCP products can validate their indexing, chunking, evidence, and
  navigation behavior against Synth fixtures instead of duplicating ad hoc
  parser expectations.
- Synth can improve language coverage and performance once for the portfolio.
- Product-specific semantics stay out of Synth core packages.
- Future shared primitives should be generated contracts, fixtures, crates, or
  package exports, not cross-repo private imports.

## Migration

1. Add a roadmap that defines Synth's MCP-family foundation role.
2. Generate a package capability catalog from `packages/*/package.json` before
   relying on a hand-written package list as current truth.
3. Define AST fixture groups for repository architecture and code retrieval
   consumers.
4. Add Rust/WASM bridge acceptance criteria for downstream Rust MCP products:
   deterministic parse output, stable spans, bounded memory, and benchmark
   thresholds.
5. Have product repositories consume the public contract through their own ADRs,
   tests, and release gates.

## Verification

- JSON project manifests parse.
- Repo-local documentation points to this decision and keeps current-state facts
  in project manifests.
- No MCP server package or TypeScript MCP adapter is added to Synth by this
  decision.
- Future implementation slices add generated contract checks or fixture tests
  before claiming product consumption is complete.

---
"@sylphx/synth": patch
"@sylphx/synth-c": patch
"@sylphx/synth-css": patch
"@sylphx/synth-docs": patch
"@sylphx/synth-go": patch
"@sylphx/synth-graphql": patch
"@sylphx/synth-html": patch
"@sylphx/synth-ini": patch
"@sylphx/synth-java": patch
"@sylphx/synth-js": patch
"@sylphx/synth-js-format": patch
"@sylphx/synth-js-minify": patch
"@sylphx/synth-json": patch
"@sylphx/synth-jsx": patch
"@sylphx/synth-lint": patch
"@sylphx/synth-md": patch
"@sylphx/synth-md-gfm": patch
"@sylphx/synth-md-katex": patch
"@sylphx/synth-md-mermaid": patch
"@sylphx/synth-metrics": patch
"@sylphx/synth-msgpack": patch
"@sylphx/synth-php": patch
"@sylphx/synth-protobuf": patch
"@sylphx/synth-python": patch
"@sylphx/synth-ruby": patch
"@sylphx/synth-rust": patch
"@sylphx/synth-sql": patch
"@sylphx/synth-toml": patch
"@sylphx/synth-typecheck": patch
"@sylphx/synth-vue": patch
"@sylphx/synth-xml": patch
"@sylphx/synth-yaml": patch
---

fix: resolve workspace:^ dependencies to actual version numbers

v0.1.0 was published with broken dependencies containing literal "workspace:^" 
instead of actual version numbers. This patch release fixes the dependency 
declarations so packages can be installed correctly.

The root cause was that changesets uses npm publish internally, which doesn't 
understand the workspace:^ protocol used by bun/pnpm workspaces.

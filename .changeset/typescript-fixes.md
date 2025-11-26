---
"@sylphx/synth": patch
"@sylphx/synth-js": minor
"@sylphx/synth-md": patch
"@sylphx/synth-html": patch
"@sylphx/synth-json": patch
"@sylphx/synth-yaml": patch
"@sylphx/synth-css": patch
"@sylphx/synth-sql": patch
"@sylphx/synth-graphql": patch
"@sylphx/synth-c": patch
"@sylphx/synth-go": patch
"@sylphx/synth-java": patch
"@sylphx/synth-php": patch
"@sylphx/synth-python": patch
"@sylphx/synth-ruby": patch
"@sylphx/synth-rust": patch
"@sylphx/synth-ini": patch
"@sylphx/synth-toml": patch
"@sylphx/synth-xml": patch
"@sylphx/synth-jsx": patch
"@sylphx/synth-vue": patch
"@sylphx/synth-protobuf": patch
"@sylphx/synth-msgpack": patch
"@sylphx/synth-md-gfm": patch
"@sylphx/synth-md-katex": patch
"@sylphx/synth-md-mermaid": patch
"@sylphx/synth-js-format": patch
"@sylphx/synth-js-minify": patch
"@sylphx/synth-lint": patch
"@sylphx/synth-metrics": patch
"@sylphx/synth-docs": patch
"@sylphx/synth-typecheck": patch
---

Fix TypeScript support:

- **synth-js**: Enable TypeScript parsing by default (`typescript: true`)
- **All packages**: Fix exports order (`types` before `import`) for proper TypeScript module resolution

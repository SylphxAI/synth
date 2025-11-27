import { defineWorkspace } from 'bunup'

export default defineWorkspace(
  [
    { name: 'synth', root: 'packages/synth' },
    { name: 'synth-c', root: 'packages/synth-c' },
    { name: 'synth-css', root: 'packages/synth-css' },
    { name: 'synth-docs', root: 'packages/synth-docs' },
    { name: 'synth-go', root: 'packages/synth-go' },
    { name: 'synth-graphql', root: 'packages/synth-graphql' },
    { name: 'synth-html', root: 'packages/synth-html' },
    { name: 'synth-ini', root: 'packages/synth-ini' },
    { name: 'synth-java', root: 'packages/synth-java' },
    { name: 'synth-js', root: 'packages/synth-js' },
    { name: 'synth-js-format', root: 'packages/synth-js-format' },
    { name: 'synth-js-minify', root: 'packages/synth-js-minify' },
    { name: 'synth-json', root: 'packages/synth-json' },
    { name: 'synth-jsx', root: 'packages/synth-jsx' },
    { name: 'synth-lint', root: 'packages/synth-lint' },
    { name: 'synth-md', root: 'packages/synth-md' },
    { name: 'synth-md-gfm', root: 'packages/synth-md-gfm' },
    { name: 'synth-md-katex', root: 'packages/synth-md-katex' },
    { name: 'synth-md-mermaid', root: 'packages/synth-md-mermaid' },
    { name: 'synth-metrics', root: 'packages/synth-metrics' },
    { name: 'synth-msgpack', root: 'packages/synth-msgpack' },
    { name: 'synth-php', root: 'packages/synth-php' },
    { name: 'synth-protobuf', root: 'packages/synth-protobuf' },
    { name: 'synth-python', root: 'packages/synth-python' },
    { name: 'synth-ruby', root: 'packages/synth-ruby' },
    { name: 'synth-rust', root: 'packages/synth-rust' },
    { name: 'synth-sql', root: 'packages/synth-sql' },
    { name: 'synth-toml', root: 'packages/synth-toml' },
    { name: 'synth-typecheck', root: 'packages/synth-typecheck' },
    { name: 'synth-vue', root: 'packages/synth-vue' },
    { name: 'synth-xml', root: 'packages/synth-xml' },
    { name: 'synth-yaml', root: 'packages/synth-yaml' },
  ],
  {
    // Shared options for all packages
    format: ['esm'],
    target: 'node',
    dts: true, // Generate TypeScript declaration files
  }
)

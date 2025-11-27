import { defineConfig } from 'bunup'

export default defineConfig({
  entry: ['src/index.ts'],
  outDir: 'dist',
  format: ['esm'],
  dts: true,
  external: ['../wasm/synth_wasm_md.js'],
})

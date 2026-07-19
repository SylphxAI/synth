import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'
import test from 'node:test'

const readText = (path) => readFileSync(path, 'utf8')
const readJson = (path) => JSON.parse(readFileSync(path, 'utf8'))

test('javascript golden parity corpus and tests exist', () => {
  assert.ok(existsSync('test/fixtures/javascript-parity/golden.json'))
  assert.ok(existsSync('packages/synth-wasm-js/test/parity-golden.test.ts'))
  assert.ok(existsSync('scripts/generate-javascript-golden.mjs'))
  assert.ok(existsSync('packages/synth-wasm-js/src/parity-normalize.ts'))
  assert.ok(existsSync('packages/synth-wasm-js/src/node-kind-names.ts'))

  const golden = readJson('test/fixtures/javascript-parity/golden.json')
  assert.ok(Object.keys(golden).length >= 10)
})

test('rust wasm-js crate includes golden parity test hook', () => {
  const lib = readText('crates/wasm-js/src/lib.rs')
  assert.ok(lib.includes('golden_fixtures_match_ts_baseline'))
  assert.ok(lib.includes('javascript-parity/golden.json'))
})

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

test('migration ledger records javascript golden evidence without fantasy parity_proven on residual TS', () => {
  const ledger = readJson('docs/specs/migration-ledger.json')
  const jsTs = ledger.capabilities.find((cap) => cap.id === 'parser/javascript-ts')
  const jsWasm = ledger.capabilities.find((cap) => cap.id === 'parser/javascript-wasm')

  assert.ok(jsTs)
  // Honesty: residual TS parser remains ts_only until plugins/incremental retire (not parity_proven).
  assert.equal(jsTs.state, 'ts_only')

  assert.ok(jsWasm)
  assert.ok(['rust_impl', 'parity_proven', 'authority_rust', 'ts_deleted'].includes(jsWasm.state))
  assert.ok(
    jsWasm.parityTest?.includes('test/fixtures/javascript-parity/golden.json') ||
      jsWasm.parityTest?.includes('synth-wasm-js') ||
      jsWasm.parityTest?.includes('golden_fixtures_match_ts_baseline'),
  )
})

test('rust wasm-js crate includes golden parity test hook', () => {
  const lib = readText('crates/wasm-js/src/lib.rs')
  assert.ok(lib.includes('golden_fixtures_match_ts_baseline'))
  assert.ok(lib.includes('javascript-parity/golden.json'))
})

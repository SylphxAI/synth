import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'
import { test } from 'node:test'

function readText(path) {
  return readFileSync(path, 'utf8')
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'))
}

test('check-no-ts-js-parser gate script exists and enforces wasm authority routing', () => {
  const script = readText('scripts/check-no-ts-js-parser.sh')

  assert.ok(script.includes('check-no-ts-js-parser'))
  assert.ok(script.includes('packages/synth-js/src/wasm-authority.ts'))
  assert.ok(script.includes('packages/synth-wasm-js/src/parse-sync.ts'))
  assert.ok(script.includes('packages/synth-wasm-js/src/tree-bridge.ts'))
  assert.ok(script.includes('parser/javascript-wasm'))
  assert.ok(script.includes('parser/javascript-ts'))
  // Honesty-aligned: accepts rust_impl; does not demand fantasy parity_proven/slices for residual TS.
  assert.ok(script.includes('rust_impl'))
  assert.ok(script.includes('docs/specs/migration-ledger.json'))
  assert.ok(script.includes('isWasmAuthorityEligible(options)'))
  assert.ok(script.includes('parseViaWasmAuthority(code)'))
  assert.ok(script.includes('parseViaWasmAuthorityAsync(code)'))
  assert.ok(script.includes('golden_fixtures_match_ts_baseline'))
  assert.ok(script.includes('javascript-parity/golden.json'))
  assert.ok(script.includes('javascript-parity-golden.node-test.mjs'))

  assert.ok(existsSync('packages/synth-js/src/wasm-authority.ts'))
  assert.ok(existsSync('packages/synth-js/src/wasm-authority.test.ts'))
  assert.ok(existsSync('packages/synth-wasm-js/src/parse-sync.ts'))
  assert.ok(existsSync('packages/synth-wasm-js/src/init-sync.ts'))
  assert.ok(existsSync('packages/synth-wasm-js/src/tree-bridge.ts'))
  assert.ok(existsSync('test/javascript-parity-golden.node-test.mjs'))
  assert.ok(existsSync('test/fixtures/javascript-parity/golden.json'))
  assert.ok(existsSync('docs/specs/migration-ledger.json'))
})

test('default @sylphx/synth-js parse exports gate TS fallback behind wasm authority', () => {
  const parser = readText('packages/synth-js/src/parser.ts')
  const wasmAuthority = readText('packages/synth-js/src/wasm-authority.ts')
  const parseSync = readText('packages/synth-wasm-js/src/parse-sync.ts')
  const treeBridge = readText('packages/synth-wasm-js/src/tree-bridge.ts')

  assert.equal((parser.match(/if \(isWasmAuthorityEligible\(options\)\)/g) ?? []).length, 2)
  assert.ok(parser.includes('parseViaWasmAuthority(code)'))
  assert.ok(parser.includes('parseViaWasmAuthorityAsync(code)'))
  assert.ok(wasmAuthority.includes('@sylphx/synth-wasm-js'))
  assert.ok(wasmAuthority.includes('parseSync as parseWasmSync'))
  assert.ok(wasmAuthority.includes('parse as parseWasmAsync'))
  assert.ok(wasmAuthority.includes("return override !== 'ts'"))
  assert.ok(wasmAuthority.includes('useTsParser === true'))
  assert.ok(wasmAuthority.includes('parseWasmSync'))
  assert.ok(wasmAuthority.includes('await initWasm()'))
  assert.ok(wasmAuthority.includes('parseWasmAsync'))
  assert.ok(parseSync.includes('parseBinary'))
  assert.ok(parseSync.includes('binaryToTree'))
  assert.ok(parseSync.includes('initWasmSync'))
  assert.ok(treeBridge.includes('binaryToTree'))
})

test('ledger honesty: javascript-wasm rust_impl + residual javascript-ts; golden evidence on disk', () => {
  const ledger = readJson('docs/specs/migration-ledger.json')
  const jsTs = ledger.capabilities.find((cap) => cap.id === 'parser/javascript-ts')
  const jsWasm = ledger.capabilities.find((cap) => cap.id === 'parser/javascript-wasm')

  assert.ok(jsTs)
  // Residual TS path for plugins/incremental — not parity_proven without full cutover.
  assert.equal(jsTs.state, 'ts_only')
  assert.ok(jsTs.tsSurface?.includes('packages/synth-js/src/parser.ts'))

  assert.ok(jsWasm)
  assert.ok(['rust_impl', 'parity_proven', 'authority_rust', 'ts_deleted'].includes(jsWasm.state))
  if (jsWasm.state === 'rust_impl') {
    assert.equal(jsWasm.promotionHold?.active, true)
    assert.equal(jsWasm.promotionHold?.rejectionRef, 'rej-010')
  }
  assert.ok(
    jsWasm.parityTest?.includes('test/fixtures/javascript-parity/golden.json') ||
      jsWasm.parityTest?.includes('synth-wasm-js') ||
      jsWasm.parityTest?.includes('check-no-ts-js-parser'),
  )

  const golden = readJson('test/fixtures/javascript-parity/golden.json')
  assert.ok(Object.keys(golden).length >= 10)
})

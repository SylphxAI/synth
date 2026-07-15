import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'
import test from 'node:test'

const readText = (path) => readFileSync(path, 'utf8')

test('check-no-ts-parser gate script exists and enforces wasm authority routing', () => {
  const script = readText('scripts/check-no-ts-parser.sh')

  assert.ok(script.includes('check-no-ts-parser'))
  assert.ok(script.includes('isWasmAuthorityEligible'))
  assert.ok(script.includes('parseViaWasmAuthority'))
  assert.ok(script.includes('parseViaWasmAuthorityAsync'))
  assert.ok(script.includes('useTsParser'))
  assert.ok(script.includes('parser/markdown-wasm'))
  // Honesty-aligned: gate accepts rust_impl under rej-010; still may mention authority_rust as allowed state.
  assert.ok(script.includes('rust_impl'))
  assert.ok(script.includes('docs/specs/migration-ledger.json'))
  assert.ok(script.includes('crates/wasm-md/src/parser_v2.rs'))
  assert.ok(script.includes('parse-sync.ts'))
  assert.ok(script.includes('init-sync.ts'))
  assert.ok(script.includes('@sylphx/synth-wasm-md'))
  assert.ok(script.includes('check:no-ts-parser'))
  assert.ok(existsSync('test/fixtures/markdown-parity/golden.json'))
  assert.ok(existsSync('packages/synth-wasm-md/test/parity-golden.test.ts'))
  assert.ok(existsSync('packages/synth-md/src/wasm-authority.test.ts'))
  assert.ok(existsSync('docs/specs/migration-ledger.json'))
  assert.ok(existsSync('docs/specs/synth-migration-ledger.json'))
})

test('default @sylphx/synth-md parse exports gate TS fallback behind wasm authority', () => {
  const parser = readText('packages/synth-md/src/parser.ts')
  const wasmAuthority = readText('packages/synth-md/src/wasm-authority.ts')
  const parseSync = readText('packages/synth-wasm-md/src/parse-sync.ts')

  assert.equal((parser.match(/if \(isWasmAuthorityEligible\(options\)\)/g) ?? []).length, 2)
  assert.ok(parser.includes('parseViaWasmAuthority(markdown)'))
  assert.ok(parser.includes('parseViaWasmAuthorityAsync(markdown)'))
  assert.ok(wasmAuthority.includes("return override !== 'ts'"))
  assert.ok(wasmAuthority.includes('useTsParser === true'))
  assert.ok(wasmAuthority.includes('@sylphx/synth-wasm-md'))
  assert.ok(wasmAuthority.includes('parseWasmSync'))
  assert.ok(wasmAuthority.includes('await initWasm()'))
  assert.ok(wasmAuthority.includes('parseWasmAsync'))
  assert.ok(parseSync.includes('parseToJson'))
  assert.ok(parseSync.includes('initWasmSync'))
})

test('migration ledger records markdown-wasm rust_impl with honesty demote + routing evidence', () => {
  const ledger = JSON.parse(readText('docs/specs/migration-ledger.json'))
  const entry = ledger.capabilities.find((cap) => cap.id === 'parser/markdown-wasm')

  assert.ok(entry)
  assert.ok(['rust_impl', 'parity_proven', 'authority_rust', 'ts_deleted'].includes(entry.state))
  // Under rej-010 freeze the honest residual state is rust_impl with promotion hold.
  if (entry.state === 'rust_impl') {
    assert.equal(entry.promotionHold?.active, true)
    assert.equal(entry.promotionHold?.rejectionRef, 'rej-010')
  }
  assert.ok(entry.parityTest.includes('scripts/check-no-ts-parser.sh') || entry.parityTest.includes('golden.json'))
  assert.equal(entry.branch, 'rust-first/markdown-wasm-authority-rust')
  assert.equal(entry.prUrl, 'https://github.com/SylphxAI/synth/pull/39')
})

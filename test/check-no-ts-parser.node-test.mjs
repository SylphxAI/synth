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
	assert.ok(script.includes('authority_rust'))
	assert.ok(script.includes('crates/wasm-md/src/parser_v2.rs'))
	assert.ok(script.includes('parse-sync.ts'))
	assert.ok(script.includes('init-sync.ts'))
	assert.ok(script.includes('@sylphx/synth-wasm-md'))
	assert.ok(script.includes('check:no-ts-parser'))
	assert.ok(existsSync('test/fixtures/markdown-parity/golden.json'))
	assert.ok(existsSync('packages/synth-wasm-md/test/parity-golden.test.ts'))
	assert.ok(existsSync('packages/synth-md/src/wasm-authority.test.ts'))
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

test('migration ledger records markdown-wasm authority_rust with hard gate evidence', () => {
	const ledger = JSON.parse(readText('docs/specs/synth-migration-ledger.json'))
	const entry = ledger.capabilities.find((cap) => cap.id === 'parser/markdown-wasm')

	assert.equal(entry.state, 'authority_rust')
	assert.ok(entry.parityTest.includes('scripts/check-no-ts-parser.sh'))
	assert.ok(entry.parityTest.includes('test/check-no-ts-parser.node-test.mjs'))
	assert.equal(entry.branch, 'rust-first/markdown-wasm-authority-rust')
	assert.equal(entry.prUrl, 'https://github.com/SylphxAI/synth/pull/39')
})
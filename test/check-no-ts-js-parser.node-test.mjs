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
	assert.ok(script.includes('parity_proven'))
	assert.ok(script.includes('authority_rust'))
	assert.ok(script.includes('isWasmAuthorityEligible(options)'))
	assert.ok(script.includes('parseViaWasmAuthority(code)'))
	assert.ok(script.includes('parseViaWasmAuthorityAsync(code)'))
	assert.ok(script.includes('golden_fixtures_match_ts_baseline'))
	assert.ok(script.includes('javascript-parity/golden.json'))
	assert.ok(script.includes('javascript-parity-golden.node-test.mjs'))
	assert.ok(script.includes('slices.S3'))
	assert.ok(script.includes('slices.S4'))

	assert.ok(existsSync('packages/synth-js/src/wasm-authority.ts'))
	assert.ok(existsSync('packages/synth-js/src/wasm-authority.test.ts'))
	assert.ok(existsSync('packages/synth-wasm-js/src/parse-sync.ts'))
	assert.ok(existsSync('packages/synth-wasm-js/src/init-sync.ts'))
	assert.ok(existsSync('packages/synth-wasm-js/src/tree-bridge.ts'))
	assert.ok(existsSync('test/javascript-parity-golden.node-test.mjs'))
	assert.ok(existsSync('test/fixtures/javascript-parity/golden.json'))
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

test('golden parity prerequisite is recorded before javascript-wasm authority_rust', () => {
	const ledger = readJson('docs/specs/synth-migration-ledger.json')
	const jsTs = ledger.capabilities.find((cap) => cap.id === 'parser/javascript-ts')
	const jsWasm = ledger.capabilities.find((cap) => cap.id === 'parser/javascript-wasm')

	assert.ok(jsTs)
	assert.equal(jsTs.state, 'parity_proven')
	assert.ok(jsTs.parityTest.includes('test/fixtures/javascript-parity/golden.json'))
	assert.ok(jsTs.parityTest.includes('packages/synth-wasm-js/test/parity-golden.test.ts'))
	assert.ok(jsTs.parityTest.includes('cargo test -p synth-wasm-js golden_fixtures_match_ts_baseline'))
	assert.equal(jsTs.branch, 'rust-first/javascript-parity-golden')

	assert.ok(jsWasm)
	assert.equal(jsWasm.state, 'authority_rust')
	assert.ok(jsWasm.notes.includes('S4'))
	assert.ok(jsWasm.parityTest.includes('test/fixtures/javascript-parity/golden.json'))
	assert.ok(jsWasm.parityTest.includes('scripts/check-no-ts-js-parser.sh'))
	assert.ok(jsWasm.parityTest.includes('test/check-no-ts-js-parser.node-test.mjs'))
	assert.equal(jsWasm.branch, 'rust-first/javascript-wasm-authority-rust')
	assert.equal(jsWasm.baseBranch, 'rust-first/javascript-parity-golden')

	const golden = readJson('test/fixtures/javascript-parity/golden.json')
	assert.ok(Object.keys(golden).length >= 10)

	assert.equal(ledger.slices.S3.status, 'shipped')
	assert.equal(ledger.slices.S4.status, 'shipped')
	assert.ok(ledger.slices.S4.evidence.includes('packages/synth-js/src/wasm-authority.ts'))
	assert.ok(ledger.slices.S4.evidence.includes('packages/synth-wasm-js/src/tree-bridge.ts'))
})
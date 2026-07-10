import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'
import test from 'node:test'

function readText(path) {
	return readFileSync(path, 'utf8')
}

function readJson(path) {
	return JSON.parse(readFileSync(path, 'utf8'))
}

test('differential harness exists for markdown-wasm + javascript-wasm (rej-010)', () => {
	const harness = readText('scripts/run-synth-differential.sh')
	assert.ok(harness.includes('synth-differential'))
	assert.ok(harness.includes('synth-oracle.ts'))
	assert.ok(harness.includes('synth_differential_matches_ts_oracle'))
	assert.ok(harness.includes('differential_green'))
	assert.ok(harness.includes('no SKIP-as-pass'))

	assert.ok(existsSync('scripts/differential/synth-oracle.ts'))
	assert.ok(existsSync('scripts/differential/javascript-ts-baseline.ts'))
	assert.ok(existsSync('scripts/differential/markdown-ts-baseline.ts'))
	assert.ok(existsSync('scripts/differential/fixtures/synth-corpus.json'))
	assert.ok(existsSync('verification/synth-2026-07-10T2130-parser-wasm-differential.json'))
	assert.ok(existsSync('docs/specs/synth-parser-wasm-parity-slice.json'))
})

test('parity slice manifest binds parser wasm domains', () => {
	const slice = readJson('docs/specs/synth-parser-wasm-parity-slice.json')
	assert.equal(slice.rejectionRef, 'rej-010')
	assert.ok(slice.slice.includes('parser/markdown-wasm'))
	assert.ok(slice.slice.includes('parser/javascript-wasm'))
	assert.equal(slice.differentialHarness, 'scripts/run-synth-differential.sh')
	assert.ok(slice.capabilities.some((cap) => cap.id === 'parser/markdown-wasm'))
	assert.ok(slice.capabilities.some((cap) => cap.id === 'parser/javascript-wasm'))
})

test('migration ledger records rej-010 differential harness without promotions', () => {
	const ledger = readJson('docs/specs/synth-migration-ledger.json')
	assert.equal(ledger.reauditRef, 'rej-010')
	assert.equal(ledger.promotionHold?.active, true)
	assert.equal(ledger.promotionHold?.rejectionRef, 'rej-010')

	for (const id of ['parser/markdown-wasm', 'parser/javascript-wasm']) {
		const capability = ledger.capabilities.find((cap) => cap.id === id)
		assert.ok(capability, `missing capability ${id}`)
		assert.equal(capability.state, 'authority_rust')
		assert.equal(capability.promotionHold?.active, true)
		assert.equal(capability.promotionHold?.rejectionRef, 'rej-010')
		assert.equal(capability.proof?.status, 'missing')
		assert.ok(capability.differentialTest?.includes('run-synth-differential.sh'))
		assert.ok(
			capability.proof?.verificationRef?.includes('parser-wasm-differential.json'),
		)
	}
})
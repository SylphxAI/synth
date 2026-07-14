/**
 * Golden fixture parity: Rust WASM JS parser vs @sylphx/synth-js TS baseline.
 *
 * TS baselines are captured once in test/fixtures/javascript-parity/golden.json.
 * WASM node kind histograms must match TS baseline (order-independent).
 */

import { beforeAll, describe, expect, it } from 'bun:test'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { initWasm, parseFlat } from '../src/index.js'
import {
	normalizeWasmCountsForTsParity,
	wasmKindCounts,
	type KindCounts,
} from '../src/parity-normalize.js'

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), '../../../test/fixtures/javascript-parity')
const golden = JSON.parse(readFileSync(join(fixtureDir, 'golden.json'), 'utf8')) as Record<
	string,
	{ source: string; kindCounts: KindCounts }
>

describe('javascript-wasm golden parity', () => {
	beforeAll(async () => {
		await initWasm()
	})

	for (const [fixtureId, fixture] of Object.entries(golden)) {
		it(`matches TS baseline kind histogram for ${fixtureId}`, async () => {
			const result = await parseFlat(fixture.source)
			expect(normalizeWasmCountsForTsParity(wasmKindCounts(result.nodes))).toEqual(
				fixture.kindCounts,
			)
		})
	}
})
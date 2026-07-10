/**
 * Rust WASM authority routing tests for default @sylphx/synth-js consumers.
 */

import { afterEach, beforeAll, describe, expect, it } from 'bun:test'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import {
	normalizeWasmCountsForTsParity,
	type KindCounts,
} from '@sylphx/synth-wasm-js'
import { parse, parseAsync } from './parser.js'
import { isWasmAuthorityEligible } from './wasm-authority.js'

function treeKindCounts(tree: { nodes: Array<{ type: string }> }): KindCounts {
	const counts: KindCounts = {}
	for (const node of tree.nodes) {
		if (node.type === 'root') {
			continue
		}
		counts[node.type] = (counts[node.type] ?? 0) + 1
	}
	return counts
}

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), '../../../test/fixtures/javascript-parity')
const golden = JSON.parse(readFileSync(join(fixtureDir, 'golden.json'), 'utf8')) as Record<
	string,
	{ source: string; kindCounts: KindCounts }
>

const originalAuthority = process.env.SYNTH_JS_AUTHORITY

afterEach(() => {
	if (originalAuthority === undefined) {
		delete process.env.SYNTH_JS_AUTHORITY
	} else {
		process.env.SYNTH_JS_AUTHORITY = originalAuthority
	}
})

describe('wasm authority routing', () => {
	beforeAll(() => {
		process.env.SYNTH_JS_AUTHORITY = 'wasm'
	})

	describe('isWasmAuthorityEligible', () => {
		it('routes default options to wasm', () => {
			expect(isWasmAuthorityEligible()).toBe(true)
			expect(isWasmAuthorityEligible({})).toBe(true)
		})

		it('opts out when plugins are present', () => {
			expect(
				isWasmAuthorityEligible({ plugins: [{ name: 'x', transform: () => {} }] }),
			).toBe(false)
		})

		it('opts out when useTsParser is explicitly set', () => {
			expect(isWasmAuthorityEligible({ useTsParser: true })).toBe(false)
		})

		it('opts out when SYNTH_JS_AUTHORITY=ts', () => {
			process.env.SYNTH_JS_AUTHORITY = 'ts'
			expect(isWasmAuthorityEligible()).toBe(false)
		})
	})

	describe('default parse()', () => {
		for (const [fixtureId, fixture] of Object.entries(golden)) {
			it(`uses Rust WASM authority for ${fixtureId}`, () => {
				const tree = parse(fixture.source)
				expect(normalizeWasmCountsForTsParity(treeKindCounts(tree))).toEqual(
					fixture.kindCounts,
				)
			})
		}

		it('parseAsync uses Rust WASM authority for default options', async () => {
			const tree = await parseAsync('const x = 1;')
			expect(treeKindCounts(tree)).toMatchObject({
				Program: 1,
				VariableDeclaration: 1,
				VariableDeclarator: 1,
			})
		})
	})
})
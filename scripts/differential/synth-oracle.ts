#!/usr/bin/env bun
/**
 * TS live-parser oracle for synth markdown-wasm + javascript-wasm differential parity (rej-010).
 *
 * Runs @sylphx/synth-md and @sylphx/synth-js TS baselines against golden fixture corpora.
 * Emits canonical JSON consumed by native Rust differential tests in wasm-md / wasm-js.
 *
 * Fail-closed: requires bun — no SKIP-as-pass.
 */
import { createHash } from 'node:crypto'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { javascriptTsBaseline } from './javascript-ts-baseline.ts'
import { markdownTsBaseline } from './markdown-ts-baseline.ts'

const __dirname = dirname(fileURLToPath(import.meta.url))
const REPO_ROOT = join(__dirname, '../..')
const CORPUS_PATH = join(__dirname, 'fixtures/synth-corpus.json')
const JS_GOLDEN = join(REPO_ROOT, 'test/fixtures/javascript-parity/golden.json')
const MD_GOLDEN = join(REPO_ROOT, 'test/fixtures/markdown-parity/golden.json')

type Json = null | boolean | number | string | Json[] | { [key: string]: Json }

export interface DifferentialCase {
	readonly id: string
	readonly slice: 'parser/markdown-wasm' | 'parser/javascript-wasm'
	readonly source: string
	readonly output: Json
}

export interface DifferentialCorpus {
	readonly corpusVersion: number
	readonly fixtureCorpusHash: string
	readonly javascriptGoldenHash: string
	readonly markdownGoldenHash: string
	readonly cases: readonly DifferentialCase[]
}

interface JavascriptGoldenFixture {
	source: string
	kindCounts: Record<string, number>
}

interface MarkdownGoldenFixture {
	source: string
	blocks: Json[]
}

function sha256Hex(content: string): string {
	return createHash('sha256').update(content).digest('hex')
}

function stableJson(value: unknown): string {
	return JSON.stringify(value)
}

function assertDeepEqual(actual: unknown, expected: unknown, label: string): void {
	if (stableJson(actual) !== stableJson(expected)) {
		throw new Error(
			`${label} drifted from frozen golden baseline\nexpected: ${stableJson(expected)}\nactual: ${stableJson(actual)}`,
		)
	}
}

function buildCorpus(): DifferentialCorpus {
	const corpusRaw = readFileSync(CORPUS_PATH, 'utf8')
	const corpus = JSON.parse(corpusRaw) as { corpusVersion: number }
	if (corpus.corpusVersion !== 1) {
		throw new Error(`unsupported corpusVersion: ${corpus.corpusVersion}`)
	}

	const jsGoldenRaw = readFileSync(JS_GOLDEN, 'utf8')
	const mdGoldenRaw = readFileSync(MD_GOLDEN, 'utf8')
	const jsGolden = JSON.parse(jsGoldenRaw) as Record<string, JavascriptGoldenFixture>
	const mdGolden = JSON.parse(mdGoldenRaw) as Record<string, MarkdownGoldenFixture>

	const fixtureCorpusHash = sha256Hex(corpusRaw + jsGoldenRaw + mdGoldenRaw)
	const javascriptGoldenHash = sha256Hex(jsGoldenRaw)
	const markdownGoldenHash = sha256Hex(mdGoldenRaw)
	const cases: DifferentialCase[] = []

	for (const [fixtureId, fixture] of Object.entries(jsGolden)) {
		const live = javascriptTsBaseline(fixture.source)
		assertDeepEqual(live, fixture.kindCounts, `javascript fixture ${fixtureId}`)
		cases.push({
			id: `javascript-wasm:${fixtureId}`,
			slice: 'parser/javascript-wasm',
			source: fixture.source,
			output: live,
		})
	}

	for (const [fixtureId, fixture] of Object.entries(mdGolden)) {
		const live = markdownTsBaseline(fixture.source)
		assertDeepEqual(live, fixture.blocks, `markdown fixture ${fixtureId}`)
		cases.push({
			id: `markdown-wasm:${fixtureId}`,
			slice: 'parser/markdown-wasm',
			source: fixture.source,
			output: live,
		})
	}

	if (cases.length < 10) {
		throw new Error(`oracle must emit at least 10 cases (got ${cases.length})`)
	}

	return {
		corpusVersion: corpus.corpusVersion,
		fixtureCorpusHash,
		javascriptGoldenHash,
		markdownGoldenHash,
		cases,
	}
}

const corpus = buildCorpus()
process.stdout.write(`${JSON.stringify(corpus)}\n`)
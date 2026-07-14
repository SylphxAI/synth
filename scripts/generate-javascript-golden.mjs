#!/usr/bin/env node
/**
 * Capture @sylphx/synth-js TS parser baselines for javascript-wasm golden parity.
 * Run: bun scripts/generate-javascript-golden.mjs
 */
import { writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { parse } from '../packages/synth-js/src/parser.ts'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const outPath = join(root, 'test/fixtures/javascript-parity/golden.json')

/** Map ESTree export variants to WASM NodeKind name. */
const TS_TO_WASM_KIND = {
	ExportNamedDeclaration: 'ExportDeclaration',
	ExportDefaultDeclaration: 'ExportDeclaration',
	ExportAllDeclaration: 'ExportDeclaration',
}

const FIXTURES = {
	'const-binding': 'const x = 1;',
	'function-decl': 'function add(a, b) { return a + b; }',
	'arrow-fn': 'const fn = x => x * 2;',
	'class-decl': 'class Foo { constructor() {} }',
	'import-export': "import { foo } from 'bar'; export default x;",
	'if-stmt': 'if (x) { return 1; } else { return 0; }',
	'for-of': 'for (const item of items) { console.log(item); }',
	'try-catch': 'try { risky(); } catch (e) { handle(e); }',
	'async-await': 'async function load() { const r = await fetch(url); return r.json(); }',
	'template-literal': 'const s = `hello ${name}`;',
}

function normalizeTsKind(type) {
	return TS_TO_WASM_KIND[type] ?? type
}

function tsKindCounts(tree) {
	const counts = {}
	for (const node of tree.nodes) {
		if (node.type === 'root') continue
		const kind = normalizeTsKind(node.type)
		counts[kind] = (counts[kind] ?? 0) + 1
	}
	return counts
}

const golden = {}
for (const [id, source] of Object.entries(FIXTURES)) {
	const tree = parse(source, { typescript: false, plugins: [] })
	golden[id] = { source, kindCounts: tsKindCounts(tree) }
}

writeFileSync(outPath, `${JSON.stringify(golden, null, 2)}\n`)
console.log(`Wrote ${outPath} (${Object.keys(golden).length} fixtures)`)
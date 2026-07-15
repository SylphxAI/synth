/**
 * Live @sylphx/synth-js TS parser baseline for javascript-wasm differential parity (rej-010).
 * Mirrors scripts/generate-javascript-golden.mjs kind normalization.
 */
import { parse } from '../../packages/synth-js/src/parser.ts'

export type KindCounts = Record<string, number>

const TS_TO_WASM_KIND: Record<string, string> = {
  ExportNamedDeclaration: 'ExportDeclaration',
  ExportDefaultDeclaration: 'ExportDeclaration',
  ExportAllDeclaration: 'ExportDeclaration',
}

function normalizeTsKind(type: string): string {
  return TS_TO_WASM_KIND[type] ?? type
}

export function tsKindCounts(tree: { nodes: Array<{ type: string }> }): KindCounts {
  const counts: KindCounts = {}
  for (const node of tree.nodes) {
    if (node.type === 'root') continue
    const kind = normalizeTsKind(node.type)
    counts[kind] = (counts[kind] ?? 0) + 1
  }
  return counts
}

export function javascriptTsBaseline(source: string): KindCounts {
  const tree = parse(source, {
    typescript: false,
    plugins: [],
    useTsParser: true,
    allowReturnOutsideFunction: true,
  })
  return tsKindCounts(tree)
}

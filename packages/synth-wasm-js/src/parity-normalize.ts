/**
 * Normalize WASM parse output for golden parity vs TS baseline.
 */
import { nodeKindName } from './node-kind-names.js'
import type { ASTNode } from './types.js'

export type KindCounts = Record<string, number>

export function wasmKindCounts(nodes: ASTNode[]): KindCounts {
	const counts: KindCounts = {}
	for (const node of nodes) {
		const kind = nodeKindName(node.kind)
		counts[kind] = (counts[kind] ?? 0) + 1
	}
	return counts
}

/**
 * WASM emits Identifier nodes for declared function/class names; TS stores names in node.data.
 */
export function normalizeWasmCountsForTsParity(counts: KindCounts): KindCounts {
	const normalized = { ...counts }
	const nameIdentifiers =
		(normalized.FunctionDeclaration ?? 0) + (normalized.ClassDeclaration ?? 0)

	if (nameIdentifiers > 0 && normalized.Identifier) {
		normalized.Identifier = Math.max(0, normalized.Identifier - nameIdentifiers)
		if (normalized.Identifier === 0) {
			delete normalized.Identifier
		}
	}

	return normalized
}
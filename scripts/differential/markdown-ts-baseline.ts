/**
 * Live @sylphx/synth-md TS parser baseline for markdown-wasm differential parity (rej-010).
 */
import { parse } from '../../packages/synth-md/src/parser.ts'

export interface BlockSignature {
	type: string
	depth?: number
	lang?: string
	checked?: boolean
	ordered?: boolean
}

export function normalizeBlockSignature(tree: {
	nodes: Array<{ type: string; data?: Record<string, unknown> }>
}): BlockSignature[] {
	return tree.nodes
		.filter((node) => node && !['root', 'text', 'inline'].includes(node.type))
		.map((node) => {
			const signature: BlockSignature = { type: node.type }
			if (node.data?.depth !== undefined) signature.depth = node.data.depth as number
			if (node.data?.lang !== undefined) signature.lang = node.data.lang as string
			if (node.data?.checked !== undefined) signature.checked = node.data.checked as boolean
			if (node.data?.ordered === true) signature.ordered = true
			return signature
		})
		.sort((left, right) => JSON.stringify(left).localeCompare(JSON.stringify(right)))
}

export function markdownTsBaseline(source: string): BlockSignature[] {
	const tree = parse(source, { useTsParser: true })
	return normalizeBlockSignature(tree)
}
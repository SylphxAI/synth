/**
 * Bridge Rust WASM flat AST nodes into @sylphx/synth Tree for authority routing.
 */
import { addNode, createTree, type Tree } from '@sylphx/synth'
import { nodeKindName } from './node-kind-names.js'
import type { ASTNode } from './types.js'

function offsetSpan(start: number, end: number) {
	return {
		start: { offset: start, line: 0, column: 0 },
		end: { offset: end, line: 0, column: 0 },
	}
}

/**
 * Decode compact 16-byte WASM binary nodes into ASTNode records.
 */
export function decodeBinaryAst(binary: Uint8Array): ASTNode[] {
	const view = new DataView(binary.buffer, binary.byteOffset, binary.byteLength)
	const nodeCount = view.getUint32(0, true)
	const nodes: ASTNode[] = []

	for (let i = 0; i < nodeCount; i++) {
		const offset = 4 + i * 16
		nodes.push({
			kind: view.getUint8(offset),
			flags: view.getUint8(offset + 1),
			start: view.getUint32(offset + 4, true),
			end: view.getUint32(offset + 8, true),
			extra: view.getUint32(offset + 12, true),
		})
	}

	return nodes
}

/**
 * Convert WASM flat nodes into a Synth Tree (Program root with DFS child nodes).
 *
 * Golden parity validates kind histograms; hierarchy is reconstructed as a flat
 * Program subtree until the Rust parseToJson tree bridge ships in WASM rebuild.
 */
export function wasmNodesToTree(source: string, nodes: ASTNode[]): Tree {
	const tree = createTree('javascript', source)
	const programNode = nodes.find((n) => nodeKindName(n.kind) === 'Program')
	const programId = addNode(tree, {
		type: 'Program',
		parent: tree.root,
		children: [],
		span: programNode ? offsetSpan(programNode.start, programNode.end) : undefined,
	})
	tree.nodes[tree.root]?.children.push(programId)

	for (const node of nodes) {
		if (nodeKindName(node.kind) === 'Program') {
			continue
		}

		const id = addNode(tree, {
			type: nodeKindName(node.kind),
			parent: programId,
			children: [],
			span: offsetSpan(node.start, node.end),
			data: node.extra > 0 ? { extra: node.extra } : undefined,
		})
		tree.nodes[programId]?.children.push(id)
	}

	return tree
}

/**
 * Parse binary WASM output directly into a Synth Tree.
 */
export function binaryToTree(source: string, binary: Uint8Array): Tree {
	return wasmNodesToTree(source, decodeBinaryAst(binary))
}
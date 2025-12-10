// Type declarations for WASM modules
declare module '*.wasm' {
	const content: string
	export default content
}

declare module 'tree-sitter-wasms/out/*.wasm' {
	const content: string
	export default content
}

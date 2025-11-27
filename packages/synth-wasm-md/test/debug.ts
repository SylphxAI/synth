// Debug script
import { initWasm, parse } from '../src/index.js'

await initWasm()

const tree = await parse('```rust\nfn main() {}\n```')
console.log(JSON.stringify(tree, null, 2))

/**
 * Golden fixture parity: Rust WASM markdown parser vs @sylphx/synth-md TS baseline.
 *
 * TS baselines are captured once in test/fixtures/markdown-parity/golden.json.
 * WASM output must match normalized block structure (types + semantic metadata).
 */

import { beforeAll, describe, expect, it } from 'bun:test'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { initWasm, parse } from '../src/index.js'

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), '../../../test/fixtures/markdown-parity')
const golden = JSON.parse(readFileSync(join(fixtureDir, 'golden.json'), 'utf8')) as Record<
  string,
  { source: string; blocks: BlockSignature[] }
>

export interface BlockSignature {
  type: string
  depth?: number
  lang?: string
  checked?: boolean
  ordered?: boolean
}

/** Normalize tree to comparable block signatures (TS baseline shape). */
export function normalizeBlockSignature(tree: {
  nodes: Array<{ type: string; data?: Record<string, unknown> }>
}): BlockSignature[] {
  return tree.nodes
    .filter((n) => n && !['root', 'text', 'inline'].includes(n.type))
    .map((n) => {
      const sig: BlockSignature = { type: n.type }
      if (n.data?.depth !== undefined) sig.depth = n.data.depth as number
      if (n.data?.lang !== undefined) sig.lang = n.data.lang as string
      if (n.data?.checked !== undefined) sig.checked = n.data.checked as boolean
      if (n.data?.ordered === true) sig.ordered = true
      return sig
    })
    .sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)))
}

describe('markdown-wasm golden parity', () => {
  beforeAll(async () => {
    await initWasm()
  })

  for (const [fixtureId, fixture] of Object.entries(golden)) {
    it(`matches TS baseline for ${fixtureId}`, async () => {
      const tree = await parse(fixture.source)
      expect(normalizeBlockSignature(tree)).toEqual(fixture.blocks)
    })
  }
})

/**
 * Rust WASM authority routing tests for default @sylphx/synth-md consumers.
 */

import { afterEach, beforeAll, describe, expect, it } from 'bun:test'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { parse, parseAsync } from './parser.js'
import { isWasmAuthorityEligible } from './wasm-authority.js'

interface BlockSignature {
  type: string
  depth?: number
  lang?: string
  checked?: boolean
  ordered?: boolean
}

function normalizeBlockSignature(tree: {
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

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), '../../../test/fixtures/markdown-parity')
const golden = JSON.parse(readFileSync(join(fixtureDir, 'golden.json'), 'utf8')) as Record<
  string,
  { source: string; blocks: unknown[] }
>

const originalAuthority = process.env.SYNTH_MD_AUTHORITY

afterEach(() => {
  if (originalAuthority === undefined) {
    delete process.env.SYNTH_MD_AUTHORITY
  } else {
    process.env.SYNTH_MD_AUTHORITY = originalAuthority
  }
})

describe('wasm authority routing', () => {
  beforeAll(() => {
    process.env.SYNTH_MD_AUTHORITY = 'wasm'
  })

  describe('isWasmAuthorityEligible', () => {
    it('routes default options to wasm', () => {
      expect(isWasmAuthorityEligible()).toBe(true)
      expect(isWasmAuthorityEligible({})).toBe(true)
    })

    it('opts out when plugins are present', () => {
      expect(isWasmAuthorityEligible({ plugins: [{ name: 'x', transform: () => {} }] })).toBe(false)
    })

    it('opts out when batch tokenizer is enabled', () => {
      expect(isWasmAuthorityEligible({ useBatchTokenizer: true })).toBe(false)
    })

    it('opts out when SYNTH_MD_AUTHORITY=ts', () => {
      process.env.SYNTH_MD_AUTHORITY = 'ts'
      expect(isWasmAuthorityEligible()).toBe(false)
    })
  })

  describe('default parse()', () => {
    for (const [fixtureId, fixture] of Object.entries(golden)) {
      it(`uses Rust WASM authority for ${fixtureId}`, () => {
        const tree = parse(fixture.source)
        expect(normalizeBlockSignature(tree)).toEqual(fixture.blocks)
      })
    }

    it('parseAsync uses Rust WASM authority for default options', async () => {
      const tree = await parseAsync('# Async Authority')
      expect(normalizeBlockSignature(tree)).toEqual([{ type: 'heading', depth: 1 }])
    })
  })
})

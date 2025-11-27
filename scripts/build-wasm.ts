#!/usr/bin/env bun

/**
 * Build all WASM packages
 *
 * Usage:
 *   bun scripts/build-wasm.ts [crate-name] [--release|--debug] [--target web|nodejs|bundler]
 *
 * Examples:
 *   bun scripts/build-wasm.ts                    # Build all crates
 *   bun scripts/build-wasm.ts wasm-md            # Build specific crate
 *   bun scripts/build-wasm.ts --debug            # Build in debug mode
 *   bun scripts/build-wasm.ts --target nodejs    # Build for Node.js
 */

import { existsSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { $ } from 'bun'

const ROOT = join(import.meta.dir, '..')
const CRATES_DIR = join(ROOT, 'crates')

// Parse arguments
const args = process.argv.slice(2)
const isDebug = args.includes('--debug')
const targetIndex = args.indexOf('--target')
const target = targetIndex !== -1 ? args[targetIndex + 1] : 'web'
const crateName = args.find((a) => !a.startsWith('--') && a !== target)

// Get all crates that produce WASM (have cdylib in Cargo.toml)
function getWasmCrates(): string[] {
  const crates: string[] = []
  const entries = readdirSync(CRATES_DIR, { withFileTypes: true })

  for (const entry of entries) {
    if (!entry.isDirectory()) continue

    const cargoPath = join(CRATES_DIR, entry.name, 'Cargo.toml')
    if (!existsSync(cargoPath)) continue

    const cargo = Bun.file(cargoPath).text()
    if (cargo.then ? false : (cargo as string).includes('cdylib')) {
      crates.push(entry.name)
    }
  }

  return crates
}

async function buildCrate(name: string): Promise<boolean> {
  const cratePath = join(CRATES_DIR, name)

  if (!existsSync(cratePath)) {
    console.error(`❌ Crate not found: ${name}`)
    return false
  }

  console.log(`\n📦 Building ${name}...`)

  const mode = isDebug ? '--dev' : '--release'

  try {
    await $`cd ${cratePath} && wasm-pack build --target ${target} ${mode}`.quiet()
    console.log(`✅ ${name} built successfully`)
    return true
  } catch (error) {
    console.error(`❌ Failed to build ${name}`)
    console.error(error)
    return false
  }
}

async function main() {
  console.log('🔨 Building WASM packages...')
  console.log(`   Target: ${target}`)
  console.log(`   Mode: ${isDebug ? 'debug' : 'release'}`)

  let crates: string[]

  if (crateName) {
    crates = [crateName]
  } else {
    // Get all WASM crates
    const allCrates = getWasmCrates()

    // Filter out 'core' as it's a dependency, not a standalone package
    crates = allCrates.filter((c) => c !== 'core')

    // Manually specify order if needed
    if (crates.length === 0) {
      crates = ['wasm-md', 'wasm-js']
    }
  }

  console.log(`   Crates: ${crates.join(', ')}`)

  let success = 0
  let failed = 0

  for (const crate of crates) {
    const ok = await buildCrate(crate)
    if (ok) success++
    else failed++
  }

  console.log(`\n📊 Results: ${success} succeeded, ${failed} failed`)

  if (failed > 0) {
    process.exit(1)
  }
}

main()

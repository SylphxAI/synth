import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

const readJson = (path) => JSON.parse(readFileSync(path, 'utf8'))
const readText = (path) => readFileSync(path, 'utf8')

test('project manifest remains valid project metadata without GroundAtlas product dogfood', () => {
	const manifest = readJson('project.manifest.json')

	assert.equal(manifest.schemaVersion, 1)
	assert.equal(manifest.project.id, 'synth')
	assert.equal(manifest.project.repository, 'https://github.com/SylphxAI/synth')
	assert.equal(manifest.project.visibility, 'open-source')
	assert.equal(manifest.project.lifecycle, 'production')
	assert.equal(manifest.adoption.status, 'adopted')
	assert.equal(manifest.truth.agentAdapter, 'AGENTS.md')
	assert.ok(manifest.truth.source.includes('crates/'))
	assert.ok(
		manifest.surfaces.some(
			(surface) =>
				surface.path === '.doctrine/project.json' &&
				surface.description.includes('not the vendor-neutral GroundAtlas default'),
		),
	)
	const commandNames = (manifest.commands || []).map((c) => c.name)
	assert.ok(!commandNames.includes('groundatlas:fleet'))
	assert.ok(
		String(manifest.adoption?.notes || '').includes('ADR-0014') ||
			String(manifest.adoption?.notes || '').toLowerCase().includes('retired'),
	)
})

test('Doctrine adapter remains Sylphx-specific and package proof stays package-owned', () => {
	const doctrine = readJson('.doctrine/project.json')

	assert.equal(doctrine.project.repo, 'SylphxAI/synth')
	assert.equal(doctrine.adoption.status, 'adopted')
	assert.ok(
		doctrine.boundaries.publicSurfaces.some(
			(surface) => surface.type === 'manifest' && surface.location === 'project.manifest.json',
		),
	)
	assert.ok(!String(doctrine.delivery?.productionProof || '').toLowerCase().includes('groundatlas'))
	assert.ok(!String(doctrine.delivery?.ciModel || '').toLowerCase().includes('groundatlas'))
	assert.ok(doctrine.delivery.productionProof.includes('bun run validate'))
	assert.ok(doctrine.delivery.packageRelease.publishProof.includes('npm registry readback'))
})

test('CI runs package validation and does not pin GroundAtlas package/action', () => {
	const workflow = readText('.github/workflows/ci.yml')

	assert.ok(workflow.includes('bun install --frozen-lockfile'))
	assert.ok(workflow.includes('bun run validate'))
	assert.ok(!workflow.includes('uses: SylphxAI/groundatlas@'))
	assert.ok(!workflow.includes('package-spec: groundatlas@'))
	assert.ok(workflow.includes('project.manifest.json') || workflow.includes('project-control'))
	assert.ok(workflow.includes('.doctrine/project.json') || workflow.includes('project-control'))
	assert.ok(workflow.includes('wasm32-unknown-unknown'))
})

test('root validation follows the current CI baseline', () => {
	const manifest = readJson('package.json')

	assert.equal(
		manifest.scripts.validate,
		'bun run lint && bunx turbo build --concurrency=1 && bun run typecheck && bun test',
	)
	assert.equal(manifest.scripts.build, 'turbo build')
	assert.equal(manifest.scripts.typecheck, 'turbo typecheck')
})

test('release delegates to the shared trusted publishing workflow', () => {
	const workflow = readText('.github/workflows/release.yml')

	assert.ok(workflow.includes('id-token: write'))
	assert.ok(workflow.includes('secrets: inherit'))
	assert.ok(workflow.includes('SylphxAI/.github/.github/workflows/release.yml@main'))
	assert.ok(workflow.includes('sh.rustup.rs'))
	assert.ok(workflow.includes('wasm-pack/installer/init.sh'))
	assert.ok(workflow.includes('sh -s -- -f'))
	assert.ok(workflow.includes('wasm32-unknown-unknown'))
})

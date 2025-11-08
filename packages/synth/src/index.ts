/**
 * @sylphx/synth - The world's fastest AST processor
 *
 * Main entry point for the Synth library.
 * Exports core types, utilities, optimizations, and plugin system.
 */

// Core types and interfaces
export * from './types/index.js'

// Tree operations
export * from './traverse.js'
export * from './zipper.js'
export * from './query-index.js'
export * from './incremental.js'

// Optimizations
export * from './batch-processor.js'
export * from './node-pool.js'

// Plugin system
export * from './plugin.js'
export * from './manager.js'

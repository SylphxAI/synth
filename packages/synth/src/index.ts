/**
 * @module @sylphx/synth - The world's fastest AST processor
 *
 * Main entry point for the Synth library.
 * Exports core types, utilities, optimizations, and plugin system.
 *
 * @since 0.1.0
 * @packageDocumentation
 */

// Optimizations
export * from './batch-processor.js'

// Error classes
export * from './errors.js'
export * from './incremental.js'
export * from './incremental-parser-manager.js'
export * from './incremental-tokenizer.js'
export * from './manager.js'
export * from './node-pool.js'
// Plugin system
export * from './plugin.js'
export * from './query-index.js'
// Tree operations
export * from './traverse.js'
// Core types and interfaces
export * from './types/index.js'
export * from './zipper.js'

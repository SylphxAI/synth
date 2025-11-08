/**
 * Generic Plugin System
 *
 * Language-agnostic plugin architecture for AST manipulation
 */

import type { Tree, BaseNode } from './types/index.js'

/**
 * Plugin metadata
 */
export interface PluginMetadata {
  /** Plugin name */
  name: string

  /** Plugin version */
  version?: string

  /** Plugin description */
  description?: string

  /** Plugin author */
  author?: string
}

/**
 * Transform plugin - receives and returns the entire tree
 */
export interface TransformPlugin<T extends Tree = Tree> {
  /** Plugin metadata */
  meta: PluginMetadata

  /** Transform function */
  transform: (tree: T) => T | Promise<T>
}

/**
 * Node visitor function type (simpler than VisitorFn from types)
 */
export type NodeVisitorFn = (node: BaseNode) => BaseNode | void

/**
 * Visitor plugin - processes individual nodes
 */
export interface VisitorPlugin<V extends Record<string, NodeVisitorFn> = Record<string, NodeVisitorFn>> {
  /** Plugin metadata */
  meta: PluginMetadata

  /** Visitor functions for specific node types */
  visitors: V

  /** Optional setup hook */
  setup?: (tree: Tree) => void | Promise<void>

  /** Optional teardown hook */
  teardown?: (tree: Tree) => void | Promise<void>
}

/**
 * Union of all plugin types
 */
export type Plugin = TransformPlugin | VisitorPlugin

/**
 * Type guard for transform plugins
 */
export function isTransformPlugin(plugin: Plugin): plugin is TransformPlugin {
  return 'transform' in plugin && typeof plugin.transform === 'function'
}

/**
 * Type guard for visitor plugins
 */
export function isVisitorPlugin(plugin: Plugin): plugin is VisitorPlugin {
  return 'visitors' in plugin && typeof plugin.visitors === 'object'
}

/**
 * Create a transform plugin
 */
export function createTransformPlugin<T extends Tree = Tree>(
  meta: PluginMetadata,
  transform: (tree: T) => T | Promise<T>
): TransformPlugin<T> {
  return { meta, transform }
}

/**
 * Create a visitor plugin
 */
export function createVisitorPlugin<V extends Record<string, NodeVisitorFn> = Record<string, NodeVisitorFn>>(
  meta: PluginMetadata,
  visitors: V,
  hooks?: {
    setup?: (tree: Tree) => void | Promise<void>
    teardown?: (tree: Tree) => void | Promise<void>
  }
): VisitorPlugin<V> {
  return {
    meta,
    visitors,
    setup: hooks?.setup,
    teardown: hooks?.teardown,
  }
}

/**
 * JavaScript Minifier
 *
 * Main API for minifying JavaScript code using Synth AST
 */

import type { Tree } from '@sylphx/synth'
import { parse } from '@sylphx/synth-js'
import { Compressor } from './compressor.js'
import type { MinifyOptions } from './options.js'

export class Minifier {
  private options: MinifyOptions

  constructor(options: MinifyOptions = {}) {
    this.options = options
  }

  /**
   * Minify JavaScript code string
   */
  minify(code: string, options?: MinifyOptions): string {
    const opts = { ...this.options, ...options }
    const tree = parse(code)
    return this.minifyTree(tree, opts)
  }

  /**
   * Minify a Synth AST tree directly
   */
  minifyTree(tree: Tree, options?: MinifyOptions): string {
    const opts = { ...this.options, ...options }
    const compressor = new Compressor(opts)
    return compressor.compress(tree)
  }

  /**
   * Calculate compression ratio
   */
  compressionRatio(original: string, minified?: string): number {
    const min = minified || this.minify(original)
    return 1 - min.length / original.length
  }
}

/**
 * Minify JavaScript code (standalone function)
 */
export function minify(code: string, options?: MinifyOptions): string {
  const minifier = new Minifier(options)
  return minifier.minify(code)
}

/**
 * Calculate size savings
 */
export function savings(
  original: string,
  minified: string
): {
  originalSize: number
  minifiedSize: number
  savedBytes: number
  savedPercent: number
} {
  const originalSize = original.length
  const minifiedSize = minified.length
  const savedBytes = originalSize - minifiedSize
  const savedPercent = (savedBytes / originalSize) * 100

  return {
    originalSize,
    minifiedSize,
    savedBytes,
    savedPercent,
  }
}

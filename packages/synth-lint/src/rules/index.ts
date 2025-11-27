/**
 * Built-in lint rules
 */

export { noEmptyBlocks } from './no-empty-blocks.js'
export { noConsole } from './no-console.js'
export { maxDepth } from './max-depth.js'

import { maxDepth } from './max-depth.js'
import { noConsole } from './no-console.js'
import { noEmptyBlocks } from './no-empty-blocks.js'

import type { Rule } from '../types.js'

/**
 * All built-in rules
 */
export const builtinRules: readonly Rule[] = [noEmptyBlocks, noConsole, maxDepth]

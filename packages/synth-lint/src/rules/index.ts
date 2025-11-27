/**
 * Built-in lint rules
 */

export { maxDepth } from './max-depth.js'
export { noConsole } from './no-console.js'
export { noEmptyBlocks } from './no-empty-blocks.js'

import type { Rule } from '../types.js'
import { maxDepth } from './max-depth.js'
import { noConsole } from './no-console.js'
import { noEmptyBlocks } from './no-empty-blocks.js'

/**
 * All built-in rules
 */
export const builtinRules: readonly Rule[] = [noEmptyBlocks, noConsole, maxDepth]

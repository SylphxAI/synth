/**
 * @module @sylphx/synth-vue
 *
 * Vue SFC parser using Synth's universal AST
 * Conversion layer over @vue/compiler-sfc
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { VueParseOptions } from './parser.js'
export { createParser, parse, parseAsync, VueParser } from './parser.js'

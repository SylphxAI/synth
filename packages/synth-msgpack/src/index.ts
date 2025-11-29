/**
 * @module @sylphx/synth-msgpack
 *
 * MessagePack parser using Synth's universal AST
 *
 * @since 0.1.0
 * @packageDocumentation
 */

export type { Node, Plugin, Tree } from '@sylphx/synth'
export {
  createParser,
  type MsgPackParseOptions,
  MsgPackParser,
  parse,
  parseAsync,
} from './parser.js'

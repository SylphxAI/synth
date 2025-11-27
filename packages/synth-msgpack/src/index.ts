/**
 * @sylphx/synth-msgpack
 *
 * MessagePack parser using Synth's universal AST
 */

export type { Node, Plugin, Tree } from '@sylphx/synth'
export {
  createParser,
  type MsgPackParseOptions,
  MsgPackParser,
  parse,
  parseAsync,
} from './parser.js'

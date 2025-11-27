/**
 * @sylphx/synth-protobuf
 *
 * Protocol Buffers parser using Synth's universal AST
 */

export type { Node, Plugin, Tree } from '@sylphx/synth'
export {
  createParser,
  type ProtobufParseOptions,
  ProtobufParser,
  parse,
  parseAsync,
} from './parser.js'

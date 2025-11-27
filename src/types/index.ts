/**
 * Core type exports
 */

export type {
  BaseNode,
  Node,
  NodeId,
  ParentNode,
  Position,
  RootNode,
  Span,
  TextNode,
} from './node.js'

export {
  isParentNode,
  isTextNode,
} from './node.js'

export type {
  Tree,
  TreeMetadata,
} from './tree.js'

export {
  addNode,
  createTree,
  getChildren,
  getNode,
  getParent,
  getRoot,
  internString,
  removeNode,
  updateNode,
} from './tree.js'

export type {
  TraversalOptions,
  Visitor,
  VisitorContext,
  VisitorFn,
} from './visitor.js'

export { TraversalOrder } from './visitor.js'

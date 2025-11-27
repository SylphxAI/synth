/**
 * Core engine exports
 */

export type { BatchProcessingOptions, BatchVisitor } from './batch-processor.js'
export {
  batchFilter,
  batchMap,
  batchProcess,
  batchSelect,
  batchTransform,
  batchTraverse,
} from './batch-processor.js'
export type {
  AffectedRange,
  Edit,
  IncrementalStats,
  SimpleEdit,
} from './incremental.js'
export {
  applyEdit,
  createIncrementalParser,
  IncrementalParser,
} from './incremental.js'
export type { PoolConfig, PoolStats } from './node-pool.js'
export {
  createNodePool,
  globalNodePool,
  NodePoolManager,
} from './node-pool.js'
export type { IndexStats, QueryObject, QueryPredicate, QuerySelector } from './query-index.js'
export {
  ASTIndex,
  createIndex,
} from './query-index.js'
export {
  find,
  select,
  selectByType,
  traverse,
} from './traverse.js'
export type { Zipper } from './zipper.js'
export {
  appendChild,
  createZipper,
  createZipperAt,
  down,
  edit,
  getFocus,
  insertLeft,
  insertRight,
  left,
  remove,
  replace,
  right,
  root,
  up,
} from './zipper.js'

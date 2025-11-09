/**
 * Type definitions for PHP parser
 */

import type { Plugin } from '@sylphx/synth'

export interface PhpParseOptions {
  /** Build query index for AST */
  buildIndex?: boolean

  /** Plugins to apply during parsing */
  plugins?: Plugin[]

  /** PHP version compatibility (default: 8) */
  phpVersion?: 7 | 8
}

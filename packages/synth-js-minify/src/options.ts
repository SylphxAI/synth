/**
 * Minification Options
 */

export interface MinifyOptions {
  /** Compress code (remove unnecessary whitespace, parentheses) */
  compress?: boolean

  /** Mangle variable names (shorten identifiers) */
  mangle?: boolean

  /** Remove comments */
  removeComments?: boolean

  /** Compact object/array literals on one line */
  compact?: boolean

  /** Preserve specific identifier names from mangling */
  reserved?: string[]
}

export const DEFAULT_OPTIONS: Required<MinifyOptions> = {
  compress: true,
  mangle: false, // Disabled by default for safety
  removeComments: true,
  compact: true,
  reserved: [],
}

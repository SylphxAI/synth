/**
 * Documentation types and interfaces
 */

/**
 * Documentation for a symbol (function, class, variable, etc.)
 */
export interface SymbolDoc {
  /** Symbol name */
  name: string

  /** Symbol kind */
  kind: 'function' | 'class' | 'interface' | 'variable' | 'constant' | 'type' | 'method' | 'property'

  /** Description */
  description?: string

  /** Parameters (for functions/methods) */
  params?: ParamDoc[]

  /** Return value (for functions/methods) */
  returns?: ReturnDoc

  /** Examples */
  examples?: string[]

  /** Tags (e.g., @deprecated, @internal) */
  tags?: Map<string, string>

  /** Source location */
  location?: {
    file?: string
    line: number
    column: number
  }

  /** Type information */
  type?: string

  /** Whether symbol is exported */
  exported?: boolean

  /** Whether symbol is async */
  async?: boolean

  /** Access modifier */
  access?: 'public' | 'private' | 'protected'
}

/**
 * Parameter documentation
 */
export interface ParamDoc {
  /** Parameter name */
  name: string

  /** Parameter type */
  type?: string

  /** Description */
  description?: string

  /** Whether parameter is optional */
  optional?: boolean

  /** Default value */
  defaultValue?: string
}

/**
 * Return value documentation
 */
export interface ReturnDoc {
  /** Return type */
  type?: string

  /** Description */
  description?: string
}

/**
 * Module documentation
 */
export interface ModuleDoc {
  /** Module name */
  name: string

  /** Description */
  description?: string

  /** Exported symbols */
  symbols: SymbolDoc[]

  /** Examples */
  examples?: string[]

  /** Source file */
  file?: string
}

/**
 * Documentation generation options
 */
export interface DocOptions {
  /** Output format */
  format?: 'markdown' | 'json' | 'html'

  /** Include private symbols */
  includePrivate?: boolean

  /** Include internal symbols */
  includeInternal?: boolean

  /** Title for documentation */
  title?: string

  /** File path (for location info) */
  file?: string
}

/**
 * Documentation result
 */
export interface DocResult {
  /** Module documentation */
  module: ModuleDoc

  /** Generated documentation (formatted) */
  output: string

  /** Format used */
  format: 'markdown' | 'json' | 'html'
}

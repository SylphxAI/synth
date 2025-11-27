/**
 * Documentation generator implementation
 */

import type { BaseNode, NodeId, Tree } from '@sylphx/synth'
import type { DocOptions, DocResult, ModuleDoc, ParamDoc, ReturnDoc, SymbolDoc } from './types.js'

/**
 * Documentation generator
 */
export class DocGenerator {
  /**
   * Generate documentation from a tree
   */
  generate(tree: Tree, options: DocOptions = {}): DocResult {
    const module = this.extractModule(tree, options)
    const format = options.format || 'markdown'
    const output = this.format(module, format, options)

    return {
      module,
      output,
      format,
    }
  }

  /**
   * Extract module documentation from tree
   */
  private extractModule(tree: Tree, options: DocOptions): ModuleDoc {
    const symbols: SymbolDoc[] = []

    // Traverse tree to find documented symbols
    this.traverse(tree, tree.root, (node) => {
      const symbol = this.extractSymbol(tree, node, options)
      if (symbol) {
        symbols.push(symbol)
      }
    })

    return {
      name: options.file || 'Module',
      description: this.extractModuleDescription(tree),
      symbols,
      file: options.file,
    }
  }

  /**
   * Extract symbol documentation from node
   */
  private extractSymbol(tree: Tree, node: BaseNode, options: DocOptions): SymbolDoc | null {
    // Extract based on node type
    if (this.isFunctionNode(node)) {
      return this.extractFunction(tree, node, options)
    }

    if (this.isClassNode(node)) {
      return this.extractClass(tree, node, options)
    }

    if (this.isVariableNode(node)) {
      return this.extractVariable(tree, node, options)
    }

    return null
  }

  /**
   * Extract function documentation
   */
  private extractFunction(tree: Tree, node: BaseNode, options: DocOptions): SymbolDoc | null {
    const name = this.getNodeName(node)
    if (!name) return null

    const comment = this.findComment(tree, node)
    const description = this.parseDescription(comment)
    const params = this.parseParams(comment, node)
    const returns = this.parseReturns(comment)
    const examples = this.parseExamples(comment)
    const tags = this.parseTags(comment)

    // Check access
    const access = tags.get('access') as 'public' | 'private' | 'protected' | undefined
    if (!options.includePrivate && access === 'private') return null
    if (!options.includeInternal && tags.has('internal')) return null

    return {
      name,
      kind: 'function',
      description,
      params,
      returns,
      examples,
      tags,
      location: this.getLocation(node, options.file),
      exported: this.isExported(node),
      async: this.isAsync(node),
      access,
    }
  }

  /**
   * Extract class documentation
   */
  private extractClass(tree: Tree, node: BaseNode, options: DocOptions): SymbolDoc | null {
    const name = this.getNodeName(node)
    if (!name) return null

    const comment = this.findComment(tree, node)
    const description = this.parseDescription(comment)
    const examples = this.parseExamples(comment)
    const tags = this.parseTags(comment)

    return {
      name,
      kind: 'class',
      description,
      examples,
      tags,
      location: this.getLocation(node, options.file),
      exported: this.isExported(node),
    }
  }

  /**
   * Extract variable documentation
   */
  private extractVariable(tree: Tree, node: BaseNode, options: DocOptions): SymbolDoc | null {
    const name = this.getNodeName(node)
    if (!name) return null

    const comment = this.findComment(tree, node)
    const description = this.parseDescription(comment)
    const tags = this.parseTags(comment)
    const type = tags.get('type')

    const kind = this.isConstant(node) ? 'constant' : 'variable'

    return {
      name,
      kind,
      description,
      tags,
      type,
      location: this.getLocation(node, options.file),
      exported: this.isExported(node),
    }
  }

  /**
   * Find comment for a node
   */
  private findComment(_tree: Tree, node: BaseNode): string {
    // Look for comment in node data
    if (node.data?.comment) {
      return String(node.data.comment)
    }

    // Look for leadingComments
    if (node.data?.leadingComments && Array.isArray(node.data.leadingComments)) {
      const comments = node.data.leadingComments.map((c: any) => c.value || '').join('\n')
      return comments
    }

    return ''
  }

  /**
   * Parse description from comment
   */
  private parseDescription(comment: string): string {
    if (!comment) return ''

    // Remove comment markers
    const cleaned = comment
      .replace(/^\/\*\*\s*/gm, '')
      .replace(/^\s*\*\s?/gm, '')
      .replace(/\*\/$/, '')
      .trim()

    // Get text before first @tag
    const lines = cleaned.split('\n')
    const descLines: string[] = []

    for (const line of lines) {
      if (line.trim().startsWith('@')) break
      descLines.push(line)
    }

    return descLines.join('\n').trim()
  }

  /**
   * Parse parameters from comment
   */
  private parseParams(comment: string, node: BaseNode): ParamDoc[] {
    const params: ParamDoc[] = []

    // Extract @param tags
    const paramRegex = /@param\s+(?:\{([^}]+)\}\s+)?(\w+)\s*(.*)/g
    let match

    while ((match = paramRegex.exec(comment)) !== null) {
      params.push({
        name: match[2] ?? '',
        type: match[1],
        description: match[3]?.trim(),
        optional: match[1]?.includes('?') || false,
      })
    }

    // If no @param tags, extract from node
    if (params.length === 0 && node.data?.params) {
      const nodeParams = Array.isArray(node.data.params) ? node.data.params : []
      for (const param of nodeParams) {
        const name = typeof param === 'string' ? param : param.name
        if (name) {
          params.push({ name: String(name) })
        }
      }
    }

    return params
  }

  /**
   * Parse return value from comment
   */
  private parseReturns(comment: string): ReturnDoc | undefined {
    const returnRegex = /@returns?\s+(?:\{([^}]+)\}\s+)?(.*)/
    const match = comment.match(returnRegex)

    if (match) {
      return {
        type: match[1],
        description: match[2]?.trim(),
      }
    }

    return undefined
  }

  /**
   * Parse examples from comment
   */
  private parseExamples(comment: string): string[] {
    const examples: string[] = []
    const exampleRegex = /@example\s+([\s\S]*?)(?=@\w+|$)/g
    let match

    while ((match = exampleRegex.exec(comment)) !== null) {
      if (match[1]) examples.push(match[1].trim())
    }

    return examples
  }

  /**
   * Parse tags from comment
   */
  private parseTags(comment: string): Map<string, string> {
    const tags = new Map<string, string>()

    const tagRegex = /@(\w+)(?:\s+([^\n@]*))?/g
    let match

    while ((match = tagRegex.exec(comment)) !== null) {
      const tag = match[1]
      if (!tag) continue
      const value = match[2]?.trim() || ''

      // Skip param, returns, example (handled separately)
      if (['param', 'returns', 'return', 'example'].includes(tag)) {
        continue
      }

      tags.set(tag, value)
    }

    return tags
  }

  /**
   * Extract module description from file-level comments
   */
  private extractModuleDescription(tree: Tree): string {
    // Look for file-level comment in root children
    const rootNode = tree.nodes[tree.root]
    if (!rootNode?.children) return ''

    for (const childId of rootNode.children) {
      const child = tree.nodes[childId]
      if (child?.data?.leadingComments) {
        const comments = child.data.leadingComments as any[]
        if (comments.length > 0) {
          return this.parseDescription(comments[0].value || '')
        }
      }
    }

    return ''
  }

  /**
   * Format documentation
   */
  private format(module: ModuleDoc, format: string, options: DocOptions): string {
    switch (format) {
      case 'json':
        return JSON.stringify(module, null, 2)
      case 'html':
        return this.formatHTML(module, options)
      default:
        return this.formatMarkdown(module, options)
    }
  }

  /**
   * Format as Markdown
   */
  private formatMarkdown(module: ModuleDoc, options: DocOptions): string {
    const parts: string[] = []

    // Title
    parts.push(`# ${options.title || module.name}\n`)

    // Module description
    if (module.description) {
      parts.push(module.description)
      parts.push('')
    }

    // Symbols
    for (const symbol of module.symbols) {
      parts.push(this.formatSymbolMarkdown(symbol))
      parts.push('')
    }

    return parts.join('\n')
  }

  /**
   * Format symbol as Markdown
   */
  private formatSymbolMarkdown(symbol: SymbolDoc): string {
    const parts: string[] = []

    // Header
    parts.push(`## ${symbol.name}`)
    parts.push('')

    // Kind badge
    parts.push(`**${symbol.kind}**`)
    if (symbol.exported) parts.push(' • **exported**')
    if (symbol.async) parts.push(' • **async**')
    parts.push('')

    // Description
    if (symbol.description) {
      parts.push(symbol.description)
      parts.push('')
    }

    // Parameters
    if (symbol.params && symbol.params.length > 0) {
      parts.push('### Parameters')
      parts.push('')
      for (const param of symbol.params) {
        const typeStr = param.type ? `: ${param.type}` : ''
        const optStr = param.optional ? ' (optional)' : ''
        const desc = param.description ? ` - ${param.description}` : ''
        parts.push(`- \`${param.name}\`${typeStr}${optStr}${desc}`)
      }
      parts.push('')
    }

    // Returns
    if (symbol.returns) {
      parts.push('### Returns')
      parts.push('')
      const typeStr = symbol.returns.type ? `**${symbol.returns.type}**` : ''
      const desc = symbol.returns.description || ''
      parts.push(`${typeStr}${typeStr && desc ? ' - ' : ''}${desc}`)
      parts.push('')
    }

    // Examples
    if (symbol.examples && symbol.examples.length > 0) {
      parts.push('### Examples')
      parts.push('')
      for (const example of symbol.examples) {
        parts.push('```javascript')
        parts.push(example)
        parts.push('```')
        parts.push('')
      }
    }

    return parts.join('\n')
  }

  /**
   * Format as HTML
   */
  private formatHTML(module: ModuleDoc, options: DocOptions): string {
    const parts: string[] = []

    parts.push('<!DOCTYPE html>')
    parts.push('<html>')
    parts.push('<head>')
    parts.push(`<title>${options.title || module.name}</title>`)
    parts.push('<style>')
    parts.push('body { font-family: sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }')
    parts.push('h1 { border-bottom: 2px solid #eee; }')
    parts.push('h2 { border-bottom: 1px solid #eee; margin-top: 40px; }')
    parts.push(
      '.badge { display: inline-block; background: #007bff; color: white; padding: 2px 8px; border-radius: 3px; font-size: 12px; margin-right: 5px; }'
    )
    parts.push('pre { background: #f5f5f5; padding: 10px; border-radius: 4px; }')
    parts.push('</style>')
    parts.push('</head>')
    parts.push('<body>')

    parts.push(`<h1>${options.title || module.name}</h1>`)

    if (module.description) {
      parts.push(`<p>${module.description}</p>`)
    }

    for (const symbol of module.symbols) {
      parts.push(this.formatSymbolHTML(symbol))
    }

    parts.push('</body>')
    parts.push('</html>')

    return parts.join('\n')
  }

  /**
   * Format symbol as HTML
   */
  private formatSymbolHTML(symbol: SymbolDoc): string {
    const parts: string[] = []

    parts.push(`<h2>${symbol.name}</h2>`)
    parts.push(`<span class="badge">${symbol.kind}</span>`)

    if (symbol.description) {
      parts.push(`<p>${symbol.description}</p>`)
    }

    if (symbol.params && symbol.params.length > 0) {
      parts.push('<h3>Parameters</h3>')
      parts.push('<ul>')
      for (const param of symbol.params) {
        parts.push(`<li><code>${param.name}</code>`)
        if (param.type) parts.push(`: ${param.type}`)
        if (param.description) parts.push(` - ${param.description}`)
        parts.push('</li>')
      }
      parts.push('</ul>')
    }

    return parts.join('\n')
  }

  /**
   * Check if node is a function
   */
  private isFunctionNode(node: BaseNode): boolean {
    return [
      'FunctionDeclaration',
      'FunctionExpression',
      'ArrowFunctionExpression',
      'MethodDefinition',
    ].includes(node.type)
  }

  /**
   * Check if node is a class
   */
  private isClassNode(node: BaseNode): boolean {
    return node.type === 'ClassDeclaration'
  }

  /**
   * Check if node is a variable
   */
  private isVariableNode(node: BaseNode): boolean {
    return ['VariableDeclaration', 'VariableDeclarator'].includes(node.type)
  }

  /**
   * Get node name
   */
  private getNodeName(node: BaseNode): string {
    if (node.data?.name) return String(node.data.name)
    const id = node.data?.id as { name?: string } | undefined
    if (id?.name) return String(id.name)
    return ''
  }

  /**
   * Check if node is exported
   */
  private isExported(node: BaseNode): boolean {
    return node.data?.exported === true
  }

  /**
   * Check if function is async
   */
  private isAsync(node: BaseNode): boolean {
    return node.data?.async === true
  }

  /**
   * Check if variable is constant
   */
  private isConstant(node: BaseNode): boolean {
    return node.data?.kind === 'const'
  }

  /**
   * Get location info
   */
  private getLocation(node: BaseNode, file?: string) {
    if (!node.span) return undefined

    return {
      file,
      line: node.span.start.line,
      column: node.span.start.column,
    }
  }

  /**
   * Traverse tree
   */
  private traverse(tree: Tree, nodeId: NodeId, callback: (node: BaseNode) => void): void {
    const node = tree.nodes[nodeId]
    if (!node) return

    callback(node)

    for (const childId of node.children) {
      this.traverse(tree, childId, callback)
    }
  }
}

/**
 * Create a documentation generator
 */
export function createGenerator(): DocGenerator {
  return new DocGenerator()
}

/**
 * Generate documentation from a tree
 */
export function generate(tree: Tree, options?: DocOptions): DocResult {
  const generator = new DocGenerator()
  return generator.generate(tree, options)
}

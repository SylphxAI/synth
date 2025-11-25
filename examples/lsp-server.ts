/**
 * Example: Language Server Protocol (LSP) Server with Incremental Parsing
 *
 * Demonstrates how to build a high-performance LSP server using Synth's
 * incremental parsing system.
 *
 * Features:
 * - Real-time document synchronization
 * - Incremental AST updates
 * - Fast diagnostics
 * - Symbol indexing
 * - <1ms response time for typical edits
 *
 * Usage:
 * ```typescript
 * const server = new MarkdownLSPServer()
 * server.start()
 * ```
 */

import {
  IncrementalParserManager,
  detectLanguage,
  detectEdit,
  type Language,
} from '@sylphx/synth'
import type { Tree } from '@sylphx/synth'

/**
 * Document change event
 */
interface DocumentChange {
  uri: string
  version: number
  text: string
}

/**
 * Diagnostic message
 */
interface Diagnostic {
  severity: 'error' | 'warning' | 'info'
  message: string
  line: number
  column: number
}

/**
 * Symbol information
 */
interface Symbol {
  name: string
  kind: 'heading' | 'function' | 'class' | 'variable'
  line: number
  column: number
}

/**
 * LSP Server with Incremental Parsing
 */
export class IncrementalLSPServer {
  private manager: IncrementalParserManager
  private documents = new Map<string, { version: number; language: Language }>()

  constructor() {
    this.manager = new IncrementalParserManager({
      maxSessions: 1000,
      enableMonitoring: true,
      debug: process.env.DEBUG === 'true',
    })
  }

  /**
   * Start LSP server
   */
  start(): void {
    console.log('🚀 Incremental LSP Server started')
    console.log('   - Incremental parsing enabled')
    console.log('   - Multi-language support')
    console.log('   - <1ms response time target')
  }

  /**
   * Handle document open
   */
  onDocumentOpen(uri: string, languageId: string, version: number, text: string): void {
    const startTime = performance.now()

    // Detect language
    const language = this.mapLanguageId(languageId)

    if (!language) {
      console.warn(`Unsupported language: ${languageId}`)
      return
    }

    // Initial parse
    const tree = this.manager.parse(uri, text, language)

    // Store document info
    this.documents.set(uri, { version, language })

    // Generate diagnostics
    const diagnostics = this.getDiagnostics(tree)

    // Index symbols
    const symbols = this.getSymbols(tree)

    const time = performance.now() - startTime

    console.log(
      `📄 Document opened: ${uri} [${language}] ` +
      `(${diagnostics.length} diagnostics, ${symbols.length} symbols, ${time.toFixed(2)}ms)`
    )
  }

  /**
   * Handle document change (incremental update)
   */
  onDocumentChange(change: DocumentChange): void {
    const startTime = performance.now()

    const docInfo = this.documents.get(change.uri)
    if (!docInfo) {
      console.warn(`Document not found: ${change.uri}`)
      return
    }

    // Get previous text
    const session = this.manager.getSession(change.uri, docInfo.language)
    const oldText = session.text

    // Detect edit
    const edit = detectEdit(oldText, change.text)

    // Incremental update
    const { tree, tokenReuseRate, speedup } = this.manager.update(
      change.uri,
      change.text,
      edit
    )

    // Update version
    docInfo.version = change.version
    this.documents.set(change.uri, docInfo)

    // Re-generate diagnostics
    const diagnostics = this.getDiagnostics(tree)

    const time = performance.now() - startTime

    console.log(
      `✏️  Document changed: ${change.uri} ` +
      `(${(tokenReuseRate * 100).toFixed(1)}% reuse, ${speedup.toFixed(1)}x speedup, ` +
      `${diagnostics.length} diagnostics, ${time.toFixed(2)}ms)`
    )

    // Send diagnostics to client
    this.publishDiagnostics(change.uri, diagnostics)
  }

  /**
   * Handle document close
   */
  onDocumentClose(uri: string): void {
    this.manager.closeSession(uri)
    this.documents.delete(uri)
    console.log(`📕 Document closed: ${uri}`)
  }

  /**
   * Get document symbols
   */
  getDocumentSymbols(uri: string): Symbol[] {
    const tree = this.manager.getTree(uri)
    if (!tree) return []

    return this.getSymbols(tree)
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalDocuments: number
    totalUpdates: number
    avgTokenReuse: number
    avgSpeedup: number
  } {
    const stats = this.manager.getGlobalStats()

    return {
      totalDocuments: stats.totalSessions,
      totalUpdates: stats.totalUpdates,
      avgTokenReuse: stats.avgTokenReuseRate,
      avgSpeedup: stats.avgSpeedup,
    }
  }

  /**
   * Map LSP language ID to Synth language
   */
  private mapLanguageId(languageId: string): Language | null {
    const mapping: Record<string, Language> = {
      markdown: 'markdown',
      javascript: 'javascript',
      typescript: 'typescript',
      html: 'html',
      json: 'json',
      css: 'css',
      yaml: 'yaml',
      toml: 'toml',
      xml: 'xml',
    }

    return mapping[languageId] ?? null
  }

  /**
   * Extract diagnostics from AST
   */
  private getDiagnostics(tree: Tree): Diagnostic[] {
    const diagnostics: Diagnostic[] = []

    // Example: Check for empty headings (Markdown-specific)
    for (const node of tree.nodes) {
      if (node.type === 'heading' && node.children.length === 0) {
        diagnostics.push({
          severity: 'warning',
          message: 'Empty heading',
          line: node.span?.start.line ?? 0,
          column: node.span?.start.column ?? 0,
        })
      }
    }

    return diagnostics
  }

  /**
   * Extract symbols from AST
   */
  private getSymbols(tree: Tree): Symbol[] {
    const symbols: Symbol[] = []

    for (const node of tree.nodes) {
      if (node.type === 'heading') {
        symbols.push({
          name: this.getHeadingText(node, tree),
          kind: 'heading',
          line: node.span?.start.line ?? 0,
          column: node.span?.start.column ?? 0,
        })
      }
    }

    return symbols
  }

  /**
   * Get heading text
   */
  private getHeadingText(node: { children: number[] }, tree: Tree): string {
    const textNodes = node.children
      .map(id => tree.nodes[id])
      .filter(n => n?.type === 'text')

    return textNodes.map(n => n?.data?.value ?? '').join('')
  }

  /**
   * Publish diagnostics to client
   */
  private publishDiagnostics(uri: string, diagnostics: Diagnostic[]): void {
    // In real LSP, this would send to client via JSON-RPC
    if (diagnostics.length > 0) {
      console.log(`   ⚠️  ${diagnostics.length} diagnostic(s)`)
    }
  }
}

/**
 * Example usage
 */
export function runExample(): void {
  console.log('='.repeat(60))
  console.log('Incremental LSP Server Example')
  console.log('='.repeat(60))
  console.log()

  const server = new IncrementalLSPServer()
  server.start()
  console.log()

  // Simulate document lifecycle
  const uri = 'file:///example.md'
  const version1 = 1
  const text1 = `# Hello World

This is a test document.

## Section 1

Some content here.

## Section 2

More content.`

  console.log('📖 Opening document...')
  server.onDocumentOpen(uri, 'markdown', version1, text1)
  console.log()

  // Simulate typing
  console.log('⌨️  Simulating user typing...')
  const text2 = text1.replace('Hello World', 'Hello Universe')
  server.onDocumentChange({ uri, version: 2, text: text2 })
  console.log()

  const text3 = text2.replace('test document', 'sample document')
  server.onDocumentChange({ uri, version: 3, text: text3 })
  console.log()

  // Add new section
  console.log('➕ Adding new section...')
  const text4 = text3 + '\n\n## Section 3\n\nNew section content.'
  server.onDocumentChange({ uri, version: 4, text: text4 })
  console.log()

  // Get symbols
  console.log('🔍 Extracting symbols...')
  const symbols = server.getDocumentSymbols(uri)
  console.log(`   Found ${symbols.length} symbols:`)
  for (const symbol of symbols) {
    console.log(`   - ${symbol.kind}: "${symbol.name}" (line ${symbol.line + 1})`)
  }
  console.log()

  // Show statistics
  console.log('📊 Server Statistics:')
  const stats = server.getStatistics()
  console.log(`   - Total documents: ${stats.totalDocuments}`)
  console.log(`   - Total updates: ${stats.totalUpdates}`)
  console.log(`   - Avg token reuse: ${(stats.avgTokenReuse * 100).toFixed(1)}%`)
  console.log(`   - Avg speedup: ${stats.avgSpeedup.toFixed(1)}x`)
  console.log()

  // Close document
  console.log('📕 Closing document...')
  server.onDocumentClose(uri)
  console.log()

  console.log('='.repeat(60))
  console.log('✅ Example completed successfully!')
  console.log('='.repeat(60))
}

// Run example if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  runExample()
}

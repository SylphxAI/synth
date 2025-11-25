/**
 * Example: Real-Time Code Editor with Incremental Parsing
 *
 * Demonstrates building a VS Code/Zed-style editor with <1ms response time.
 *
 * Features:
 * - Real-time syntax highlighting
 * - Live AST view
 * - Incremental updates
 * - Performance monitoring
 * - Multi-language support
 */

import { TrueIncrementalParser, detectEdit } from '@sylphx/synth-md'
import type { Tree } from '@sylphx/synth'

/**
 * Editor state
 */
interface EditorState {
  text: string
  tree: Tree | null
  cursorPosition: number
  selection: { start: number; end: number } | null
}

/**
 * Performance metrics
 */
interface PerformanceMetrics {
  keystrokeCount: number
  totalParseTime: number
  avgParseTime: number
  minParseTime: number
  maxParseTime: number
  avgTokenReuse: number
  avgSpeedup: number
}

/**
 * Real-Time Code Editor
 */
export class RealTimeEditor {
  private parser: TrueIncrementalParser
  private state: EditorState
  private metrics: PerformanceMetrics

  constructor() {
    this.parser = new TrueIncrementalParser()
    this.state = {
      text: '',
      tree: null,
      cursorPosition: 0,
      selection: null,
    }
    this.metrics = {
      keystrokeCount: 0,
      totalParseTime: 0,
      avgParseTime: 0,
      minParseTime: Infinity,
      maxParseTime: 0,
      avgTokenReuse: 0,
      avgSpeedup: 0,
    }
  }

  /**
   * Initialize editor with content
   */
  init(initialText: string): void {
    console.log('🚀 Initializing Real-Time Editor...')
    const startTime = performance.now()

    this.state.text = initialText
    this.state.tree = this.parser.parse(initialText)
    this.state.cursorPosition = 0

    const initTime = performance.now() - startTime
    console.log(`✅ Initialized in ${initTime.toFixed(2)}ms`)
    console.log(`   - ${initialText.length} characters`)
    console.log(`   - ${this.state.tree.nodes.length} AST nodes`)
    console.log()
  }

  /**
   * Handle user typing (single character)
   */
  onType(character: string, position?: number): void {
    const insertPos = position ?? this.state.cursorPosition
    const oldText = this.state.text
    const newText = oldText.slice(0, insertPos) + character + oldText.slice(insertPos)

    this.updateDocument(newText)

    // Update cursor position
    this.state.cursorPosition = insertPos + character.length
  }

  /**
   * Handle backspace
   */
  onBackspace(position?: number): void {
    const deletePos = position ?? this.state.cursorPosition
    if (deletePos === 0) return

    const oldText = this.state.text
    const newText = oldText.slice(0, deletePos - 1) + oldText.slice(deletePos)

    this.updateDocument(newText)

    // Update cursor position
    this.state.cursorPosition = deletePos - 1
  }

  /**
   * Handle paste
   */
  onPaste(pastedText: string, position?: number): void {
    const insertPos = position ?? this.state.cursorPosition
    const oldText = this.state.text
    const newText = oldText.slice(0, insertPos) + pastedText + oldText.slice(insertPos)

    this.updateDocument(newText)

    // Update cursor position
    this.state.cursorPosition = insertPos + pastedText.length
  }

  /**
   * Update document (core incremental parsing logic)
   */
  private updateDocument(newText: string): void {
    const startTime = performance.now()

    // Detect edit
    const edit = detectEdit(this.state.text, newText)

    // Incremental update
    const { tree, stats } = this.parser.update(newText, edit)

    // Update state
    this.state.text = newText
    this.state.tree = tree

    // Update metrics
    const parseTime = performance.now() - startTime
    this.updateMetrics(parseTime, stats.tokenReuseRate, stats.speedup)

    // Render (in real editor, this would update UI)
    this.render()
  }

  /**
   * Update performance metrics
   */
  private updateMetrics(
    parseTime: number,
    tokenReuseRate: number,
    speedup: number
  ): void {
    this.metrics.keystrokeCount++
    this.metrics.totalParseTime += parseTime
    this.metrics.minParseTime = Math.min(this.metrics.minParseTime, parseTime)
    this.metrics.maxParseTime = Math.max(this.metrics.maxParseTime, parseTime)

    const n = this.metrics.keystrokeCount
    this.metrics.avgParseTime = this.metrics.totalParseTime / n
    this.metrics.avgTokenReuse =
      (this.metrics.avgTokenReuse * (n - 1) + tokenReuseRate) / n
    this.metrics.avgSpeedup = (this.metrics.avgSpeedup * (n - 1) + speedup) / n
  }

  /**
   * Render editor (simulate)
   */
  private render(): void {
    // In real editor, this would:
    // 1. Update syntax highlighting
    // 2. Update AST view
    // 3. Update minimap
    // 4. Update diagnostics
  }

  /**
   * Get current metrics
   */
  getMetrics(): PerformanceMetrics {
    return { ...this.metrics }
  }

  /**
   * Get current state
   */
  getState(): EditorState {
    return { ...this.state }
  }

  /**
   * Get AST at cursor position
   */
  getNodeAtCursor(): unknown {
    if (!this.state.tree) return null

    // Find node at cursor position
    for (const node of this.state.tree.nodes) {
      if (!node.span) continue

      if (
        node.span.start.offset <= this.state.cursorPosition &&
        this.state.cursorPosition <= node.span.end.offset
      ) {
        return node
      }
    }

    return null
  }

  /**
   * Print performance report
   */
  printReport(): void {
    console.log('📊 Performance Report')
    console.log('━'.repeat(60))
    console.log(`Total keystrokes:      ${this.metrics.keystrokeCount}`)
    console.log(`Total parse time:      ${this.metrics.totalParseTime.toFixed(2)}ms`)
    console.log(`Average parse time:    ${this.metrics.avgParseTime.toFixed(3)}ms per keystroke`)
    console.log(`Min parse time:        ${this.metrics.minParseTime.toFixed(3)}ms`)
    console.log(`Max parse time:        ${this.metrics.maxParseTime.toFixed(3)}ms`)
    console.log(`Average token reuse:   ${(this.metrics.avgTokenReuse * 100).toFixed(1)}%`)
    console.log(`Average speedup:       ${this.metrics.avgSpeedup.toFixed(1)}x`)
    console.log('━'.repeat(60))

    // Performance rating
    const rating = this.getPerformanceRating()
    console.log(`Performance Rating:    ${rating}`)
    console.log()
  }

  /**
   * Get performance rating
   */
  private getPerformanceRating(): string {
    if (this.metrics.avgParseTime < 0.1) return '⚡ EXCEPTIONAL (<0.1ms)'
    if (this.metrics.avgParseTime < 0.5) return '🚀 EXCELLENT (<0.5ms)'
    if (this.metrics.avgParseTime < 1.0) return '✅ GOOD (<1ms)'
    if (this.metrics.avgParseTime < 5.0) return '⚠️  ACCEPTABLE (<5ms)'
    return '❌ SLOW (>5ms)'
  }
}

/**
 * Example usage: Typing simulation
 */
export function simulateTyping(): void {
  console.log('='.repeat(60))
  console.log('Real-Time Editor - Typing Simulation')
  console.log('='.repeat(60))
  console.log()

  const editor = new RealTimeEditor()

  // Initialize with starter content
  const starter = `# My Document

This is a paragraph.`

  editor.init(starter)

  // Simulate typing a new heading
  console.log('⌨️  Simulating typing: "## New Section"')
  const toType = '\n\n## New Section'

  for (const char of toType) {
    editor.onType(char)
  }

  console.log()

  // Simulate typing paragraph
  console.log('⌨️  Simulating typing a paragraph...')
  const paragraph = '\n\nThis is new content that I am typing in real-time.'

  for (const char of paragraph) {
    editor.onType(char)
  }

  console.log()

  // Simulate backspace
  console.log('⌨️  Simulating backspace (delete 5 characters)...')
  for (let i = 0; i < 5; i++) {
    editor.onBackspace()
  }

  console.log()

  // Simulate paste
  console.log('⌨️  Simulating paste...')
  editor.onPaste(' with incremental parsing')

  console.log()

  // Print final state
  const state = editor.getState()
  console.log('📄 Final Document:')
  console.log('─'.repeat(60))
  console.log(state.text)
  console.log('─'.repeat(60))
  console.log()

  // Print performance report
  editor.printReport()

  console.log('='.repeat(60))
  console.log('✅ Simulation completed successfully!')
  console.log('='.repeat(60))
}

/**
 * Example usage: Live preview simulation
 */
export function simulateLivePreview(): void {
  console.log('='.repeat(60))
  console.log('Real-Time Editor - Live Preview Simulation')
  console.log('='.repeat(60))
  console.log()

  const editor = new RealTimeEditor()

  const initialDoc = `# Documentation

## Introduction

Welcome to the docs.

## Getting Started

Install the package:

\`\`\`bash
npm install package
\`\`\`

## Usage

Example code here.`

  editor.init(initialDoc)

  // Simulate editing multiple sections
  console.log('✏️  Editing multiple sections...')
  console.log()

  const edits = [
    { description: 'Updating introduction', text: 'Welcome to our amazing docs!' },
    { description: 'Adding more details', text: '\n\nFor more info, see the API reference.' },
    { description: 'Updating code example', text: 'npm install @company/package' },
  ]

  for (const edit of edits) {
    console.log(`   📝 ${edit.description}`)
    editor.onType(edit.text)
  }

  console.log()
  editor.printReport()
}

// Run examples if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  simulateTyping()
  console.log('\n\n')
  simulateLivePreview()
}

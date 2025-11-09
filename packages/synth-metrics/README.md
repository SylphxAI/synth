# @sylphx/synth-metrics

Code metrics and complexity analysis for Synth AST - works across all languages.

## Features

- ✅ **Universal Metrics** - Calculate metrics for any language
- 📊 **Comprehensive Analysis** - Basic, complexity, Halstead, and maintainability metrics
- 🎯 **Function-Level** - Per-function complexity and metrics
- 🚀 **Language-Agnostic** - Works on Synth's universal AST
- 📈 **Industry Standard** - Cyclomatic complexity, Halstead volume, maintainability index
- ⚡ **Fast** - Leverages Synth's performance-optimized AST

## Installation

```bash
npm install @sylphx/synth-metrics
```

## Usage

### Quick Start

```typescript
import { analyze } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'

const tree = parse(`
function fibonacci(n) {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}
`)

const metrics = analyze(tree)

console.log(metrics.basic.loc)              // Lines of code
console.log(metrics.complexity.cyclomatic)  // Cyclomatic complexity
console.log(metrics.maintainability.index)  // Maintainability index
```

### Full Report

```typescript
import { report } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'

const tree = parse(sourceCode)
const metricsReport = report(tree, 'example.js')

console.log(metricsReport)
// {
//   file: 'example.js',
//   language: 'javascript',
//   metrics: { basic, complexity, halstead, maintainability },
//   functions: [{ name, loc, complexity, params, ... }],
//   timestamp: 1234567890
// }
```

## Metrics Categories

### Basic Metrics

```typescript
interface BasicMetrics {
  loc: number          // Total lines of code
  sloc: number         // Source lines (non-comment, non-blank)
  cloc: number         // Comment lines
  blank: number        // Blank lines
  nodes: number        // Total AST nodes
  maxDepth: number     // Maximum nesting depth
  avgDepth: number     // Average node depth
}
```

**Example:**

```typescript
const metrics = analyze(tree)

console.log(`Total lines: ${metrics.basic.loc}`)
console.log(`Source lines: ${metrics.basic.sloc}`)
console.log(`Comments: ${metrics.basic.cloc}`)
console.log(`Blank lines: ${metrics.basic.blank}`)
console.log(`Max nesting: ${metrics.basic.maxDepth}`)
```

### Complexity Metrics

```typescript
interface ComplexityMetrics {
  cyclomatic: number      // Cyclomatic complexity
  decisionPoints: number  // Number of decision points
  cognitive: number       // Cognitive complexity (weighted)
}
```

**Cyclomatic Complexity** - Measures number of linearly independent paths:
- Formula: `E - N + 2P` or simply `decision points + 1`
- Decision points: if, else, for, while, case, catch, &&, ||, ?

**Cognitive Complexity** - Measures how difficult code is to understand:
- Weighted by nesting level
- Nested conditions add more complexity

**Example:**

```typescript
const metrics = analyze(tree)

console.log(`Cyclomatic complexity: ${metrics.complexity.cyclomatic}`)
console.log(`Decision points: ${metrics.complexity.decisionPoints}`)
console.log(`Cognitive complexity: ${metrics.complexity.cognitive}`)

if (metrics.complexity.cyclomatic > 10) {
  console.warn('High complexity - consider refactoring')
}
```

### Halstead Complexity Metrics

```typescript
interface HalsteadMetrics {
  n1: number              // Distinct operators
  n2: number              // Distinct operands
  N1: number              // Total operators
  N2: number              // Total operands
  vocabulary: number      // n1 + n2
  length: number          // N1 + N2
  calculatedLength: number
  volume: number          // Length * log2(vocabulary)
  difficulty: number      // (n1/2) * (N2/n2)
  effort: number          // Volume * difficulty
  time: number            // Time to program (seconds)
  bugs: number            // Estimated bugs
}
```

**Example:**

```typescript
const metrics = analyze(tree)

console.log(`Program volume: ${metrics.halstead.volume.toFixed(2)}`)
console.log(`Difficulty: ${metrics.halstead.difficulty.toFixed(2)}`)
console.log(`Est. bugs: ${metrics.halstead.bugs.toFixed(3)}`)
console.log(`Est. time: ${(metrics.halstead.time / 60).toFixed(1)} minutes`)
```

### Maintainability Metrics

```typescript
interface MaintainabilityMetrics {
  index: number        // 0-100, higher is better
  level: 'low' | 'moderate' | 'high' | 'very high'
  maintainable: boolean  // index >= 20
}
```

**Maintainability Index** - Microsoft's metric:
- Formula: `171 - 5.2 * ln(V) - 0.23 * G - 16.2 * ln(LOC)`
- Range: 0-100 (higher is better)
- Levels:
  - 85-100: Low difficulty (excellent)
  - 65-84: Moderate difficulty (good)
  - 20-64: High difficulty (needs attention)
  - 0-19: Very high difficulty (critical)

**Example:**

```typescript
const metrics = analyze(tree)

console.log(`Maintainability: ${metrics.maintainability.index.toFixed(2)}`)
console.log(`Level: ${metrics.maintainability.level}`)

if (!metrics.maintainability.maintainable) {
  console.error('Code is not maintainable!')
}
```

## Function-Level Metrics

```typescript
interface FunctionMetrics {
  name: string
  location: { start, end }
  loc: number          // Lines in function
  complexity: number   // Function complexity
  params: number       // Parameter count
  maxDepth: number     // Max nesting in function
}
```

**Example:**

```typescript
const metricsReport = report(tree)

for (const fn of metricsReport.functions) {
  console.log(`Function: ${fn.name}`)
  console.log(`  LOC: ${fn.loc}`)
  console.log(`  Complexity: ${fn.complexity}`)
  console.log(`  Parameters: ${fn.params}`)

  if (fn.complexity > 10) {
    console.warn(`  ⚠️ High complexity!`)
  }
  if (fn.params > 5) {
    console.warn(`  ⚠️ Too many parameters!`)
  }
}
```

## API

### analyze(tree)

Analyze a tree and return all metrics.

```typescript
import { analyze } from '@sylphx/synth-metrics'

const metrics = analyze(tree)
```

### report(tree, file?)

Generate a full metrics report with timestamp and file info.

```typescript
import { report } from '@sylphx/synth-metrics'

const metricsReport = report(tree, 'example.js')
```

### MetricsAnalyzer

Advanced usage with analyzer instance.

```typescript
import { MetricsAnalyzer } from '@sylphx/synth-metrics'

const analyzer = new MetricsAnalyzer()
const metrics = analyzer.analyze(tree)
const metricsReport = analyzer.report(tree, 'example.js')
```

## Examples

### Check Code Quality

```typescript
import { analyze } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'

const tree = parse(sourceCode)
const metrics = analyze(tree)

// Check maintainability
if (metrics.maintainability.index < 20) {
  console.error('❌ Code is not maintainable')
  process.exit(1)
}

// Check complexity
if (metrics.complexity.cyclomatic > 15) {
  console.warn('⚠️ High cyclomatic complexity')
}

// Check size
if (metrics.basic.loc > 500) {
  console.warn('⚠️ File is too large')
}
```

### CI/CD Integration

```typescript
import { report } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'
import { readFileSync } from 'fs'

const source = readFileSync('src/index.js', 'utf-8')
const tree = parse(source)
const metricsReport = report(tree, 'src/index.js')

// Check quality gates
const passed =
  metricsReport.metrics.maintainability.index >= 65 &&
  metricsReport.metrics.complexity.cyclomatic <= 15

if (!passed) {
  console.error('Quality gates failed!')
  console.log(JSON.stringify(metricsReport, null, 2))
  process.exit(1)
}

console.log('✅ Quality gates passed')
```

### Generate Report for All Files

```typescript
import { report } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'
import { readdirSync, readFileSync, writeFileSync } from 'fs'

const files = readdirSync('src').filter(f => f.endsWith('.js'))
const reports = []

for (const file of files) {
  const source = readFileSync(`src/${file}`, 'utf-8')
  const tree = parse(source)
  const metricsReport = report(tree, file)
  reports.push(metricsReport)
}

// Save reports
writeFileSync('metrics.json', JSON.stringify(reports, null, 2))

// Summary
const avgComplexity = reports.reduce((sum, r) =>
  sum + r.metrics.complexity.cyclomatic, 0) / reports.length

console.log(`Average complexity: ${avgComplexity.toFixed(2)}`)
```

### Compare Metrics Before/After

```typescript
import { analyze } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'

const beforeTree = parse(beforeCode)
const afterTree = parse(afterCode)

const beforeMetrics = analyze(beforeTree)
const afterMetrics = analyze(afterTree)

console.log('Complexity:')
console.log(`  Before: ${beforeMetrics.complexity.cyclomatic}`)
console.log(`  After: ${afterMetrics.complexity.cyclomatic}`)
console.log(`  Change: ${afterMetrics.complexity.cyclomatic - beforeMetrics.complexity.cyclomatic}`)

console.log('Maintainability:')
console.log(`  Before: ${beforeMetrics.maintainability.index.toFixed(2)}`)
console.log(`  After: ${afterMetrics.maintainability.index.toFixed(2)}`)
```

### Function Complexity Report

```typescript
import { report } from '@sylphx/synth-metrics'
import { parse } from '@sylphx/synth-js'

const tree = parse(sourceCode)
const metricsReport = report(tree)

// Sort functions by complexity
const sorted = metricsReport.functions.sort((a, b) => b.complexity - a.complexity)

console.log('Top 10 most complex functions:')
for (const fn of sorted.slice(0, 10)) {
  console.log(`  ${fn.name}: complexity ${fn.complexity}`)
}
```

### Language-Agnostic Analysis

```typescript
import { analyze } from '@sylphx/synth-metrics'
import { parse as parseJS } from '@sylphx/synth-js'
import { parse as parsePy } from '@sylphx/synth-python'
import { parse as parseGo } from '@sylphx/synth-go'

// Analyze JavaScript
const jsTree = parseJS(jsCode)
const jsMetrics = analyze(jsTree)

// Analyze Python
const pyTree = parsePy(pyCode)
const pyMetrics = analyze(pyTree)

// Analyze Go
const goTree = parseGo(goCode)
const goMetrics = analyze(goTree)

// Compare across languages
console.log('Complexity comparison:')
console.log(`  JavaScript: ${jsMetrics.complexity.cyclomatic}`)
console.log(`  Python: ${pyMetrics.complexity.cyclomatic}`)
console.log(`  Go: ${goMetrics.complexity.cyclomatic}`)
```

## Interpreting Metrics

### Cyclomatic Complexity

- **1-10**: Simple, well-structured code
- **11-20**: Moderately complex, acceptable
- **21-50**: Complex, needs refactoring
- **51+**: Very complex, high risk

### Maintainability Index

- **85-100**: Highly maintainable (green)
- **65-84**: Moderately maintainable (yellow)
- **20-64**: Difficult to maintain (orange)
- **0-19**: Unmaintainable (red)

### Function Metrics

- **Parameters**: Ideally ≤ 3, max 5
- **LOC**: Ideally ≤ 50, max 100
- **Complexity**: Ideally ≤ 5, max 10
- **Depth**: Ideally ≤ 3, max 5

## Use Cases

- **Code quality enforcement** - Fail builds on poor metrics
- **Refactoring priorities** - Identify complex functions to refactor
- **Code review** - Objective metrics for discussions
- **Technical debt tracking** - Monitor trends over time
- **CI/CD quality gates** - Maintain code quality standards
- **Performance correlation** - Complex code often performs poorly
- **Multi-language projects** - Consistent metrics across languages

## Performance

Leverages Synth's performance-optimized AST:
- Fast traversal using arena-based storage
- O(1) node access
- Single-pass analysis
- Minimal memory overhead

## License

MIT

---

**Note:** This is a universal metrics analyzer. Works across all languages supported by Synth parsers.

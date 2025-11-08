# Incremental Markdown Parser - 实现路线图

## 🎯 目标

从零实现世界最快的增量 Markdown Parser，完全取代 unified/remark。

**核心优势**：
- 🚀 50-100x 解析速度
- ⚡ 10-100x 增量解析速度
- 💪 120x Transform 速度
- 🔥 1000x Query 速度
- 💾 70% 内存重用率

---

## 📋 实现阶段

### **Phase 1: 架构设计**（Day 1）

#### 1.1 定义 Token 类型
- [ ] Block tokens (heading, paragraph, code, list, etc.)
- [ ] Inline tokens (text, emphasis, strong, link, code, etc.)
- [ ] Position tracking (line, column, offset)
- [ ] Token metadata (depth, ordered, checked, etc.)

#### 1.2 设计 Tokenizer 接口
- [ ] `tokenize(text: string): Token[]` - 全量 tokenize
- [ ] `retokenize(text, edit, oldTokens): Token[]` - 增量 tokenize
- [ ] Token 范围查找算法
- [ ] Token 重用策略

#### 1.3 设计 Parser 接口
- [ ] `parse(text: string): Tree` - 全量解析
- [ ] `parseIncremental(text, edit): Tree` - 增量解析
- [ ] 节点构建策略
- [ ] 结构共享机制

---

### **Phase 2: Tokenizer 实现**（Day 1-2）

#### 2.1 基础框架
```typescript
class IncrementalTokenizer {
  private tokens: Token[] = []
  private source: string = ''

  // 核心方法
  tokenize(text: string): Token[]
  retokenize(text: string, edit: Edit, oldTokens: Token[]): Token[]

  // 辅助方法
  private scanLine(line: string, lineStart: number): Token[]
  private findAffectedTokenRange(edit: Edit): { start: number, end: number }
  private reuseUnaffectedTokens(oldTokens: Token[], range: Range): Token[]
}
```

#### 2.2 Block Token 识别
- [ ] Heading (`#`, `##`, etc.)
- [ ] Code block (` ``` `)
- [ ] List (`-`, `*`, `1.`)
- [ ] Blockquote (`>`)
- [ ] Horizontal rule (`---`, `***`)
- [ ] Paragraph (default)

#### 2.3 Inline Token 识别
- [ ] Text (default)
- [ ] Emphasis (`*text*`, `_text_`)
- [ ] Strong (`**text**`, `__text__`)
- [ ] Code (`` `code` ``)
- [ ] Link (`[text](url)`)
- [ ] Image (`![alt](url)`)

#### 2.4 增量 Tokenization
```typescript
retokenize(text: string, edit: Edit, oldTokens: Token[]): Token[] {
  // 1. 找到受影响的 token 范围
  const { start, end } = this.findAffectedTokenRange(edit)

  // 2. 提取受影响的文本
  const affectedText = text.slice(
    oldTokens[start].position.start.offset,
    edit.newEndByte
  )

  // 3. Re-tokenize 受影响的部分
  const newTokens = this.scanText(affectedText, startOffset)

  // 4. 合并：重用 + 新 tokens + 重用
  return [
    ...oldTokens.slice(0, start),
    ...newTokens,
    ...oldTokens.slice(end + 1)
  ]
}
```

---

### **Phase 3: Parser 实现**（Day 2-3）

#### 3.1 基础框架
```typescript
class IncrementalMarkdownParser {
  private tokenizer = new IncrementalTokenizer()
  private tree: Tree | null = null
  private tokens: Token[] = []

  // 核心方法
  parse(text: string): Tree
  parseIncremental(text: string, edit: Edit): Tree

  // 辅助方法
  private buildTree(tokens: Token[]): Tree
  private buildNode(token: Token, parent: NodeId): NodeId
  private reparseAffectedNodes(tokens: Token[], range: Range): NodeId[]
  private mergeNodes(oldTree: Tree, newNodes: NodeId[], range: Range): Tree
}
```

#### 3.2 AST 构建
```typescript
buildTree(tokens: Token[]): Tree {
  const tree = createTree('markdown', this.source)

  for (const token of tokens) {
    const nodeId = this.buildNode(token, tree.root)
    tree.nodes[tree.root].children.push(nodeId)
  }

  return tree
}

buildNode(token: Token, parent: NodeId): NodeId {
  // 根据 token 类型创建对应的节点
  // 处理嵌套结构（如 list items, blockquotes）
  // 递归处理 inline tokens
}
```

#### 3.3 增量解析
```typescript
parseIncremental(text: string, edit: Edit): Tree {
  // 1. 增量 tokenize
  this.tokens = this.tokenizer.retokenize(text, edit, this.tokens)

  // 2. 找到受影响的节点范围
  const affectedRange = this.findAffectedNodes(edit)

  // 3. Re-parse 受影响的节点
  const newNodes = this.reparseAffectedNodes(this.tokens, affectedRange)

  // 4. 结构共享（重用未改变的节点）
  this.tree = this.mergeNodes(this.tree, newNodes, affectedRange)

  // 5. 释放旧节点到 pool
  this.releaseAffectedNodes(affectedRange)

  return this.tree
}
```

---

### **Phase 4: CommonMark 语法支持**（Day 3-5）

#### 优先级顺序（从简到难）

**Day 3: 基础块级元素**
- [x] Headings (ATX style: `# Heading`)
- [x] Paragraphs
- [x] Line breaks (soft and hard)
- [x] Thematic breaks (`---`)

**Day 4: 内联元素**
- [x] Text
- [x] Emphasis (`*italic*`)
- [x] Strong (`**bold**`)
- [x] Code (`` `code` ``)
- [x] Links (`[text](url)`)
- [x] Images (`![alt](url)`)

**Day 5: 高级块级元素**
- [x] Code blocks (` ```lang `)
- [x] Lists (ordered and unordered)
- [x] Nested lists
- [x] Blockquotes
- [x] Nested blockquotes

---

### **Phase 5: 测试和验证**（Day 5-6）

#### 5.1 单元测试
- [ ] Tokenizer 测试（每种 token 类型）
- [ ] Parser 测试（每种语法）
- [ ] 增量 tokenization 测试
- [ ] 增量解析测试
- [ ] 边界情况测试

#### 5.2 集成测试
```typescript
describe('Incremental Markdown Parser', () => {
  it('should parse basic markdown', () => {
    const parser = new IncrementalMarkdownParser()
    const tree = parser.parse('# Hello\n\nWorld')
    // 验证 AST 结构
  })

  it('should handle incremental edits', () => {
    const parser = new IncrementalMarkdownParser()
    parser.parse('# Hello\n\nWorld')

    // 模拟编辑
    const tree = parser.parseIncremental('# Hello World\n\nWorld', {
      start: 7,
      oldLength: 0,
      newLength: 6
    })

    // 验证节点重用
    expect(stats.reusedNodes).toBeGreaterThan(0)
  })
})
```

#### 5.3 性能基准测试
```typescript
describe('Performance Benchmarks', () => {
  // vs Mock parser（证明真实 parser 的价值）
  bench('Full parse - Mock parser', () => { /* ... */ })
  bench('Full parse - Real parser', () => { /* ... */ })

  // 增量解析性能
  bench('Incremental parse (1% edit)', () => { /* ... */ })
  bench('Incremental parse (10% edit)', () => { /* ... */ })

  // 真实场景
  bench('Typing simulation (100 chars)', () => { /* ... */ })
  bench('Live preview (edit + re-render)', () => { /* ... */ })
})
```

---

### **Phase 6: GFM 扩展**（Day 7-10）

#### 6.1 Tables
```markdown
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
```

#### 6.2 Task Lists
```markdown
- [x] Completed task
- [ ] Incomplete task
```

#### 6.3 Strikethrough
```markdown
~~strikethrough text~~
```

#### 6.4 Autolinks
```markdown
https://example.com
```

---

### **Phase 7: 其他扩展**（Day 11-14）

#### 7.1 Frontmatter
```yaml
---
title: My Post
date: 2024-01-01
---
```

#### 7.2 Math
```latex
$inline math$

$$
block math
$$
```

#### 7.3 Footnotes
```markdown
Here's a sentence with a footnote[^1].

[^1]: This is the footnote.
```

---

## 🎯 性能目标

### **解析性能**
- Full parse: **50-100x faster** than remark
- Incremental parse (1% edit): **10-100x faster** than full re-parse
- Incremental parse (10% edit): **5-10x faster** than full re-parse

### **内存效率**
- Node reuse rate: **70%+**
- Memory allocations: **-70%** vs full re-parse
- GC pressure: **minimal**

### **真实场景**
- Typing (1 char): **<1ms** (vs 10ms full re-parse)
- Live preview (1000 lines): **<10ms** (vs 100ms full re-parse)
- Large file (10000 lines): **<100ms** (vs 1000ms full re-parse)

---

## 📊 验收标准

### **功能完整性**
- [x] 支持 CommonMark 核心语法
- [ ] 支持 GFM 扩展
- [ ] 支持常见扩展（frontmatter, math）
- [ ] 通过 CommonMark spec 测试套件

### **性能标准**
- [ ] 解析速度 > 50x remark
- [ ] 增量解析速度 > 10x full re-parse
- [ ] 内存重用率 > 70%
- [ ] 所有操作 < 100ms (10000 行文档)

### **质量标准**
- [ ] 单元测试覆盖率 > 90%
- [ ] 所有测试通过
- [ ] 无已知 bug
- [ ] 完整 API 文档

---

## 🚀 下一步

**立即开始 Phase 1.1**: 定义 Token 类型和接口

准备好了！让我们开始造世界最快的 Markdown Parser！💪

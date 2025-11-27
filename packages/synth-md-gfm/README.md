# @sylphx/synth-md-gfm

GitHub Flavored Markdown (GFM) extensions for Synth.

## Installation

```bash
npm install @sylphx/synth @sylphx/synth-md @sylphx/synth-md-gfm
```

## Features

- **Tables** with column alignment
- **Strikethrough** text (~~deleted~~)
- **Autolinks** for URLs and emails
- **Task lists** with checkboxes

## Usage

```typescript
import { parse } from '@sylphx/synth-md'
import { gfmPlugin } from '@sylphx/synth-md-gfm'

const markdown = `
# GFM Examples

## Tables

| Feature | Supported |
|---------|:---------:|
| Tables  | ✓         |
| Align   | ✓         |

## Strikethrough

This is ~~deleted~~ text.

## Autolinks

Visit https://github.com

## Task Lists

- [x] Completed task
- [ ] Pending task
`

const tree = parse(markdown, { plugins: [gfmPlugin()] })
```

## Options

```typescript
interface GFMPluginOptions {
  tables?: boolean        // Enable tables (default: true)
  strikethrough?: boolean // Enable ~~text~~ (default: true)
  autolinks?: boolean     // Enable URL autolinks (default: true)
  taskLists?: boolean     // Enable task lists (default: true)
}
```

## GFM Specification

This plugin implements the [GitHub Flavored Markdown Spec](https://github.github.com/gfm/):

### Tables

```markdown
| Left | Center | Right |
|:-----|:------:|------:|
| A    | B      | C     |
```

- Column alignment with `:---`, `:---:`, `---:`
- Header row required
- Separator row required

### Strikethrough

```markdown
~~This text is deleted~~
```

### Autolinks

```markdown
https://example.com
user@example.com
```

Automatically converts URLs and emails to links.

### Task Lists

```markdown
- [x] Completed
- [ ] Todo
```

Checkbox syntax for task items.

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

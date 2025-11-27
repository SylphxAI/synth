# @sylphx/synth-vue

Vue Single File Component (SFC) parser using Synth's universal AST. Conversion layer over @vue/compiler-sfc.

## Features

- ✅ **Official Parser** - Uses @vue/compiler-sfc (Vue's official SFC compiler)
- 🚀 **Full SFC Support** - Template, Script, Script Setup, Style, Custom Blocks
- 🎯 **Universal AST** - Converts Vue SFC to Synth's language-agnostic format
- 🔌 **Plugin System** - Transform AST with sync/async plugins
- 📦 **Production Ready** - @vue/compiler-sfc powers Vue 3 and Vite

## Installation

```bash
npm install @sylphx/synth-vue
```

## Usage

### Quick Start

```typescript
import { parse } from '@sylphx/synth-vue'

const vue = `
<template>
  <div>{{ message }}</div>
</template>

<script setup>
import { ref } from 'vue'

const message = ref('Hello Vue!')
</script>

<style scoped>
div {
  color: blue;
}
</style>
`

const tree = parse(vue)
console.log(tree.nodes[tree.root])
```

### Parser API

```typescript
import { VueParser, createParser, parse, parseAsync } from '@sylphx/synth-vue'

// Standalone function (recommended)
const tree = parse(vueSource)

// With options
const tree = parse(vueSource, {
  filename: 'MyComponent.vue',
  sourceMap: true
})

// Async parsing (for plugins)
const tree = await parseAsync(vueSource)

// Class instance
const parser = new VueParser()
const tree = parser.parse(vueSource)

// Factory function
const parser = createParser()
const tree = parser.parse(vueSource)
```

### Plugin System

```typescript
import { parse, type Tree } from '@sylphx/synth-vue'

// Sync plugin
const myPlugin = {
  name: 'my-plugin',
  transform(tree: Tree) {
    // Modify tree
    return tree
  }
}

const tree = parse(vueSource, { plugins: [myPlugin] })

// Async plugin
const asyncPlugin = {
  name: 'async-plugin',
  async transform(tree: Tree) {
    // Async modifications
    return tree
  }
}

const tree = await parseAsync(vueSource, { plugins: [asyncPlugin] })
```

## AST Structure

The parser generates a universal Synth AST by converting Vue's SFC descriptor. Each node includes:

### Node Structure

```typescript
{
  type: 'VueSFC' | 'VueTemplate' | 'VueScript' | 'VueScriptSetup' | 'VueStyle' | ...,
  parent: NodeId,
  children: [NodeId],
  span: {
    start: { offset, line, column },
    end: { offset, line, column }
  },
  data: {
    content?: string,       // Block content
    lang?: string,          // Language (html, js, ts, css, scss, etc.)
    scoped?: boolean,       // Scoped styles
    module?: boolean,       // CSS modules
    attrs?: object,         // Block attributes
    blockType?: string      // Block type
  }
}
```

## Supported Vue Features

### Template Block
- ✅ Vue template syntax
- ✅ Directives (`v-if`, `v-for`, `v-bind`, `v-on`, etc.)
- ✅ Mustache interpolation `{{ }}`
- ✅ Slots
- ✅ Components

### Script Block
- ✅ Options API
- ✅ Composition API
- ✅ TypeScript support (`lang="ts"`)
- ✅ Imports and exports

### Script Setup
- ✅ `<script setup>` syntax
- ✅ `defineProps()`
- ✅ `defineEmits()`
- ✅ `defineExpose()`
- ✅ TypeScript with generics
- ✅ Top-level await

### Style Block
- ✅ Regular styles
- ✅ Scoped styles (`scoped`)
- ✅ CSS Modules (`module`)
- ✅ Preprocessors (SCSS, Less, Stylus via `lang`)
- ✅ Multiple style blocks

### Custom Blocks
- ✅ `<i18n>` blocks
- ✅ `<docs>` blocks
- ✅ Any custom block type

## Examples

### Basic Component

```typescript
const vue = `
<template>
  <div class="greeting">
    <h1>{{ message }}</h1>
  </div>
</template>

<script>
export default {
  data() {
    return {
      message: 'Hello World'
    }
  }
}
</script>

<style scoped>
.greeting {
  text-align: center;
}
</style>
`

const tree = parse(vue)
```

### Script Setup Component

```typescript
const vue = `
<template>
  <div>
    <p>Count: {{ count }}</p>
    <button @click="increment">Increment</button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const count = ref(0)
const increment = () => count.value++
</script>
`

const tree = parse(vue)
```

### TypeScript Component

```typescript
const vue = `
<template>
  <div>{{ message }}</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface User {
  name: string
  email: string
}

const message = ref<string>('Hello')
const user = ref<User>({ name: 'John', email: 'john@example.com' })
</script>
`

const tree = parse(vue)
```

### Props and Emits

```typescript
const vue = `
<template>
  <button @click="handleClick">{{ label }}</button>
</template>

<script setup>
const props = defineProps({
  label: String,
  disabled: Boolean
})

const emit = defineEmits(['click', 'update'])

const handleClick = () => {
  emit('click')
}
</script>
`

const tree = parse(vue)
```

### Counter Component

```typescript
const vue = `
<template>
  <div class="counter">
    <h2>Counter: {{ count }}</h2>
    <button @click="increment">+</button>
    <button @click="decrement">-</button>
    <button @click="reset">Reset</button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const count = ref(0)

const increment = () => count.value++
const decrement = () => count.value--
const reset = () => count.value = 0
</script>

<style scoped>
.counter {
  text-align: center;
  padding: 20px;
}

button {
  margin: 0 5px;
  padding: 10px 20px;
  font-size: 16px;
}
</style>
`

const tree = parse(vue)
```

### Form Component

```typescript
const vue = `
<template>
  <form @submit.prevent="handleSubmit">
    <div>
      <label>Name:</label>
      <input v-model="form.name" type="text" required />
    </div>
    <div>
      <label>Email:</label>
      <input v-model="form.email" type="email" required />
    </div>
    <button type="submit">Submit</button>
  </form>
</template>

<script setup>
import { reactive } from 'vue'

const form = reactive({
  name: '',
  email: ''
})

const handleSubmit = () => {
  console.log('Form submitted:', form)
}
</script>

<style scoped>
form {
  max-width: 400px;
  margin: 0 auto;
}

div {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
}

input {
  width: 100%;
  padding: 8px;
}
</style>
`

const tree = parse(vue)
```

### Todo List

```typescript
const vue = `
<template>
  <div class="todo-list">
    <input
      v-model="newTodo"
      @keyup.enter="addTodo"
      placeholder="Add a todo..."
    />
    <ul>
      <li
        v-for="todo in todos"
        :key="todo.id"
        :class="{ done: todo.done }"
        @click="toggleTodo(todo.id)"
      >
        {{ todo.text }}
        <button @click.stop="removeTodo(todo.id)">×</button>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const newTodo = ref('')
const todos = ref([])
let nextId = 1

const addTodo = () => {
  if (newTodo.value.trim()) {
    todos.value.push({
      id: nextId++,
      text: newTodo.value,
      done: false
    })
    newTodo.value = ''
  }
}

const toggleTodo = (id) => {
  const todo = todos.value.find(t => t.id === id)
  if (todo) todo.done = !todo.done
}

const removeTodo = (id) => {
  todos.value = todos.value.filter(t => t.id !== id)
}
</script>

<style scoped>
.todo-list {
  max-width: 500px;
  margin: 0 auto;
}

input {
  width: 100%;
  padding: 10px;
  margin-bottom: 20px;
}

ul {
  list-style: none;
  padding: 0;
}

li {
  padding: 10px;
  border-bottom: 1px solid #ddd;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
}

li.done {
  text-decoration: line-through;
  opacity: 0.6;
}

button {
  background: red;
  color: white;
  border: none;
  padding: 5px 10px;
  cursor: pointer;
}
</style>
`

const tree = parse(vue)
```

### With SCSS

```typescript
const vue = `
<template>
  <div class="button">Button</div>
</template>

<style lang="scss" scoped>
$primary-color: #007bff;

.button {
  background-color: $primary-color;
  padding: 10px 20px;

  &:hover {
    opacity: 0.8;
  }
}
</style>
`

const tree = parse(vue)
```

### With i18n Custom Block

```typescript
const vue = `
<template>
  <div>{{ $t('hello') }}</div>
</template>

<i18n>
{
  "en": {
    "hello": "Hello"
  },
  "zh": {
    "hello": "你好"
  }
}
</i18n>
`

const tree = parse(vue)
```

## Performance

Leverages @vue/compiler-sfc's proven performance:
- Fast parsing of Vue SFC files
- Optimized for production use
- Used by Vue 3 and Vite
- Efficient AST generation

## Development Philosophy

This package uses a **strategic dependency** approach:

- **Third-party parser:** @vue/compiler-sfc (Vue's official SFC compiler)
- **Our conversion layer:** Vue SFC descriptor → Synth universal AST
- **Our value:** Universal format, cross-language tools, plugin system

### Why @vue/compiler-sfc?

- ❌ Writing Vue SFC parser: 150+ hours, complex spec, Vue updates
- ✅ Using @vue/compiler-sfc: Official implementation, always up-to-date
- **Our focus:** Universal AST format, transformations, cross-language operations

## Use Cases

- **Component analysis:** Analyze Vue components
- **Code transformation:** Transform Vue SFC patterns
- **Linting:** Build custom Vue linters
- **Documentation:** Extract component docs
- **Migration tools:** Migrate Vue 2 to Vue 3
- **Static analysis:** Component complexity, prop usage
- **Cross-language tools:** Analyze Vue + TypeScript + CSS together

## Parser Options

```typescript
interface VueParseOptions {
  // Source filename (for better error messages)
  filename?: string

  // Enable source map support
  sourceMap?: boolean

  // Plugin system
  plugins?: Plugin[]
}
```

## Block Types

The parser recognizes these block types:

- `VueTemplate` - Template block
- `VueScript` - Script block (Options API or Composition API)
- `VueScriptSetup` - Script setup block
- `VueStyle` - Style block (can have multiple)
- `VueCustomBlock_*` - Custom blocks (i18n, docs, etc.)

Each block preserves:
- Content (the actual code/markup)
- Language (`lang` attribute)
- Scoped status (for styles)
- Module status (for CSS modules)
- All other attributes

## License

MIT

---

**Note:** This package uses @vue/compiler-sfc for parsing. See [@vue/compiler-sfc](https://github.com/vuejs/core/tree/main/packages/compiler-sfc) for parser details.

---

<div align="center">
  <sub>Powered by Sylphx</sub>
</div>

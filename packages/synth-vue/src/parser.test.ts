import { describe, it, expect } from 'bun:test'
import { parse, parseAsync, createParser, VueParser } from './parser.js'
import type { Tree } from '@sylphx/synth'

describe('VueParser', () => {
  describe('Basic SFC', () => {
    it('should parse simple SFC', () => {
      const vue = `<template>
  <div>Hello World</div>
</template>

<script>
export default {
  name: 'HelloWorld'
}
</script>

<style>
div {
  color: red;
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
      expect(tree.meta.language).toBe('vue')
      expect(tree.meta.source).toBe(vue)
      expect(Object.keys(tree.nodes).length).toBeGreaterThan(1)
    })

    it('should parse SFC with only template', () => {
      const vue = `<template>
  <div>Content</div>
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse SFC with only script', () => {
      const vue = `<script>
export default {
  name: 'Component'
}
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse empty SFC', () => {
      const vue = `<template></template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Template Block', () => {
    it('should parse template with expressions', () => {
      const vue = `<template>
  <div>{{ message }}</div>
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse template with v-if', () => {
      const vue = `<template>
  <div v-if="show">Visible</div>
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse template with v-for', () => {
      const vue = `<template>
  <ul>
    <li v-for="item in items" :key="item.id">{{ item.name }}</li>
  </ul>
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse template with v-bind', () => {
      const vue = `<template>
  <img :src="imageUrl" :alt="imageAlt" />
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse template with v-on', () => {
      const vue = `<template>
  <button @click="handleClick">Click me</button>
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse template with slots', () => {
      const vue = `<template>
  <div>
    <slot></slot>
    <slot name="footer"></slot>
  </div>
</template>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Script Block', () => {
    it('should parse Options API script', () => {
      const vue = `<template>
  <div>{{ count }}</div>
</template>

<script>
export default {
  data() {
    return {
      count: 0
    }
  },
  methods: {
    increment() {
      this.count++
    }
  }
}
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse script with TypeScript', () => {
      const vue = `<template>
  <div>{{ message }}</div>
</template>

<script lang="ts">
export default {
  name: 'Component',
  data(): { message: string } {
    return {
      message: 'Hello'
    }
  }
}
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse script with imports', () => {
      const vue = `<script>
import { ref } from 'vue'
import MyComponent from './MyComponent.vue'

export default {
  components: {
    MyComponent
  }
}
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Script Setup', () => {
    it('should parse script setup', () => {
      const vue = `<template>
  <div>{{ count }}</div>
</template>

<script setup>
import { ref } from 'vue'

const count = ref(0)
const increment = () => count.value++
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse script setup with TypeScript', () => {
      const vue = `<template>
  <div>{{ message }}</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const message = ref<string>('Hello')
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse script setup with defineProps', () => {
      const vue = `<script setup>
const props = defineProps({
  title: String,
  count: Number
})
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse script setup with defineEmits', () => {
      const vue = `<script setup>
const emit = defineEmits(['update', 'delete'])

const handleClick = () => {
  emit('update', { id: 1 })
}
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Style Block', () => {
    it('should parse single style block', () => {
      const vue = `<template>
  <div class="container">Content</div>
</template>

<style>
.container {
  padding: 20px;
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse scoped style', () => {
      const vue = `<template>
  <div class="component">Content</div>
</template>

<style scoped>
.component {
  color: blue;
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse style with SCSS', () => {
      const vue = `<template>
  <div class="button">Button</div>
</template>

<style lang="scss">
$primary-color: #007bff;

.button {
  background-color: $primary-color;

  &:hover {
    opacity: 0.8;
  }
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse multiple style blocks', () => {
      const vue = `<template>
  <div class="component">Content</div>
</template>

<style>
/* Global styles */
body {
  margin: 0;
}
</style>

<style scoped>
/* Component styles */
.component {
  padding: 10px;
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse style module', () => {
      const vue = `<template>
  <div :class="$style.red">Red text</div>
</template>

<style module>
.red {
  color: red;
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Custom Blocks', () => {
    it('should parse i18n custom block', () => {
      const vue = `<template>
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
</i18n>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse docs custom block', () => {
      const vue = `<template>
  <button>Click</button>
</template>

<docs>
# Button Component

A simple button component.
</docs>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Real-World Examples', () => {
    it('should parse counter component', () => {
      const vue = `<template>
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
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse form component', () => {
      const vue = `<template>
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
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse todo list component', () => {
      const vue = `<template>
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
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse modal component', () => {
      const vue = `<template>
  <Teleport to="body">
    <div v-if="isOpen" class="modal-overlay" @click="close">
      <div class="modal-content" @click.stop>
        <button class="close-button" @click="close">×</button>
        <slot></slot>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
defineProps({
  isOpen: Boolean
})

const emit = defineEmits(['close'])

const close = () => {
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-content {
  background: white;
  padding: 20px;
  border-radius: 8px;
  max-width: 500px;
  position: relative;
}

.close-button {
  position: absolute;
  top: 10px;
  right: 10px;
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
}
</style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('Edge Cases', () => {
    it('should parse SFC with comments', () => {
      const vue = `<!-- Component template -->
<template>
  <div>Content</div>
</template>

<!-- Component logic -->
<script>
export default {
  name: 'Component'
}
</script>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse SFC with empty blocks', () => {
      const vue = `<template></template>
<script></script>
<style></style>`

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })

    it('should parse SFC with whitespace', () => {
      const vue = `

<template>
  <div>Content</div>
</template>

<script>
export default {}
</script>

      `

      const tree = parse(vue)
      expect(tree).toBeDefined()
    })
  })

  describe('API', () => {
    it('should support standalone parse function', () => {
      const tree = parse('<template><div>Test</div></template>')
      expect(tree).toBeDefined()
      expect(tree.meta.language).toBe('vue')
    })

    it('should support async parsing', async () => {
      const tree = await parseAsync('<template><div>Test</div></template>')
      expect(tree).toBeDefined()
      expect(tree.meta.language).toBe('vue')
    })

    it('should support createParser factory', () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(VueParser)

      const tree = parser.parse('<template><div>Test</div></template>')
      expect(tree).toBeDefined()
    })

    it('should support VueParser class', () => {
      const parser = new VueParser()
      const tree = parser.parse('<template><div>Test</div></template>')
      expect(tree).toBeDefined()

      const retrieved = parser.getTree()
      expect(retrieved).toBe(tree)
    })

    it('should support plugins', () => {
      let transformed = false

      const plugin = {
        name: 'test-plugin',
        transform(tree: Tree) {
          transformed = true
          return tree
        },
      }

      parse('<template><div>Test</div></template>', { plugins: [plugin] })
      expect(transformed).toBe(true)
    })

    it('should support async plugins', async () => {
      let transformed = false

      const plugin = {
        name: 'async-plugin',
        async transform(tree: Tree) {
          await new Promise((resolve) => setTimeout(resolve, 10))
          transformed = true
          return tree
        },
      }

      await parseAsync('<template><div>Test</div></template>', { plugins: [plugin] })
      expect(transformed).toBe(true)
    })

    it('should throw error when using async plugin with sync parse', () => {
      const asyncPlugin = {
        name: 'async-plugin',
        async transform(tree: Tree) {
          return tree
        },
      }

      expect(() => {
        parse('<template><div>Test</div></template>', { plugins: [asyncPlugin] })
      }).toThrow('Detected async plugins')
    })

    it('should support use() method for plugins', () => {
      let count = 0

      const plugin1 = {
        name: 'plugin1',
        transform(tree: Tree) {
          count++
          return tree
        },
      }

      const plugin2 = {
        name: 'plugin2',
        transform(tree: Tree) {
          count++
          return tree
        },
      }

      const parser = new VueParser()
      parser.use(plugin1).use(plugin2)
      parser.parse('<template><div>Test</div></template>')

      expect(count).toBe(2)
    })
  })

  describe('Parser Options', () => {
    it('should parse with filename option', () => {
      const vue = '<template><div>Test</div></template>'

      const tree = parse(vue, { filename: 'TestComponent.vue' })
      expect(tree).toBeDefined()
    })

    it('should parse with source map option', () => {
      const vue = '<template><div>Test</div></template>'

      const tree = parse(vue, { sourceMap: true })
      expect(tree).toBeDefined()
    })
  })
})

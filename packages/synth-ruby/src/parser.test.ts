/**
 * Ruby Parser Tests (WASM-based)
 */

import { describe, expect, it } from 'bun:test'
import { createParser, init, parse, parseAsync, RubyParser } from './parser.js'

describe('RubyParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty Ruby', async () => {
      const tree = await parseAsync('')
      expect(tree.meta.language).toBe('ruby')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse simple puts', async () => {
      const ruby = 'puts "Hello, World!"'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')
      expect(tree.nodes[tree.root]).toBeDefined()

      // Should have program root and children
      const rootChildren = tree.nodes[tree.root]?.children
      expect(rootChildren.length).toBeGreaterThan(0)
    })

    it('should parse variable assignment', async () => {
      const ruby = 'x = 42'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find assignment
      const assignNode = tree.nodes.find((n) => n.type.includes('Assignment'))
      expect(assignNode).toBeDefined()
    })

    it('should parse method definition', async () => {
      const ruby = `
def greet(name)
  "Hello, #{name}!"
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find method definition
      const methodNode = tree.nodes.find((n) => n.type === 'Method' || n.type.includes('Method'))
      expect(methodNode).toBeDefined()
    })

    it('should parse class definition', async () => {
      const ruby = `
class Person
  def initialize(name)
    @name = name
  end
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find class
      const classNode = tree.nodes.find((n) => n.type === 'Class')
      expect(classNode).toBeDefined()
    })
  })

  describe('Data Types', () => {
    it('should parse string literals', async () => {
      const ruby = 'text = "Hello, World!"'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find string node
      const stringNode = tree.nodes.find((n) => n.type === 'String' || n.type.includes('String'))
      expect(stringNode).toBeDefined()
    })

    it('should parse single quoted strings', async () => {
      const ruby = "text = 'Hello'"
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find string node
      const stringNode = tree.nodes.find((n) => n.type === 'String' || n.type.includes('String'))
      expect(stringNode).toBeDefined()
    })

    it('should parse string interpolation', async () => {
      const ruby = 'name = "World"; greeting = "Hello, #{name}!"'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find string with interpolation
      const interpolNode = tree.nodes.find((n) => n.type.includes('Interpolation'))
      expect(interpolNode).toBeDefined()
    })

    it('should parse symbols', async () => {
      const ruby = 'status = :pending'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find symbol
      const symbolNode = tree.nodes.find((n) => n.type === 'Symbol' || n.data?.text?.startsWith(':'))
      expect(symbolNode).toBeDefined()
    })

    it('should parse integer literals', async () => {
      const ruby = 'num = 42'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find integer node
      const intNode = tree.nodes.find((n) => n.type === 'Integer' || n.data?.text === '42')
      expect(intNode).toBeDefined()
    })

    it('should parse float literals', async () => {
      const ruby = 'pi = 3.14159'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find float node
      const floatNode = tree.nodes.find((n) => n.type === 'Float' || n.data?.text?.includes('.'))
      expect(floatNode).toBeDefined()
    })

    it('should parse boolean literals', async () => {
      const ruby = `
flag1 = true
flag2 = false
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find boolean nodes
      const boolNodes = tree.nodes.filter(
        (n) => n.type === 'True' || n.type === 'False' || n.data?.text === 'true' || n.data?.text === 'false'
      )
      expect(boolNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse arrays', async () => {
      const ruby = 'numbers = [1, 2, 3, 4, 5]'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find array
      const arrayNode = tree.nodes.find((n) => n.type === 'Array')
      expect(arrayNode).toBeDefined()
    })

    it('should parse hashes', async () => {
      const ruby = 'person = { name: "John", age: 30 }'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find hash
      const hashNode = tree.nodes.find((n) => n.type === 'Hash')
      expect(hashNode).toBeDefined()
    })

    it('should parse ranges', async () => {
      const ruby = 'range = 1..10'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find range
      const rangeNode = tree.nodes.find((n) => n.type === 'Range' || n.data?.text?.includes('..'))
      expect(rangeNode).toBeDefined()
    })

    it('should parse nil', async () => {
      const ruby = 'value = nil'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find nil literal
      const nilNode = tree.nodes.find((n) => n.type === 'Nil' || n.data?.text === 'nil')
      expect(nilNode).toBeDefined()
    })
  })

  describe('Control Flow', () => {
    it('should parse if statement', async () => {
      const ruby = `
if x > 0
  puts "positive"
elsif x < 0
  puts "negative"
else
  puts "zero"
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find if statement
      const ifNode = tree.nodes.find((n) => n.type === 'If' || n.type === 'IfStatement')
      expect(ifNode).toBeDefined()
    })

    it('should parse unless statement', async () => {
      const ruby = `
unless x.nil?
  puts x
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find unless statement
      const unlessNode = tree.nodes.find((n) => n.type === 'Unless')
      expect(unlessNode).toBeDefined()
    })

    it('should parse while loop', async () => {
      const ruby = `
while x < 10
  x += 1
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find while statement
      const whileNode = tree.nodes.find((n) => n.type === 'While')
      expect(whileNode).toBeDefined()
    })

    it('should parse until loop', async () => {
      const ruby = `
until x >= 10
  x += 1
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find until statement
      const untilNode = tree.nodes.find((n) => n.type === 'Until')
      expect(untilNode).toBeDefined()
    })

    it('should parse for loop', async () => {
      const ruby = `
for i in 1..10
  puts i
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find for statement
      const forNode = tree.nodes.find((n) => n.type === 'For')
      expect(forNode).toBeDefined()
    })

    it('should parse case statement', async () => {
      const ruby = `
case day
when "Monday"
  puts "Start of week"
when "Friday"
  puts "End of week"
else
  puts "Midweek"
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find case statement
      const caseNode = tree.nodes.find((n) => n.type === 'Case')
      expect(caseNode).toBeDefined()
    })

    it('should parse begin-rescue-end', async () => {
      const ruby = `
begin
  risky_operation
rescue StandardError => e
  puts "Error: #{e.message}"
ensure
  cleanup
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find begin statement
      const beginNode = tree.nodes.find((n) => n.type === 'Begin')
      expect(beginNode).toBeDefined()
    })
  })

  describe('Methods', () => {
    it('should parse method with parameters', async () => {
      const ruby = `
def add(a, b)
  a + b
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find method definition
      const methodNode = tree.nodes.find((n) => n.type === 'Method' || n.type.includes('Method'))
      expect(methodNode).toBeDefined()
    })

    it('should parse method with default parameters', async () => {
      const ruby = `
def greet(name = "World")
  "Hello, #{name}!"
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find method definition
      const methodNode = tree.nodes.find((n) => n.type === 'Method' || n.type.includes('Method'))
      expect(methodNode).toBeDefined()
    })

    it('should parse method with splat operator', async () => {
      const ruby = `
def sum(*numbers)
  numbers.reduce(0, :+)
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find method definition
      const methodNode = tree.nodes.find((n) => n.type === 'Method' || n.type.includes('Method'))
      expect(methodNode).toBeDefined()
    })

    it('should parse method with keyword arguments', async () => {
      const ruby = `
def greet(name:, greeting: "Hello")
  "#{greeting}, #{name}!"
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find method definition
      const methodNode = tree.nodes.find((n) => n.type === 'Method' || n.type.includes('Method'))
      expect(methodNode).toBeDefined()
    })

    it('should parse method with block parameter', async () => {
      const ruby = `
def apply(&block)
  block.call
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find method definition
      const methodNode = tree.nodes.find((n) => n.type === 'Method' || n.type.includes('Method'))
      expect(methodNode).toBeDefined()
    })
  })

  describe('Blocks', () => {
    it('should parse block with do-end', async () => {
      const ruby = `
[1, 2, 3].each do |n|
  puts n
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find block
      const blockNode = tree.nodes.find((n) => n.type === 'Block' || n.type === 'DoBlock')
      expect(blockNode).toBeDefined()
    })

    it('should parse block with braces', async () => {
      const ruby = '[1, 2, 3].each { |n| puts n }'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find block
      const blockNode = tree.nodes.find((n) => n.type === 'Block')
      expect(blockNode).toBeDefined()
    })

    it('should parse block parameters', async () => {
      const ruby = `
hash.each do |key, value|
  puts "#{key}: #{value}"
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find block with parameters
      const blockNode = tree.nodes.find((n) => n.type === 'Block' || n.type === 'DoBlock')
      expect(blockNode).toBeDefined()
    })

    it('should parse proc', async () => {
      const ruby = 'double = Proc.new { |x| x * 2 }'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Should parse successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse lambda', async () => {
      const ruby = 'double = lambda { |x| x * 2 }'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Should parse successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse stabby lambda', async () => {
      const ruby = 'double = ->(x) { x * 2 }'
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find lambda
      const lambdaNode = tree.nodes.find((n) => n.type === 'Lambda' || n.data?.text?.includes('->'))
      expect(lambdaNode).toBeDefined()
    })
  })

  describe('Classes', () => {
    it('should parse class with methods', async () => {
      const ruby = `
class Calculator
  def add(a, b)
    a + b
  end

  def subtract(a, b)
    a - b
  end
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find class
      const classNode = tree.nodes.find((n) => n.type === 'Class')
      expect(classNode).toBeDefined()
    })

    it('should parse initialize method', async () => {
      const ruby = `
class Person
  def initialize(name)
    @name = name
  end
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find initialize method
      const initNode = tree.nodes.find((n) => n.data?.text?.includes('initialize'))
      expect(initNode).toBeDefined()
    })

    it('should parse instance variables', async () => {
      const ruby = `
class Person
  def set_name(name)
    @name = name
  end
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find instance variable
      const ivarNode = tree.nodes.find((n) => n.type.includes('Instance') || n.data?.text?.startsWith('@'))
      expect(ivarNode).toBeDefined()
    })

    it('should parse class variables', async () => {
      const ruby = `
class Counter
  @@count = 0
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find class variable
      const cvarNode = tree.nodes.find((n) => n.data?.text?.startsWith('@@'))
      expect(cvarNode).toBeDefined()
    })

    it('should parse class methods', async () => {
      const ruby = `
class Math
  def self.square(x)
    x * x
  end
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find class method
      const methodNode = tree.nodes.find((n) => n.data?.text?.includes('self'))
      expect(methodNode).toBeDefined()
    })

    it('should parse inheritance', async () => {
      const ruby = `
class Dog < Animal
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find class with inheritance
      const classNode = tree.nodes.find((n) => n.type === 'Class')
      expect(classNode).toBeDefined()
    })

    it('should parse module', async () => {
      const ruby = `
module Greetable
  def greet
    puts "Hello!"
  end
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find module
      const moduleNode = tree.nodes.find((n) => n.type === 'Module')
      expect(moduleNode).toBeDefined()
    })

    it('should parse attr_accessor', async () => {
      const ruby = `
class Person
  attr_accessor :name, :age
end
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find attr_accessor call
      const attrNode = tree.nodes.find((n) => n.data?.text?.includes('attr_accessor'))
      expect(attrNode).toBeDefined()
    })
  })

  describe('Comments', () => {
    it('should parse line comments', async () => {
      const ruby = `
# This is a comment
x = 42
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Find comment node
      const commentNode = tree.nodes.find((n) => n.type.includes('Comment') || n.data?.text?.includes('#'))
      expect(commentNode).toBeDefined()
    })

    it('should parse block comments', async () => {
      const ruby = `
=begin
This is a
multi-line comment
=end
x = 42
      `
      const tree = await parseAsync(ruby)

      expect(tree.meta.language).toBe('ruby')

      // Should have parsed successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })
  })

  describe('API', () => {
    it('should create parser with factory', async () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(RubyParser)
    })

    it('should parse with standalone function', async () => {
      const tree = await parseAsync('x = 42')
      expect(tree.meta.language).toBe('ruby')
    })

    it('should parse async', async () => {
      const tree = await parseAsync('x = 42')
      expect(tree.meta.language).toBe('ruby')
    })

    it('should throw error for synchronous parse()', () => {
      expect(() => {
        parse('x = 42')
      }).toThrow(/WASM/)
    })

    it('should support getTree method', async () => {
      const parser = new RubyParser()
      const tree1 = await parser.parseAsync('x = 42')
      const tree2 = parser.getTree()

      expect(tree2).toBe(tree1)
    })

    it('should support plugin system with use()', async () => {
      const parser = new RubyParser()

      const plugin = {
        name: 'test-plugin',
        transform: (tree: any) => {
          tree.metadata = { processed: true }
          return tree
        },
      }

      parser.use(plugin)
      const tree = await parser.parseAsync('x = 42')

      expect(tree.metadata).toEqual({ processed: true })
    })

    it('should apply plugins from options', async () => {
      const plugin = {
        name: 'options-plugin',
        transform: (tree: any) => {
          tree.metadata = { fromOptions: true }
          return tree
        },
      }

      const tree = await parseAsync('x = 42', { plugins: [plugin] })

      expect(tree.metadata).toEqual({ fromOptions: true })
    })

    it('should support async plugins with parseAsync', async () => {
      const asyncPlugin = {
        name: 'async-plugin',
        transform: async (tree: any) => {
          await new Promise((resolve) => setTimeout(resolve, 1))
          tree.metadata = { async: true }
          return tree
        },
      }

      const tree = await parseAsync('x = 42', { plugins: [asyncPlugin] })

      expect(tree.metadata).toEqual({ async: true })
    })

    it('should support init() for pre-initialization', async () => {
      // init() should not throw
      await init()
      // Second call should be instant (cached)
      await init()
    })
  })
})

/**
 * Python Parser Tests
 */

import { describe, it, expect } from 'vitest'
import { parse, parseAsync, createParser, PythonParser } from './parser.js'

describe('PythonParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty Python', () => {
      const tree = parse('')
      expect(tree.meta.language).toBe('python')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse simple variable assignment', () => {
      const python = 'x = 42'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes[tree.root]).toBeDefined()

      // Should have module root and children
      const rootChildren = tree.nodes[tree.root]!.children
      expect(rootChildren.length).toBeGreaterThan(0)
    })

    it('should parse function definition', () => {
      const python = `
def hello():
    return "Hello, World!"
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find function definition node
      const funcNode = tree.nodes.find(n => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse class definition', () => {
      const python = `
class MyClass:
    def __init__(self):
        self.value = 0
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find class definition node
      const classNode = tree.nodes.find(n => n.type === 'ClassDefinition')
      expect(classNode).toBeDefined()
    })
  })

  describe('Data Types', () => {
    it('should parse string literals', () => {
      const python = 'text = "Hello, World!"'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find string node
      const stringNode = tree.nodes.find(n => n.type === 'String')
      expect(stringNode).toBeDefined()
    })

    it('should parse integer literals', () => {
      const python = 'num = 42'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find integer node
      const intNode = tree.nodes.find(n => n.type === 'Integer')
      expect(intNode).toBeDefined()
    })

    it('should parse float literals', () => {
      const python = 'pi = 3.14159'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find float node
      const floatNode = tree.nodes.find(n => n.type === 'Float')
      expect(floatNode).toBeDefined()
    })

    it('should parse boolean literals', () => {
      const python = `
flag1 = True
flag2 = False
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find boolean nodes
      const boolNodes = tree.nodes.filter(n => n.type === 'True' || n.type === 'False')
      expect(boolNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse list literals', () => {
      const python = 'items = [1, 2, 3, 4, 5]'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find list node
      const listNode = tree.nodes.find(n => n.type === 'List')
      expect(listNode).toBeDefined()
    })

    it('should parse dict literals', () => {
      const python = 'data = {"key": "value", "number": 42}'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find dictionary node
      const dictNode = tree.nodes.find(n => n.type === 'Dictionary')
      expect(dictNode).toBeDefined()
    })
  })

  describe('Control Flow', () => {
    it('should parse if statement', () => {
      const python = `
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find if statement
      const ifNode = tree.nodes.find(n => n.type === 'IfStatement')
      expect(ifNode).toBeDefined()
    })

    it('should parse for loop', () => {
      const python = `
for i in range(10):
    print(i)
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find for statement
      const forNode = tree.nodes.find(n => n.type === 'ForStatement')
      expect(forNode).toBeDefined()
    })

    it('should parse while loop', () => {
      const python = `
while x < 10:
    x += 1
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find while statement
      const whileNode = tree.nodes.find(n => n.type === 'WhileStatement')
      expect(whileNode).toBeDefined()
    })

    it('should parse try-except', () => {
      const python = `
try:
    risky_operation()
except ValueError:
    print("Value error occurred")
finally:
    cleanup()
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      // Find try statement
      const tryNode = tree.nodes.find(n => n.type === 'TryStatement')
      expect(tryNode).toBeDefined()
    })
  })

  describe('Functions', () => {
    it('should parse function with parameters', () => {
      const python = `
def greet(name, greeting="Hello"):
    return f"{greeting}, {name}!"
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const funcNode = tree.nodes.find(n => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse function with type hints', () => {
      const python = `
def add(a: int, b: int) -> int:
    return a + b
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const funcNode = tree.nodes.find(n => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse lambda functions', () => {
      const python = 'square = lambda x: x ** 2'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const lambdaNode = tree.nodes.find(n => n.type === 'Lambda')
      expect(lambdaNode).toBeDefined()
    })

    it('should parse decorators', () => {
      const python = `
@staticmethod
def utility_function():
    pass
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const decoratorNode = tree.nodes.find(n => n.type === 'Decorator' || n.type === 'DecoratedDefinition')
      expect(decoratorNode).toBeDefined()
    })
  })

  describe('Classes', () => {
    it('should parse class with methods', () => {
      const python = `
class Calculator:
    def add(self, a, b):
        return a + b

    def multiply(self, a, b):
        return a * b
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const classNode = tree.nodes.find(n => n.type === 'ClassDefinition')
      expect(classNode).toBeDefined()
    })

    it('should parse class inheritance', () => {
      const python = `
class Animal:
    pass

class Dog(Animal):
    pass
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const classNodes = tree.nodes.filter(n => n.type === 'ClassDefinition')
      expect(classNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse class with __init__', () => {
      const python = `
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const classNode = tree.nodes.find(n => n.type === 'ClassDefinition')
      expect(classNode).toBeDefined()
    })
  })

  describe('Imports', () => {
    it('should parse import statement', () => {
      const python = 'import os'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const importNode = tree.nodes.find(n => n.type === 'ImportStatement')
      expect(importNode).toBeDefined()
    })

    it('should parse from import statement', () => {
      const python = 'from os import path'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const fromImportNode = tree.nodes.find(n => n.type === 'ImportFromStatement')
      expect(fromImportNode).toBeDefined()
    })

    it('should parse import with alias', () => {
      const python = 'import numpy as np'
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')

      const importNode = tree.nodes.find(n => n.type === 'ImportStatement')
      expect(importNode).toBeDefined()
    })
  })

  describe('Real-World Examples', () => {
    it('should parse Flask application', () => {
      const python = `
from flask import Flask, request

app = Flask(__name__)

@app.route('/')
def index():
    return "Hello, World!"

@app.route('/api/users', methods=['GET', 'POST'])
def users():
    if request.method == 'POST':
        return {"status": "created"}, 201
    return {"users": []}, 200

if __name__ == '__main__':
    app.run(debug=True)
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes.length).toBeGreaterThan(10)
    })

    it('should parse data processing script', () => {
      const python = `
import pandas as pd
import numpy as np

def process_data(filename):
    df = pd.read_csv(filename)
    df['processed'] = df['value'].apply(lambda x: x * 2)
    return df.groupby('category').mean()

if __name__ == '__main__':
    result = process_data('data.csv')
    print(result)
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes.length).toBeGreaterThan(10)
    })

    it('should parse class-based program', () => {
      const python = `
class BankAccount:
    def __init__(self, balance=0):
        self._balance = balance

    def deposit(self, amount):
        if amount > 0:
            self._balance += amount
            return True
        return False

    def withdraw(self, amount):
        if 0 < amount <= self._balance:
            self._balance -= amount
            return True
        return False

    @property
    def balance(self):
        return self._balance

account = BankAccount(1000)
account.deposit(500)
print(account.balance)
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes.length).toBeGreaterThan(20)
    })
  })

  describe('API', () => {
    it('should work with createParser factory', () => {
      const parser = createParser()
      const tree = parser.parse('x = 42')

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should work with PythonParser class', () => {
      const parser = new PythonParser()
      const tree = parser.parse('x = 42')

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should work with standalone parse function', () => {
      const tree = parse('x = 42')

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should work with parseAsync function', async () => {
      const tree = await parseAsync('x = 42')

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should support getTree method', () => {
      const parser = new PythonParser()
      const tree1 = parser.parse('x = 42')
      const tree2 = parser.getTree()

      expect(tree2).toBe(tree1)
    })

    it('should support plugin system with use()', () => {
      const parser = new PythonParser()

      const plugin = {
        name: 'test-plugin',
        transform: (tree: any) => {
          tree.metadata = { processed: true }
          return tree
        },
      }

      parser.use(plugin)
      const tree = parser.parse('x = 42')

      expect(tree.metadata).toEqual({ processed: true })
    })

    it('should apply plugins from options', () => {
      const plugin = {
        name: 'options-plugin',
        transform: (tree: any) => {
          tree.metadata = { fromOptions: true }
          return tree
        },
      }

      const tree = parse('x = 42', { plugins: [plugin] })

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

    it('should throw error if async plugin used in sync parse', () => {
      const asyncPlugin = {
        name: 'async-plugin',
        transform: async (tree: any) => tree,
      }

      expect(() => {
        parse('x = 42', { plugins: [asyncPlugin] })
      }).toThrow(/async plugins/i)
    })
  })

  describe('Edge Cases', () => {
    it('should handle comments', () => {
      const python = `
# This is a comment
x = 42  # Inline comment
"""
Multi-line
comment
"""
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should handle multiline strings', () => {
      const python = `
text = """
This is a
multi-line
string
"""
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
    })

    it('should handle indentation', () => {
      const python = `
def outer():
    def inner():
        return 42
    return inner()
      `
      const tree = parse(python)

      expect(tree.meta.language).toBe('python')
    })
  })
})

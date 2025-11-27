/**
 * Parser Coverage Tests - Cover remaining uncovered lines
 */

import { describe, expect, it } from 'bun:test'
import { createTransformPlugin, SynthError, TreeStructureError } from '@sylphx/synth'
import { createParser, Parser, parse, parseAsync } from './parser.js'

describe('Parser Coverage', () => {
  describe('createParser factory function', () => {
    it('should create a new Parser instance', () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(Parser)
    })

    it('should create independent instances', () => {
      const parser1 = createParser()
      const parser2 = createParser()
      expect(parser1).not.toBe(parser2)
    })
  })

  describe('parseAsync standalone function', () => {
    it('should parse markdown asynchronously', async () => {
      const tree = await parseAsync('# Hello World')
      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should accept options', async () => {
      const tree = await parseAsync('# Hello', {
        buildIndex: true,
        useNodePool: false,
      })
      expect(tree).toBeDefined()
    })

    it('should work with async plugins', async () => {
      const asyncPlugin = createTransformPlugin(
        {
          name: 'async-test',
          version: '1.0.0',
        },
        async (tree) => {
          // Simulate async operation
          await new Promise((resolve) => setTimeout(resolve, 1))
          return tree
        }
      )

      const tree = await parseAsync('# Test', {
        plugins: [asyncPlugin],
      })

      expect(tree).toBeDefined()
    })
  })

  describe('Parser.use() method', () => {
    it('should register a plugin', () => {
      const parser = new Parser()
      const plugin = createTransformPlugin({ name: 'test', version: '1.0.0' }, (tree) => tree)

      const result = parser.use(plugin)
      expect(result).toBe(parser) // Should return this for chaining
    })

    it('should allow chaining', () => {
      const parser = new Parser()
      const plugin1 = createTransformPlugin({ name: 'test1', version: '1.0.0' }, (tree) => tree)
      const plugin2 = createTransformPlugin({ name: 'test2', version: '1.0.0' }, (tree) => tree)

      const result = parser.use(plugin1).use(plugin2)
      expect(result).toBe(parser)
    })

    it('should apply registered plugins on parse', () => {
      const parser = new Parser()

      let pluginWasCalled = false
      const plugin = createTransformPlugin({ name: 'test', version: '1.0.0' }, (tree) => {
        pluginWasCalled = true
        return tree
      })

      parser.use(plugin)
      parser.parse('# Test')

      expect(pluginWasCalled).toBe(true)
    })
  })

  describe('Parser.getTree() method', () => {
    it('should return null before parsing', () => {
      const parser = new Parser()
      expect(parser.getTree()).toBeNull()
    })

    it('should return tree after parsing', () => {
      const parser = new Parser()
      const tree = parser.parse('# Hello')

      expect(parser.getTree()).toBe(tree)
      expect(parser.getTree()).not.toBeNull()
    })

    it('should return updated tree after second parse', () => {
      const parser = new Parser()
      const tree1 = parser.parse('# First')
      const tree2 = parser.parse('# Second')

      expect(parser.getTree()).toBe(tree2)
      expect(parser.getTree()).not.toBe(tree1)
    })
  })

  describe('Parser.parseIncremental() method', () => {
    it('should throw error if called before parse', () => {
      const parser = new Parser()

      expect(() => {
        parser.parseIncremental('# New text', {
          startIndex: 0,
          oldEndIndex: 0,
          newEndIndex: 10,
        })
      }).toThrow(TreeStructureError)

      expect(() => {
        parser.parseIncremental('# New text', {
          startIndex: 0,
          oldEndIndex: 0,
          newEndIndex: 10,
        })
      }).toThrow('Must call parse() before parseIncremental()')
    })

    it('should work after initial parse', () => {
      const parser = new Parser()
      parser.parse('# Original')

      const tree = parser.parseIncremental('# Modified', {
        startIndex: 2,
        oldEndIndex: 10,
        newEndIndex: 10,
      })

      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should accept options', () => {
      const parser = new Parser()
      parser.parse('# Original')

      const tree = parser.parseIncremental(
        '# Modified',
        {
          startIndex: 0,
          oldEndIndex: 10,
          newEndIndex: 10,
        },
        {
          buildIndex: true,
          useNodePool: false,
        }
      )

      expect(tree).toBeDefined()
    })
  })

  describe('Async plugin detection in sync parse', () => {
    it('should detect async plugins in sync parse and throw error', () => {
      const parser = new Parser()

      const asyncPlugin = createTransformPlugin(
        {
          name: 'async-plugin',
          version: '1.0.0',
        },
        async (tree) => {
          await new Promise((resolve) => setTimeout(resolve, 1))
          return tree
        }
      )

      expect(() => {
        parser.parse('# Test', {
          plugins: [asyncPlugin],
        })
      }).toThrow(SynthError)

      expect(() => {
        parser.parse('# Test', {
          plugins: [asyncPlugin],
        })
      }).toThrow('Detected async plugins. Use parseAsync() instead of parse()')
    })

    it('should detect async plugins in registered plugins', () => {
      const parser = new Parser()

      const asyncPlugin = createTransformPlugin(
        {
          name: 'async-registered',
          version: '1.0.0',
        },
        async (tree) => {
          await new Promise((resolve) => setTimeout(resolve, 1))
          return tree
        }
      )

      parser.use(asyncPlugin)

      expect(() => {
        parser.parse('# Test')
      }).toThrow(SynthError)

      expect(() => {
        parser.parse('# Test')
      }).toThrow('Use parseAsync()')
    })

    it('should allow sync plugins in sync parse', () => {
      const parser = new Parser()

      const syncPlugin = createTransformPlugin(
        {
          name: 'sync-plugin',
          version: '1.0.0',
        },
        (tree) => tree // Sync function
      )

      expect(() => {
        parser.parse('# Test', {
          plugins: [syncPlugin],
        })
      }).not.toThrow()
    })

    it('should merge registered and one-off plugins', () => {
      const parser = new Parser()

      let registeredCalled = false
      let oneOffCalled = false

      const registeredPlugin = createTransformPlugin({ name: 'registered', version: '1.0.0' }, (tree) => {
        registeredCalled = true
        return tree
      })

      const oneOffPlugin = createTransformPlugin({ name: 'one-off', version: '1.0.0' }, (tree) => {
        oneOffCalled = true
        return tree
      })

      parser.use(registeredPlugin)
      parser.parse('# Test', {
        plugins: [oneOffPlugin],
      })

      expect(registeredCalled).toBe(true)
      expect(oneOffCalled).toBe(true)
    })
  })

  describe('Parser.getIndex() edge cases', () => {
    it('should throw error when called before parse', () => {
      const parser = new Parser()

      expect(() => {
        parser.getIndex()
      }).toThrow(TreeStructureError)

      expect(() => {
        parser.getIndex()
      }).toThrow('No tree available. Call parse() first.')
    })

    it('should build index lazily if not exists', () => {
      const parser = new Parser()
      parser.parse('# Test') // Parse without buildIndex option

      const index = parser.getIndex()
      expect(index).toBeDefined()
    })

    it('should return existing index if already built', () => {
      const parser = new Parser()
      parser.parse('# Test', { buildIndex: true })

      const index1 = parser.getIndex()
      const index2 = parser.getIndex()

      expect(index1).toBe(index2) // Should be same instance
    })
  })

  describe('parse standalone function', () => {
    it('should parse markdown', () => {
      const tree = parse('# Hello World')
      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should accept options', () => {
      const tree = parse('# Test', {
        buildIndex: true,
        useNodePool: false,
      })
      expect(tree).toBeDefined()
    })

    it('should work with sync plugins', () => {
      let wasCalled = false
      const plugin = createTransformPlugin({ name: 'test', version: '1.0.0' }, (tree) => {
        wasCalled = true
        return tree
      })

      parse('# Test', { plugins: [plugin] })
      expect(wasCalled).toBe(true)
    })

    it('should parse empty string', () => {
      const tree = parse('')
      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThanOrEqual(1)
    })
  })
})

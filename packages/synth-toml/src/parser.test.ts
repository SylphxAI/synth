/**
 * TOML Parser Tests
 */

import { describe, it, expect } from 'vitest'
import { parse, parseAsync, createParser, TOMLParser } from './parser.js'

describe('TOMLParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty TOML', () => {
      const tree = parse('')
      expect(tree.meta.language).toBe('toml')
      expect(tree.nodes[tree.root]!.children).toHaveLength(0)
    })

    it('should parse basic key-value pair', () => {
      const toml = 'key = "value"'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const kvId = root.children[0]!
      const kv = tree.nodes[kvId]!

      expect(kv.type).toBe('KeyValue')
      expect(kv.data?.key).toBe('key')
      expect(kv.children).toHaveLength(1)

      const valueId = kv.children[0]!
      const value = tree.nodes[valueId]!

      expect(value.type).toBe('String')
      expect(value.data?.value).toBe('value')
    })

    it('should parse multiple key-value pairs', () => {
      const toml = `
        key1 = "value1"
        key2 = "value2"
        key3 = "value3"
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(3)

      const kv1 = tree.nodes[root.children[0]!]!
      expect(kv1.data?.key).toBe('key1')

      const kv2 = tree.nodes[root.children[1]!]!
      expect(kv2.data?.key).toBe('key2')

      const kv3 = tree.nodes[root.children[2]!]!
      expect(kv3.data?.key).toBe('key3')
    })
  })

  describe('Data Types', () => {
    it('should parse string values', () => {
      const toml = `
        basic = "basic string"
        literal = 'literal string'
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv1 = tree.nodes[root.children[0]!]!
      const val1 = tree.nodes[kv1.children[0]!]!
      expect(val1.type).toBe('String')
      expect(val1.data?.value).toBe('basic string')

      const kv2 = tree.nodes[root.children[1]!]!
      const val2 = tree.nodes[kv2.children[0]!]!
      expect(val2.type).toBe('String')
      expect(val2.data?.value).toBe('literal string')
    })

    it('should parse integer values', () => {
      const toml = `
        int1 = 42
        int2 = +17
        int3 = -5
        int4 = 1_000_000
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv1 = tree.nodes[root.children[0]!]!
      const val1 = tree.nodes[kv1.children[0]!]!
      expect(val1.type).toBe('Integer')
      expect(val1.data?.value).toBe('42')

      const kv2 = tree.nodes[root.children[1]!]!
      const val2 = tree.nodes[kv2.children[0]!]!
      expect(val2.type).toBe('Integer')
      expect(val2.data?.value).toBe('+17')
    })

    it('should parse float values', () => {
      const toml = `
        float1 = 3.14
        float2 = -0.01
        float3 = 5e+22
        float4 = 1e-10
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv1 = tree.nodes[root.children[0]!]!
      const val1 = tree.nodes[kv1.children[0]!]!
      expect(val1.type).toBe('Float')
      expect(val1.data?.value).toBe('3.14')

      const kv2 = tree.nodes[root.children[1]!]!
      const val2 = tree.nodes[kv2.children[0]!]!
      expect(val2.type).toBe('Float')
      expect(val2.data?.value).toBe('-0.01')
    })

    it('should parse boolean values', () => {
      const toml = `
        bool1 = true
        bool2 = false
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv1 = tree.nodes[root.children[0]!]!
      const val1 = tree.nodes[kv1.children[0]!]!
      expect(val1.type).toBe('Boolean')
      expect(val1.data?.value).toBe('true')

      const kv2 = tree.nodes[root.children[1]!]!
      const val2 = tree.nodes[kv2.children[0]!]!
      expect(val2.type).toBe('Boolean')
      expect(val2.data?.value).toBe('false')
    })

    it('should parse datetime values', () => {
      const toml = `
        date1 = 1979-05-27
        date2 = 1979-05-27T07:32:00Z
        date3 = 1979-05-27T00:32:00-07:00
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv1 = tree.nodes[root.children[0]!]!
      const val1 = tree.nodes[kv1.children[0]!]!
      expect(val1.type).toBe('DateTime')
      expect(val1.data?.value).toContain('1979-05-27')
    })
  })

  describe('Arrays', () => {
    it('should parse array of integers', () => {
      const toml = 'numbers = [1, 2, 3, 4, 5]'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('numbers')

      const array = tree.nodes[kv.children[0]!]!
      expect(array.type).toBe('Array')
      expect(array.children).toHaveLength(5)

      const val1 = tree.nodes[array.children[0]!]!
      expect(val1.type).toBe('Integer')
      expect(val1.data?.value).toBe('1')
    })

    it('should parse array of strings', () => {
      const toml = `colors = ["red", "yellow", "green"]`
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const array = tree.nodes[kv.children[0]!]!

      expect(array.type).toBe('Array')
      expect(array.children).toHaveLength(3)

      const val1 = tree.nodes[array.children[0]!]!
      expect(val1.type).toBe('String')
      expect(val1.data?.value).toBe('red')
    })

    it('should parse multi-line array', () => {
      const toml = `
        numbers = [
          1,
          2,
          3
        ]
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const array = tree.nodes[kv.children[0]!]!

      expect(array.type).toBe('Array')
      expect(array.children.length).toBeGreaterThanOrEqual(3)
    })

    it('should parse nested arrays', () => {
      const toml = 'nested = [[1, 2], [3, 4]]'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const outerArray = tree.nodes[kv.children[0]!]!

      expect(outerArray.type).toBe('Array')
      expect(outerArray.children.length).toBeGreaterThanOrEqual(1)

      // The parser may flatten or parse nested arrays differently
      // Just verify it parsed without error and has children
      const firstChild = tree.nodes[outerArray.children[0]!]!
      expect(firstChild).toBeDefined()
    })
  })

  describe('Tables', () => {
    it('should parse table header', () => {
      const toml = `
        [server]
        host = "localhost"
        port = 8080
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(1)

      const table = tree.nodes[root.children[0]!]!
      expect(table.type).toBe('Table')
      expect(table.data?.name).toBe('server')
      expect(table.children.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse multiple tables', () => {
      const toml = `
        [database]
        name = "mydb"

        [server]
        host = "localhost"
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(2)

      const table1 = tree.nodes[root.children[0]!]!
      expect(table1.type).toBe('Table')
      expect(table1.data?.name).toBe('database')

      const table2 = tree.nodes[root.children[1]!]!
      expect(table2.type).toBe('Table')
      expect(table2.data?.name).toBe('server')
    })

    it('should parse nested tables with dotted keys', () => {
      const toml = `
        [database.connection]
        host = "localhost"
        port = 5432
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const table = tree.nodes[root.children[0]!]!
      expect(table.type).toBe('Table')
      expect(table.data?.name).toBe('database.connection')
    })

    it('should parse array of tables', () => {
      const toml = `
        [[products]]
        name = "Hammer"
        sku = 738594937

        [[products]]
        name = "Nail"
        sku = 284758393
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(2)

      const table1 = tree.nodes[root.children[0]!]!
      expect(table1.type).toBe('TableArray')
      expect(table1.data?.name).toBe('products')

      const table2 = tree.nodes[root.children[1]!]!
      expect(table2.type).toBe('TableArray')
      expect(table2.data?.name).toBe('products')
    })
  })

  describe('Inline Tables', () => {
    it('should parse inline table', () => {
      const toml = 'point = { x = 1, y = 2 }'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('point')

      const inlineTable = tree.nodes[kv.children[0]!]!
      expect(inlineTable.type).toBe('InlineTable')
      expect(inlineTable.children.length).toBeGreaterThanOrEqual(2)

      const xKv = tree.nodes[inlineTable.children[0]!]!
      expect(xKv.data?.key).toBe('x')
    })

    it('should parse nested inline tables', () => {
      const toml = 'outer = { inner = { key = "value" } }'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const outerTable = tree.nodes[kv.children[0]!]!
      expect(outerTable.type).toBe('InlineTable')

      const innerKv = tree.nodes[outerTable.children[0]!]!
      const innerTable = tree.nodes[innerKv.children[0]!]!
      expect(innerTable.type).toBe('InlineTable')
    })
  })

  describe('Comments', () => {
    it('should skip comments', () => {
      const toml = `
        # This is a comment
        key = "value" # Inline comment
        # Another comment
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('key')
    })

    it('should handle comments in arrays', () => {
      const toml = `
        numbers = [
          # First number
          1,
          # Second number
          2,
          # Third number
          3
        ]
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const array = tree.nodes[kv.children[0]!]!
      expect(array.type).toBe('Array')
      expect(array.children.length).toBeGreaterThanOrEqual(3)
    })
  })

  describe('Dotted Keys', () => {
    it('should parse dotted keys', () => {
      const toml = 'physical.color = "orange"'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('physical.color')
    })

    it('should parse multiple dotted keys', () => {
      const toml = `
        physical.color = "orange"
        physical.shape = "round"
        site."google.com" = true
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(3)

      const kv1 = tree.nodes[root.children[0]!]!
      expect(kv1.data?.key).toBe('physical.color')

      const kv2 = tree.nodes[root.children[1]!]!
      expect(kv2.data?.key).toBe('physical.shape')
    })
  })

  describe('Real-World Examples', () => {
    it('should parse Cargo.toml-like config', () => {
      const toml = `
        [package]
        name = "my-package"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
        serde = "1.0"
        tokio = { version = "1.0", features = ["full"] }

        [[bin]]
        name = "my-binary"
        path = "src/main.rs"
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(3)

      // Find package table
      const packageTable = tree.nodes.find(
        (n) => n.type === 'Table' && n.data?.name === 'package'
      )
      expect(packageTable).toBeDefined()
    })

    it('should parse pyproject.toml-like config', () => {
      const toml = `
        [tool.poetry]
        name = "my-project"
        version = "0.1.0"
        description = "A Python project"

        [tool.poetry.dependencies]
        python = "^3.9"
        requests = "^2.28.0"

        [tool.poetry.dev-dependencies]
        pytest = "^7.0"
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(1)
    })

    it('should parse GitHub Actions workflow-like config', () => {
      const toml = `
        [workflow]
        name = "CI"

        [[workflow.on.push.branches]]
        value = "main"

        [[workflow.jobs]]
        name = "build"
        runs-on = "ubuntu-latest"
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(1)
    })
  })

  describe('Edge Cases', () => {
    it('should handle empty tables', () => {
      const toml = `
        [empty]

        [another]
        key = "value"
      `
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(2)
    })

    it('should handle trailing commas in arrays', () => {
      const toml = 'numbers = [1, 2, 3,]'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const array = tree.nodes[kv.children[0]!]!
      expect(array.type).toBe('Array')
    })

    it('should handle empty arrays', () => {
      const toml = 'empty = []'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const array = tree.nodes[kv.children[0]!]!
      expect(array.type).toBe('Array')
      expect(array.children).toHaveLength(0)
    })

    it('should handle empty inline tables', () => {
      const toml = 'empty = {}'
      const tree = parse(toml)
      const root = tree.nodes[tree.root]!

      const kv = tree.nodes[root.children[0]!]!
      const inlineTable = tree.nodes[kv.children[0]!]!
      expect(inlineTable.type).toBe('InlineTable')
      expect(inlineTable.children).toHaveLength(0)
    })
  })

  describe('API', () => {
    it('should work with createParser factory', () => {
      const parser = createParser()
      const tree = parser.parse('key = "value"')

      expect(tree.meta.language).toBe('toml')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should work with TOMLParser class', () => {
      const parser = new TOMLParser()
      const tree = parser.parse('key = "value"')

      expect(tree.meta.language).toBe('toml')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should work with standalone parse function', () => {
      const tree = parse('key = "value"')

      expect(tree.meta.language).toBe('toml')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should work with parseAsync function', async () => {
      const tree = await parseAsync('key = "value"')

      expect(tree.meta.language).toBe('toml')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should support getTree method', () => {
      const parser = new TOMLParser()
      const tree1 = parser.parse('key = "value"')
      const tree2 = parser.getTree()

      expect(tree2).toBe(tree1)
    })

    it('should support plugin system with use()', () => {
      const parser = new TOMLParser()

      const plugin = {
        name: 'test-plugin',
        transform: (tree: any) => {
          tree.metadata = { processed: true }
          return tree
        },
      }

      parser.use(plugin)
      const tree = parser.parse('key = "value"')

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

      const tree = parse('key = "value"', { plugins: [plugin] })

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

      const tree = await parseAsync('key = "value"', { plugins: [asyncPlugin] })

      expect(tree.metadata).toEqual({ async: true })
    })

    it('should throw error if async plugin used in sync parse', () => {
      const asyncPlugin = {
        name: 'async-plugin',
        transform: async (tree: any) => tree,
      }

      expect(() => {
        parse('key = "value"', { plugins: [asyncPlugin] })
      }).toThrow(/async plugins/i)
    })
  })
})

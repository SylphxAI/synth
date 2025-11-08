/**
 * JSON Parser Tests
 */

import { describe, it, expect } from 'vitest'
import { JSONParser, parse, parseAsync } from './parser.js'

describe('JSONParser', () => {
  describe('Basic Values', () => {
    it('should parse null', () => {
      const tree = parse('null')
      const nodes = tree.nodes.filter(n => n.type === 'Null')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(null)
    })

    it('should parse true', () => {
      const tree = parse('true')
      const nodes = tree.nodes.filter(n => n.type === 'Boolean')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(true)
    })

    it('should parse false', () => {
      const tree = parse('false')
      const nodes = tree.nodes.filter(n => n.type === 'Boolean')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(false)
    })
  })

  describe('Numbers', () => {
    it('should parse integer', () => {
      const tree = parse('42')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(42)
    })

    it('should parse negative integer', () => {
      const tree = parse('-42')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(-42)
    })

    it('should parse float', () => {
      const tree = parse('3.14')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(3.14)
    })

    it('should parse negative float', () => {
      const tree = parse('-3.14')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(-3.14)
    })

    it('should parse number with exponent', () => {
      const tree = parse('1.5e10')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(1.5e10)
    })

    it('should parse number with negative exponent', () => {
      const tree = parse('1.5e-10')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(1.5e-10)
    })

    it('should parse zero', () => {
      const tree = parse('0')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(0)
    })
  })

  describe('Strings', () => {
    it('should parse empty string', () => {
      const tree = parse('""')
      const nodes = tree.nodes.filter(n => n.type === 'String')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe('')
    })

    it('should parse simple string', () => {
      const tree = parse('"hello"')
      const nodes = tree.nodes.filter(n => n.type === 'String')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe('hello')
    })

    it('should parse string with spaces', () => {
      const tree = parse('"hello world"')
      const nodes = tree.nodes.filter(n => n.type === 'String')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe('hello world')
    })

    it('should parse string with escape sequences', () => {
      const tree = parse('"hello\\nworld"')
      const nodes = tree.nodes.filter(n => n.type === 'String')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe('hello\nworld')
    })

    it('should parse string with all escape sequences', () => {
      const tree = parse('"\\"\\\\\\/\\b\\f\\n\\r\\t"')
      const nodes = tree.nodes.filter(n => n.type === 'String')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe('"\\/\b\f\n\r\t')
    })

    it('should parse string with unicode escape', () => {
      const tree = parse('"\\u0048\\u0065\\u006C\\u006C\\u006F"')
      const nodes = tree.nodes.filter(n => n.type === 'String')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe('Hello')
    })
  })

  describe('Arrays', () => {
    it('should parse empty array', () => {
      const tree = parse('[]')
      const nodes = tree.nodes.filter(n => n.type === 'Array')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.children).toHaveLength(0)
    })

    it('should parse array with one element', () => {
      const tree = parse('[42]')
      const arrayNode = tree.nodes.find(n => n.type === 'Array')

      expect(arrayNode).toBeDefined()
      expect(arrayNode?.children).toHaveLength(1)

      const numNode = tree.nodes[arrayNode!.children[0]!]
      expect(numNode?.type).toBe('Number')
      expect(numNode?.data?.value).toBe(42)
    })

    it('should parse array with multiple elements', () => {
      const tree = parse('[1, 2, 3]')
      const arrayNode = tree.nodes.find(n => n.type === 'Array')

      expect(arrayNode).toBeDefined()
      expect(arrayNode?.children).toHaveLength(3)

      const values = arrayNode!.children.map(id => tree.nodes[id]!.data?.value)
      expect(values).toEqual([1, 2, 3])
    })

    it('should parse array with mixed types', () => {
      const tree = parse('[1, "hello", true, null]')
      const arrayNode = tree.nodes.find(n => n.type === 'Array')

      expect(arrayNode).toBeDefined()
      expect(arrayNode?.children).toHaveLength(4)

      const types = arrayNode!.children.map(id => tree.nodes[id]!.type)
      expect(types).toEqual(['Number', 'String', 'Boolean', 'Null'])
    })

    it('should parse nested arrays', () => {
      const tree = parse('[[1, 2], [3, 4]]')
      const outerArray = tree.nodes.find(n => n.type === 'Array')

      expect(outerArray).toBeDefined()
      expect(outerArray?.children).toHaveLength(2)

      const innerArrays = outerArray!.children.map(id => tree.nodes[id]!)
      expect(innerArrays[0]?.type).toBe('Array')
      expect(innerArrays[1]?.type).toBe('Array')
    })
  })

  describe('Objects', () => {
    it('should parse empty object', () => {
      const tree = parse('{}')
      const nodes = tree.nodes.filter(n => n.type === 'Object')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.children).toHaveLength(0)
    })

    it('should parse object with one property', () => {
      const tree = parse('{"name": "Alice"}')
      const objectNode = tree.nodes.find(n => n.type === 'Object')

      expect(objectNode).toBeDefined()
      expect(objectNode?.children).toHaveLength(1)

      const propNode = tree.nodes[objectNode!.children[0]!]
      expect(propNode?.type).toBe('Property')
      expect(propNode?.data?.key).toBe('name')

      const valueNode = tree.nodes[propNode!.children[0]!]
      expect(valueNode?.type).toBe('String')
      expect(valueNode?.data?.value).toBe('Alice')
    })

    it('should parse object with multiple properties', () => {
      const tree = parse('{"name": "Alice", "age": 30, "active": true}')
      const objectNode = tree.nodes.find(n => n.type === 'Object')

      expect(objectNode).toBeDefined()
      expect(objectNode?.children).toHaveLength(3)

      const props = objectNode!.children.map(id => tree.nodes[id]!)
      expect(props[0]?.data?.key).toBe('name')
      expect(props[1]?.data?.key).toBe('age')
      expect(props[2]?.data?.key).toBe('active')
    })

    it('should parse nested objects', () => {
      const tree = parse('{"person": {"name": "Alice", "age": 30}}')
      const outerObject = tree.nodes.find(n => n.type === 'Object')

      expect(outerObject).toBeDefined()
      expect(outerObject?.children).toHaveLength(1)

      const prop = tree.nodes[outerObject!.children[0]!]
      expect(prop?.data?.key).toBe('person')

      const innerObject = tree.nodes[prop!.children[0]!]
      expect(innerObject?.type).toBe('Object')
    })
  })

  describe('Complex Structures', () => {
    it('should parse complex nested structure', () => {
      const json = `{
        "users": [
          {"name": "Alice", "age": 30},
          {"name": "Bob", "age": 25}
        ],
        "count": 2
      }`

      const tree = parse(json)
      const rootObject = tree.nodes.find(n => n.type === 'Object')

      expect(rootObject).toBeDefined()
      expect(rootObject?.children).toHaveLength(2)
    })

    it('should parse deeply nested structure', () => {
      const json = '{"a": {"b": {"c": {"d": {"e": 1}}}}}'
      const tree = parse(json)

      const objects = tree.nodes.filter(n => n.type === 'Object')
      expect(objects.length).toBeGreaterThan(4)
    })
  })

  describe('Whitespace', () => {
    it('should handle whitespace around values', () => {
      const tree = parse('  42  ')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
      expect(nodes[0]?.data?.value).toBe(42)
    })

    it('should handle whitespace in arrays', () => {
      const tree = parse('[  1  ,  2  ,  3  ]')
      const arrayNode = tree.nodes.find(n => n.type === 'Array')

      expect(arrayNode?.children).toHaveLength(3)
    })

    it('should handle whitespace in objects', () => {
      const tree = parse('{ "name" : "Alice" }')
      const objectNode = tree.nodes.find(n => n.type === 'Object')

      expect(objectNode?.children).toHaveLength(1)
    })

    it('should handle newlines', () => {
      const tree = parse('{\n  "name": "Alice"\n}')
      const objectNode = tree.nodes.find(n => n.type === 'Object')

      expect(objectNode?.children).toHaveLength(1)
    })
  })

  describe('Error Handling', () => {
    it('should throw on empty input', () => {
      expect(() => parse('')).toThrow('Empty JSON document')
    })

    it('should throw on invalid value', () => {
      expect(() => parse('invalid')).toThrow()
    })

    it('should throw on unterminated string', () => {
      expect(() => parse('"hello')).toThrow()
    })

    it('should throw on invalid escape sequence', () => {
      expect(() => parse('"\\x"')).toThrow('Invalid escape sequence')
    })

    it('should throw on trailing content', () => {
      expect(() => parse('42 extra')).toThrow('Unexpected content after JSON value')
    })

    it('should throw on missing comma in array', () => {
      expect(() => parse('[1 2]')).toThrow()
    })

    it('should throw on missing comma in object', () => {
      expect(() => parse('{"a": 1 "b": 2}')).toThrow()
    })

    it('should throw on trailing comma by default', () => {
      expect(() => parse('[1, 2,]')).toThrow('Trailing commas not allowed')
    })

    it('should throw on invalid unicode escape', () => {
      expect(() => parse('"\\u00GG"')).toThrow('Invalid unicode escape')
    })
  })

  describe('Options', () => {
    it('should allow trailing commas when enabled', () => {
      const tree = parse('[1, 2,]', { allowTrailingCommas: true })
      const arrayNode = tree.nodes.find(n => n.type === 'Array')

      expect(arrayNode?.children).toHaveLength(2)
    })

    it('should allow trailing commas in objects', () => {
      const tree = parse('{"a": 1,}', { allowTrailingCommas: true })
      const objectNode = tree.nodes.find(n => n.type === 'Object')

      expect(objectNode?.children).toHaveLength(1)
    })
  })

  describe('Standalone Functions', () => {
    it('should parse with standalone parse()', () => {
      const tree = parse('42')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
    })

    it('should parse with parseAsync()', async () => {
      const tree = await parseAsync('42')
      const nodes = tree.nodes.filter(n => n.type === 'Number')

      expect(nodes).toHaveLength(1)
    })
  })

  describe('Parser Class', () => {
    it('should create parser', () => {
      const parser = new JSONParser()
      expect(parser).toBeDefined()
    })

    it('should parse with parser instance', () => {
      const parser = new JSONParser()
      const tree = parser.parse('42')

      expect(tree).toBeDefined()
    })

    it('should get last parsed tree', () => {
      const parser = new JSONParser()
      parser.parse('42')

      const tree = parser.getTree()
      expect(tree).toBeDefined()
    })
  })

  describe('Span Information', () => {
    it('should include span information', () => {
      const tree = parse('42')
      const numNode = tree.nodes.find(n => n.type === 'Number')

      expect(numNode?.span).toBeDefined()
      expect(numNode?.span?.start.offset).toBe(0)
      expect(numNode?.span?.end.offset).toBe(2)
    })

    it('should track line and column', () => {
      const tree = parse('{\n  "name": "Alice"\n}')
      const objectNode = tree.nodes.find(n => n.type === 'Object')

      expect(objectNode?.span).toBeDefined()
      expect(objectNode?.span?.start.line).toBe(1)
      expect(objectNode?.span?.end.line).toBeGreaterThan(1)
    })
  })

  describe('Real-World Examples', () => {
    it('should parse package.json-like structure', () => {
      const json = `{
        "name": "my-package",
        "version": "1.0.0",
        "dependencies": {
          "lodash": "^4.17.21"
        },
        "scripts": {
          "test": "vitest"
        }
      }`

      const tree = parse(json)
      const rootObject = tree.nodes.find(n => n.type === 'Object')

      expect(rootObject).toBeDefined()
      expect(rootObject?.children.length).toBeGreaterThan(3)
    })

    it('should parse API response-like structure', () => {
      const json = `{
        "users": [
          {
            "id": 1,
            "name": "Alice",
            "email": "alice@example.com",
            "active": true
          },
          {
            "id": 2,
            "name": "Bob",
            "email": "bob@example.com",
            "active": false
          }
        ],
        "total": 2,
        "page": 1
      }`

      const tree = parse(json)
      const arrays = tree.nodes.filter(n => n.type === 'Array')

      expect(arrays.length).toBeGreaterThan(0)
    })
  })
})

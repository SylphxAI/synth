/**
 * INI Parser Tests
 */

import { describe, expect, it } from 'bun:test'
import { createParser, INIParser, parse, parseAsync } from './parser.js'

describe('INIParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty INI', () => {
      const tree = parse('')
      expect(tree.meta.language).toBe('ini')
      expect(tree.nodes[tree.root]?.children).toHaveLength(0)
    })

    it('should parse basic key-value pair', () => {
      const ini = 'key = value'
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.type).toBe('KeyValue')
      expect(kv.data?.key).toBe('key')
      expect(kv.data?.value).toBe('value')
    })

    it('should parse multiple key-value pairs', () => {
      const ini = `
        key1 = value1
        key2 = value2
        key3 = value3
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(3)

      const kv1 = tree.nodes[root.children[0]!]!
      expect(kv1.data?.key).toBe('key1')
      expect(kv1.data?.value).toBe('value1')

      const kv2 = tree.nodes[root.children[1]!]!
      expect(kv2.data?.key).toBe('key2')
      expect(kv2.data?.value).toBe('value2')
    })
  })

  describe('Separators', () => {
    it('should parse with equals separator', () => {
      const ini = 'key = value'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('key')
      expect(kv.data?.value).toBe('value')
    })

    it('should parse with colon separator', () => {
      const ini = 'key: value'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('key')
      expect(kv.data?.value).toBe('value')
    })

    it('should handle no spaces around separator', () => {
      const ini = 'key=value'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('key')
      expect(kv.data?.value).toBe('value')
    })
  })

  describe('Sections', () => {
    it('should parse section header', () => {
      const ini = `
        [section]
        key = value
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const section = tree.nodes[root.children[0]!]!
      expect(section.type).toBe('Section')
      expect(section.data?.name).toBe('section')
      expect(section.children).toHaveLength(1)

      const kv = tree.nodes[section.children[0]!]!
      expect(kv.data?.key).toBe('key')
    })

    it('should parse multiple sections', () => {
      const ini = `
        [section1]
        key1 = value1

        [section2]
        key2 = value2
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)

      const section1 = tree.nodes[root.children[0]!]!
      expect(section1.data?.name).toBe('section1')

      const section2 = tree.nodes[root.children[1]!]!
      expect(section2.data?.name).toBe('section2')
    })

    it('should parse nested section names', () => {
      const ini = `
        [section.subsection]
        key = value
      `
      const tree = parse(ini)
      const section = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(section.data?.name).toBe('section.subsection')
    })

    it('should group keys under correct section', () => {
      const ini = `
        global = value

        [section]
        key1 = value1
        key2 = value2
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      // Global key
      const globalKv = tree.nodes[root.children[0]!]!
      expect(globalKv.data?.key).toBe('global')

      // Section with keys
      const section = tree.nodes[root.children[1]!]!
      expect(section.type).toBe('Section')
      expect(section.children).toHaveLength(2)
    })
  })

  describe('Comments', () => {
    it('should skip semicolon comments', () => {
      const ini = `
        ; This is a comment
        key = value
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('key')
    })

    it('should skip hash comments', () => {
      const ini = `
        # This is a comment
        key = value
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('key')
    })

    it('should handle inline comments', () => {
      const ini = 'key = value ; inline comment'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('key')
      expect(kv.data?.value).toBe('value')
    })

    it('should handle mixed comment styles', () => {
      const ini = `
        ; Semicolon comment
        key1 = value1
        # Hash comment
        key2 = value2
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)
    })
  })

  describe('Quoted Values', () => {
    it('should handle double quoted values', () => {
      const ini = 'key = "quoted value"'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.value).toBe('quoted value')
    })

    it('should handle single quoted values', () => {
      const ini = "key = 'quoted value'"
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.value).toBe('quoted value')
    })

    it('should preserve spaces in quoted values', () => {
      const ini = 'key = "  spaced  "'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.value).toBe('  spaced  ')
    })
  })

  describe('Real-World Examples', () => {
    it('should parse .gitconfig-like file', () => {
      const ini = `
        [user]
        name = John Doe
        email = john@example.com

        [core]
        editor = vim
        autocrlf = input

        [alias]
        st = status
        co = checkout
        br = branch

        [color]
        ui = auto
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(4)

      // Find user section
      const userSection = tree.nodes.find((n) => n.type === 'Section' && n.data?.name === 'user')
      expect(userSection).toBeDefined()
      expect(userSection?.children.length).toBe(2)

      // Check a key-value in user section
      const nameKv = tree.nodes[userSection?.children[0]!]!
      expect(nameKv.data?.key).toBe('name')
      expect(nameKv.data?.value).toBe('John Doe')
    })

    it('should parse .editorconfig-like file', () => {
      const ini = `
        # EditorConfig is awesome
        root = true

        [*]
        charset = utf-8
        end_of_line = lf
        insert_final_newline = true

        [*.{js,ts}]
        indent_style = space
        indent_size = 2

        [*.md]
        trim_trailing_whitespace = false
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(1)

      // Find first key (root = true) in global section
      const rootKv = tree.nodes[root.children[0]!]!
      expect(rootKv.data?.key).toBe('root')
      expect(rootKv.data?.value).toBe('true')
    })

    it('should parse Windows INI file', () => {
      const ini = `
        [Settings]
        WindowWidth=1024
        WindowHeight=768
        Fullscreen=false

        [Graphics]
        Quality=High
        VSync=true
        AntiAliasing=4x
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)

      const settingsSection = tree.nodes[root.children[0]!]!
      expect(settingsSection.data?.name).toBe('Settings')
      expect(settingsSection.children).toHaveLength(3)
    })

    it('should parse PHP INI-like file', () => {
      const ini = `
        ; PHP Configuration

        [PHP]
        engine = On
        short_open_tag = Off
        precision = 14
        output_buffering = 4096

        [Date]
        date.timezone = America/New_York

        [Session]
        session.save_handler = files
        session.use_cookies = 1
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children.length).toBeGreaterThanOrEqual(3)
    })
  })

  describe('Edge Cases', () => {
    it('should handle empty sections', () => {
      const ini = `
        [empty]

        [nonempty]
        key = value
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)

      const emptySection = tree.nodes[root.children[0]!]!
      expect(emptySection.children).toHaveLength(0)
    })

    it('should handle empty values', () => {
      const ini = 'key ='
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('key')
      expect(kv.data?.value).toBe('')
    })

    it('should handle values with equals signs', () => {
      const ini = 'url = https://example.com?foo=bar'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('url')
      expect(kv.data?.value).toBe('https://example.com?foo=bar')
    })

    it('should skip blank lines', () => {
      const ini = `
        key1 = value1


        key2 = value2
      `
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)
    })

    it('should handle Windows line endings', () => {
      const ini = 'key1 = value1\r\nkey2 = value2\r\n'
      const tree = parse(ini)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)
    })

    it('should handle keys with spaces', () => {
      const ini = 'key with spaces = value'
      const tree = parse(ini)
      const kv = tree.nodes[tree.nodes[tree.root]?.children[0]!]!

      expect(kv.data?.key).toBe('key with spaces')
    })
  })

  describe('API', () => {
    it('should work with createParser factory', () => {
      const parser = createParser()
      const tree = parser.parse('key = value')

      expect(tree.meta.language).toBe('ini')
      expect(tree.nodes[tree.root]?.children).toHaveLength(1)
    })

    it('should work with INIParser class', () => {
      const parser = new INIParser()
      const tree = parser.parse('key = value')

      expect(tree.meta.language).toBe('ini')
      expect(tree.nodes[tree.root]?.children).toHaveLength(1)
    })

    it('should work with standalone parse function', () => {
      const tree = parse('key = value')

      expect(tree.meta.language).toBe('ini')
      expect(tree.nodes[tree.root]?.children).toHaveLength(1)
    })

    it('should work with parseAsync function', async () => {
      const tree = await parseAsync('key = value')

      expect(tree.meta.language).toBe('ini')
      expect(tree.nodes[tree.root]?.children).toHaveLength(1)
    })

    it('should support getTree method', () => {
      const parser = new INIParser()
      const tree1 = parser.parse('key = value')
      const tree2 = parser.getTree()

      expect(tree2).toBe(tree1)
    })

    it('should support plugin system with use()', () => {
      const parser = new INIParser()

      const plugin = {
        name: 'test-plugin',
        transform: (tree: any) => {
          tree.metadata = { processed: true }
          return tree
        },
      }

      parser.use(plugin)
      const tree = parser.parse('key = value')

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

      const tree = parse('key = value', { plugins: [plugin] })

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

      const tree = await parseAsync('key = value', { plugins: [asyncPlugin] })

      expect(tree.metadata).toEqual({ async: true })
    })

    it('should throw error if async plugin used in sync parse', () => {
      const asyncPlugin = {
        name: 'async-plugin',
        transform: async (tree: any) => tree,
      }

      expect(() => {
        parse('key = value', { plugins: [asyncPlugin] })
      }).toThrow(/async plugins/i)
    })

    it('should support custom comment characters', () => {
      const ini = `
        // This is a comment
        key = value
      `
      const tree = parse(ini, { commentChars: ['//'] })
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const kv = tree.nodes[root.children[0]!]!
      expect(kv.data?.key).toBe('key')
    })
  })
})

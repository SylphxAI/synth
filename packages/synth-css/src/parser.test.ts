/**
 * CSS Parser Tests
 */

import { describe, it, expect } from 'vitest'
import { parse, parseAsync, createParser, CSSParser } from './parser.js'

describe('CSSParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty CSS', () => {
      const tree = parse('')
      expect(tree.meta.language).toBe('css')
      expect(tree.nodes[tree.root]!.children).toHaveLength(0)
    })

    it('should parse single rule with one declaration', () => {
      const css = `
        .class {
          color: red;
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const ruleId = root.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.type).toBe('StyleRule')
      expect(rule.data?.selector).toBe('.class')
      expect(rule.children).toHaveLength(1)

      const declId = rule.children[0]!
      const decl = tree.nodes[declId]!

      expect(decl.type).toBe('Declaration')
      expect(decl.data?.property).toBe('color')
      expect(decl.data?.value).toBe('red')
    })

    it('should parse rule with multiple declarations', () => {
      const css = `
        #id {
          color: blue;
          font-size: 16px;
          margin: 0;
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!
      const ruleId = root.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.type).toBe('StyleRule')
      expect(rule.data?.selector).toBe('#id')
      expect(rule.children).toHaveLength(3)

      const decl1 = tree.nodes[rule.children[0]!]!
      expect(decl1.data?.property).toBe('color')
      expect(decl1.data?.value).toBe('blue')

      const decl2 = tree.nodes[rule.children[1]!]!
      expect(decl2.data?.property).toBe('font-size')
      expect(decl2.data?.value).toBe('16px')

      const decl3 = tree.nodes[rule.children[2]!]!
      expect(decl3.data?.property).toBe('margin')
      expect(decl3.data?.value).toBe('0')
    })
  })

  describe('Selectors', () => {
    it('should parse element selector', () => {
      const tree = parse('body { margin: 0; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toBe('body')
    })

    it('should parse class selector', () => {
      const tree = parse('.container { width: 100%; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toBe('.container')
    })

    it('should parse ID selector', () => {
      const tree = parse('#header { height: 60px; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toBe('#header')
    })

    it('should parse universal selector', () => {
      const tree = parse('* { box-sizing: border-box; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toBe('*')
    })

    it('should parse descendant selector', () => {
      const tree = parse('div p { color: gray; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('div')
      expect(rule.data?.selector).toContain('p')
    })

    it('should parse child selector', () => {
      const tree = parse('ul > li { list-style: none; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('ul')
      expect(rule.data?.selector).toContain('>')
      expect(rule.data?.selector).toContain('li')
    })

    it('should parse adjacent sibling selector', () => {
      const tree = parse('h1 + p { margin-top: 0; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('h1')
      expect(rule.data?.selector).toContain('+')
      expect(rule.data?.selector).toContain('p')
    })

    it('should parse general sibling selector', () => {
      const tree = parse('h1 ~ p { color: gray; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('h1')
      expect(rule.data?.selector).toContain('~')
      expect(rule.data?.selector).toContain('p')
    })

    it('should parse attribute selector', () => {
      const tree = parse('input[type="text"] { border: 1px solid; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('input')
      expect(rule.data?.selector).toContain('[')
      expect(rule.data?.selector).toContain('type')
      expect(rule.data?.selector).toContain('text')
    })

    it('should parse pseudo-class selector', () => {
      const tree = parse('a:hover { color: blue; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toBe('a:hover')
    })

    it('should parse pseudo-element selector', () => {
      const tree = parse('p::before { content: "→"; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toBe('p::before')
    })

    it('should parse multiple selectors', () => {
      const tree = parse('h1, h2, h3 { font-weight: bold; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('h1')
      expect(rule.data?.selector).toContain('h2')
      expect(rule.data?.selector).toContain('h3')
      expect(rule.data?.selector).toContain(',')
    })

    it('should parse complex selector', () => {
      const tree = parse('.nav > ul > li:first-child a[href^="http"] { color: red; }')
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      expect(rule.data?.selector).toContain('.nav')
      expect(rule.data?.selector).toContain('ul')
      expect(rule.data?.selector).toContain('li')
      expect(rule.data?.selector).toContain('first-child')
      expect(rule.data?.selector).toContain('href')
    })
  })

  describe('Declarations', () => {
    it('should parse color values', () => {
      const css = `
        div {
          color: #ff0000;
          background-color: rgb(255, 0, 0);
          border-color: rgba(255, 0, 0, 0.5);
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      const decl1 = tree.nodes[rule.children[0]!]!
      expect(decl1.data?.value).toBe('#ff0000')

      const decl2 = tree.nodes[rule.children[1]!]!
      expect(decl2.data?.value).toContain('rgb')
      expect(decl2.data?.value).toContain('255')

      const decl3 = tree.nodes[rule.children[2]!]!
      expect(decl3.data?.value).toContain('rgba')
      expect(decl3.data?.value).toContain('0.5')
    })

    it('should parse length values', () => {
      const css = `
        div {
          width: 100px;
          height: 50%;
          margin: 1em;
          padding: 2rem;
          font-size: 16pt;
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      const decl1 = tree.nodes[rule.children[0]!]!
      expect(decl1.data?.value).toBe('100px')

      const decl2 = tree.nodes[rule.children[1]!]!
      expect(decl2.data?.value).toBe('50%')

      const decl3 = tree.nodes[rule.children[2]!]!
      expect(decl3.data?.value).toBe('1em')
    })

    it('should parse shorthand properties', () => {
      const css = `
        div {
          margin: 10px 20px;
          padding: 5px 10px 15px 20px;
          border: 1px solid black;
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      const decl1 = tree.nodes[rule.children[0]!]!
      expect(decl1.data?.value).toContain('10px')
      expect(decl1.data?.value).toContain('20px')

      const decl2 = tree.nodes[rule.children[1]!]!
      expect(decl2.data?.value).toContain('5px')
      expect(decl2.data?.value).toContain('10px')
      expect(decl2.data?.value).toContain('15px')
      expect(decl2.data?.value).toContain('20px')

      const decl3 = tree.nodes[rule.children[2]!]!
      expect(decl3.data?.value).toContain('1px')
      expect(decl3.data?.value).toContain('solid')
      expect(decl3.data?.value).toContain('black')
    })

    it('should parse function values', () => {
      const css = `
        div {
          transform: translateX(100px);
          background: linear-gradient(to right, red, blue);
          filter: blur(5px);
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      const decl1 = tree.nodes[rule.children[0]!]!
      expect(decl1.data?.value).toContain('translateX')
      expect(decl1.data?.value).toContain('100px')

      const decl2 = tree.nodes[rule.children[1]!]!
      expect(decl2.data?.value).toContain('linear-gradient')
      expect(decl2.data?.value).toContain('red')
      expect(decl2.data?.value).toContain('blue')
    })

    it('should parse important declarations', () => {
      const css = `
        div {
          color: red !important;
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      const decl = tree.nodes[rule.children[0]!]!

      expect(decl.data?.value).toContain('red')
      expect(decl.data?.value).toContain('!important')
    })

    it('should handle declarations without semicolon', () => {
      const css = `
        div {
          color: red
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!
      const decl = tree.nodes[rule.children[0]!]!

      expect(decl.data?.property).toBe('color')
      expect(decl.data?.value).toBe('red')
    })
  })

  describe('At-Rules', () => {
    it('should parse @media rule', () => {
      const css = `
        @media screen and (max-width: 768px) {
          .container {
            width: 100%;
          }
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!
      const atRuleId = root.children[0]!
      const atRule = tree.nodes[atRuleId]!

      expect(atRule.type).toBe('AtRule')
      expect(atRule.data?.name).toBe('media')
      expect(atRule.data?.params).toContain('screen')
      expect(atRule.data?.params).toContain('max-width')
      expect(atRule.data?.params).toContain('768')
      expect(atRule.children).toHaveLength(1)

      const styleRuleId = atRule.children[0]!
      const styleRule = tree.nodes[styleRuleId]!
      expect(styleRule.type).toBe('StyleRule')
      expect(styleRule.data?.selector).toBe('.container')
    })

    it('should parse @keyframes rule', () => {
      const css = `
        @keyframes fadeIn {
          0% { opacity: 0; }
          100% { opacity: 1; }
        }
      `
      const tree = parse(css)
      const atRuleId = tree.nodes[tree.root]!.children[0]!
      const atRule = tree.nodes[atRuleId]!

      expect(atRule.type).toBe('AtRule')
      expect(atRule.data?.name).toBe('keyframes')
      expect(atRule.data?.params).toBe('fadeIn')
      expect(atRule.children.length).toBeGreaterThan(0)
    })

    it('should parse @import rule', () => {
      const css = `@import url("styles.css");`
      const tree = parse(css)
      const atRuleId = tree.nodes[tree.root]!.children[0]!
      const atRule = tree.nodes[atRuleId]!

      expect(atRule.type).toBe('AtRule')
      expect(atRule.data?.name).toBe('import')
      expect(atRule.data?.params).toContain('url')
      expect(atRule.data?.params).toContain('styles.css')
    })

    it('should parse @font-face rule', () => {
      const css = `
        @font-face {
          font-family: "MyFont";
          src: url("myfont.woff2");
        }
      `
      const tree = parse(css)
      const atRuleId = tree.nodes[tree.root]!.children[0]!
      const atRule = tree.nodes[atRuleId]!

      expect(atRule.type).toBe('AtRule')
      expect(atRule.data?.name).toBe('font-face')
      expect(atRule.children.length).toBeGreaterThanOrEqual(1)

      // Check children exist (can be StyleRule with declarations inside)
      const child1 = tree.nodes[atRule.children[0]!]
      expect(child1).toBeDefined()
    })

    it('should parse nested @media rules', () => {
      const css = `
        @media screen {
          @media (max-width: 768px) {
            div { width: 100%; }
          }
        }
      `
      const tree = parse(css)
      const outerMedia = tree.nodes[tree.root]!.children[0]!
      const outerMediaNode = tree.nodes[outerMedia]!

      expect(outerMediaNode.type).toBe('AtRule')
      expect(outerMediaNode.data?.name).toBe('media')

      const innerMedia = outerMediaNode.children[0]!
      const innerMediaNode = tree.nodes[innerMedia]!

      expect(innerMediaNode.type).toBe('AtRule')
      expect(innerMediaNode.data?.name).toBe('media')
    })
  })

  describe('Comments', () => {
    it('should skip comments', () => {
      const css = `
        /* This is a comment */
        .class {
          /* Another comment */
          color: red; /* Inline comment */
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)

      const ruleId = root.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.children).toHaveLength(1)
    })

    it('should handle multi-line comments', () => {
      const css = `
        /*
         * Multi-line comment
         * spanning multiple lines
         */
        div { color: blue; }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(1)
    })
  })

  describe('Multiple Rules', () => {
    it('should parse multiple top-level rules', () => {
      const css = `
        body { margin: 0; }
        .container { width: 100%; }
        #header { height: 60px; }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(3)

      const rule1 = tree.nodes[root.children[0]!]!
      expect(rule1.data?.selector).toBe('body')

      const rule2 = tree.nodes[root.children[1]!]!
      expect(rule2.data?.selector).toBe('.container')

      const rule3 = tree.nodes[root.children[2]!]!
      expect(rule3.data?.selector).toBe('#header')
    })

    it('should parse mixed at-rules and style rules', () => {
      const css = `
        body { margin: 0; }
        @media (max-width: 768px) {
          body { margin: 10px; }
        }
        .container { width: 100%; }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(3)

      const rule1 = tree.nodes[root.children[0]!]!
      expect(rule1.type).toBe('StyleRule')

      const rule2 = tree.nodes[root.children[1]!]!
      expect(rule2.type).toBe('AtRule')

      const rule3 = tree.nodes[root.children[2]!]!
      expect(rule3.type).toBe('StyleRule')
    })
  })

  describe('Real-World Examples', () => {
    it('should parse Bootstrap-like grid system', () => {
      const css = `
        .container {
          width: 100%;
          padding-right: 15px;
          padding-left: 15px;
          margin-right: auto;
          margin-left: auto;
        }

        @media (min-width: 768px) {
          .container {
            max-width: 720px;
          }
        }

        @media (min-width: 992px) {
          .container {
            max-width: 960px;
          }
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(3)

      const containerRule = tree.nodes[root.children[0]!]!
      expect(containerRule.type).toBe('StyleRule')
      expect(containerRule.data?.selector).toBe('.container')
      expect(containerRule.children).toHaveLength(5)
    })

    it('should parse CSS animations', () => {
      const css = `
        @keyframes slideIn {
          from {
            transform: translateX(-100%);
            opacity: 0;
          }
          to {
            transform: translateX(0);
            opacity: 1;
          }
        }

        .animated {
          animation: slideIn 0.3s ease-in-out;
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)

      const keyframes = tree.nodes[root.children[0]!]!
      expect(keyframes.type).toBe('AtRule')
      expect(keyframes.data?.name).toBe('keyframes')

      const animated = tree.nodes[root.children[1]!]!
      expect(animated.type).toBe('StyleRule')
    })

    it('should parse CSS variables', () => {
      const css = `
        :root {
          --primary-color: #007bff;
          --secondary-color: #6c757d;
          --font-size: 16px;
        }

        .button {
          background-color: var(--primary-color);
          font-size: var(--font-size);
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)

      const rootRule = tree.nodes[root.children[0]!]!
      expect(rootRule.data?.selector).toBe(':root')

      const buttonRule = tree.nodes[root.children[1]!]!
      const bgDecl = tree.nodes[buttonRule.children[0]!]!
      expect(bgDecl.data?.value).toBe('var(--primary-color)')
    })

    it('should parse flexbox layout', () => {
      const css = `
        .flex-container {
          display: flex;
          flex-direction: row;
          justify-content: space-between;
          align-items: center;
          gap: 1rem;
        }

        .flex-item {
          flex: 1 1 auto;
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)

      const container = tree.nodes[root.children[0]!]!
      expect(container.children).toHaveLength(5)

      const item = tree.nodes[root.children[1]!]!
      expect(item.children).toHaveLength(1)
    })

    it('should parse CSS grid layout', () => {
      const css = `
        .grid-container {
          display: grid;
          grid-template-columns: repeat(3, 1fr);
          grid-template-rows: auto;
          grid-gap: 20px;
        }

        .grid-item {
          grid-column: span 2;
          grid-row: 1 / 3;
        }
      `
      const tree = parse(css)
      const root = tree.nodes[tree.root]!

      expect(root.children).toHaveLength(2)
    })
  })

  describe('Edge Cases', () => {
    it('should handle empty rule', () => {
      const css = `.empty {}`
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.type).toBe('StyleRule')
      expect(rule.children).toHaveLength(0)
    })

    it('should handle rule with only whitespace', () => {
      const css = `
        .whitespace {

        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.type).toBe('StyleRule')
      expect(rule.children).toHaveLength(0)
    })

    it('should handle missing semicolons', () => {
      const css = `
        div {
          color: red;
          font-size: 16px
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.children.length).toBeGreaterThanOrEqual(1)
    })

    it('should handle trailing semicolon', () => {
      const css = `
        div {
          color: red;
        }
      `
      const tree = parse(css)
      const ruleId = tree.nodes[tree.root]!.children[0]!
      const rule = tree.nodes[ruleId]!

      expect(rule.children).toHaveLength(1)
    })
  })

  describe('API', () => {
    it('should work with createParser factory', () => {
      const parser = createParser()
      const tree = parser.parse('div { color: red; }')

      expect(tree.meta.language).toBe('css')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should work with CSSParser class', () => {
      const parser = new CSSParser()
      const tree = parser.parse('div { color: red; }')

      expect(tree.meta.language).toBe('css')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should work with standalone parse function', () => {
      const tree = parse('div { color: red; }')

      expect(tree.meta.language).toBe('css')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should work with parseAsync function', async () => {
      const tree = await parseAsync('div { color: red; }')

      expect(tree.meta.language).toBe('css')
      expect(tree.nodes[tree.root]!.children).toHaveLength(1)
    })

    it('should support getTree method', () => {
      const parser = new CSSParser()
      const tree1 = parser.parse('div { color: red; }')
      const tree2 = parser.getTree()

      expect(tree2).toBe(tree1)
    })

    it('should support plugin system with use()', () => {
      const parser = new CSSParser()

      const plugin = {
        name: 'test-plugin',
        transform: (tree: any) => {
          tree.metadata = { processed: true }
          return tree
        },
      }

      parser.use(plugin)
      const tree = parser.parse('div { color: red; }')

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

      const tree = parse('div { color: red; }', { plugins: [plugin] })

      expect(tree.metadata).toEqual({ fromOptions: true })
    })

    it('should support async plugins with parseAsync', async () => {
      const asyncPlugin = {
        name: 'async-plugin',
        transform: async (tree: any) => {
          await new Promise(resolve => setTimeout(resolve, 1))
          tree.metadata = { async: true }
          return tree
        },
      }

      const tree = await parseAsync('div { color: red; }', { plugins: [asyncPlugin] })

      expect(tree.metadata).toEqual({ async: true })
    })

    it('should throw error if async plugin used in sync parse', () => {
      const asyncPlugin = {
        name: 'async-plugin',
        transform: async (tree: any) => tree,
      }

      expect(() => {
        parse('div { color: red; }', { plugins: [asyncPlugin] })
      }).toThrow(/async plugins/i)
    })
  })
})

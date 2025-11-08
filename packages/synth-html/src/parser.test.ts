/**
 * HTML Parser Tests
 */

import { describe, it, expect } from 'vitest'
import { HTMLParser, createParser, parse, parseAsync } from './parser.js'
import { createTransformPlugin } from '@sylphx/synth'
import {
  isElementNode,
  isTextNode,
  isDocumentNode,
  isDoctypeNode,
  getTagName,
  getTextValue,
  getDoctypeName,
  getDoctypePublicId,
  getDoctypeSystemId,
  getAttribute,
  isSelfClosing,
  isVoidElement,
} from './types.js'

describe('HTMLParser', () => {
  describe('Basic Parsing', () => {
    it('should parse simple HTML', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div>Hello</div>')

      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)

      const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')
      expect(div).toBeDefined()

      const text = tree.nodes.find(n => isTextNode(n))
      expect(text).toBeDefined()
      expect(getTextValue(text!)).toBe('Hello')
    })

    it('should parse nested elements', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div><p>Text</p></div>')

      const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')
      const p = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'p')

      expect(div).toBeDefined()
      expect(p).toBeDefined()
      expect(p!.parent).toBe(div!.id)
    })

    it('should parse elements with attributes', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div id="main" class="container"></div>')

      const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')
      expect(div).toBeDefined()

      expect(getAttribute(div!, 'id')).toBe('main')
      expect(getAttribute(div!, 'class')).toBe('container')
    })
  })

  describe('DOCTYPE', () => {
    it('should parse HTML5 DOCTYPE', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<!DOCTYPE html><html></html>')

      const doctype = tree.nodes.find(n => isDoctypeNode(n))
      expect(doctype).toBeDefined()
      expect(getDoctypeName(doctype!)).toBe('html')
    })

    it('should parse DOCTYPE with public ID', () => {
      const parser = new HTMLParser()
      const tree = parser.parse(
        '<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">'
      )

      const doctype = tree.nodes.find(n => isDoctypeNode(n))
      expect(getDoctypePublicId(doctype!)).toBe('-//W3C//DTD HTML 4.01//EN')
      expect(getDoctypeSystemId(doctype!)).toBe('http://www.w3.org/TR/html4/strict.dtd')
    })
  })

  describe('Void Elements', () => {
    it('should handle void elements correctly', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div><br><img src="test.jpg"></div>')

      const br = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'br')
      const img = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'img')
      const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')

      expect(br).toBeDefined()
      expect(img).toBeDefined()

      expect(isVoidElement(br!)).toBe(true)
      expect(br!.parent).toBe(div!.id)
      expect(br!.children).toEqual([])

      expect(isVoidElement(img!)).toBe(true)
      expect(getAttribute(img!, 'src')).toBe('test.jpg')
    })
  })

  describe('Self-Closing Tags', () => {
    it('should handle self-closing tags', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<component />')

      const element = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'component')
      expect(element).toBeDefined()
      expect(isSelfClosing(element!)).toBe(true)
    })

    it('should handle self-closing tags with attributes', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<component name="test" value="123" />')

      const element = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'component')
      expect(isSelfClosing(element!)).toBe(true)
      expect(getAttribute(element!, 'name')).toBe('test')
      expect(getAttribute(element!, 'value')).toBe('123')
    })
  })

  describe('Complex Documents', () => {
    it('should parse complete HTML document', () => {
      const html = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Test Page</title>
  </head>
  <body>
    <header>
      <h1>Welcome</h1>
    </header>
    <main>
      <p>Content here</p>
    </main>
  </body>
</html>`

      const parser = new HTMLParser()
      const tree = parser.parse(html)

      expect(tree.nodes.length).toBeGreaterThan(0)

      const doctype = tree.nodes.find(n => isDoctypeNode(n))
      const html_elem = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'html')
      const head = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'head')
      const body = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'body')

      expect(doctype).toBeDefined()
      expect(html_elem).toBeDefined()
      expect(head).toBeDefined()
      expect(body).toBeDefined()
    })

    it('should maintain correct tree structure', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div><span><a>Link</a></span></div>')

      const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')
      const span = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'span')
      const a = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'a')

      expect(span!.parent).toBe(div!.id)
      expect(a!.parent).toBe(span!.id)
    })
  })

  describe('Plugin Support', () => {
    it('should apply transform plugins', () => {
      const parser = new HTMLParser()

      let pluginCalled = false
      const plugin = createTransformPlugin(
        { name: 'test', version: '1.0.0' },
        (tree) => {
          pluginCalled = true
          return tree
        }
      )

      parser.parse('<div>Test</div>', { plugins: [plugin] })
      expect(pluginCalled).toBe(true)
    })

    it('should support registered plugins', () => {
      const parser = new HTMLParser()

      let pluginCalled = false
      const plugin = createTransformPlugin(
        { name: 'test', version: '1.0.0' },
        (tree) => {
          pluginCalled = true
          return tree
        }
      )

      parser.use(plugin)
      parser.parse('<div>Test</div>')

      expect(pluginCalled).toBe(true)
    })

    it('should support plugin chaining', () => {
      const parser = new HTMLParser()

      const plugin1 = createTransformPlugin(
        { name: 'test1', version: '1.0.0' },
        (tree) => tree
      )
      const plugin2 = createTransformPlugin(
        { name: 'test2', version: '1.0.0' },
        (tree) => tree
      )

      const result = parser.use(plugin1).use(plugin2)
      expect(result).toBe(parser)
    })

    it('should merge registered and one-off plugins', () => {
      const parser = new HTMLParser()

      let registered = false
      let oneOff = false

      const registeredPlugin = createTransformPlugin(
        { name: 'registered', version: '1.0.0' },
        (tree) => {
          registered = true
          return tree
        }
      )

      const oneOffPlugin = createTransformPlugin(
        { name: 'one-off', version: '1.0.0' },
        (tree) => {
          oneOff = true
          return tree
        }
      )

      parser.use(registeredPlugin)
      parser.parse('<div>Test</div>', { plugins: [oneOffPlugin] })

      expect(registered).toBe(true)
      expect(oneOff).toBe(true)
    })
  })

  describe('Async Parsing', () => {
    it('should parse asynchronously', async () => {
      const parser = new HTMLParser()
      const tree = await parser.parseAsync('<div>Hello</div>')

      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should support async plugins', async () => {
      const parser = new HTMLParser()

      let pluginCalled = false
      const asyncPlugin = createTransformPlugin(
        { name: 'async-test', version: '1.0.0' },
        async (tree) => {
          await new Promise(resolve => setTimeout(resolve, 1))
          pluginCalled = true
          return tree
        }
      )

      await parser.parseAsync('<div>Test</div>', { plugins: [asyncPlugin] })
      expect(pluginCalled).toBe(true)
    })

    it('should detect async plugins in sync parse', () => {
      const parser = new HTMLParser()

      const asyncPlugin = createTransformPlugin(
        { name: 'async', version: '1.0.0' },
        async (tree) => {
          await new Promise(resolve => setTimeout(resolve, 1))
          return tree
        }
      )

      expect(() => {
        parser.parse('<div>Test</div>', { plugins: [asyncPlugin] })
      }).toThrow('Detected async plugins. Use parseAsync() instead of parse()')
    })
  })

  describe('getTree()', () => {
    it('should return null before parsing', () => {
      const parser = new HTMLParser()
      expect(parser.getTree()).toBe(null)
    })

    it('should return tree after parsing', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div>Test</div>')

      expect(parser.getTree()).toBe(tree)
    })

    it('should return updated tree after second parse', () => {
      const parser = new HTMLParser()
      const tree1 = parser.parse('<div>First</div>')
      const tree2 = parser.parse('<div>Second</div>')

      expect(parser.getTree()).toBe(tree2)
      expect(parser.getTree()).not.toBe(tree1)
    })
  })

  describe('Factory and Standalone Functions', () => {
    it('should create parser with factory', () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(HTMLParser)
    })

    it('should parse with standalone function', () => {
      const tree = parse('<div>Hello</div>')
      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should parse async with standalone function', async () => {
      const tree = await parseAsync('<div>Hello</div>')
      expect(tree).toBeDefined()
      expect(tree.nodes.length).toBeGreaterThan(0)
    })

    it('should accept options in standalone functions', () => {
      const tree = parse('<div>Test</div>', { buildIndex: true })
      expect(tree).toBeDefined()
    })
  })

  describe('Edge Cases', () => {
    it('should handle empty input', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('')

      expect(tree).toBeDefined()
      const docNode = tree.nodes.find(n => isDocumentNode(n))
      expect(docNode).toBeDefined()
    })

    it('should handle whitespace trimming in text nodes', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div>   \n  \t  </div>')

      // Whitespace-only text should be ignored
      const div = tree.nodes.find(n => isElementNode(n) && getTagName(n) === 'div')
      expect(div!.children).toEqual([])
    })

    it('should handle multiple root elements', () => {
      const parser = new HTMLParser()
      const tree = parser.parse('<div>First</div><div>Second</div>')

      const divs = tree.nodes.filter(n => isElementNode(n) && getTagName(n) === 'div')
      expect(divs.length).toBe(2)
    })
  })
})

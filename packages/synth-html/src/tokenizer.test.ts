/**
 * HTML Tokenizer Tests
 */

import { describe, it, expect } from 'vitest'
import { HTMLTokenizer } from './tokenizer.js'

describe('HTMLTokenizer', () => {
  describe('Basic HTML Elements', () => {
    it('should tokenize simple div element', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div>Hello</div>')

      expect(tokens).toHaveLength(3)
      expect(tokens[0]!.type).toBe('startTag')
      expect(tokens[0]!.tagName).toBe('div')
      expect(tokens[1]!.type).toBe('text')
      expect(tokens[1]!.value).toBe('Hello')
      expect(tokens[2]!.type).toBe('endTag')
      expect(tokens[2]!.tagName).toBe('div')
    })

    it('should handle nested elements', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div><span>Text</span></div>')

      expect(tokens).toHaveLength(5)
      expect(tokens[0]!.tagName).toBe('div')
      expect(tokens[1]!.tagName).toBe('span')
      expect(tokens[2]!.type).toBe('text')
      expect(tokens[3]!.tagName).toBe('span')
      expect(tokens[4]!.tagName).toBe('div')
    })

    it('should handle empty elements', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div></div>')

      expect(tokens).toHaveLength(2)
      expect(tokens[0]!.type).toBe('startTag')
      expect(tokens[1]!.type).toBe('endTag')
    })
  })

  describe('Attributes', () => {
    it('should parse double-quoted attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div class="container"></div>')

      const startTag = tokens[0] as any
      expect(startTag.attributes).toEqual({ class: 'container' })
    })

    it('should parse single-quoted attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize("<div class='container'></div>")

      const startTag = tokens[0] as any
      expect(startTag.attributes).toEqual({ class: 'container' })
    })

    it('should parse unquoted attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div class=container></div>')

      const startTag = tokens[0] as any
      expect(startTag.attributes).toEqual({ class: 'container' })
    })

    it('should parse boolean attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<input disabled checked>')

      const startTag = tokens[0] as any
      expect(startTag.attributes).toEqual({
        disabled: 'disabled',
        checked: 'checked',
      })
    })

    it('should parse multiple attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div id="main" class="container" data-test="value"></div>')

      const startTag = tokens[0] as any
      expect(startTag.attributes).toEqual({
        id: 'main',
        class: 'container',
        'data-test': 'value',
      })
    })

    it('should handle attributes with whitespace', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div   class  =  "container"   ></div>')

      const startTag = tokens[0] as any
      expect(startTag.attributes).toEqual({ class: 'container' })
    })
  })

  describe('Self-Closing Tags', () => {
    it('should parse self-closing tags', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<br/>')

      expect(tokens).toHaveLength(1)
      expect(tokens[0]!.type).toBe('selfClosingTag')
      expect(tokens[0]!.tagName).toBe('br')
    })

    it('should parse self-closing tags with attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<img src="test.jpg" alt="Test"/>')

      const tag = tokens[0] as any
      expect(tag.type).toBe('selfClosingTag')
      expect(tag.tagName).toBe('img')
      expect(tag.attributes).toEqual({
        src: 'test.jpg',
        alt: 'Test',
      })
    })
  })

  describe('Void Elements', () => {
    it('should parse br element', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<br>')

      expect(tokens).toHaveLength(1)
      expect(tokens[0]!.type).toBe('startTag')
      expect(tokens[0]!.tagName).toBe('br')
    })

    it('should parse img element', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<img src="test.jpg">')

      const tag = tokens[0] as any
      expect(tag.type).toBe('startTag')
      expect(tag.tagName).toBe('img')
      expect(tag.attributes.src).toBe('test.jpg')
    })

    it('should parse input element', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<input type="text" name="username">')

      const tag = tokens[0] as any
      expect(tag.type).toBe('startTag')
      expect(tag.tagName).toBe('input')
      expect(tag.attributes).toEqual({
        type: 'text',
        name: 'username',
      })
    })

    it('should parse meta element', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<meta charset="utf-8">')

      const tag = tokens[0] as any
      expect(tag.tagName).toBe('meta')
      expect(tag.attributes.charset).toBe('utf-8')
    })
  })

  describe('DOCTYPE', () => {
    it('should parse HTML5 doctype', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<!DOCTYPE html>')

      expect(tokens).toHaveLength(1)
      const doctype = tokens[0] as any
      expect(doctype.type).toBe('doctype')
      expect(doctype.name).toBe('html')
    })

    it('should parse doctype with public ID', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize(
        '<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">'
      )

      const doctype = tokens[0] as any
      expect(doctype.type).toBe('doctype')
      expect(doctype.name).toBe('html')
      expect(doctype.publicId).toBe('-//W3C//DTD HTML 4.01//EN')
      expect(doctype.systemId).toBe('http://www.w3.org/TR/html4/strict.dtd')
    })

    it('should parse doctype with system ID', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<!DOCTYPE html SYSTEM "about:legacy-compat">')

      const doctype = tokens[0] as any
      expect(doctype.type).toBe('doctype')
      expect(doctype.systemId).toBe('about:legacy-compat')
    })

    it('should handle case-insensitive doctype', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<!doctype HTML>')

      const doctype = tokens[0] as any
      expect(doctype.type).toBe('doctype')
      expect(doctype.name).toBe('HTML')
    })
  })

  describe('Comments', () => {
    it('should parse HTML comments', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<!-- This is a comment -->')

      expect(tokens).toHaveLength(1)
      const comment = tokens[0] as any
      expect(comment.type).toBe('comment')
      expect(comment.value).toBe(' This is a comment ')
    })

    it('should parse multi-line comments', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<!-- Line 1\nLine 2\nLine 3 -->')

      const comment = tokens[0] as any
      expect(comment.type).toBe('comment')
      expect(comment.value).toContain('Line 1')
      expect(comment.value).toContain('Line 3')
    })

    it('should handle empty comments', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<!---->')

      const comment = tokens[0] as any
      expect(comment.type).toBe('comment')
      expect(comment.value).toBe('')
    })
  })

  describe('CDATA', () => {
    it('should parse CDATA sections', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<![CDATA[Some data here]]>')

      expect(tokens).toHaveLength(1)
      const cdata = tokens[0] as any
      expect(cdata.type).toBe('cdata')
      expect(cdata.value).toBe('Some data here')
    })

    it('should parse CDATA with special characters', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<![CDATA[<>&"\']]>')

      const cdata = tokens[0] as any
      expect(cdata.value).toBe('<>&"\'')
    })

    it('should handle empty CDATA', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<![CDATA[]]>')

      const cdata = tokens[0] as any
      expect(cdata.type).toBe('cdata')
      expect(cdata.value).toBe('')
    })
  })

  describe('Text Nodes', () => {
    it('should parse text content', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('Hello World')

      expect(tokens).toHaveLength(1)
      const text = tokens[0] as any
      expect(text.type).toBe('text')
      expect(text.value).toBe('Hello World')
    })

    it('should parse text with whitespace', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('  Text with spaces  ')

      const text = tokens[0] as any
      expect(text.value).toBe('  Text with spaces  ')
    })

    it('should handle special characters in text', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div>Price: $100 & tax</div>')

      const text = tokens[1] as any
      expect(text.value).toBe('Price: $100 & tax')
    })
  })

  describe('Token Position Information', () => {
    it('should track token positions', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div>Text</div>')

      expect(tokens[0]!.start).toBe(0)
      expect(tokens[0]!.end).toBe(5)
      expect(tokens[1]!.start).toBe(5)
      expect(tokens[1]!.end).toBe(9)
      expect(tokens[2]!.start).toBe(9)
      expect(tokens[2]!.end).toBe(15)
    })

    it('should include raw content', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div class="test">Text</div>')

      expect(tokens[0]!.raw).toBe('<div class="test">')
      expect(tokens[1]!.raw).toBe('Text')
      expect(tokens[2]!.raw).toBe('</div>')
    })
  })

  describe('Complex HTML', () => {
    it('should parse complete HTML document', () => {
      const html = `<!DOCTYPE html>
<html>
  <head>
    <title>Test</title>
    <meta charset="utf-8">
  </head>
  <body>
    <div id="main" class="container">
      <h1>Hello World</h1>
      <p>This is a test.</p>
    </div>
  </body>
</html>`

      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize(html)

      expect(tokens.length).toBeGreaterThan(0)
      expect(tokens[0]!.type).toBe('doctype')

      const htmlTags = tokens.filter(t => t.tagName === 'html')
      expect(htmlTags.length).toBe(2) // Start and end

      const divTag = tokens.find(t => t.tagName === 'div' && t.type === 'startTag') as any
      expect(divTag.attributes.id).toBe('main')
      expect(divTag.attributes.class).toBe('container')
    })

    it('should handle mixed content', () => {
      const html = `
        <!-- Header -->
        <header>
          <h1>Title</h1>
          <!-- Navigation -->
          <nav>
            <a href="/">Home</a>
          </nav>
        </header>
        <![CDATA[Some data]]>
      `

      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize(html)

      const comments = tokens.filter(t => t.type === 'comment')
      expect(comments.length).toBe(2)

      const cdata = tokens.filter(t => t.type === 'cdata')
      expect(cdata.length).toBe(1)

      const elements = tokens.filter(t => t.type === 'startTag')
      expect(elements.length).toBeGreaterThan(0)
    })
  })

  describe('Edge Cases', () => {
    it('should handle empty input', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('')

      expect(tokens).toEqual([])
    })

    it('should handle whitespace-only input', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('   \n  \t  ')

      expect(tokens).toHaveLength(1)
      expect(tokens[0]!.type).toBe('text')
    })

    it('should handle unclosed tags gracefully', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div>Text')

      expect(tokens.length).toBeGreaterThan(0)
      expect(tokens[0]!.type).toBe('startTag')
    })

    it('should handle malformed attributes', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div class=test id=main>')

      const tag = tokens[0] as any
      expect(tag.attributes.class).toBe('test')
      expect(tag.attributes.id).toBe('main')
    })

    it('should normalize tag names to lowercase', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<DIV><SPAN>Text</SPAN></DIV>')

      expect(tokens[0]!.tagName).toBe('div')
      expect(tokens[1]!.tagName).toBe('span')
    })

    it('should normalize attribute names to lowercase', () => {
      const tokenizer = new HTMLTokenizer()
      const tokens = tokenizer.tokenize('<div CLASS="test" ID="main"></div>')

      const tag = tokens[0] as any
      expect(tag.attributes.class).toBe('test')
      expect(tag.attributes.id).toBe('main')
    })
  })
})

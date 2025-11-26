/**
 * HTML Types Tests
 */

import { describe, it, expect } from 'bun:test'
import type { BaseNode } from '@sylphx/synth'
import {
  VOID_ELEMENTS,
  isDocumentNode,
  isDoctypeNode,
  isElementNode,
  isTextNode,
  isCommentNode,
  isCDATANode,
  getTagName,
  getAttributes,
  getAttribute,
  isVoidElement,
  isSelfClosing,
  getTextValue,
  getCommentValue,
  getCDATAValue,
  getDoctypeName,
  getDoctypePublicId,
  getDoctypeSystemId,
} from './types.js'

describe('HTML Types', () => {
  describe('Type Guards', () => {
    it('should identify document nodes', () => {
      const node: BaseNode = {
        id: 0,
        type: 'root',
        parent: null,
        children: [],
      }

      expect(isDocumentNode(node)).toBe(true)
      expect(isElementNode(node)).toBe(false)
    })

    it('should identify doctype nodes', () => {
      const node: BaseNode = {
        id: 1,
        type: 'doctype',
        parent: 0,
        children: [],
        data: { name: 'html' },
      }

      expect(isDoctypeNode(node)).toBe(true)
      expect(isDocumentNode(node)).toBe(false)
    })

    it('should identify element nodes', () => {
      const node: BaseNode = {
        id: 1,
        type: 'element',
        parent: 0,
        children: [],
        data: { tagName: 'div', attributes: {}, selfClosing: false, void: false },
      }

      expect(isElementNode(node)).toBe(true)
      expect(isTextNode(node)).toBe(false)
    })

    it('should identify text nodes', () => {
      const node: BaseNode = {
        id: 1,
        type: 'text',
        parent: 0,
        children: [],
        data: { value: 'Hello' },
      }

      expect(isTextNode(node)).toBe(true)
      expect(isCommentNode(node)).toBe(false)
    })

    it('should identify comment nodes', () => {
      const node: BaseNode = {
        id: 1,
        type: 'comment',
        parent: 0,
        children: [],
        data: { value: ' comment ' },
      }

      expect(isCommentNode(node)).toBe(true)
      expect(isCDATANode(node)).toBe(false)
    })

    it('should identify CDATA nodes', () => {
      const node: BaseNode = {
        id: 1,
        type: 'cdata',
        parent: 0,
        children: [],
        data: { value: 'data' },
      }

      expect(isCDATANode(node)).toBe(true)
      expect(isElementNode(node)).toBe(false)
    })

    it('should handle undefined nodes', () => {
      expect(isDocumentNode(undefined)).toBe(false)
      expect(isElementNode(undefined)).toBe(false)
    })
  })

  describe('VOID_ELEMENTS', () => {
    it('should contain standard void elements', () => {
      expect(VOID_ELEMENTS.has('br')).toBe(true)
      expect(VOID_ELEMENTS.has('img')).toBe(true)
      expect(VOID_ELEMENTS.has('input')).toBe(true)
      expect(VOID_ELEMENTS.has('meta')).toBe(true)
      expect(VOID_ELEMENTS.has('link')).toBe(true)
      expect(VOID_ELEMENTS.has('hr')).toBe(true)
    })

    it('should not contain non-void elements', () => {
      expect(VOID_ELEMENTS.has('div')).toBe(false)
      expect(VOID_ELEMENTS.has('span')).toBe(false)
      expect(VOID_ELEMENTS.has('p')).toBe(false)
      expect(VOID_ELEMENTS.has('a')).toBe(false)
    })
  })

  describe('Data Accessor Functions', () => {
    describe('Element accessors', () => {
      const element: BaseNode = {
        id: 1,
        type: 'element',
        parent: 0,
        children: [],
        data: {
          tagName: 'div',
          attributes: { id: 'main', class: 'container' },
          selfClosing: false,
          void: false,
        },
      }

      it('should get tag name', () => {
        expect(getTagName(element)).toBe('div')
      })

      it('should get attributes', () => {
        const attrs = getAttributes(element)
        expect(attrs).toEqual({ id: 'main', class: 'container' })
      })

      it('should get specific attribute', () => {
        expect(getAttribute(element, 'id')).toBe('main')
        expect(getAttribute(element, 'class')).toBe('container')
      })

      it('should return undefined for non-element', () => {
        const text: BaseNode = {
          id: 2,
          type: 'text',
          parent: 0,
          children: [],
          data: { value: 'Hello' },
        }

        expect(getTagName(text)).toBeUndefined()
        expect(getAttributes(text)).toBeUndefined()
        expect(getAttribute(text, 'id')).toBeUndefined()
      })
    })

    describe('Void and self-closing', () => {
      it('should identify void elements', () => {
        const br: BaseNode = {
          id: 1,
          type: 'element',
          parent: 0,
          children: [],
          data: { tagName: 'br', attributes: {}, void: true, selfClosing: false },
        }

        expect(isVoidElement(br)).toBe(true)
      })

      it('should identify self-closing elements', () => {
        const component: BaseNode = {
          id: 1,
          type: 'element',
          parent: 0,
          children: [],
          data: { tagName: 'component', attributes: {}, void: false, selfClosing: true },
        }

        expect(isSelfClosing(component)).toBe(true)
      })

      it('should return false for non-void elements', () => {
        const div: BaseNode = {
          id: 1,
          type: 'element',
          parent: 0,
          children: [],
          data: { tagName: 'div', attributes: {}, void: false, selfClosing: false },
        }

        expect(isVoidElement(div)).toBe(false)
        expect(isSelfClosing(div)).toBe(false)
      })
    })

    describe('Text accessors', () => {
      it('should get text value', () => {
        const text: BaseNode = {
          id: 1,
          type: 'text',
          parent: 0,
          children: [],
          data: { value: 'Hello World' },
        }

        expect(getTextValue(text)).toBe('Hello World')
      })

      it('should return undefined for non-text', () => {
        const element: BaseNode = {
          id: 1,
          type: 'element',
          parent: 0,
          children: [],
          data: { tagName: 'div' },
        }

        expect(getTextValue(element)).toBeUndefined()
      })
    })

    describe('Comment accessors', () => {
      it('should get comment value', () => {
        const comment: BaseNode = {
          id: 1,
          type: 'comment',
          parent: 0,
          children: [],
          data: { value: ' This is a comment ' },
        }

        expect(getCommentValue(comment)).toBe(' This is a comment ')
      })
    })

    describe('CDATA accessors', () => {
      it('should get CDATA value', () => {
        const cdata: BaseNode = {
          id: 1,
          type: 'cdata',
          parent: 0,
          children: [],
          data: { value: 'Some data' },
        }

        expect(getCDATAValue(cdata)).toBe('Some data')
      })
    })

    describe('DOCTYPE accessors', () => {
      it('should get doctype name', () => {
        const doctype: BaseNode = {
          id: 1,
          type: 'doctype',
          parent: 0,
          children: [],
          data: { name: 'html' },
        }

        expect(getDoctypeName(doctype)).toBe('html')
      })

      it('should get doctype with public ID', () => {
        const doctype: BaseNode = {
          id: 1,
          type: 'doctype',
          parent: 0,
          children: [],
          data: {
            name: 'html',
            publicId: '-//W3C//DTD HTML 4.01//EN',
            systemId: 'http://www.w3.org/TR/html4/strict.dtd',
          },
        }

        expect(getDoctypeName(doctype)).toBe('html')
        expect(getDoctypePublicId(doctype)).toBe('-//W3C//DTD HTML 4.01//EN')
        expect(getDoctypeSystemId(doctype)).toBe('http://www.w3.org/TR/html4/strict.dtd')
      })

      it('should return undefined for non-doctype', () => {
        const element: BaseNode = {
          id: 1,
          type: 'element',
          parent: 0,
          children: [],
          data: { tagName: 'div' },
        }

        expect(getDoctypeName(element)).toBeUndefined()
      })
    })
  })
})

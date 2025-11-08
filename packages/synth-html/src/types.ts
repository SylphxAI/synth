/**
 * HTML-specific AST utilities
 *
 * Following the language-agnostic BaseNode interface from @sylphx/synth
 * All HTML-specific data is stored in the node.data field
 */

import type { BaseNode } from '@sylphx/synth'

// HTML node types
export type HTMLNodeType =
  | 'root'
  | 'doctype'
  | 'element'
  | 'text'
  | 'comment'
  | 'cdata'

// HTML void elements (self-closing, cannot have children)
export const VOID_ELEMENTS = new Set([
  'area',
  'base',
  'br',
  'col',
  'embed',
  'hr',
  'img',
  'input',
  'link',
  'meta',
  'param',
  'source',
  'track',
  'wbr',
])

// Type guards
export function isDocumentNode(node?: BaseNode): node is BaseNode {
  return node?.type === 'root'
}

export function isDoctypeNode(node?: BaseNode): node is BaseNode {
  return node?.type === 'doctype'
}

export function isElementNode(node?: BaseNode): node is BaseNode {
  return node?.type === 'element'
}

export function isTextNode(node?: BaseNode): node is BaseNode {
  return node?.type === 'text'
}

export function isCommentNode(node?: BaseNode): node is BaseNode {
  return node?.type === 'comment'
}

export function isCDATANode(node?: BaseNode): node is BaseNode {
  return node?.type === 'cdata'
}

// Data accessor functions
export function getTagName(node: BaseNode): string | undefined {
  if (!isElementNode(node)) return undefined
  return node.data?.tagName as string | undefined
}

export function getAttributes(node: BaseNode): Record<string, string> | undefined {
  if (!isElementNode(node)) return undefined
  return node.data?.attributes as Record<string, string> | undefined
}

export function getAttribute(node: BaseNode, name: string): string | undefined {
  const attrs = getAttributes(node)
  return attrs?.[name]
}

export function isVoidElement(node: BaseNode): boolean {
  const tagName = getTagName(node)
  return tagName ? VOID_ELEMENTS.has(tagName) : false
}

export function isSelfClosing(node: BaseNode): boolean {
  if (!isElementNode(node)) return false
  return node.data?.selfClosing as boolean
}

export function getTextValue(node: BaseNode): string | undefined {
  if (!isTextNode(node)) return undefined
  return node.data?.value as string | undefined
}

export function getCommentValue(node: BaseNode): string | undefined {
  if (!isCommentNode(node)) return undefined
  return node.data?.value as string | undefined
}

export function getCDATAValue(node: BaseNode): string | undefined {
  if (!isCDATANode(node)) return undefined
  return node.data?.value as string | undefined
}

export function getDoctypeName(node: BaseNode): string | undefined {
  if (!isDoctypeNode(node)) return undefined
  return node.data?.name as string | undefined
}

export function getDoctypePublicId(node: BaseNode): string | undefined {
  if (!isDoctypeNode(node)) return undefined
  return node.data?.publicId as string | undefined
}

export function getDoctypeSystemId(node: BaseNode): string | undefined {
  if (!isDoctypeNode(node)) return undefined
  return node.data?.systemId as string | undefined
}

/**
 * AST → Minified Code Compressor
 *
 * Converts Synth JavaScript AST to compact code
 */

import type { BaseNode, Tree } from '@sylphx/synth'
import type { MinifyOptions } from './options.js'
import { DEFAULT_OPTIONS } from './options.js'

export class Compressor {
  private options: Required<MinifyOptions>
  private output: string[] = []
  private mangleMap = new Map<string, string>()
  private mangleCounter = 0

  constructor(options: MinifyOptions = {}) {
    this.options = { ...DEFAULT_OPTIONS, ...options }
  }

  compress(tree: Tree): string {
    this.output = []
    this.mangleMap.clear()
    this.mangleCounter = 0

    // Find Program node
    const program = tree.nodes.find(n => n.type === 'Program')
    if (!program) {
      throw new Error('No Program node found in tree')
    }

    // Compress all children
    const children = this.getChildren(tree, program)
    this.compressStatements(tree, children)

    return this.output.join('')
  }

  private compressStatements(tree: Tree, nodes: BaseNode[]): void {
    nodes.forEach(node => {
      this.compressNode(tree, node)
    })
  }

  private compressNode(tree: Tree, node: BaseNode): void {
    switch (node.type) {
      case 'Program':
        this.compressProgram(tree, node)
        break

      case 'VariableDeclaration':
        this.compressVariableDeclaration(tree, node)
        break

      case 'FunctionDeclaration':
        this.compressFunctionDeclaration(tree, node)
        break

      case 'ExpressionStatement':
        this.compressExpressionStatement(tree, node)
        break

      case 'ReturnStatement':
        this.compressReturnStatement(tree, node)
        break

      case 'IfStatement':
        this.compressIfStatement(tree, node)
        break

      case 'BlockStatement':
        this.compressBlockStatement(tree, node)
        break

      case 'CallExpression':
        this.compressCallExpression(tree, node)
        break

      case 'MemberExpression':
        this.compressMemberExpression(tree, node)
        break

      case 'Identifier':
        this.compressIdentifier(node)
        break

      case 'Literal':
        this.compressLiteral(node)
        break

      case 'BinaryExpression':
        this.compressBinaryExpression(tree, node)
        break

      case 'UnaryExpression':
        this.compressUnaryExpression(tree, node)
        break

      case 'AwaitExpression':
        this.compressAwaitExpression(tree, node)
        break

      case 'ArrowFunctionExpression':
        this.compressArrowFunction(tree, node)
        break

      case 'ArrayExpression':
        this.compressArrayExpression(tree, node)
        break

      case 'ObjectExpression':
        this.compressObjectExpression(tree, node)
        break

      case 'Property':
        this.compressProperty(tree, node)
        break

      case 'VariableDeclarator':
        this.compressVariableDeclarator(tree, node)
        break

      case 'ImportDeclaration':
        this.compressImportDeclaration(tree, node)
        break

      case 'ExportNamedDeclaration':
      case 'ExportDefaultDeclaration':
        this.compressExportDeclaration(tree, node)
        break

      case 'ClassDeclaration':
        this.compressClassDeclaration(tree, node)
        break

      case 'MethodDefinition':
        this.compressMethodDefinition(tree, node)
        break

      default:
        // Fallback
        this.write(`/*${node.type}*/`)
    }
  }

  private compressProgram(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    this.compressStatements(tree, children)
  }

  private compressVariableDeclaration(tree: Tree, node: BaseNode): void {
    const kind = node.data?.kind || 'const'
    this.write(`${kind} `)

    const declarators = this.getChildren(tree, node)
    declarators.forEach((decl, i) => {
      this.compressNode(tree, decl)
      if (i < declarators.length - 1) {
        this.write(',')
      }
    })

    this.write(';')
  }

  private compressVariableDeclarator(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    const id = children.find(n => n.type === 'Identifier' || n.type.includes('Pattern'))
    const init = children.find(n => n !== id)

    if (id) {
      this.compressNode(tree, id)
    }

    if (init) {
      this.write('=')
      this.compressNode(tree, init)
    }
  }

  private compressFunctionDeclaration(tree: Tree, node: BaseNode): void {
    if (node.data?.async) {
      this.write('async ')
    }

    this.write('function')

    if (node.data?.generator) {
      this.write('*')
    }

    const name = node.data?.id
    if (name) {
      this.write(` ${this.mangleName(String(name))}`)
    }

    this.write('(')

    const children = this.getChildren(tree, node)
    const body = children.find(n => n.type === 'BlockStatement')
    const params = children.filter(n => n !== body)

    params.forEach((param, i) => {
      this.compressNode(tree, param)
      if (i < params.length - 1) {
        this.write(',')
      }
    })

    this.write(')')

    if (body) {
      this.compressNode(tree, body)
    }
  }

  private compressArrowFunction(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    const body = children[children.length - 1]
    const params = children.slice(0, -1)

    // Single param without parens if not mangling
    if (params.length === 1 && !this.options.mangle) {
      this.compressNode(tree, params[0]!)
    } else {
      this.write('(')
      params.forEach((param, i) => {
        this.compressNode(tree, param)
        if (i < params.length - 1) {
          this.write(',')
        }
      })
      this.write(')')
    }

    this.write('=>')

    if (body) {
      // Expression body doesn't need braces
      if (body.type === 'BlockStatement') {
        this.compressNode(tree, body)
      } else {
        this.compressNode(tree, body)
      }
    }
  }

  private compressBlockStatement(tree: Tree, node: BaseNode): void {
    this.write('{')

    const children = this.getChildren(tree, node)
    if (children.length > 0) {
      this.compressStatements(tree, children)
    }

    this.write('}')
  }

  private compressExpressionStatement(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    if (children[0]) {
      this.compressNode(tree, children[0])
    }
    this.write(';')
  }

  private compressReturnStatement(tree: Tree, node: BaseNode): void {
    this.write('return')

    const children = this.getChildren(tree, node)
    if (children[0]) {
      this.write(' ')
      this.compressNode(tree, children[0])
    }

    this.write(';')
  }

  private compressIfStatement(tree: Tree, node: BaseNode): void {
    this.write('if(')

    const children = this.getChildren(tree, node)
    const test = children[0]
    const consequent = children[1]
    const alternate = children[2]

    if (test) {
      this.compressNode(tree, test)
    }

    this.write(')')

    if (consequent) {
      this.compressNode(tree, consequent)
    }

    if (alternate) {
      this.write('else ')
      this.compressNode(tree, alternate)
    }
  }

  private compressCallExpression(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    const callee = children[0]
    const args = children.slice(1)

    if (callee) {
      this.compressNode(tree, callee)
    }

    this.write('(')
    args.forEach((arg, i) => {
      this.compressNode(tree, arg)
      if (i < args.length - 1) {
        this.write(',')
      }
    })
    this.write(')')
  }

  private compressMemberExpression(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    const object = children[0]
    const property = children[1]
    const computed = node.data?.computed

    if (object) {
      this.compressNode(tree, object)
    }

    if (computed) {
      this.write('[')
      if (property) {
        this.compressNode(tree, property)
      }
      this.write(']')
    } else {
      this.write('.')
      if (property) {
        this.compressNode(tree, property)
      }
    }
  }

  private compressBinaryExpression(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    const left = children[0]
    const right = children[1]
    const operator = node.data?.operator

    if (left) {
      this.compressNode(tree, left)
    }

    // Some operators need spaces (like 'in', 'instanceof')
    const needsSpace = ['in', 'instanceof'].includes(operator as string)
    if (needsSpace) {
      this.write(` ${operator} `)
    } else {
      this.write(operator as string)
    }

    if (right) {
      this.compressNode(tree, right)
    }
  }

  private compressUnaryExpression(tree: Tree, node: BaseNode): void {
    const operator = node.data?.operator
    const prefix = node.data?.prefix !== false

    if (prefix) {
      this.write(operator as string)
      const children = this.getChildren(tree, node)
      if (children[0]) {
        this.compressNode(tree, children[0])
      }
    }
  }

  private compressAwaitExpression(tree: Tree, node: BaseNode): void {
    this.write('await ')
    const children = this.getChildren(tree, node)
    if (children[0]) {
      this.compressNode(tree, children[0])
    }
  }

  private compressArrayExpression(tree: Tree, node: BaseNode): void {
    this.write('[')
    const children = this.getChildren(tree, node)
    children.forEach((elem, i) => {
      this.compressNode(tree, elem)
      if (i < children.length - 1) {
        this.write(',')
      }
    })
    this.write(']')
  }

  private compressObjectExpression(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)

    if (children.length === 0) {
      this.write('{}')
      return
    }

    this.write('{')

    children.forEach((prop, i) => {
      this.compressNode(tree, prop)
      if (i < children.length - 1) {
        this.write(',')
      }
    })

    this.write('}')
  }

  private compressProperty(tree: Tree, node: BaseNode): void {
    const children = this.getChildren(tree, node)
    const key = children[0]
    const value = children[1]

    if (key) {
      this.compressNode(tree, key)
    }

    this.write(':')

    if (value) {
      this.compressNode(tree, value)
    }
  }

  private compressImportDeclaration(_tree: Tree, _node: BaseNode): void {
    // Simplified import handling
    this.write('import;')
  }

  private compressExportDeclaration(tree: Tree, node: BaseNode): void {
    this.write('export ')

    if (node.type === 'ExportDefaultDeclaration') {
      this.write('default ')
    }

    const children = this.getChildren(tree, node)
    if (children[0]) {
      this.compressNode(tree, children[0])
    }
  }

  private compressClassDeclaration(tree: Tree, node: BaseNode): void {
    this.write('class ')

    const name = node.data?.id
    if (name && typeof name === 'string') {
      this.write(`${this.mangleName(name)} `)
    }

    const children = this.getChildren(tree, node)
    const classBody = children.find(n => n.type === 'ClassBody')

    if (classBody) {
      const methods = this.getChildren(tree, classBody)

      this.write('{')

      methods.forEach(method => {
        this.compressNode(tree, method)
      })

      this.write('}')
    } else {
      this.write('{}')
    }
  }

  private compressMethodDefinition(tree: Tree, node: BaseNode): void {
    const kind = node.data?.kind
    if (kind && kind !== 'method') {
      this.write(`${kind} `)
    }

    const key = this.getChildren(tree, node).find(n => n.type === 'Identifier')
    if (key) {
      this.compressNode(tree, key)
    }

    this.write('()')

    const value = this.getChildren(tree, node).find(n => n.type === 'FunctionExpression')
    if (value) {
      const body = this.getChildren(tree, value).find(n => n.type === 'BlockStatement')
      if (body) {
        this.compressNode(tree, body)
      }
    }
  }

  private compressIdentifier(node: BaseNode): void {
    const name = node.data?.name
    if (name) {
      this.write(this.mangleName(String(name)))
    }
  }

  private compressLiteral(node: BaseNode): void {
    const raw = node.data?.raw
    const value = node.data?.value

    if (raw !== undefined) {
      // Use shortest representation
      if (typeof value === 'string') {
        // Always use double quotes for consistency
        this.write(`"${value}"`)
      } else {
        this.write(String(raw))
      }
    } else if (value !== undefined) {
      this.write(JSON.stringify(value))
    }
  }

  private mangleName(name: string): string {
    if (!this.options.mangle) {
      return name
    }

    // Don't mangle reserved names
    if (this.options.reserved.includes(name)) {
      return name
    }

    // Check if already mangled
    if (this.mangleMap.has(name)) {
      return this.mangleMap.get(name)!
    }

    // Generate short name
    const mangled = this.generateMangledName()
    this.mangleMap.set(name, mangled)
    return mangled
  }

  private generateMangledName(): string {
    // Generate a, b, c, ..., z, aa, ab, ...
    const alphabet = 'abcdefghijklmnopqrstuvwxyz'
    let num = this.mangleCounter++
    let name = ''

    do {
      name = alphabet[num % 26] + name
      num = Math.floor(num / 26) - 1
    } while (num >= 0)

    return name
  }

  private getChildren(tree: Tree, node: BaseNode): BaseNode[] {
    return node.children.map(id => tree.nodes[id]!).filter(Boolean)
  }

  private write(str: string): void {
    this.output.push(str)
  }
}

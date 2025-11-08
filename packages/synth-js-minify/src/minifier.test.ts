/**
 * Minifier Tests
 */

import { describe, it, expect } from 'vitest'
import { Minifier, minify, savings } from './minifier.js'

describe('Minifier', () => {
  describe('Basic Minification', () => {
    it('should minify variable declarations', () => {
      const code = 'const x = 42;'
      const minified = minify(code)

      expect(minified).toBe('const x=42;')
    })

    it('should minify multiple variable declarations', () => {
      const code = 'const x = 1; const y = 2;'
      const minified = minify(code)

      expect(minified).toBe('const x=1;const y=2;')
    })

    it('should minify function declarations', () => {
      const code = 'function hello() { return "world"; }'
      const minified = minify(code)

      expect(minified).toBe('function hello(){return "world";}')
    })

    it('should minify arrow functions', () => {
      const code = 'const add = (a, b) => a + b;'
      const minified = minify(code)

      expect(minified).toBe('const add=(a,b)=>a+b;')
    })

    it('should minify single-param arrow functions', () => {
      const code = 'const double = (x) => x * 2;'
      const minified = minify(code)

      expect(minified).toBe('const double=x=>x*2;')
    })
  })

  describe('Options', () => {
    it('should respect compress option', () => {
      const code = 'const x = 42;'

      const compressed = minify(code, { compress: true })
      expect(compressed).toBe('const x=42;')

      const notCompressed = minify(code, { compress: false })
      expect(notCompressed).toBeDefined()
    })

    it('should respect mangle option', () => {
      const code = 'function myLongFunctionName() { const myVar = 42; return myVar; }'

      const notMangled = minify(code, { mangle: false })
      expect(notMangled).toContain('myLongFunctionName')
      expect(notMangled).toContain('myVar')

      const mangled = minify(code, { mangle: true })
      expect(mangled).not.toContain('myLongFunctionName')
      expect(mangled).not.toContain('myVar')
      expect(mangled).toContain('function')
    })

    it('should respect reserved names', () => {
      const code = 'function importantName() { return 42; }'

      const mangled = minify(code, {
        mangle: true,
        reserved: ['importantName'],
      })

      expect(mangled).toContain('importantName')
    })
  })

  describe('Statements', () => {
    it('should minify if statements', () => {
      const code = 'if (x > 0) { console.log("positive"); }'
      const minified = minify(code)

      expect(minified).toBe('if(x>0){console.log("positive");}')
    })

    it('should minify if-else statements', () => {
      const code = 'function test(x) { if (x > 0) { return 1; } else { return -1; } }'
      const minified = minify(code)

      expect(minified).toBe('function test(x){if(x>0){return 1;}else {return -1;}}')
    })

    it('should minify return statements', () => {
      const code = 'function getValue() { return x + y; }'
      const minified = minify(code)

      expect(minified).toBe('function getValue(){return x+y;}')
    })

    it('should minify block statements', () => {
      const code = 'function test() { const x = 1; const y = 2; return x + y; }'
      const minified = minify(code)

      expect(minified).toBe('function test(){const x=1;const y=2;return x+y;}')
    })
  })

  describe('Expressions', () => {
    it('should minify binary expressions', () => {
      const code = 'const result = a + b * c;'
      const minified = minify(code)

      expect(minified).toBe('const result=a+b*c;')
    })

    it('should minify binary expressions with word operators', () => {
      const code = 'const result = x in obj;'
      const minified = minify(code)

      expect(minified).toBe('const result=x in obj;')
    })

    it('should minify call expressions', () => {
      const code = 'console.log("hello", "world");'
      const minified = minify(code)

      expect(minified).toBe('console.log("hello","world");')
    })

    it('should minify member expressions', () => {
      const code = 'obj.prop.nested;'
      const minified = minify(code)

      expect(minified).toBe('obj.prop.nested;')
    })

    it('should minify array expressions', () => {
      const code = 'const arr = [1, 2, 3, 4];'
      const minified = minify(code)

      expect(minified).toBe('const arr=[1,2,3,4];')
    })

    it('should minify object expressions', () => {
      const code = 'const obj = { a: 1, b: 2 };'
      const minified = minify(code)

      expect(minified).toBe('const obj={a:1,b:2};')
    })

    it('should minify empty objects', () => {
      const code = 'const obj = {};'
      const minified = minify(code)

      expect(minified).toBe('const obj={};')
    })
  })

  describe('Async/Await', () => {
    it('should minify async functions', () => {
      const code = 'async function fetchData() { return await fetch("/api"); }'
      const minified = minify(code)

      expect(minified).toBe('async function fetchData(){return await fetch("/api");}')
    })
  })

  describe('Classes', () => {
    it('should minify class declarations', () => {
      const code = 'class MyClass { constructor() { this.value = 42; } }'
      const minified = minify(code)

      expect(minified).toContain('class MyClass')
      expect(minified).toContain('{')
      expect(minified).toContain('}')
    })
  })

  describe('Imports/Exports', () => {
    it('should minify export declarations', () => {
      const code = 'export const x = 42;'
      const minified = minify(code)

      expect(minified).toContain('export')
      expect(minified).toContain('const x=42')
    })

    it('should minify default exports', () => {
      const code = 'export default function foo() {}'
      const minified = minify(code)

      expect(minified).toContain('export default')
    })
  })

  describe('Complex Code', () => {
    it('should minify complete functions', () => {
      const code = `
        function calculate(a, b) {
          const sum = a + b;
          const product = a * b;
          return { sum, product };
        }
      `

      const minified = minify(code)

      expect(minified).toContain('function calculate(a,b)')
      expect(minified).toContain('const sum=a+b')
      expect(minified).toContain('const product=a*b')
      expect(minified).toContain('return {sum:sum,product:product}')
      expect(minified).not.toContain('\n')
      expect(minified).not.toContain('  ')
    })

    it('should minify nested structures', () => {
      const code = 'const nested = { a: { b: { c: 1 } } };'
      const minified = minify(code)

      expect(minified).toBe('const nested={a:{b:{c:1}}};')
    })

    it('should save significant space', () => {
      const code = `
        function fibonacci(n) {
          if (n <= 1) {
            return n;
          }
          return fibonacci(n - 1) + fibonacci(n - 2);
        }
      `

      const minified = minify(code)
      const ratio = 1 - minified.length / code.length

      expect(ratio).toBeGreaterThan(0.3) // At least 30% reduction
    })
  })

  describe('Minifier class', () => {
    it('should create minifier with options', () => {
      const minifier = new Minifier({ mangle: true })
      const code = 'const x = 42;'
      const minified = minifier.minify(code)

      expect(minified).toBeDefined()
    })

    it('should calculate compression ratio', () => {
      const minifier = new Minifier()
      const code = 'const x = 42;'
      const ratio = minifier.compressionRatio(code)

      expect(ratio).toBeGreaterThan(0)
      expect(ratio).toBeLessThan(1)
    })
  })

  describe('savings() function', () => {
    it('should calculate size savings', () => {
      const original = 'const x = 42;'
      const minified = 'const x=42;'

      const result = savings(original, minified)

      expect(result.originalSize).toBe(original.length)
      expect(result.minifiedSize).toBe(minified.length)
      expect(result.savedBytes).toBe(original.length - minified.length)
      expect(result.savedPercent).toBeGreaterThan(0)
    })

    it('should show percentage savings', () => {
      const original = 'const x = 42;'
      const minified = minify(original)

      const result = savings(original, minified)

      expect(result.savedPercent).toBeGreaterThan(0)
      expect(result.savedPercent).toBeLessThan(100)
    })
  })

  describe('Error Handling', () => {
    it('should handle empty code', () => {
      const minified = minify('')
      expect(minified).toBeDefined()
    })

    it('should handle syntax errors gracefully', () => {
      expect(() => {
        minify('const x = ')
      }).toThrow()
    })
  })

  describe('Quote Style', () => {
    it('should use consistent quote style', () => {
      const code = 'const str = "hello";'
      const minified = minify(code)

      // Should use double quotes for consistency
      expect(minified).toContain('"hello"')
    })
  })

  describe('Name Mangling', () => {
    it('should generate short mangled names', () => {
      const code = `
        function veryLongFunctionName(veryLongParameterName) {
          const anotherLongVariableName = veryLongParameterName * 2;
          return anotherLongVariableName;
        }
      `

      const minified = minify(code, { mangle: true })

      // Should be significantly shorter with mangling
      expect(minified.length).toBeLessThan(code.length * 0.5)
      expect(minified).toMatch(/function [a-z]\(/)
    })

    it('should generate sequential names', () => {
      const code = `
        function name1() { return 1; }
        function name2() { return 2; }
        function name3() { return 3; }
      `

      const minified = minify(code, { mangle: true })

      // Should have short sequential names
      expect(minified).toBeDefined()
      expect(minified.length).toBeLessThan(code.length)
    })
  })
})

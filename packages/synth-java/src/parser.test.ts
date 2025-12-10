/**
 * Java Parser Tests (WASM-based)
 */

import { describe, expect, it } from 'bun:test'
import { createParser, init, JavaParser, parse, parseAsync } from './parser.js'

describe('JavaParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty Java', async () => {
      const tree = await parseAsync('')
      expect(tree.meta.language).toBe('java')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse simple class', async () => {
      const java = `
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')
      expect(tree.nodes[tree.root]).toBeDefined()

      // Should have program root and children
      const rootChildren = tree.nodes[tree.root]?.children
      expect(rootChildren.length).toBeGreaterThan(0)
    })

    it('should parse variable declaration', async () => {
      const java = 'int x = 42;'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find variable declaration
      const varNode = tree.nodes.find((n) => n.type.includes('VariableDecl') || n.type.includes('Local'))
      expect(varNode).toBeDefined()
    })

    it('should parse method definition', async () => {
      const java = `
public int add(int a, int b) {
    return a + b;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find method declaration
      const methodNode = tree.nodes.find((n) => n.type === 'MethodDeclaration')
      expect(methodNode).toBeDefined()
    })
  })

  describe('Data Types', () => {
    it('should parse string literals', async () => {
      const java = 'String text = "Hello, World!";'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find string literal
      const stringNode = tree.nodes.find((n) => n.type === 'StringLiteral')
      expect(stringNode).toBeDefined()
    })

    it('should parse integer literals', async () => {
      const java = 'int num = 42;'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find decimal integer literal
      const intNode = tree.nodes.find((n) => n.type === 'DecimalIntegerLiteral')
      expect(intNode).toBeDefined()
    })

    it('should parse floating point literals', async () => {
      const java = 'double pi = 3.14159;'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find floating point literal
      const floatNode = tree.nodes.find((n) => n.type.includes('Float'))
      expect(floatNode).toBeDefined()
    })

    it('should parse boolean literals', async () => {
      const java = `
boolean flag1 = true;
boolean flag2 = false;
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find boolean literals
      const boolNodes = tree.nodes.filter(
        (n) => n.type === 'True' || n.type === 'False' || n.data?.text === 'true' || n.data?.text === 'false'
      )
      expect(boolNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse arrays', async () => {
      const java = 'int[] numbers = {1, 2, 3, 4, 5};'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find array type or array initializer
      const arrayNode = tree.nodes.find((n) => n.type.includes('Array'))
      expect(arrayNode).toBeDefined()
    })

    it('should parse null literal', async () => {
      const java = 'String value = null;'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find null literal
      const nullNode = tree.nodes.find((n) => n.type === 'NullLiteral' || n.data?.text === 'null')
      expect(nullNode).toBeDefined()
    })
  })

  describe('Control Flow', () => {
    it('should parse if statement', async () => {
      const java = `
if (x > 0) {
    System.out.println("positive");
} else if (x < 0) {
    System.out.println("negative");
} else {
    System.out.println("zero");
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find if statement
      const ifNode = tree.nodes.find((n) => n.type === 'IfStatement')
      expect(ifNode).toBeDefined()
    })

    it('should parse for loop', async () => {
      const java = `
for (int i = 0; i < 10; i++) {
    System.out.println(i);
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find for statement
      const forNode = tree.nodes.find((n) => n.type === 'ForStatement')
      expect(forNode).toBeDefined()
    })

    it('should parse enhanced for loop', async () => {
      const java = `
for (String item : items) {
    System.out.println(item);
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find enhanced for statement
      const forNode = tree.nodes.find((n) => n.type === 'EnhancedForStatement')
      expect(forNode).toBeDefined()
    })

    it('should parse while loop', async () => {
      const java = `
while (x < 10) {
    x++;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find while statement
      const whileNode = tree.nodes.find((n) => n.type === 'WhileStatement')
      expect(whileNode).toBeDefined()
    })

    it('should parse do-while loop', async () => {
      const java = `
do {
    x++;
} while (x < 10);
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find do statement
      const doNode = tree.nodes.find((n) => n.type === 'DoStatement')
      expect(doNode).toBeDefined()
    })

    it('should parse switch statement', async () => {
      const java = `
switch (day) {
    case MONDAY:
        System.out.println("Monday");
        break;
    case TUESDAY:
        System.out.println("Tuesday");
        break;
    default:
        System.out.println("Other day");
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find switch statement
      const switchNode = tree.nodes.find((n) => n.type.includes('Switch'))
      expect(switchNode).toBeDefined()
    })

    it('should parse try-catch', async () => {
      const java = `
try {
    riskyOperation();
} catch (IOException e) {
    System.err.println("IO error: " + e.getMessage());
} finally {
    cleanup();
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find try statement
      const tryNode = tree.nodes.find((n) => n.type === 'TryStatement')
      expect(tryNode).toBeDefined()
    })
  })

  describe('Classes and Objects', () => {
    it('should parse class with fields', async () => {
      const java = `
public class Person {
    private String name;
    private int age;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find class declaration
      const classNode = tree.nodes.find((n) => n.type === 'ClassDeclaration')
      expect(classNode).toBeDefined()

      // Find field declarations
      const fieldNodes = tree.nodes.filter((n) => n.type === 'FieldDeclaration')
      expect(fieldNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse constructor', async () => {
      const java = `
public class Person {
    public Person(String name) {
        this.name = name;
    }
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find constructor declaration
      const constructorNode = tree.nodes.find((n) => n.type === 'ConstructorDeclaration')
      expect(constructorNode).toBeDefined()
    })

    it('should parse interface', async () => {
      const java = `
public interface Drawable {
    void draw();
    int getSize();
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find interface declaration
      const interfaceNode = tree.nodes.find((n) => n.type === 'InterfaceDeclaration')
      expect(interfaceNode).toBeDefined()
    })

    it('should parse enum', async () => {
      const java = `
public enum Day {
    MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find enum declaration
      const enumNode = tree.nodes.find((n) => n.type === 'EnumDeclaration')
      expect(enumNode).toBeDefined()
    })

    it('should parse class inheritance', async () => {
      const java = `
public class Dog extends Animal implements Pet {
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find class with extends
      const classNode = tree.nodes.find((n) => n.type === 'ClassDeclaration')
      expect(classNode).toBeDefined()

      // Find superclass
      const superNode = tree.nodes.find((n) => n.type.includes('Super'))
      expect(superNode).toBeDefined()
    })
  })

  describe('Methods', () => {
    it('should parse method with parameters', async () => {
      const java = `
public int add(int a, int b) {
    return a + b;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find method declaration
      const methodNode = tree.nodes.find((n) => n.type === 'MethodDeclaration')
      expect(methodNode).toBeDefined()

      // Find formal parameters
      const paramsNode = tree.nodes.find((n) => n.type === 'FormalParameters')
      expect(paramsNode).toBeDefined()
    })

    it('should parse static method', async () => {
      const java = `
public static void main(String[] args) {
    System.out.println("Hello");
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find method with static modifier
      const methodNode = tree.nodes.find((n) => n.type === 'MethodDeclaration')
      expect(methodNode).toBeDefined()
    })

    it('should parse abstract method', async () => {
      const java = `
public abstract void draw();
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find abstract method
      const methodNode = tree.nodes.find((n) => n.type === 'MethodDeclaration')
      expect(methodNode).toBeDefined()
    })

    it('should parse method with throws', async () => {
      const java = `
public void readFile() throws IOException {
    // Read file
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find throws clause
      const throwsNode = tree.nodes.find((n) => n.type.includes('Throws'))
      expect(throwsNode).toBeDefined()
    })
  })

  describe('Generics', () => {
    it('should parse generic class', async () => {
      const java = `
public class Box<T> {
    private T value;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find type parameters
      const typeParamsNode = tree.nodes.find((n) => n.type === 'TypeParameters')
      expect(typeParamsNode).toBeDefined()
    })

    it('should parse generic method', async () => {
      const java = `
public <T> T getValue(T input) {
    return input;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find type parameters
      const typeParamsNode = tree.nodes.find((n) => n.type === 'TypeParameters')
      expect(typeParamsNode).toBeDefined()
    })

    it('should parse generic type usage', async () => {
      const java = 'List<String> names = new ArrayList<String>();'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find type arguments
      const typeArgsNode = tree.nodes.find((n) => n.type === 'TypeArguments')
      expect(typeArgsNode).toBeDefined()
    })
  })

  describe('Annotations', () => {
    it('should parse class annotation', async () => {
      const java = `
@Entity
@Table(name = "users")
public class User {
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find annotations
      const annotationNodes = tree.nodes.filter((n) => n.type.includes('Annotation'))
      expect(annotationNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse method annotation', async () => {
      const java = `
@Override
public String toString() {
    return "object";
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find annotation
      const annotationNode = tree.nodes.find((n) => n.type.includes('Annotation'))
      expect(annotationNode).toBeDefined()
    })
  })

  describe('Lambda Expressions', () => {
    it('should parse simple lambda', async () => {
      const java = 'Function<Integer, Integer> square = x -> x * x;'
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find lambda expression
      const lambdaNode = tree.nodes.find((n) => n.type === 'LambdaExpression')
      expect(lambdaNode).toBeDefined()
    })

    it('should parse lambda with block', async () => {
      const java = `
Consumer<String> printer = s -> {
    System.out.println(s);
};
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find lambda expression
      const lambdaNode = tree.nodes.find((n) => n.type === 'LambdaExpression')
      expect(lambdaNode).toBeDefined()
    })
  })

  describe('Comments', () => {
    it('should parse line comments', async () => {
      const java = `
// This is a comment
int x = 42;
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Find comment node
      const commentNode = tree.nodes.find((n) => n.type.includes('Comment') || n.data?.text?.includes('//'))
      expect(commentNode).toBeDefined()
    })

    it('should parse block comments', async () => {
      const java = `
/* This is a
   multi-line comment */
int x = 42;
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Should have parsed successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse javadoc comments', async () => {
      const java = `
/**
 * Adds two numbers
 * @param a First number
 * @param b Second number
 * @return Sum of a and b
 */
public int add(int a, int b) {
    return a + b;
}
      `
      const tree = await parseAsync(java)

      expect(tree.meta.language).toBe('java')

      // Should have parsed successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })
  })

  describe('API', () => {
    it('should create parser with factory', async () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(JavaParser)
    })

    it('should parse with standalone function', async () => {
      const tree = await parseAsync('int x = 42;')
      expect(tree.meta.language).toBe('java')
    })

    it('should parse async', async () => {
      const tree = await parseAsync('int x = 42;')
      expect(tree.meta.language).toBe('java')
    })

    it('should support plugins', async () => {
      let called = false
      const plugin = {
        transform: (tree: any) => {
          called = true
          return tree
        },
      }

      const parser = createParser()
      parser.use(plugin)
      await parser.parseAsync('int x = 42;')

      expect(called).toBe(true)
    })

    it('should support async plugins', async () => {
      let called = false
      const plugin = {
        transform: async (tree: any) => {
          called = true
          return tree
        },
      }

      const parser = createParser()
      parser.use(plugin)
      await parser.parseAsync('int x = 42;')

      expect(called).toBe(true)
    })

    it('should throw error for synchronous parse()', () => {
      expect(() => {
        parse('int x = 42;')
      }).toThrow(/WASM/)
    })

    it('should get last parsed tree', async () => {
      const parser = createParser()
      await parser.parseAsync('int x = 42;')
      const tree = parser.getTree()

      expect(tree).toBeDefined()
      expect(tree?.meta.language).toBe('java')
    })

    it('should support init() for pre-initialization', async () => {
      // init() should not throw
      await init()
      // Second call should be instant (cached)
      await init()
    })
  })
})

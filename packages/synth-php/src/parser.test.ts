/**
 * PHP Parser Tests
 */

import { describe, expect, it } from 'bun:test'
import { PhpParser, createParser, parse, parseAsync } from './parser.js'

describe('PhpParser', () => {
  describe('Basic Parsing', () => {
    it('should parse empty PHP', () => {
      const tree = parse('<?php ?>')
      expect(tree.meta.language).toBe('php')
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse simple echo', () => {
      const php = `<?php
echo "Hello, World!";
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')
      expect(tree.nodes[tree.root]).toBeDefined()

      // Should have program root and children
      const rootChildren = tree.nodes[tree.root]?.children
      expect(rootChildren.length).toBeGreaterThan(0)
    })

    it('should parse variable assignment', () => {
      const php = '<?php $x = 42; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find assignment or variable
      const varNode = tree.nodes.find(
        (n) => n.type.includes('Variable') || n.type.includes('Assignment')
      )
      expect(varNode).toBeDefined()
    })

    it('should parse function definition', () => {
      const php = `<?php
function greet($name) {
    return "Hello, " . $name;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find function definition
      const funcNode = tree.nodes.find((n) => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse class definition', () => {
      const php = `<?php
class Person {
    private $name;

    public function __construct($name) {
        $this->name = $name;
    }
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find class declaration
      const classNode = tree.nodes.find((n) => n.type === 'ClassDeclaration')
      expect(classNode).toBeDefined()
    })
  })

  describe('Variables', () => {
    it('should parse variable names', () => {
      const php = '<?php $name = "John"; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find variable name
      const varNode = tree.nodes.find(
        (n) => n.type === 'VariableName' || n.data?.text?.includes('$')
      )
      expect(varNode).toBeDefined()
    })

    it('should parse superglobals', () => {
      const php = '<?php $data = $_POST["name"]; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find variable usage
      const varNode = tree.nodes.find((n) => n.data?.text?.includes('$_POST'))
      expect(varNode).toBeDefined()
    })

    it('should parse variable variables', () => {
      const php = '<?php $name = "foo"; $$name = "bar"; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Should parse successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })
  })

  describe('Data Types', () => {
    it('should parse string literals', () => {
      const php = '<?php $text = "Hello, World!"; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find string node
      const stringNode = tree.nodes.find((n) => n.type === 'String' || n.type.includes('String'))
      expect(stringNode).toBeDefined()
    })

    it('should parse single quoted strings', () => {
      const php = "<?php $text = 'Hello'; ?>"
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find string node
      const stringNode = tree.nodes.find((n) => n.type === 'String' || n.type.includes('String'))
      expect(stringNode).toBeDefined()
    })

    it('should parse integer literals', () => {
      const php = '<?php $num = 42; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find integer node
      const intNode = tree.nodes.find((n) => n.type === 'Integer' || n.data?.text === '42')
      expect(intNode).toBeDefined()
    })

    it('should parse float literals', () => {
      const php = '<?php $pi = 3.14159; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find float node
      const floatNode = tree.nodes.find((n) => n.type === 'Float' || n.data?.text?.includes('.'))
      expect(floatNode).toBeDefined()
    })

    it('should parse boolean literals', () => {
      const php = `<?php
$flag1 = true;
$flag2 = false;
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find boolean nodes
      const boolNodes = tree.nodes.filter(
        (n) =>
          n.type === 'True' ||
          n.type === 'False' ||
          n.data?.text === 'true' ||
          n.data?.text === 'false'
      )
      expect(boolNodes.length).toBeGreaterThanOrEqual(2)
    })

    it('should parse arrays', () => {
      const php = '<?php $numbers = array(1, 2, 3, 4, 5); ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find array
      const arrayNode = tree.nodes.find((n) => n.type.includes('Array'))
      expect(arrayNode).toBeDefined()
    })

    it('should parse short array syntax', () => {
      const php = '<?php $numbers = [1, 2, 3, 4, 5]; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find array
      const arrayNode = tree.nodes.find((n) => n.type.includes('Array'))
      expect(arrayNode).toBeDefined()
    })

    it('should parse null', () => {
      const php = '<?php $value = null; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find null literal
      const nullNode = tree.nodes.find((n) => n.type === 'Null' || n.data?.text === 'null')
      expect(nullNode).toBeDefined()
    })
  })

  describe('Control Flow', () => {
    it('should parse if statement', () => {
      const php = `<?php
if ($x > 0) {
    echo "positive";
} elseif ($x < 0) {
    echo "negative";
} else {
    echo "zero";
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find if statement
      const ifNode = tree.nodes.find((n) => n.type === 'IfStatement')
      expect(ifNode).toBeDefined()
    })

    it('should parse for loop', () => {
      const php = `<?php
for ($i = 0; $i < 10; $i++) {
    echo $i;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find for statement
      const forNode = tree.nodes.find((n) => n.type === 'ForStatement')
      expect(forNode).toBeDefined()
    })

    it('should parse foreach loop', () => {
      const php = `<?php
foreach ($items as $item) {
    echo $item;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find foreach statement
      const foreachNode = tree.nodes.find((n) => n.type === 'ForeachStatement')
      expect(foreachNode).toBeDefined()
    })

    it('should parse foreach with key and value', () => {
      const php = `<?php
foreach ($array as $key => $value) {
    echo "$key: $value";
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find foreach statement
      const foreachNode = tree.nodes.find((n) => n.type === 'ForeachStatement')
      expect(foreachNode).toBeDefined()
    })

    it('should parse while loop', () => {
      const php = `<?php
while ($x < 10) {
    $x++;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find while statement
      const whileNode = tree.nodes.find((n) => n.type === 'WhileStatement')
      expect(whileNode).toBeDefined()
    })

    it('should parse do-while loop', () => {
      const php = `<?php
do {
    $x++;
} while ($x < 10);
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find do statement
      const doNode = tree.nodes.find((n) => n.type === 'DoStatement')
      expect(doNode).toBeDefined()
    })

    it('should parse switch statement', () => {
      const php = `<?php
switch ($day) {
    case "Monday":
        echo "Start of week";
        break;
    case "Friday":
        echo "End of week";
        break;
    default:
        echo "Midweek";
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find switch statement
      const switchNode = tree.nodes.find((n) => n.type.includes('Switch'))
      expect(switchNode).toBeDefined()
    })

    it('should parse try-catch', () => {
      const php = `<?php
try {
    riskyOperation();
} catch (Exception $e) {
    echo "Error: " . $e->getMessage();
} finally {
    cleanup();
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find try statement
      const tryNode = tree.nodes.find((n) => n.type === 'TryStatement')
      expect(tryNode).toBeDefined()
    })
  })

  describe('Functions', () => {
    it('should parse function with parameters', () => {
      const php = `<?php
function add($a, $b) {
    return $a + $b;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find function definition
      const funcNode = tree.nodes.find((n) => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse function with default parameters', () => {
      const php = `<?php
function greet($name = "World") {
    return "Hello, $name!";
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find function definition
      const funcNode = tree.nodes.find((n) => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse function with type hints', () => {
      const php = `<?php
function add(int $a, int $b): int {
    return $a + $b;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find function definition with types
      const funcNode = tree.nodes.find((n) => n.type === 'FunctionDefinition')
      expect(funcNode).toBeDefined()
    })

    it('should parse anonymous function', () => {
      const php = `<?php
$square = function($x) {
    return $x * $x;
};
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find anonymous function
      const funcNode = tree.nodes.find(
        (n) => n.type.includes('Anonymous') || n.type.includes('Function')
      )
      expect(funcNode).toBeDefined()
    })

    it('should parse arrow function', () => {
      const php = '<?php $square = fn($x) => $x * $x; ?>'
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find arrow function
      const arrowNode = tree.nodes.find(
        (n) => n.type.includes('Arrow') || n.data?.text?.includes('=>')
      )
      expect(arrowNode).toBeDefined()
    })
  })

  describe('Classes', () => {
    it('should parse class with properties', () => {
      const php = `<?php
class Person {
    public $name;
    private $age;
    protected $email;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find class declaration
      const classNode = tree.nodes.find((n) => n.type === 'ClassDeclaration')
      expect(classNode).toBeDefined()

      // Find property declarations
      const propNodes = tree.nodes.filter((n) => n.type.includes('Property'))
      expect(propNodes.length).toBeGreaterThanOrEqual(1)
    })

    it('should parse constructor', () => {
      const php = `<?php
class Person {
    public function __construct($name) {
        $this->name = $name;
    }
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find method with __construct
      const constructorNode = tree.nodes.find((n) => n.data?.text?.includes('__construct'))
      expect(constructorNode).toBeDefined()
    })

    it('should parse class methods', () => {
      const php = `<?php
class Calculator {
    public function add($a, $b) {
        return $a + $b;
    }

    private function validate($num) {
        return is_numeric($num);
    }
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find methods
      const methodNodes = tree.nodes.filter((n) => n.type.includes('Method'))
      expect(methodNodes.length).toBeGreaterThanOrEqual(1)
    })

    it('should parse static methods', () => {
      const php = `<?php
class Math {
    public static function square($x) {
        return $x * $x;
    }
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find static method
      const staticNode = tree.nodes.find((n) => n.data?.text?.includes('static'))
      expect(staticNode).toBeDefined()
    })

    it('should parse class inheritance', () => {
      const php = `<?php
class Dog extends Animal {
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find class with extends
      const classNode = tree.nodes.find((n) => n.type === 'ClassDeclaration')
      expect(classNode).toBeDefined()
    })

    it('should parse interface', () => {
      const php = `<?php
interface Drawable {
    public function draw();
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find interface declaration
      const interfaceNode = tree.nodes.find((n) => n.type === 'InterfaceDeclaration')
      expect(interfaceNode).toBeDefined()
    })

    it('should parse trait', () => {
      const php = `<?php
trait Loggable {
    public function log($message) {
        echo $message;
    }
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find trait declaration
      const traitNode = tree.nodes.find((n) => n.type === 'TraitDeclaration')
      expect(traitNode).toBeDefined()
    })
  })

  describe('Comments', () => {
    it('should parse line comments', () => {
      const php = `<?php
// This is a comment
$x = 42;
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Find comment node
      const commentNode = tree.nodes.find(
        (n) => n.type.includes('Comment') || n.data?.text?.includes('//')
      )
      expect(commentNode).toBeDefined()
    })

    it('should parse block comments', () => {
      const php = `<?php
/* This is a
   multi-line comment */
$x = 42;
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Should have parsed successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse hash comments', () => {
      const php = `<?php
# This is a hash comment
$x = 42;
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Should have parsed successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })

    it('should parse doc comments', () => {
      const php = `<?php
/**
 * Adds two numbers
 * @param int $a First number
 * @param int $b Second number
 * @return int Sum
 */
function add($a, $b) {
    return $a + $b;
}
?>`
      const tree = parse(php)

      expect(tree.meta.language).toBe('php')

      // Should have parsed successfully
      expect(tree.nodes[tree.root]).toBeDefined()
    })
  })

  describe('API', () => {
    it('should create parser with factory', () => {
      const parser = createParser()
      expect(parser).toBeInstanceOf(PhpParser)
    })

    it('should parse with standalone function', () => {
      const tree = parse('<?php $x = 42; ?>')
      expect(tree.meta.language).toBe('php')
    })

    it('should parse async', async () => {
      const tree = await parseAsync('<?php $x = 42; ?>')
      expect(tree.meta.language).toBe('php')
    })

    it('should support plugins', () => {
      let called = false
      const plugin = {
        transform: (tree: any) => {
          called = true
          return tree
        },
      }

      const parser = createParser()
      parser.use(plugin)
      parser.parse('<?php $x = 42; ?>')

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
      await parser.parseAsync('<?php $x = 42; ?>')

      expect(called).toBe(true)
    })

    it('should throw error for async plugin in sync parse', () => {
      const plugin = {
        transform: async (tree: any) => tree,
      }

      const parser = createParser()
      parser.use(plugin)

      expect(() => parser.parse('<?php $x = 42; ?>')).toThrow('async')
    })

    it('should get last parsed tree', () => {
      const parser = createParser()
      parser.parse('<?php $x = 42; ?>')
      const tree = parser.getTree()

      expect(tree).toBeDefined()
      expect(tree?.meta.language).toBe('php')
    })
  })
})

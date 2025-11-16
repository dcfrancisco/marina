# Compiler Architecture

The Marina compiler transforms Clipper-2025 source code into portable, executable bytecode.
This pipeline is designed to be:

* simple
* fast
* predictable
* testable
* VM-friendly
* extensible

## Compiler Pipeline

```
Source (.prg)
    ↓
Lexer
    ↓
Tokens
    ↓
Parser
    ↓
AST
    ↓
Compiler
    ↓
Bytecode (.bc)
    ↓
Marina VM Execute
```

Each subsystem is intentionally isolated so the language can evolve without breaking the VM.

## Lexer (Tokenizer)

The lexer converts raw characters into recognizable **tokens**.

### Responsibilities:

* remove whitespace
* classify identifiers
* detect keywords
* parse literals
* parse string escapes
* detect operators
* handle comments
* record line & column for error reporting

### Example:

Source:

```clipper
local a := 10 + 20
```

Token stream:

```
LOCAL
IDENT("a")
ASSIGN
NUMBER(10)
PLUS
NUMBER(20)
EOF
```

### Token Types:

```
IDENTIFIER
NUMBER
STRING
BOOLEAN
NIL
KEYWORD (function, local, return, import)
SYMBOL ((), {}, [], etc.)
OPERATOR (+, -, *, /, ==, :=)
```

## Parser

The parser converts tokens into an **Abstract Syntax Tree (AST)**.

### It uses:

* recursive descent (Pratt parsing for expressions)
* precedence rules
* clean error recovery

### AST Node Types:

* Program
* FunctionDeclaration
* Block
* VariableDeclaration
* Assignment
* BinaryExpression
* UnaryExpression
* CallExpression
* Literal
* Identifier
* ReturnStatement
* CodeblockLiteral

### Example AST:

Source:

```clipper
function add(a, b)
    return a + b
```

AST:

```
FunctionDeclaration(
  name="add",
  params=["a","b"],
  body=Return(
      BinaryExpression(
          left=Identifier("a"),
          op="+",
          right=Identifier("b")
      )
  )
)
```

## AST Design Goals

✔ Easy to walk
✔ Easy to convert to bytecode
✔ Human-readable for debugging
✔ Friendly for IDE/LSP integration
✔ Extendable for future features (classes, async, macros)

## Compiler (AST → Bytecode)

The compiler translates AST nodes into Marina VM bytecode.

### Responsibilities:

* allocate local variables
* resolve variable scope
* manage constant pool
* manage function table
* emit instructions in sequence
* track bytecode offsets for jumps
* verify correctness (arity, identifiers)

### Example: compile an expression

AST:

```
a + b * 2
```

Bytecode:

```
LOAD_LOCAL 0        ; a
LOAD_LOCAL 1        ; b
PUSH_CONST 2
MUL
ADD
```

Stack Machine Evaluation:

```
push a
push b
push 2
mul
add
```

### Compile-time Failures

Compiler detects:

* unknown identifiers
* wrong number of function args
* assigning to undeclared variable
* returning outside function
* duplicate parameters
* syntax errors passed from parser
* type errors in operators (optional future checks)

## Debug Information

Even though bytecode is portable, debug info is included:

* source file mapping
* line numbers
* function names
* instruction mapping

This allows future:

* breakpoints
* stepping
* debugging
* IDE integration

## Future Compiler Enhancements

### Inline Optimization

```clipper
x := 1 + 2 + 3
```

Becomes:

```
PUSH_CONST 6
```

### Constant Folding

Expressions with literals precomputed.

### Dead Code Elimination

Unused local variables or unreachable code removed.

### Tail Call Optimization

Safe tail recursion.

### Macro Expansion

AST macros transform code before bytecode generation.

## Error Reporting Philosophy

Errors must be:

* **precise**
* **readable**
* **actionable**
* **non-cryptic**

### Example:

```clipper
local x := 10 +
```

Compiler output:

```
Syntax Error: Unexpected end of expression
  at line 1, col 16
```

## Summary

Clipper-2025's compiler is:

* modern
* clean
* modular
* predictable
* extendable
* xBase-free

It is built to last the next 30 years.

# Syntax Overview

Clipper-2025 syntax emphasizes:

* lower-case keywords
* expression-based constructs
* simplified grammar
* no commands
* no legacy xBase clutter

## Basic Example

```clipper
function main()
    local a := 10
    local b := 20
    Print("Sum:", a + b)
return nil
```

## Function Definition

```clipper
function add(x, y)
    return x + y
```

## Variables

```clipper
local name := "Danny"
```

## Blocks

```clipper
{
    local x := 10
    Print(x)
}
```

## Codeblocks

```clipper
b := {|| x + 10 }
result := Eval(b)
```

## Whitespace & Indentation

Clipper-2025 adopts a minimalistic whitespace approach:

* Whitespace is **not** syntactically significant
* Newlines terminate expressions
* Indentation is stylistic only
* Blocks are explicitly delimited by `{ ... }`

Example:

```clipper
{
    local x := 1
    local y := 2
    return x + y
}
```

## Comments

```clipper
// single line

/* 
multi
line
*/
```

## Identifiers

Identifiers follow:

```
[a-zA-Z_][a-zA-Z0-9_]*
```

CamelCase is allowed, encouraged for functions:

```
customerLookup()
loadDataFile()
```

## Keywords

Reserved words:

```
function
local
return
import
nil
true
false
```

Future reserved:

```
class
method
try
catch
finally
await
spawn
```

## Expressions

### Arithmetic

```clipper
+, -, *, /, %
```

### Comparison

```clipper
==, !=, >, <, >=, <=
```

### Logical

```clipper
and, or, not
```

### String concatenation

```clipper
"Hello " + name
```

### Parentheses

```clipper
Print((1 + 2) * 3)
```

## Variables & Scoping

### LOCAL variables

Block-scoped:

```clipper
function example()
    local a := 10
    {
        local b := a * 2
        Print(b)
    }
    // b is not visible here
return nil
```

### FUTURE: Module-level variables

Introduced via:

```
export const MAX := 100
```

## Functions

Functions are first-class citizens.

### Declaration

```clipper
function multiply(a, b)
    return a * b
```

### Call

```clipper
Print(multiply(3, 4))
```

### Return

```clipper
return value
```

### Multiple return values (future)

```clipper
return x, y, z
```

## Codeblocks — A Clipper Legacy Preserved

Clipper's `{||}` block syntax becomes a core modern feature.

### Define:

```clipper
block := {|n| n * 2 }
```

### Execute:

```clipper
result := Eval(block, 5)
```

### Pass to functions:

```clipper
map(numbers, {|x| x * x })
```

### Blocks will support:

* closures
* captures
* async usage
* pipelines
* higher-order functions

Clipper was early.
Clipper-2025 finishes the idea.

## Types

Clipper-2025 uses simple dynamic types:

* number
* string
* boolean
* nil
* list (future)
* map/dict (future)
* object (future OOP)
* function/codeblock

### Dynamic typing example:

```clipper
local x := 10
x := "hello"
```

## Error Handling (Future)

Planned syntax:

```clipper
try
    risky()
catch err
    Print("Error:", err)
end
```

The VM already supports exception throwing.

## Differences From Clipper 5.x

| Clipper 5.x         | Clipper-2025            |
| ------------------- | ----------------------- |
| Commands            | No commands             |
| Workareas           | Removed                 |
| Preprocessor macros | Safe macro engine later |
| `@ SAY`             | No UI commands          |
| DBF-only            | Multi-DB engine         |
| .OBJ linking        | VM bytecode             |
| Uppercase style     | Lowercase idiomatic     |
| Codeblocks          | Enhanced                |
| Class(y) OOP        | Native OOP (future)     |

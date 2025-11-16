# Standard Library Reference

## Built-in Functions (v1.0)

These functions exist in every runtime.

### Core

```
Print(...)
PrintNoCR(...)
TypeOf(value)
Str(value)
Val(value)
Length(string)
```

### Math

```
Abs(n)
Sin(n)
Cos(n)
Tan(n)
Sqrt(n)
Round(n)
```

### String

```
Upper(s)
Lower(s)
Left(s, n)
Right(s, n)
Substr(s, start, len)
Trim(s)
```

### System

```
Now()
Env("VAR")
Sleep(ms)
```

### Blocks

```
Eval(block, args...)
```

## Standard Modules

### 1. core

* Print
* basic math
* types

### 2. string

* string manipulation, trimming, slicing

### 3. math

* complete math toolkit

### 4. system

* system info
* time
* environment

### 5. dbx

* DBF/CDX engine
* SQL backends (future)
* NoSQL backends (Mongo)

### 6. tui (optional)

* terminal UI
* cursor positioning
* direct console drawing

## Codeblock Specification

Clipper's most unique feature — preserved.

### Codeblock structure internally:

```rust
struct Block {
    function_id: usize,
    captured_locals: Vec<Value>
}
```

### Execution:

```
Eval(block, args...)
```

Supports:

* closures
* captured variables (future)
* passing as callbacks
* async usage (future)

## File Extensions

| Extension | Meaning                 |
| --------- | ----------------------- |
| `.prg`    | Clipper source code     |
| `.ch`     | Macro include file      |
| `.bc`     | Marina bytecode         |
| `.mod`    | Compiled module wrapper |
| `.cdx`    | Index file              |
| `.dbf`    | Database file           |

## CLI Commands (marina)

### Compile

```
marina compile hello.prg
```

Produces:

```
hello.bc
```

### Run

```
marina run hello.bc
```

### Build module

```
marina build mymodule/
```

### Inspect bytecode

```
marina dump file.bc
```

### Format source (future)

```
marina fmt file.prg
```

## Reserved Keywords (Complete)

```
function
local
return
import
true
false
nil
class
method
end
try
catch
finally
await
spawn
export
```

These ensure long-term language stability.

## Compatibility Matrix

| Feature     | Clipper 5.x  | Harbour   | Marina              |
| ----------- | ------------ | --------- | ------------------- |
| Macros      | dangerous    | dangerous | SAFE AST macros     |
| Workareas   | yes          | yes       | no                  |
| RDD         | yes          | yes       | module engines      |
| Commands    | yes          | yes       | no                  |
| Codeblocks  | yes          | yes       | enhanced            |
| VM          | no           | no        | yes                 |
| Async       | no           | no        | yes                 |
| SQL engines | no           | partial   | full                |
| MongoDB     | no           | no        | yes                 |
| GUI         | VO only      | partial   | AST GUI DSL         |
| Modules     | object files | limited   | full modern modules |

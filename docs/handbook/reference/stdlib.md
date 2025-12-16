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
Val(s)                      // Convert string to number
Upper(s)                    // Convert to uppercase
Lower(s)                    // Convert to lowercase
Left(s, n)                  // Get leftmost n characters
Right(s, n)                 // Get rightmost n characters
SubStr(s, start, len)       // Extract substring (1-based indexing)
Trim(s)                     // Remove leading & trailing spaces
RTrim(s)                    // Remove trailing spaces
LTrim(s)                    // Remove leading spaces
AllTrim(s)                  // Same as Trim(s)
Space(n)                    // Create string of n spaces
Replicate(s, n)             // Repeat string s, n times
Len(s)                      // Get string length
Chr(n)                      // Convert ASCII code to character
Asc(s)                      // Get ASCII code of first character
```

### System

```
Now()
Env("VAR")
Sleep(ms)
```

### Console/Terminal

```
SetPos(row, col)            // Set cursor position (0-based)
GotoXY(col, row)            // Same as SetPos but col, row order
OutStd(s)                   // Output string to stdout
ClearScreen()               // Clear entire screen
SavePos()                   // Save current cursor position
RestorePos()                // Restore saved cursor position
SetColor(n)                 // Set text color (0-15)
SetCursor(visible)          // Show/hide cursor (true/false)
```

#### Color Codes

| Code | Color          | Code | Color          |
|------|----------------|------|----------------|
| 0    | Black          | 8    | Bright Black   |
| 1    | Blue           | 9    | Bright Blue    |
| 2    | Green          | 10   | Bright Green   |
| 3    | Cyan           | 11   | Bright Cyan    |
| 4    | Red            | 12   | Bright Red     |
| 5    | Magenta        | 13   | Bright Magenta |
| 6    | Yellow (Brown) | 14   | Bright Yellow  |
| 7    | White (Gray)   | 15   | Bright White   |

*Default color is 7 (White/Gray)*

### Input

```
Inkey(timeout)              // Wait for keypress (seconds, 0=no wait)
GetInput(cDefault, nRow, nColumn, lSay, cPrompt)
    // Field input with editing
GetSecret(cDefault, nRow, nColumn, lSay, cPrompt)
    // Field input with editing
    // cDefault: default value (defines field length)
    // nRow: row position (optional)
    // nColumn: column position (optional)
    // lSay: true to display default before input (optional)
    // cPrompt: prompt string to display (optional)
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

| Feature     | Clipper 5.x  | Marina              |
| ----------- | ------------ | ------------------- |
| Macros      | dangerous    | SAFE AST macros     |
| Workareas   | yes          | no                  |
| RDD         | yes          | module engines      |
| Commands    | yes          | no                  |
| Codeblocks  | yes          | enhanced            |
| VM          | no           | yes                 |
| Async       | no           | yes                 |
| SQL engines | no           | full                |
| MongoDB     | no           | yes                 |
| GUI         | VO only      | AST GUI DSL         |
| Modules     | object files | full modern modules |

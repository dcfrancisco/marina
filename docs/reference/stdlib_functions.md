# Standard Library Functions

Complete reference for Marina/Clipper-2025 built-in functions.

## Core Functions

### Print(...)

Print values to standard output with newline.

```clipper
Print("Hello")
Print("Sum:", 10 + 20)
Print(42, true, "mixed types")
```

### TypeOf(value)

Returns the type of a value as a string.

```clipper
Print(TypeOf(42))        // "number"
Print(TypeOf("hello"))   // "string"
Print(TypeOf(true))      // "boolean"
Print(TypeOf(nil))       // "nil"
```

### Str(value)

Converts a value to string representation.

```clipper
local x := 42
local s := Str(x)
Print(s)  // "42"
```

### Val(string)

Converts a string to a number.

```clipper
local s := "123"
local n := Val(s)
Print(n + 10)  // 133
```

### Length(string)

Returns the length of a string.

```clipper
Print(Length("Hello"))  // 5
```

## Math Functions

### Abs(n)

Returns absolute value.

```clipper
Print(Abs(-10))   // 10
Print(Abs(5))     // 5
```

### Sqrt(n)

Returns square root.

```clipper
Print(Sqrt(16))   // 4
Print(Sqrt(25))   // 5
```

### Sin(n), Cos(n), Tan(n)

Trigonometric functions (radians).

```clipper
Print(Sin(0))      // 0
Print(Cos(0))      // 1
```

### Round(n)

Rounds to nearest integer.

```clipper
Print(Round(3.7))   // 4
Print(Round(3.2))   // 3
```

## String Functions

### Upper(s)

Converts string to uppercase.

```clipper
Print(Upper("hello"))  // "HELLO"
```

### Lower(s)

Converts string to lowercase.

```clipper
Print(Lower("WORLD"))  // "world"
```

### Left(s, n)

Returns leftmost n characters.

```clipper
Print(Left("Hello", 3))  // "Hel"
```

### Right(s, n)

Returns rightmost n characters.

```clipper
Print(Right("Hello", 3))  // "llo"
```

### SubStr(s, start, len)

Returns substring starting at position (1-based indexing).

```clipper
Print(SubStr("Hello World", 7, 5))  // "World"
Print(SubStr("Marina", 2, 3))       // "ari"
```

### Trim(s) / AllTrim(s)

Removes leading and trailing whitespace.

```clipper
Print(Trim("  hello  "))    // "hello"
Print(AllTrim("  test  "))  // "test"
```

### RTrim(s)

Removes trailing whitespace only.

```clipper
Print(RTrim("hello  "))  // "hello"
```

### LTrim(s)

Removes leading whitespace only.

```clipper
Print(LTrim("  hello"))  // "hello"
```

### Space(n)

Creates a string of n spaces.

```clipper
local str := Space(10)    // "          " (10 spaces)
local name := Space(30)   // Used for input fields
```

### Len(s)

Returns the length of a string or array.

```clipper
Print(Len("Hello"))         // 5
Print(Len({1, 2, 3, 4}))   // 4
```

### Replicate(s, n)

Repeats a string n times.

```clipper
Print(Replicate("*", 5))     // "*****"
Print(Replicate("Ha", 3))    // "HaHaHa"
```

### Chr(n)

Converts ASCII code to character.

```clipper
Print(Chr(65))   // "A"
Print(Chr(97))   // "a"
Print(Chr(32))   // " " (space)
```

### Asc(s)

Returns ASCII code of first character.

```clipper
Print(Asc("A"))      // 65
Print(Asc("hello"))  // 104 (for 'h')
```

## Console/Terminal Functions

### SetPos(row, col) / DevPos(row, col)

Positions cursor at specified screen coordinates (0-based).

```clipper
SetPos(10, 20)         // Row 10, Column 20
OutStd("Hello here!")
```

### OutStd(text)

Outputs text at current cursor position without newline.

```clipper
OutStd("Name: ")
OutStd(name)
```

### ClearScreen()

Clears the screen and homes cursor.

```clipper
ClearScreen()
SetPos(0, 0)
OutStd("Fresh screen!")
```

### GotoXY(col, row)

Alternative positioning (column, row order).

```clipper
GotoXY(20, 10)  // Column 20, Row 10
```

### SavePos()

Saves current cursor position.

```clipper
SavePos()
SetPos(20, 50)
OutStd("Temp message")
RestorePos()  // Back to saved position
```

### RestorePos()

Restores previously saved cursor position.

## Input Functions

### Inkey(timeout)

Reads a single keypress and returns ASCII code. Optional timeout in seconds (0 = wait indefinitely).

```clipper
local key := Inkey(0)      // Wait for key
if key == 27               // ESC key
    ? "Escape pressed!"
endif

if key == 13               // ENTER key
    ? "Enter pressed!"
endif
```

Common key codes:
- 13 = Enter
- 27 = Escape  
- 32 = Space
- 48-57 = Numbers 0-9
- 65-90 = Uppercase A-Z
- 97-122 = Lowercase a-z

### GetInput(cDefault, [nRow], [nColumn], [lSay], [cPrompt])

Displays an input field and reads user input. Returns edited string.

**Parameters:**
- `cDefault` - Default value and field length (required)
- `nRow` - Screen row for input (optional, uses current position)
- `nColumn` - Screen column for input (optional, uses current position)
- `lSay` - Display result in normal color when done (optional, default false)
- `cPrompt` - Text to display before input field (optional)

**Returns:** String containing user input, padded/truncated to default length

```clipper
// Simple input at current position
local name := Space(30)
name := GetInput(name)

// Input with position
local email := Space(40)
email := GetInput(email, 10, 20)

// Input with prompt
local city := Space(20)
city := GetInput(city, 12, 10, .F., "City: ")

// Complete form example
ClearScreen()
local custName := Space(30)
local custEmail := Space(40)
local custPhone := Space(15)

SetPos(5, 10)
OutStd("Name.....: ")
custName := GetInput(custName, 5, 22)

SetPos(7, 10)
OutStd("Email....: ")
custEmail := GetInput(custEmail, 7, 22)

SetPos(9, 10)
OutStd("Phone....: ")
custPhone := GetInput(custPhone, 9, 22)
```

**Notes:**
- Field length is determined by the length of `cDefault`
- Use `Space(n)` to create n-character input field
- Input is left-aligned and padded with spaces
- Press Enter to accept, Esc to cancel (returns default)
- Future: Full editing with arrow keys, insert/overwrite modes

## System Functions

### Now()

Returns current date/time (future - format TBD).

```clipper
Print(Now())
```

### Env(name)

Gets environment variable.

```clipper
Print(Env("USER"))
Print(Env("HOME"))
```

### Sleep(ms)

Sleep for milliseconds (future).

```clipper
Sleep(1000)  // sleep 1 second
```

## Codeblock Functions

### Eval(block, args...)

Executes a codeblock with arguments.

```clipper
local add := {|a, b| a + b}
local result := Eval(add, 10, 20)
Print(result)  // 30
```

```clipper
local greet := {|name| Print("Hello, " + name)}
Eval(greet, "Marina")  // Hello, Marina
```

## Operators

### Arithmetic

| Operator | Description    | Example     |
| -------- | -------------- | ----------- |
| `+`      | Addition       | `10 + 5`    |
| `-`      | Subtraction    | `10 - 5`    |
| `*`      | Multiplication | `10 * 5`    |
| `/`      | Division       | `10 / 5`    |
| `%`      | Modulo         | `10 % 3`    |
| `-`      | Negation       | `-x`        |

### Comparison

| Operator | Description           | Example  |
| -------- | --------------------- | -------- |
| `==`     | Equal                 | `x == 5` |
| `!=`     | Not equal             | `x != 5` |
| `>`      | Greater than          | `x > 5`  |
| `<`      | Less than             | `x < 5`  |
| `>=`     | Greater than or equal | `x >= 5` |
| `<=`     | Less than or equal    | `x <= 5` |

### Logical

| Operator | Description | Example         |
| -------- | ----------- | --------------- |
| `and`    | Logical AND | `x > 0 and y <` |
| `or`     | Logical OR  | `x < 0 or y >`  |
| `not`    | Logical NOT | `not flag`      |

### Assignment

| Operator | Description         | Example    |
| -------- | ------------------- | ---------- |
| `:=`     | Assignment          | `x := 10`  |
| `+=`     | Add and assign      | `x += 5`   |
| `-=`     | Subtract and assign | `x -= 5`   |
| `*=`     | Multiply and assign | `x *= 2`   |
| `/=`     | Divide and assign   | `x /= 2`   |

## Reserved Keywords

```
function
local
return
if
else
elseif
endif
while
endwhile
case
otherwise
endcase
import
export
true
false
nil
and
or
not
```

## Future Functions (Planned)

### Array Functions

```clipper
Len(array)           // array length
Push(array, value)   // append
Pop(array)           // remove last
Sort(array)          // sort in place
Map(array, block)    // transform
Filter(array, block) // filter
```

### File I/O

```clipper
FileOpen(path, mode)
FileRead(handle)
FileWrite(handle, data)
FileClose(handle)
```

### JSON

```clipper
JsonParse(string)
JsonStringify(object)
```

### HTTP

```clipper
HttpGet(url)
HttpPost(url, data)
```

## See Also

- [Language Syntax Reference](language_syntax.md) - Complete syntax guide
- [Getting Started Guide](../guides/getting_started.md) - Quick start tutorial
- [Handbook Standard Library](../handbook/reference/stdlib.md) - Detailed specification

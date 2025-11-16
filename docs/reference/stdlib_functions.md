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

### Substr(s, start, len)

Returns substring.

```clipper
Print(Substr("Hello World", 7, 5))  // "World"
```

### Trim(s)

Removes leading and trailing whitespace.

```clipper
Print(Trim("  hello  "))  // "hello"
```

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

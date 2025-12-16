# Getting Started with Clipper-2025

Welcome to Marina/Clipper-2025! This guide will help you start writing and running Clipper-2025 programs.

## Installation

Currently Marina is built from source:

```bash
# Clone the repository
git clone https://github.com/dcfrancisco/marina.git
cd marina

# Build the compiler
cargo build --release

# The binaries will be in target/release/
# - clipper (main compiler/interpreter)
# - marina-lsp (language server)
# - marina-fmt (code formatter)
# - marina-dap (debugger - stub)
```

## Your First Program

Create a file `hello.prg`:

```clipper
function main()
    Print("Hello, Clipper-2025!")
return nil
```

Run it:

```bash
./target/release/clipper hello.prg
```

Output:
```
Hello, Clipper-2025!
```

## Basic Syntax

### Variables

```clipper
function main()
    local name := "Marina"
    local version := 1.0
    local ready := true
    
    Print("Language:", name)
    Print("Version:", version)
    Print("Ready:", ready)
return nil
```

### Functions

```clipper
function add(a, b)
    return a + b

function main()
    local result := add(10, 20)
    Print("Result:", result)
return nil
```

### Conditionals

```clipper
function main()
    local age := 25
    
    if age >= 21
        Print("Adult")
    else
        Print("Minor")
    endif
return nil
```

### Loops

```clipper
function main()
    local i := 1
    
    while i <= 5
        Print("Count:", i)
        i := i + 1
    endwhile
return nil
```

### Arrays (Future)

Arrays are planned for Phase 2:

```clipper
function main()
    local numbers := [1, 2, 3, 4, 5]
    Print("First:", numbers[1])
return nil
```

## Codeblocks

Clipper's most powerful feature:

```clipper
function main()
    local double := {|n| n * 2}
    local result := Eval(double, 5)
    Print("Result:", result)  // 10
return nil
```

## REPL Mode

Run the compiler without arguments for interactive mode:

```bash
./target/release/clipper
```

Try commands:

```
>>> Print("Hello")
Hello
>>> local x := 42
>>> Print(x * 2)
84
>>> exit
```

## Compiler Flags

```bash
# Show tokens
./target/release/clipper -t program.prg

# Show AST
./target/release/clipper -a program.prg

# Show disassembly
./target/release/clipper -d program.prg
```

## Console Colors

Marina supports console colors using the `SetColor()` function. Here are the available color codes:

| Code | Color          | Example Usage                    |
|------|----------------|----------------------------------|
| 0    | Black          | `SetColor(0)`                    |
| 1    | Blue           | `SetColor(1)`                    |
| 2    | Green          | `SetColor(2)`                    |
| 3    | Cyan           | `SetColor(3)`                    |
| 4    | Red            | `SetColor(4)`                    |
| 5    | Magenta        | `SetColor(5)`                    |
| 6    | Yellow (Brown) | `SetColor(6)`                    |
| 7    | White (Gray)   | `SetColor(7)` - Default          |
| 8    | Bright Black   | `SetColor(8)`                    |
| 9    | Bright Blue    | `SetColor(9)`                    |
| 10   | Bright Green   | `SetColor(10)`                   |
| 11   | Bright Cyan    | `SetColor(11)`                   |
| 12   | Bright Red     | `SetColor(12)`                   |
| 13   | Bright Magenta | `SetColor(13)`                   |
| 14   | Bright Yellow  | `SetColor(14)`                   |
| 15   | Bright White   | `SetColor(15)`                   |

### Color Example

```clipper
function main()
    SetColor(12)  // Bright Red
    OutStd("This is red!")
    SetColor(7)   // Reset to default
    OutStd("Back to normal")
return nil
```

See [color_demo.prg](../../examples/color_demo.prg) for more examples.

## Next Steps

- **Learn the language:** Read [Language Syntax Reference](../reference/language_syntax.md)
- **Understand modules:** See [Writing Modules Guide](writing_modules.md)
- **Work with databases:** See [Database Programming Guide](database_programming.md)
- **Deep dive:** Read the complete [Handbook](../handbook/README.md)

## Examples

Check the `examples/` folder for more programs:

- `simple.prg` - Basic arithmetic
- `conditionals.prg` - If/else statements
- `loops.prg` - While loops
- `simple_func.prg` - Function definitions
- `strings.prg` - String operations
- `arrays.prg` - Array examples (future)

## Common Issues

### "Unknown identifier"

Make sure variables are declared with `local`:

```clipper
// ❌ Wrong
x := 10

// ✅ Correct
local x := 10
```

### "Function not found"

Built-in functions are case-sensitive:

```clipper
// ❌ Wrong
print("hello")

// ✅ Correct
Print("hello")
```

### "Unexpected token"

Remember to use `:=` for assignment:

```clipper
// ❌ Wrong
local x = 10

// ✅ Correct
local x := 10
```

## Getting Help

- **Documentation:** [docs/README.md](../README.md)
- **Issues:** [GitHub Issues](https://github.com/dcfrancisco/marina/issues)
- **Examples:** [examples/](../../examples/)

# Quick Start Guide

## Installation

```bash
cd /path/to/clipper2025_bootstrap
cargo build --release
```

## Running Programs

### Run a Clipper program
```bash
cargo run examples/simple.prg
```

Or use the compiled binary:
```bash
./target/debug/clipper examples/simple.prg
```

### Debug Options

Show tokens:
```bash
cargo run -- -t examples/simple.prg
```

Show AST:
```bash
cargo run -- -a examples/simple.prg
```

Show bytecode disassembly:
```bash
cargo run -- -d examples/simple.prg
```

### REPL Mode

```bash
cargo run repl
```

Then type Clipper expressions:
```
> LOCAL x := 10
> LOCAL y := 20
> ? "Sum:", x + y
Sum:30
> exit
```

## Example Programs

Try these examples in the `examples/` directory:

1. **simple.prg** - Basic arithmetic and variables
2. **loops.prg** - FOR, WHILE, and DO WHILE loops
3. **strings.prg** - String operations
4. **arrays.prg** - Array creation and indexing

## Writing Your Own Programs

Create a file with `.prg` extension:

```clipper
// my_program.prg
LOCAL name, age

name := "Alice"
age := 25

? "Hello,", name
? "You are", age, "years old"

FOR i := 1 TO 5
    ? "Count:", i
NEXT
```

Run it:
```bash
cargo run my_program.prg
```

## Language Basics

### Variables
```clipper
LOCAL x, y, z         // Local variables
x := 10
y := 20
z := x + y
```

### Output
```clipper
? "Hello, World!"     // Print with newline
? "Value:", x         // Print multiple values
```

### Control Flow
```clipper
IF x > 10
    ? "Greater than 10"
ELSE
    ? "10 or less"
ENDIF

WHILE x < 100
    x := x + 1
ENDDO

FOR i := 1 TO 10 STEP 2
    ? i
NEXT
```

### Arrays
```clipper
LOCAL arr
arr := {10, 20, 30, 40}
? arr[0]              // Access first element
```

## Getting Help

For more information, see:
- README.md - Full documentation
- TEST_RESULTS.md - Test results and known limitations
- examples/ - Example programs

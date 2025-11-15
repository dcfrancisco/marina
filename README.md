# Clipper Compiler and Virtual Machine

A Rust-based compiler and virtual machine for the Clipper programming language.

## Features

- **Lexical Analysis**: Tokenizes Clipper source code
- **Parser**: Builds Abstract Syntax Tree (AST) from tokens
- **Compiler**: Generates stack-based bytecode from AST
- **Virtual Machine**: Executes compiled bytecode

## Supported Language Features

### Data Types
- Numbers (integers and floating-point)
- Strings
- Booleans (TRUE/FALSE)
- Arrays
- NIL

### Variables
- LOCAL - Local variables
- STATIC - Static variables
- PRIVATE - Private variables
- PUBLIC - Public variables

### Control Structures
- IF/ELSE/ENDIF
- WHILE/ENDDO
- DO WHILE
- FOR/TO/STEP/NEXT
- LOOP/ENDLOOP
- EXIT

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^` (power)
- Comparison: `=`, `!=`, `<>`, `<`, `>`, `<=`, `>=`
- Logical: `AND`, `OR`, `NOT`
- Assignment: `:=`, `=`

### Functions
- Function definition with FUNCTION/PROCEDURE
- Function calls with parameters
- RETURN statement

### Database Operations (stubs)
- USE - Open database
- DBSKIP - Move record pointer
- DBGOTOP - Go to first record
- DBGOBOTTOM - Go to last record
- DBSEEK - Seek record
- REPLACE - Update field

## Usage

### Compile and Run a Program

```bash
cargo run examples/simple.prg
```

### Show Tokens

```bash
cargo run -- -t examples/simple.prg
```

### Show AST

```bash
cargo run -- -a examples/simple.prg
```

### Show Disassembled Bytecode

```bash
cargo run -- -d examples/simple.prg
```

### REPL Mode

```bash
cargo run repl
```

## Example Programs

See the `examples/` directory for sample Clipper programs:

- `simple.prg` - Basic arithmetic and variables
- `loops.prg` - FOR, WHILE, and DO WHILE loops
- `strings.prg` - String operations
- `arrays.prg` - Array creation and indexing
- `conditionals.prg` - IF/ELSE conditional logic
- `factorial.prg` - Recursive factorial function (not yet working - functions WIP)

## Project Structure

```
src/
├── main.rs       - CLI and REPL
├── token.rs      - Token types and definitions
├── lexer.rs      - Lexical analyzer
├── ast.rs        - Abstract Syntax Tree nodes
├── parser.rs     - Recursive descent parser
├── bytecode.rs   - Bytecode instructions and values
├── compiler.rs   - AST to bytecode compiler
└── vm.rs         - Virtual machine executor
```

## Building

```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```

## Example Clipper Code

```clipper
FUNCTION Factorial(n)
    LOCAL result
    
    IF n <= 1
        result := 1
    ELSE
        result := n * Factorial(n - 1)
    ENDIF
    
RETURN result

LOCAL num := 5
? "Factorial of", num, "is", Factorial(num)
```

## Architecture

1. **Lexer** - Scans source code and produces tokens
2. **Parser** - Builds an AST from tokens using recursive descent
3. **Compiler** - Traverses AST and generates bytecode instructions
4. **VM** - Stack-based virtual machine that executes bytecode

## License

MIT

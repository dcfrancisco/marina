# Marina 🚢

```
          _~
       _~ )_)_~
    _~ )_))_))_~
   )_))_))_))_))_)
   _!__!__!__!__!__/o
   \_______________/
~~~~~~~~~~~~~~~~~~~~~~~
```


**A modern Rust-based compiler and virtual machine for the Clipper programming language**

[![Sponsor](https://img.shields.io/badge/Sponsor-Buy%20Me%20a%20Coffee-yellow.svg)](https://www.buymeacoffee.com/dcfrancisco) [![Sponsor](https://img.shields.io/badge/Sponsor-Bitcoin-orange.svg)](https://mempool.space/address/bc1qz7hlw44akh8vxfjjt5njnyld8ut6hc3gmz20dr)

---

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

Current runtime model:
- `LOCAL` uses local storage
- `STATIC`, `PRIVATE`, and `PUBLIC` currently compile through the shared global storage path

### Control Structures
- IF/ELSE/ENDIF
- WHILE/ENDDO
- DO WHILE
- FOR/TO/STEP/NEXT
- LOOP/ENDLOOP with EXIT
- CASE/ENDCASE with OTHERWISE

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^` (power)
- Augmented assignment: `+=`, `-=`, `*=`, `/=`
- Increment/Decrement: `++`, `--`
- Comparison: `=`, `!=`, `<>`, `<`, `>`, `<=`, `>=`
- Logical: `AND`, `OR`, `NOT`
- Compatibility aliases: `&&`, `||`
- Assignment: `:=`, `=`

### Arrays
- Array literals: `{1, 2, 3}`
- Array indexing: `arr[0]`
- Indexed assignment: `arr[1] := 99`

### Functions
- User-defined functions with FUNCTION/PROCEDURE keyword
- Built-in functions:
  - **Output**: Print/?, OutStd
  - **Console**: SetPos, GotoXY, ClearScreen, SavePos, RestorePos
  - **String**: Val, Space, Len, SubStr, Trim, RTrim, LTrim, AllTrim, Replicate, Chr, Asc
  - **Input**: Inkey, GetInput, GetSecret
- RETURN statement

### Minimal Imports
- `IMPORT "string"`
- `IMPORT "math"`
- `IMPORT "console"`
- `IMPORT "input"`
- `IMPORT "system"`
- Namespaced built-in calls such as `string.len("abc")` and `math.sqrt(81)`

### Database Operations (stubs only)
- USE, DBSKIP, DBGOTOP, DBGOBOTTOM, DBSEEK, REPLACE
- (Not yet implemented - placeholders only)

## Project Status

**Phase 1 (Core Stability):** ✅ Complete  
**Phase 2 (Language Expansion):** ✅ Complete
- CASE statements, augmented operators, increment/decrement, indexed assignment
- Nested `CASE` and `ELSEIF` carryovers closed
**Phase 2.5 (Refactoring):** ✅ Complete  
- Modular structure: parser/, compiler/, vm/
- Bug fixes: Dup opcode, CASE compilation, Halt loop

**Phase 3 (Runtime & Module Stabilization):** ✅ Release-candidate ready pending hosted CI confirmation
- Freeze bytecode/runtime behavior for the current language surface
- Finalize function call and VM execution semantics
- Expand runtime validation around function calls, returns, and entrypoints
- Keep module/import work minimal and honest
- Minimal built-in import namespaces are now supported; dynamic/lazy modules are not
- `.ch` preprocessing and `@ SAY/GET` compatibility are explicitly deferred until the future macro/preprocessor phase
- Tooling present but not phase-defining: `marina-fmt`, `marina-docs`, experimental `marina-lsp`, stub `marina-dap`

See the [supported-feature reference](docs/reference/SUPPORTED_FEATURES.md) and
[unreleased notes](docs/RELEASE_NOTES.md) for the release boundary.

**Phase 5 (Ecosystem & IDE):** 📋 Future
- VSCode extension (when language is mature)
- Package system (.cpkg/.cjar)
- Comprehensive standard library

**Future Macro/Preprocessor Direction:** 📋 Post-Phase-3
- `.ch` files, `#command`, `#translate`, and `#include`
- `@ SAY/GET`-style compatibility
- Compatibility target should prefer Clipper 5.2 behavior first, with 5.3 considered secondary
- The `include/` header files in this repository are being used as Clipper 5.2 reference material for that future compatibility work

## Usage

### Main Compiler/Interpreter (`clipper`)

```bash
# Compile and run a program
cargo run --bin clipper -- examples/simple.prg

# Show tokens
cargo run --bin clipper -- -t examples/simple.prg

# Show AST
cargo run --bin clipper -- -a examples/simple.prg

# Show disassembled bytecode
cargo run --bin clipper -- -d examples/simple.prg

# REPL mode
cargo run --bin clipper -- repl
```

### Code Formatter (`marina-fmt`)

```bash
# Preferred (integrated with the main CLI)
cargo run --bin clipper -- fmt --check examples/simple.prg
cargo run --bin clipper -- fmt examples/simple.prg

# Direct formatter binary (same behavior)
cargo run --bin marina-fmt -- --check examples/simple.prg
cargo run --bin marina-fmt -- examples/simple.prg
```

### Language Server (`marina-lsp`)

```bash
# Build the LSP server
cargo build --bin marina-lsp --features lsp

# The LSP communicates via stdio and is invoked by editors
./target/debug/marina-lsp
```

### Documentation Tools (`marina-docs`)

```bash
# Render markdown to HTML
cargo run --bin marina-docs -- html README.md

# Render markdown to PDF
cargo run --bin marina-docs -- pdf README.md

# Build a combined PDF from a docs/ directory
cargo run --bin marina-docs -- pdf docs/
```

## Example Programs

See the `examples/` directory for sample Clipper programs:

- `simple.prg` - Basic arithmetic and variables
- `loops.prg` - FOR, WHILE, and DO WHILE loops
- `strings.prg` - String operations
- `arrays.prg` - Array creation and indexing
- `conditionals.prg` - IF/ELSE conditional logic
- `case_demo.prg` - CASE/OTHERWISE/ENDCASE examples
- `augmented_ops.prg` - Augmented assignment operators (+=, -=, ++, --)
- `simple_func.prg` - Simple function example (Add function)
- `factorial.prg` - Recursive factorial function
- `console_demo.prg` - Console positioning and ANSI escape codes demo
- `xmas_tree.prg` - Christmas tree ASCII art using Replicate() and SetPos()
- `hanoi.prg` - Tower of Hanoi with recursive algorithm and ASCII animation (with user input for disk count)
- `string_functions.prg` - Comprehensive demo of all string manipulation functions
- `val_demo.prg` - Val() function demonstration with user input
- `simple_input.prg` - Simple input using GetInput() function
- `input_demo.prg` - Customer registration form demonstrating field input
- `password_demo.prg` - Password/PIN entry using GetSecret() for hidden input
- `login_demo.prg` - Secure login system with GetSecret() and attempt limiting
- `phase3_runtime_validation.prg` - Runtime validation for `ELSEIF`, nested `CASE`, recursion, `Main()`, and minimal imports

### Running non-interactive examples

Some programs in `examples/` are interactive and will prompt for input (e.g. via `GetInput`, `GetSecret`, or `Inkey`).
To avoid running interactive examples by accident, you can run only the non-interactive ones:

```bash
cargo build --release
./scripts/run_examples_noninteractive.sh
```

Useful options:

```bash
./scripts/run_examples_noninteractive.sh --dry-run
./scripts/run_examples_noninteractive.sh --verbose
```

## Project Structure

```
src/
├── lib.rs            - Marina library (shared by all binaries)
├── bin/              - All executable binaries
│   ├── clipper.rs    - Main compiler/interpreter (CLI, REPL)
│   ├── marina-lsp.rs - Language Server Protocol
│   ├── marina-dap.rs - Debug Adapter Protocol (stub)
│   ├── marina-fmt.rs - Code formatter (MVP)
│   └── marina-docs.rs - Documentation renderer
├── docs/             - Documentation rendering subsystem
│   ├── mod.rs        - Shared entry points for docs output
│   ├── markdown.rs   - Lightweight markdown parser
│   ├── html.rs       - HTML renderer
│   ├── pdf.rs        - Standalone PDF renderer
│   ├── themes.rs     - HTML theme definitions
│   └── config.rs     - CLI and renderer configuration
├── token.rs          - Token types and definitions
├── lexer.rs          - Lexical analyzer
├── ast.rs            - Abstract Syntax Tree nodes
├── bytecode.rs       - Bytecode instructions and values
├── parser/           - Modularized parser
│   ├── mod.rs        - Parser struct and utilities
│   ├── statements.rs - Statement parsing
│   └── expressions.rs - Expression parsing
├── compiler/         - Modularized compiler
│   ├── mod.rs        - Compiler struct and utilities
│   ├── statements.rs - Statement compilation
│   └── expressions.rs - Expression compilation
└── vm/               - Modularized VM
    ├── mod.rs        - VM struct and run loop
    ├── opcodes.rs    - Opcode execution
    └── stack.rs      - Stack operations

tests/
├── lexer_tests.rs    - 7 tests
├── parser_tests.rs   - 12 tests
├── compiler_tests.rs - 11 tests
├── phase2_tests.rs   - 14 tests (13 passing, 1 ignored)
└── vm_tests.rs       - 19 tests
```


## Development Install

To build Marina from source, you need the Rust toolchain installed (https://rustup.rs). This is only required for development and building binaries.

To install the development binaries to your user directory, run:

```bash
./scripts/install-dev.sh
```

This will build and symlink the binaries to `$HOME/.marina/bin`, including `marina-docs`.

**Note:** Once built, you do NOT need Rust installed to run the binaries on other machines. The binaries are self-contained native executables.

---

## Building

```bash
# Build all binaries
cargo build --release

# Build specific binaries
cargo build --bin clipper --release
cargo build --bin marina-fmt --release
cargo build --bin marina-dap --release
cargo build --bin marina-lsp --features lsp --release
cargo build --bin marina-docs --release
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

Marina follows a classic compiler pipeline architecture:

1. **Lexer** (`lexer.rs`) - Scans source code and produces tokens
2. **Parser** (`parser/`) - Builds an AST from tokens using recursive descent with operator precedence
3. **Compiler** (`compiler/`) - Traverses AST and generates bytecode instructions with constant pool
4. **VM** (`vm/`) - Stack-based virtual machine that executes bytecode with call frames

### Bytecode Format

The compiler generates a `Chunk` containing:
- **Instructions**: Vec of OpCode + optional operand (constant index, jump target, etc.)
- **Constants**: Pool of literal values (numbers, strings, booleans, arrays)

The VM executes instructions using:
- **Stack**: For expression evaluation and temporary values
- **Globals**: HashMap for global variables
- **Locals**: Per-function local variable storage
- **Call Frames**: For function invocation (in progress)

### Future Tooling

**Current tooling:** `clipper fmt` and `marina-docs` are implemented; `marina-lsp`
is experimental and `marina-dap` is a stub. Tooling is not a Phase 3 exit gate.

**Phase 5 (IDE Integration - when language is mature):**
- **VSCode Extension** - Separate TypeScript project with syntax highlighting
- **Package System** - Library distribution and dependency management
- **Standard Library** - Comprehensive built-in functions

---

## Support This Project

☕ **Keep me caffeinated so I can keep debugging things you won't have to** — [buy me a coffee](https://www.buymeacoffee.com/dcfrancisco) 🙌

---

## License

[MIT License](LICENSE) - Copyright (c) 2025 Danny Francisco

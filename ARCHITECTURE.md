# Architecture

This document describes the architecture that is implemented in the repository today.

## System Shape

Marina is currently a **single-process compiler and bytecode VM** for `.prg` source files.

The implemented pipeline is:

```text
source (.prg)
  -> lexer
  -> parser
  -> AST
  -> bytecode compiler
  -> stack-based VM
```

There is no implemented multi-file build graph, import resolver, bytecode serialization format, or lazy module loader yet.

## Core Components

### Front End

- [`src/lexer.rs`](/Users/dannyfrancisco/codes/marina/src/lexer.rs) tokenizes source text into `Token`s
- [`src/parser/`](/Users/dannyfrancisco/codes/marina/src/parser) builds the AST
- [`src/ast.rs`](/Users/dannyfrancisco/codes/marina/src/ast.rs) defines expression and statement nodes
- [`src/diagnostics.rs`](/Users/dannyfrancisco/codes/marina/src/diagnostics.rs) supports parser diagnostics collection

The parser is still single-file and statement-oriented. It supports functions, arrays, loop forms, and `CASE`, but not imports, macros, or module declarations.

### Compiler

- [`src/compiler/`](/Users/dannyfrancisco/codes/marina/src/compiler)
- [`src/bytecode.rs`](/Users/dannyfrancisco/codes/marina/src/bytecode.rs)

The compiler lowers the AST into a stack-machine bytecode format. It emits:

- Constant loads
- Arithmetic and logical ops
- Variable reads and writes
- Branches and loop jumps
- Function calls and returns
- Array creation and indexing
- Database opcodes for currently stubbed statements

Function addresses are tracked in a function table keyed by name.

## Runtime

### Virtual Machine

- [`src/vm/mod.rs`](/Users/dannyfrancisco/codes/marina/src/vm/mod.rs)
- [`src/vm/opcodes.rs`](/Users/dannyfrancisco/codes/marina/src/vm/opcodes.rs)
- [`src/vm/stack.rs`](/Users/dannyfrancisco/codes/marina/src/vm/stack.rs)

The VM is stack-based and maintains:

- An evaluation stack
- A locals vector
- A globals map
- A call-frame stack
- An instruction pointer
- Basic console cursor state

Execution model today:

- Top-level code runs first
- If `Main()` or `main()` exists, the VM executes top-level initialization and then invokes that entrypoint
- User-defined function calls create call frames
- Built-in functions are dispatched inside the VM by case-insensitive name matching

### Native Runtime Surface

Marina does not yet have a general native module ABI.

What it does have is an internal built-in function dispatch path for console, input, string, math, and timing helpers. These are runtime-integrated and part of the current execution model, but they are not the same thing as a finished native extension system.

## Tooling Components

### Main CLI

- [`src/bin/clipper.rs`](/Users/dannyfrancisco/codes/marina/src/bin/clipper.rs)

Supports:

- Running programs
- Token dump
- AST dump
- Bytecode disassembly
- REPL
- Formatter subcommand

### Formatter

- [`src/formatter.rs`](/Users/dannyfrancisco/codes/marina/src/formatter.rs)
- [`src/bin/marina-fmt.rs`](/Users/dannyfrancisco/codes/marina/src/bin/marina-fmt.rs)

The formatter is an MVP. It normalizes indentation and lowercases keywords while preserving comments and blank lines. It is not yet a full structural pretty-printer.

### Documentation Renderer

- [`src/docs/`](/Users/dannyfrancisco/codes/marina/src/docs)
- [`src/bin/marina-docs.rs`](/Users/dannyfrancisco/codes/marina/src/bin/marina-docs.rs)

This subsystem renders Markdown to HTML or PDF and is covered by dedicated tests.

### LSP and DAP

- [`src/bin/marina-lsp.rs`](/Users/dannyfrancisco/codes/marina/src/bin/marina-lsp.rs)
- [`src/bin/marina-dap.rs`](/Users/dannyfrancisco/codes/marina/src/bin/marina-dap.rs)

Current state:

- `marina-lsp` exists behind the `lsp` Cargo feature and implements initialization, document sync, diagnostics, completion, and hover
- `marina-dap` is a stub binary that prints planned features

These are not part of the Phase 3 completion criteria and should not drive the current release gate.

### VS Code Extension

- [`vscode/marina/package.json`](/Users/dannyfrancisco/codes/marina/vscode/marina/package.json)
- [`vscode/marina/syntaxes/marina.tmLanguage.json`](/Users/dannyfrancisco/codes/marina/vscode/marina/syntaxes/marina.tmLanguage.json)
- [`vscode/marina/language-configuration.json`](/Users/dannyfrancisco/codes/marina/vscode/marina/language-configuration.json)

The extension currently provides:

- Language registration for `.prg`
- Syntax highlighting
- Language configuration

It does not currently wire in `marina-lsp`, formatting, debugging, tasks, or commands.

## Known Architectural Gaps

These are the important missing pieces relative to the intended Phase 3 target:

- No implemented module/import system
- No lazy loading path
- No bytecode persistence format
- No source-to-bytecode mapping for debugging
- No distinct runtime model for `STATIC`, `PRIVATE`, and `PUBLIC`
- Database statements are present in the surface language but not backed by a real engine
- `ELSEIF` and nested `CASE` are not fully stabilized

## Architectural Boundaries

Phase 3 should preserve these boundaries:

- The core VM and compiler stay small and predictable
- Tooling does not define language scope
- Database, networking, async, and ecosystem work stay out of the release gate
- Module design should land only at the level needed to stabilize imports and runtime loading

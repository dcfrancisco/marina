# **Marina Architecture Overview**

*The modern toolchain powering Clipper-2025.*

Marina is the **compiler, virtual machine, module system, and runtime** for the Clipper-2025 language.
This document explains how Marina is structured, how each subsystem works, and how they interact.

---

# 🌊 **1. Architecture Summary**

```
   ┌──────────┐
   │  Source  │  .prg (Clipper-2025)
   └─────┬────┘
         │
         ▼
   ┌──────────┐
   │   Lexer   │  tokens
   └─────┬────┘
         │
         ▼
   ┌──────────┐
   │  Parser   │  AST
   └─────┬────┘
         │
         ▼
   ┌────────────┐
   │   Compiler  │  bytecode
   └─────┬──────┘
         │
         ▼
   ┌──────────┐
   │    VM     │  execution
   └──────────┘
```

Each component is independent and testable.

---

# 🟦 **2. Lexer**

The lexer converts raw source text into tokens.

### Responsibilities

* handle whitespace
* handle comments
* generate identifiers
* detect numbers, strings, operators
* error on unknown tokens
* provide line/column info

### Output

```
Token::Identifier("add")
Token::LParen
Token::Number(10)
...
```

---

# 🟥 **3. Parser**

The parser converts tokens into an **AST (Abstract Syntax Tree)**.

### Responsibilities

* validate grammar
* build expression tree
* handle function definitions
* handle blocks
* validate syntax
* track error positions

### Example AST (simplified)

```
FunctionDefinition {
    name: "add",
    params: ["a", "b"],
    body: BinaryOp(Add,
        Identifier("a"),
        Identifier("b")
    )
}
```

Parser uses Pratt / recursive descent techniques.

---

# 🟩 **4. Compiler**

The compiler transforms AST into **Marina bytecode**.

### Responsibilities

* allocate locals
* manage stack behavior
* translate expressions into opcodes
* generate function call frames
* ensure scope correctness
* emit instructions sequentially

### Example Bytecode

```
LOAD_LOCAL 0
LOAD_LOCAL 1
ADD
RETURN
```

---

# 🟧 **5. Bytecode Format**

Bytecode is a simple stack-based instruction set.

### Example Instruction Set

```
PUSH_CONST
LOAD_LOCAL
STORE_LOCAL
ADD, SUB, MUL, DIV
JMP, JMP_IF_FALSE
CALL
RETURN
```

Bytecode is stored in a `.bc` file (planned).

---

# 🟫 **6. Virtual Machine (VM)**

The VM executes Clipper-2025 bytecode.

### VM Responsibilities

* operand stack
* call stack
* instruction pointer
* local variable frames
* module linking
* garbage collection (future)
* built-in functions
* error mechanism

### VM Design Goals

* portable
* fast
* simple
* deterministic
* embed-friendly (Rust library)
* WASM-friendly (future)

---

# 🟩 **7. Runtime**

The runtime provides built-in functions:

### Standard Runtime Functions

* Print
* Math.*
* String.*
* List.* (future)
* Dict.* (future)
* DB API (module-based)
* IO functions
* System functions

The runtime is accessible from the VM via opcode dispatch.

---

# 🟪 **8. Module System**

Modules allow extending the language without modifying the core.

### Module Types

* built-in (Rust)
* external Rust crates
* dynamically loaded native modules (future)
* pure Clipper modules (future)

### Module Responsibilities

* register functions
* register types
* provide constants
* provide database drivers
* provide macros (future)

---

# 🟦 **9. Database Engine**

Clipper-2025 deprecates xBase commands, replacing them with an API:

### `DB.open("customer.dbf")`

Returns a DB cursor object.

### `db.seek("CRUZ")`

Uses CDX indexes.

Database engines are modules (DBF, PostgreSQL, MongoDB, etc).

---

# 🟧 **10. Errors & Diagnostics**

Marina provides:

* syntax error with line/column
* runtime error with trace
* VM halting conditions
* panic-safe failure
* compiler diagnostics (missing return, unused locals)

Future:

* IDE-level language server

---

# 🟨 **11. Tooling**

Marina CLI provides:

```
marina run file.prg
marina build file.prg
marina dump file.bc
marina repl (future)
marina mod install <module>
```

---

# 🟦 **12. Future Extensibility**

### Planned:

* JIT compiler
* bytecode optimizer
* incremental compilation
* AST macros
* plugin debugger
* hot reloading
* package manager ("Dock")
* web-based compiler playground
* VM inspector

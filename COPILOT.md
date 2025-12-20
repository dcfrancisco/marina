# Marina – Copilot Instructions (Authoritative)

This file defines the goals, scope, and architectural rules for the **Marina** repository.
Any generated code, refactoring, or suggestions MUST follow these rules.

---

## Project Identity (current repo reality)

Marina is a **Rust-based compiler and virtual machine for the Clipper / xBase-family language** (often referred to in the docs as “Clipper-2025”).

Core traits today:
- Tokenizer + parser that build an AST
- Compiler that emits stack-based bytecode
- VM that executes that bytecode
- CLI-first developer workflow (examples + tests)

Important: this repo currently supports **command-style Clipper syntax** (e.g. `IF/ENDIF`, `FOR/NEXT`, `LOCAL`, `USE`, etc.) and **built-in functions** (e.g. `OutStd`, `SetPos`, `Inkey`).

---

## What Copilot should optimize for

- **Compatibility and stability** over introducing new syntax
- **Small, targeted changes** that match existing architecture and style
- **Test-backed behavior** (update or add tests in `tests/` when changing semantics)

---

## Current scope (do) 

### Compiler / VM
- Lexer, parser, AST
- Bytecode + stack VM execution
- Diagnostics and clear runtime errors

### Language support
- Maintain and extend **existing** supported features (see `README.md` for the canonical list)
- Keep built-in function behavior consistent with docs/examples

### Tooling
- Main CLI: `clipper`
- Formatter: `marina-fmt` (also exposed via `clipper fmt ...`)
- LSP: `marina-lsp` (feature-gated via `--features lsp`)
- DAP: `marina-dap` (currently a stub)

---

## Not in scope unless explicitly requested (don’t)

### Module/import system
The module/import system is explicitly **deferred to a future phase** (see the project roadmap; this work is tracked under **Phase 5 / Ecosystem** in `docs/LANGUAGE_FEATURES_CHECKLIST.md`).

There is a module system **spec** in `docs/guides/writing_modules.md`, but `import "..."` is **not implemented in the Rust compiler/VM yet**.
Copilot MUST NOT:
- Add `import` syntax to examples/tests as if it works
- Implement a module loader/dylib system as drive-by scope creep

### “Real” database support
Database commands exist as **stubs/placeholders** (e.g. `USE`, `DBSKIP`, `DBSEEK`, `REPLACE`).
Copilot MUST NOT claim database functionality is implemented unless it actually is.

### Big new subsystems
Avoid introducing (unless explicitly requested):
- Networking stacks, HTTP clients, web frameworks
- Async runtimes or concurrency models
- Full DBF/RDD engines, ORMs
- GUI frameworks

---

## Architecture guardrails (keep consistent)

- Keep the phase separation:
  - Parsing in `src/parser/`
  - Compilation in `src/compiler/`
  - VM execution in `src/vm/`
- Add/modify syntax by updating **all layers** consistently:
  - Tokens/keywords: `src/token.rs`, `src/lexer.rs`
  - AST: `src/ast.rs`
  - Parser: `src/parser/*`
  - Compiler: `src/compiler/*`
  - VM/bytecode: `src/bytecode.rs`, `src/vm/*`
- Prefer diagnostics over `panic!` for user-facing errors

---

## Local validation

- Run tests: `cargo test`
- If you change formatting behavior: ensure formatter tests still pass

---

## If in doubt

Ask one clarifying question, or default to the smallest change that matches existing behavior.
# Roadmap

This roadmap reflects the repository state after the Phase 2 / Phase 2.5 review.

## Phase Summary

| Phase | Goal | Status | Notes |
| --- | --- | --- | --- |
| Phase 1 | Core language and VM | Completed | Lexer, parser, compiler, VM, functions, arrays, control flow |
| Phase 2 | Language expansion | Completed with carryovers | `CASE`, augmented operators, increment/decrement, indexed assignment delivered |
| Phase 2.5 | Refactoring and cleanup | Completed | Parser/compiler/VM modularization landed |
| Phase 3 | Runtime and module stabilization | In Progress | Stabilize current behavior, close carryovers, document boundaries |
| Phase 4+ | Libraries, tooling, and ecosystem | Planned / Deferred | Post-v1 work only |

---

## Phase 1 — Core Language and VM

**Status: Completed**

Delivered:

- Lexer and token model
- Recursive-descent parser
- AST representation
- Bytecode compiler
- Stack-based VM
- Arithmetic, comparison, and logical operators
- Variables and assignment
- Functions and procedures
- Core loop and conditional forms
- Built-in runtime functions
- CLI runner and REPL

This phase is functionally complete, although Phase 3 still includes stabilization work for some runtime details.

---

## Phase 2 — Language Expansion

**Status: Completed with carryovers**

Delivered:

- `CASE` / `OTHERWISE`
- Augmented assignment (`+=`, `-=`, `*=`, `/=`)
- Increment and decrement (`++`, `--`)
- Arrays and indexed assignment examples/tests
- Additional parser/compiler/VM coverage

Carryover items:

- Nested `CASE` remains an ignored test and needs to be fixed or explicitly deferred
- `ELSEIF` remains unimplemented as a full branch form

Phase 2 should be treated as delivered, but not perfectly closed until those carryovers are resolved at the Phase 3 stabilization level.

---

## Phase 2.5 — Refactoring and Cleanup

**Status: Completed**

Delivered:

- `src/parser/` modularization
- `src/compiler/` modularization
- `src/vm/` modularization
- Follow-up bug fixes for `Dup`, `CASE` compilation, and loop halt behavior

This phase successfully prepared the project for stabilization work by making the implementation easier to reason about and extend safely.

---

## Phase 3 — Runtime & Module Stabilization

**Status: In Progress**

Phase 3 is not a tooling-first phase anymore. It is a stabilization phase focused on the language/runtime that already exists.

### Objectives

- Freeze the core bytecode/runtime baseline
- Verify VM call-frame and return behavior
- Close Phase 2 carryovers
- Keep built-in runtime functions stable through the VM
- Define the module/import boundary honestly
- Align docs, tests, and examples with the real implementation

### Required before Phase 3 is done

- Nested `CASE` resolved or explicitly deferred from release scope
- `ELSEIF` resolved or explicitly deferred from release scope
- Current scope behavior (`LOCAL`, `STATIC`, `PRIVATE`, `PUBLIC`) documented accurately
- Database statement stubs documented as stubs
- Release examples curated and reliable

### Not part of Phase 3

- Database engines
- PostgreSQL, SQLite, MongoDB, Redis, or other drivers
- HTTP and web frameworks
- Async runtime
- Macro system rollout
- Package ecosystem work
- Full IDE debugger support

### Tooling status inside Phase 3

- `marina-fmt`: implemented as an MVP
- `marina-docs`: implemented
- `marina-lsp`: experimental, feature-gated
- `marina-dap`: stub only

Tooling exists, but it is no longer the definition of the phase.

---

## Post-Phase-3 Priorities

These are important, but they are not current release-gating work.

### Planned

- Standard library growth beyond current builtins
- Better packaging and project structure
- Better debugging support

### Deferred

- DBF/CDX engine work
- SQL drivers
- NoSQL drivers
- HTTP/web work
- Async/concurrency model
- Macro system rollout
- `.ch` preprocessing and legacy compatibility macros
- `@ SAY/GET` compatibility
- Ecosystem/package registry

For future macro/preprocessor work, the preferred compatibility target is **Clipper 5.2 behavior first**, with Clipper 5.3 treated as secondary compatibility guidance.

The `include/` headers already present in the repository should serve as the primary Clipper 5.2 reference set for `.ch` compatibility, especially `STD.CH` for `@ SAY`, `@ GET`, and related command forms.

---

## Roadmap Principle

Marina should finish and stabilize the language/runtime already present in the repository before expanding into databases, networking, or large-scale tooling promises.

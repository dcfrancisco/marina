# Roadmap

This roadmap reflects the repository state after the Phase 2 / Phase 2.5 review.

## Phase Summary

| Phase | Goal | Status | Notes |
| --- | --- | --- | --- |
| Phase 1 | Core language and VM | Completed | Lexer, parser, compiler, VM, functions, arrays, control flow |
| Phase 2 | Language expansion | Completed | `CASE`, `ELSEIF`, augmented ops, increment/decrement, indexed assignment |
| Phase 2.5 | Refactoring and cleanup | Completed | Parser/compiler/VM modularization landed |
| Phase 3 | Runtime and module stabilization | RC ready; CI deferred to backlog | Local runtime/docs/example gates pass |
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

**Status: Completed**

Delivered:

- `CASE` / `OTHERWISE`
- Augmented assignment (`+=`, `-=`, `*=`, `/=`)
- Increment and decrement (`++`, `--`)
- Arrays and indexed assignment examples/tests
- Additional parser/compiler/VM coverage

Carryover items are closed: nested `CASE` and `ELSEIF` are implemented and
covered by parser/compiler/VM tests.

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

**Status: Release-candidate ready; CI deferred to backlog**

See [ADR-001](../../adr/ADR-001-phase3-runtime-stabilization.md) for the
release decision and [WP-001](../../work-packages/WP-001-phase3-runtime-stabilization.md)
for tracked gap-closure work.

Phase 3 is not a tooling-first phase anymore. It is a stabilization phase focused on the language/runtime that already exists.

### Objectives

- Freeze the core bytecode/runtime baseline
- Verify VM call-frame and return behavior
- Close the remaining runtime and documentation gaps listed in WP-001
- Keep built-in runtime functions stable through the VM
- Define the module/import boundary honestly
- Align docs, tests, and examples with the real implementation

### Verified locally

- Function arity validation and value-level VM assertions completed
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

- Language safety and diagnostics (arity, structured errors, stack traces)
- Standard library growth beyond current builtins
- Module/package design and project structure
- Better debugging and LSP support

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

The sequencing decision for these next features is recorded in
[ADR-002](../../adr/ADR-002-post-phase3-feature-sequencing.md).
Language compatibility and feature gates are defined in
[ADR-003](../../adr/ADR-003-language-feature-evolution.md).
Database scope and prerequisites are defined in
[ADR-004](../../adr/ADR-004-database-feature-boundary.md).
Clipper compatibility scope is defined in
[ADR-005](../../adr/ADR-005-clipper-compatibility-boundary.md).
Execution is tracked by [WP-006](../../work-packages/WP-006-language-safety-and-diagnostics.md),
[WP-007](../../work-packages/WP-007-standard-library-expansion.md),
[WP-008](../../work-packages/WP-008-module-package-design.md), and
[WP-009](../../work-packages/WP-009-tooling-depth.md).
Database design is tracked separately in
[WP-011](../../work-packages/WP-011-database-extension-design.md).
Compatibility inventory is tracked in
[WP-012](../../work-packages/WP-012-clipper-compatibility-inventory.md).

For future macro/preprocessor work, the preferred compatibility target is **Clipper 5.2 behavior first**, with Clipper 5.3 treated as secondary compatibility guidance.

The `include/` headers already present in the repository should serve as the primary Clipper 5.2 reference set for `.ch` compatibility, especially `STD.CH` for `@ SAY`, `@ GET`, and related command forms.

---

## Roadmap Principle

Marina should finish and stabilize the language/runtime already present in the repository before expanding into databases, networking, or large-scale tooling promises.

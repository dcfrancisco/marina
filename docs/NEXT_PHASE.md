# Next Phase — Phase 3 Runtime & Module Stabilization

**This document describes the immediate work required after Phase 2 and Phase 2.5.**  
The goal of Phase 3 is to **stabilize the implemented language core and runtime**, not broaden the feature surface.

> **Phase 3 Done = Marina's current compiler/VM behavior is reliable, documented, and ready for release-candidate use.**

---

## Phase Review

### Phase 2 — Language Expansion

**Status: Complete with carryover fixes**

Delivered:

- `CASE` / `OTHERWISE`
- Augmented assignment (`+=`, `-=`, `*=`, `/=`)
- Increment and decrement (`++`, `--`)
- Indexed assignment
- Additional examples and tests

Carryover items entering Phase 3:

- Nested `CASE` still has an ignored test and is not fully supported
- `ELSEIF` is tokenized but not fully implemented in the parser

### Phase 2.5 — Refactoring

**Status: Complete**

Delivered:

- Parser modularization into `src/parser/`
- Compiler modularization into `src/compiler/`
- VM modularization into `src/vm/`
- Follow-up bug fixes around `Dup`, `CASE` compilation, and loop halting

---

## Primary Objectives (Phase 3)

1. **Finalize the bytecode/runtime baseline**
   - Freeze core opcodes used by the current language surface
   - Avoid introducing post-v1 instructions

2. **Harden the VM invocation model**
   - Confirm function calls, locals, arguments, and return behavior
   - Remove edge-case inconsistencies in top-level versus function execution

3. **Close Phase 2 carryovers**
   - Resolve nested `CASE`
   - Implement or explicitly defer `ELSEIF`

4. **Keep native runtime bindings stable**
   - Console and TUI builtins continue to execute through the VM
   - Built-in behavior is documented and release-safe

5. **Define the module boundary honestly**
   - Document the minimal module/import plan
   - Minimal built-in import namespaces are now allowed
   - Do not claim lazy loading until it exists

6. **Prepare release validation**
   - Keep a small reliable example set
   - Make docs, tests, and implementation describe the same system

---

## Out of Scope for Phase 3

The following remain explicitly deferred until after core stability:

- Database engines and adapters
- PostgreSQL, SQLite, MongoDB, Redis, and other backends
- HTTP and web services
- Async runtime or concurrency model
- Macro system rollout
- Rich standard library expansion beyond current builtins
- LSP expansion beyond the current experimental binary
- DAP implementation
- Package ecosystem work

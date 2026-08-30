# Next Phase — Phase 3 Runtime & Module Stabilization

The release decision and tracked gaps are maintained in
[ADR-001](adr/ADR-001-phase3-runtime-stabilization.md) and
[WP-001](work-packages/WP-001-phase3-runtime-stabilization.md).
Post-Phase-3 feature sequencing is proposed in
[ADR-002](adr/ADR-002-post-phase3-feature-sequencing.md).
Language-specific feature rules are in
[ADR-003](adr/ADR-003-language-feature-evolution.md).
Database remains separately designed under
[ADR-004](adr/ADR-004-database-feature-boundary.md).
Clipper 5.2/5.3 compatibility boundaries are defined in
[ADR-005](adr/ADR-005-clipper-compatibility-boundary.md).

**This document describes the immediate work required after Phase 2 and Phase 2.5.**  
The goal of Phase 3 is to **stabilize the implemented language core and runtime**, not broaden the feature surface.

> **Phase 3 Done = Marina's current compiler/VM behavior is reliable, documented, and ready for release-candidate use.**

---

## Phase Review

### Phase 2 — Language Expansion

**Status: Complete**

Delivered:

- `CASE` / `OTHERWISE`
- Augmented assignment (`+=`, `-=`, `*=`, `/=`)
- Increment and decrement (`++`, `--`)
- Indexed assignment
- Additional examples and tests

Carryover items are closed: nested `CASE` and `ELSEIF` are implemented and
covered by parser/compiler/VM tests.

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

3. **Close the remaining stabilization gaps**
   - Validate user-defined function arity
   - Add value-level VM assertions and frame-cleanup checks

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
- `.ch` preprocessing, `#command`, `#translate`, and `#include`
- `@ SAY/GET` compatibility
- Rich standard library expansion beyond current builtins
- LSP expansion beyond the current experimental binary
- DAP implementation
- Package ecosystem work

When macro/preprocessor work is scheduled, the compatibility target should be **Clipper 5.2 first**, with Clipper 5.3 as a secondary reference rather than the primary behavioral target.

The `include/` headers currently in the repository should be treated as **Clipper 5.2 reference material** for that future work, especially `STD.CH` and related compatibility headers.

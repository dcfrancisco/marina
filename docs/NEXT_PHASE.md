# Next Phase — Focused Phase 3 Completion

**This document describes the immediate work required to complete Phase 3 of the Marina project.**  
The goal of Phase 3 is to **stabilize the language core and runtime boundaries**, not add new ecosystems or feature sets.

> **Phase 3 Done = Marina core language & VM is stable, with minimal module system implemented.**

---

## Primary Objectives (Phase 3)

1. ❯ **Finalize bytecode instruction set**
   - Freeze opcodes needed for core language support
   - No additions that are for post-v1 features

2. ❯ **Finalize the VM call frame / invocation model**
   - Consistent function calls
   - Accurate locals, arguments, and return behavior

3. ❯ **Implement a minimal import + lazy module loader**
   - Modules load on first use
   - No wildcard imports or advanced aliasing yet
   - Simple resolution is enough

4. ❯ **Ensure core native bindings operate through the VM**
   - Console and TUI calls must be executed via the VM
   - Native call contract is stable

5. ❯ **One working validation program**
   - Tower of Hanoi example runs reliably
   - No new example programs are required for Phase 3 completion

---

## Out of Scope for Phase 3

The following are important but **NOT required** for Phase 3 and should be deferred until after v1.0 core stability:

- Database engines and adapters
- HTTP/Web services
- Async modules or runtime
- Rich standard libraries beyond minimal core
- Language Server Protocols (LSP)
- IDE formatter or diagnostics
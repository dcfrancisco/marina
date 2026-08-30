# WP-008: Module and package design

- **Status:** Planned
- **Priority:** P1 after WP-006
- **Related:** [ADR-002](../adr/ADR-002-post-phase3-feature-sequencing.md)

## Goal

Define a safe, versioned module boundary before implementing user-distributed
modules or a package registry.

## Scope

- Specify module discovery, import resolution, versioning, and isolation.
- Decide whether modules compile to shared bytecode, source, or both.
- Define duplicate symbols, cyclic imports, missing modules, and failure
  diagnostics.
- Produce a minimal project/package layout and migration path from built-in-only
  imports.

## Acceptance criteria

- A follow-up implementation ADR can be written without unresolved boundary
  questions.
- Existing built-in imports retain compatibility or have a documented migration.
- Security and reproducibility constraints are explicit.


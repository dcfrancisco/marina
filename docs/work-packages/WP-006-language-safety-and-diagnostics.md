# WP-006: Language safety and diagnostics

- **Status:** In progress — negative-path contract documented
- **Priority:** P0 after Phase 3
- **Related:** [ADR-002](../adr/ADR-002-post-phase3-feature-sequencing.md)

## Goal

Give compiler and VM failures stable, actionable semantics before adding larger
language features.

## Scope

- Complete function-arity validation and structured error categories.
- Add source locations to compile/runtime diagnostics and useful call context.
- Define division-by-zero, invalid index, type mismatch, and unknown-function
  behavior.
- Add exact-value and negative-path tests for the public execution API.

## Acceptance criteria

- Each covered failure has a deterministic category and source location.
- Recursive and nested calls preserve useful call context.
- Documentation specifies which failures are compile-time versus runtime.

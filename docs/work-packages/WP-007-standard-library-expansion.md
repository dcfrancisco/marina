# WP-007: Standard-library expansion

- **Status:** Planned
- **Priority:** P1 after WP-006
- **Related:** [ADR-002](../adr/ADR-002-post-phase3-feature-sequencing.md)

## Goal

Grow the standard library without creating inconsistent type or error behavior.

## Scope

- Add collection operations over arrays (length, append, slice, find, sort) in
  small, reviewable increments.
- Fill high-value string gaps only where semantics are specified and tested.
- Evaluate date/time and file I/O as separate extensions with explicit
  portability and error rules.
- Keep every function's arity, accepted types, return value, and failure mode
  in the reference documentation.

## Acceptance criteria

- New functions have parser/compiler/VM tests and reference examples.
- No new function silently changes existing coercion or indexing semantics.
- Platform-dependent APIs are gated or documented as unavailable.


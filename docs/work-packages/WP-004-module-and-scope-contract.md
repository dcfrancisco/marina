# WP-004: Document and test module and scope boundaries

- **Status:** Complete pending hosted CI confirmation
- **Priority:** P1
- **Related:** [ADR-001](../adr/ADR-001-phase3-runtime-stabilization.md)

## Goal

Make the deliberately minimal module model and current variable-scope behavior
unambiguous to users and maintainers.

## Scope

- Publish one authoritative table for the five supported namespaces and
  functions.
- Test missing imports, unsupported modules, and unsupported namespace members.
- Document that `STATIC`, `PRIVATE`, and `PUBLIC` currently use shared global
  storage, while `LOCAL` uses local storage.
- State that member access is supported only as a namespaced function-call
  target; general object/member expressions are not implemented.

## Acceptance criteria

- Reference docs and compiler validation use the same namespace/function list.
- Negative import and member-access cases have automated coverage.
- Scope examples do not imply full Clipper visibility/lifetime semantics.

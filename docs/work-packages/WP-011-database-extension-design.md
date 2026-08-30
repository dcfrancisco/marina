# WP-011: Database extension design

- **Status:** Planned
- **Priority:** P2 after Phase 3 and WP-006
- **Related:** [ADR-004](../adr/ADR-004-database-feature-boundary.md)

## Goal

Specify and test a backend-neutral database API before replacing the current
diagnostic opcode stubs.

## Scope

- Choose the first deterministic adapter target and fixture format.
- Define record/cursor/handle values, lifecycle, errors, iteration, indexing,
  mutation, transactions, and locking.
- Define namespaced API calls without implicit workareas or command syntax.
- Replace placeholder opcodes only after API and conformance tests exist.
- Keep SQL/NoSQL, remote, and concurrent adapters out of the first increment.

## Acceptance criteria

- A follow-up implementation ADR resolves all API boundary questions.
- Conformance fixtures test open/close, seek, iteration, read, and mutation.
- Stub operations cannot report success as persistence.
- Existing language and module contracts remain unchanged.


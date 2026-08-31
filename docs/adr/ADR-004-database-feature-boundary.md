# ADR-004: Treat database support as a separately designed language extension

- **Status:** Accepted for planning
- **Date:** 2026-08-31
- **Decision owners:** Marina maintainers
- **Related:** [ADR-001](ADR-001-phase3-runtime-stabilization.md), [ADR-002](ADR-002-post-phase3-feature-sequencing.md), [ADR-003](ADR-003-language-feature-evolution.md)
- **Execution package:** [WP-011](../work-packages/WP-011-database-extension-design.md)

## Review of the current state

The VM contains database opcodes (`USE`, `DBSKIP`, `DBGOTOP`, `DBGOBOTTOM`,
`DBSEEK`, and `REPLACE`). They now provide a minimal persisted DBF adapter:
open, cursor movement, first-field seek, and field replacement. CDX, memo,
locking, transactions, SQL, NoSQL, and remote cursors remain design targets.

## Decision

Advanced database support stays outside Phase 3. The shipped DBF subset is an
explicit compatibility adapter; it does not claim to be a backend-neutral API
or full Clipper RDD implementation.

The design must settle, in order:

1. Record, cursor, field, and handle value representations
2. Open/close lifecycle, ownership, and resource cleanup
3. Index/search and iteration semantics, including end-of-cursor behavior
4. Mutation, transactions, locking, and failure atomicity
5. Backend capability discovery and portability across DBF and modern stores

The first implementation target should be a deterministic local adapter (DBF or
an explicitly chosen test backend), with conformance fixtures before SQL/NoSQL,
remote, or concurrent adapters are considered.

## Language boundary

Database APIs must be callable as ordinary namespaced functions or methods; they
must not reintroduce implicit workareas or command preprocessing into the core
language. Any new record/cursor values require explicit indexing, equality,
printing, and lifetime rules consistent with [ADR-003](ADR-003-language-feature-evolution.md).

## Acceptance gates

- Implemented database operations must not be no-op stubs.
- An API ADR specifies errors, transactions, locking, and resource cleanup.
- A conformance suite covers open/close, seek, iteration, read, and mutation.
- Database behavior is tested independently of terminal I/O and macro parsing.

## Consequences

This preserves a small, predictable language core and prevents aspirational DBF,
SQL, and NoSQL documentation from being mistaken for shipped functionality.
It also makes backend work incremental: one tested adapter can land without
committing Marina to a universal database model prematurely.

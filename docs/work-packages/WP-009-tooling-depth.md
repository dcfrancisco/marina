# WP-009: Tooling depth

- **Status:** Planned
- **Priority:** P2 after WP-006
- **Related:** [ADR-002](../adr/ADR-002-post-phase3-feature-sequencing.md)

## Goal

Expose stable runtime information through editor tooling once diagnostics and
call frames are reliable.

## Scope

- Improve LSP diagnostics, symbol navigation, and document features using the
  parser's source spans.
- Define bytecode/source mappings needed for stepping and breakpoints.
- Replace the DAP stub only after the mapping and error contracts are settled.
- Keep formatter and documentation tooling independently usable.

## Acceptance criteria

- LSP diagnostics agree with compiler diagnostics.
- Debugger behavior has a documented minimum feature set and protocol tests.
- Tooling failures do not alter program execution semantics.


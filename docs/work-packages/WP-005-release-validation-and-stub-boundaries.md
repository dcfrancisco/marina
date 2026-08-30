# WP-005: Release validation and deferred-feature boundaries

- **Status:** Complete pending hosted CI confirmation
- **Priority:** P1
- **Related:** [ADR-001](../adr/ADR-001-phase3-runtime-stabilization.md)

## Goal

Provide a repeatable release check and prevent deferred features from being
mistaken for implemented behavior.

## Scope

- Run the non-interactive example suite and `cargo test --all-targets` in CI.
- Document the required Cargo dependency cache/network prerequisites.
- Label database opcodes and database guides as no-op stubs with no persistence.
- Keep `.ch`, `#command`, `#translate`, `#include`, `@ SAY/GET`, DAP, and broad
  LSP work explicitly deferred.
- Curate examples so release checks do not block on interactive input.

## Acceptance criteria

- CI publishes a pass/fail result for tests and non-interactive examples.
- A database example cannot be read as evidence of persistence.
- Deferred-feature lists are consistent across README, roadmap, and references.

Canonical commands and CI expectations are documented in
[Release validation](../RELEASE_VALIDATION.md).

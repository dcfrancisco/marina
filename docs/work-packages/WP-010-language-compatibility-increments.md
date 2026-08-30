# WP-010: Language compatibility increments

- **Status:** Planned
- **Priority:** P1 after Phase 3
- **Related:** [ADR-003](../adr/ADR-003-language-feature-evolution.md)

## Goal

Extend the procedural language with small, independently testable features
without destabilizing the existing AST, bytecode, or VM contracts.

## Delivery order

1. Constants and assignment diagnostics
2. `CONTINUE` with well-defined loop nesting behavior
3. `FOR EACH` after iterator semantics are written and tested
4. Map/dictionary values and collection helpers
5. First-class functions and closures under a separate design review

## Acceptance criteria

- Each increment has syntax, runtime, scope, mutation, and error semantics in
  the reference docs before implementation.
- Existing programs and bytecode remain compatible, or a migration note is
  published.
- Positive, boundary, invalid-input, and regression tests accompany each step.
- No class, async, macro, or exception feature is pulled into this package.


# WP-003: Add value-level VM regression tests

- **Status:** Complete; CI follow-up is backlog work
- **Priority:** P0
- **Related:** [ADR-001](../adr/ADR-001-phase3-runtime-stabilization.md)

## Goal

Test observable behavior instead of treating `Result::is_ok()` as proof of
correct execution.

## Scope

- Add a test harness that exposes final values or selected VM storage without
  changing the public runtime API unnecessarily.
- Assert function return values, recursion results, global/local writes,
  `Main()` behavior, and call-frame/local cleanup.
- Add error-path assertions for division by zero, invalid indexes, unknown
  functions, missing imports, and wrong argument counts.
- Keep output-sensitive tests deterministic by capturing or avoiding terminal
  I/O built-ins.

## Acceptance criteria

- Core function and scope tests assert exact values.
- A regression that drops or leaks a frame fails a test.
- Error messages are asserted by category and useful stable text.

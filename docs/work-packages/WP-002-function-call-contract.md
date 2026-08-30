# WP-002: Enforce the function-call contract

- **Status:** Complete pending hosted CI confirmation
- **Priority:** P0
- **Related:** [ADR-001](../adr/ADR-001-phase3-runtime-stabilization.md)

## Goal

Make user-defined calls fail deterministically when the number of arguments does
not match the declared parameters, while preserving nested calls and recursion.

## Scope

- Record function arity during compilation.
- Validate calls before emitting bytecode, including recursive calls.
- Return a diagnostic containing function name, expected arity, and received
  arity.
- Add tests for zero-, one-, and multi-parameter functions, too few arguments,
  too many arguments, nested calls, and recursion.

## Acceptance criteria

- Invalid calls fail before VM execution with a stable diagnostic.
- Valid calls preserve parameter order and clean up their call frame.
- Existing function, recursion, and `Main()` tests remain green.

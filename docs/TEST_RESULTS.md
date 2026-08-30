# Marina Compiler & VM — Test Results (reviewed 2026-08-30)

## Covered by automated tests

- Lexing, parser diagnostics, declarations, imports, expressions, arrays, and
  block structure
- Arithmetic, comparisons, boolean logic, loops, `CASE`/`ELSEIF`, and indexed
  assignment
- User-defined functions/procedures, nested calls, recursion, returns, and the
  `Main()` entrypoint
- Current `LOCAL` versus shared-global behavior for `STATIC`, `PRIVATE`, and
  `PUBLIC`
- Namespaced built-ins from `console`, `input`, `math`, `string`, and `system`
- Formatter and documentation rendering

## Known gaps

- Most VM tests assert successful execution rather than returned values,
  observable state, output, or call-frame cleanup (WP-001.2).
- User-defined function arity is not validated before invocation (WP-001.1).
- Database instructions (`USE`, `DBSKIP`, and related opcodes) are diagnostic
  no-op stubs, not persistence.
- `.ch` preprocessing, `#command`, `#translate`, `#include`, and `@ SAY/GET`
  remain deferred.

## Verification note

The review environment could not complete `cargo test --all-targets` because
Cargo dependencies were unavailable in its registry/cache. CI must run the
command with a populated cache or network access; the source test inventory is
the current behavioral reference until that run is available.

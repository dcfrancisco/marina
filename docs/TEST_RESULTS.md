# Marina Compiler & VM — Test Results (verified 2026-08-31)

## Verification summary

- `cargo test --all-targets --no-fail-fast`: **102 passed, 0 failed**
- `./scripts/run_examples_noninteractive.sh --no-build`: **28 ran, 14 skipped, 0 failed**

Tests used a temporary writable Cargo target directory because the existing
workspace `target/` directory is not writable in this environment.

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
- DBF open, cursor movement, first-field seek, replacement persistence, and
  missing-table errors
- Formatter and documentation rendering

## Known boundaries

- Database instructions have DBF integration coverage for open, cursor
  movement, seek, replacement persistence, and missing-table errors.
- `.ch` preprocessing, `#command`, `#translate`, `#include`, and `@ SAY/GET`
  remain deferred.

## Verification note

Hosted CI should repeat both commands with a writable Cargo cache. Interactive
examples remain intentionally excluded by the example runner.

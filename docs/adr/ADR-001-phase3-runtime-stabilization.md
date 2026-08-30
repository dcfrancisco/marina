# ADR-001: Define Phase 3 as runtime stabilization

- **Status:** Accepted
- **Date:** 2026-08-30
- **Decision owners:** Marina maintainers

## Context

The repository has moved beyond the original Phase 2 scope. The parser supports
`ELSEIF`, nested `CASE`, functions/procedures, returns, imports, and member-style
built-in calls. The VM has call frames, recursion, a `Main()` entrypoint, arrays,
and the current built-in modules (`console`, `input`, `math`, `string`, and
`system`). Existing roadmap and test-result pages still describe several of
these delivered behaviors as unfinished, which makes release scope ambiguous.

The implementation is intentionally small and does not yet provide a general
module loader, database engine, macro preprocessor, or IDE debugger.

## Decision

Phase 3 is a **runtime and module stabilization release**. Its release gate is
behavioral reliability and documentation alignment for the implemented language
surface, not expansion into new subsystems.

The supported Phase 3 surface is:

- core expressions, variables, arrays, control flow, `CASE`/`ELSEIF`, and
  user-defined functions/procedures;
- VM call frames, recursion, returns, and optional `Main()` execution;
- the five statically registered built-in namespaces and their documented
  functions;
- database opcodes as explicit no-op stubs (diagnostic output only).

The following remain outside the release gate: database backends, HTTP/async,
macro and `.ch` preprocessing, package distribution, DAP implementation, and
expansion of the experimental LSP.

## Gaps found during review

1. User-defined function arity is not checked before a call. Missing arguments
   fail later as a local lookup error; extra arguments are accepted and then
   discarded with the frame.
2. Most VM tests assert only `Result::is_ok()`. They do not inspect returned
   values, globals, printed output, or call-frame cleanup, so regressions can
   pass unnoticed.
3. `STATIC`, `PRIVATE`, and `PUBLIC` intentionally share one global storage path;
   this is a compatibility limitation, not distinct Clipper scope semantics.
4. Database instructions execute as stubs and can look successful because they
   only print a diagnostic and advance the instruction pointer.
5. The full test command is not reproducible in the current environment without
   the unavailable Cargo registry dependencies; CI must provide a populated
   cache or network access.

## Consequences

This decision keeps the release promise narrow and testable. Work packages must
close or explicitly defer the five gaps above before Phase 3 is marked complete.
Future database, macro, package, and IDE work should be recorded as separate
ADRs rather than silently changing the Phase 3 contract.

## Verification

The ADR is considered implemented when the linked work package is complete,
`cargo test --all-targets` passes in CI, and status documents describe the same
scope and limitations as the source code.

Execution is split into [WP-002](../work-packages/WP-002-function-call-contract.md),
[WP-003](../work-packages/WP-003-vm-observability-tests.md),
[WP-004](../work-packages/WP-004-module-and-scope-contract.md), and
[WP-005](../work-packages/WP-005-release-validation-and-stub-boundaries.md),
under the [WP-001 umbrella](../work-packages/WP-001-phase3-runtime-stabilization.md).

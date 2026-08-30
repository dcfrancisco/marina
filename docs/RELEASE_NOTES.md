# Release notes

## Unreleased — Phase 3 runtime stabilization

- Added deterministic user-function arity validation.
- Added value-level VM assertions for calls, returns, scope, errors, and frame cleanup.
- Defined the minimal built-in import boundary and documented unsupported module behavior.
- Clarified that database instructions are diagnostic no-op stubs, not persistence.
- Enabled CI test and non-interactive example validation on pushes and pull requests.
- Fixed case-insensitive interactive-example detection.
- Closed the nested `CASE` and `ELSEIF` carryovers.

Before cutting a release candidate, hosted CI must pass the checks in
[Release validation](RELEASE_VALIDATION.md).

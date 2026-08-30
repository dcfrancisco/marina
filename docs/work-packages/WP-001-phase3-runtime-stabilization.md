# WP-001: Phase 3 runtime and module stabilization

- **Status:** Complete; CI follow-up is backlog work
- **Owner:** Marina maintainers
- **Related ADR:** [ADR-001](../adr/ADR-001-phase3-runtime-stabilization.md)

## Objective

Make the current compiler/VM behavior release-candidate reliable and ensure
the roadmap, tests, examples, and references agree with the implementation.

## Work items

Detailed execution packages:

- [WP-002 — Function-call contract](WP-002-function-call-contract.md)
- [WP-003 — VM observability tests](WP-003-vm-observability-tests.md)
- [WP-004 — Module and scope contract](WP-004-module-and-scope-contract.md)
- [WP-005 — Release validation and stub boundaries](WP-005-release-validation-and-stub-boundaries.md)

| ID | Work | Acceptance criterion | Priority |
| --- | --- | --- | --- |
| WP-001.1 | Validate user-defined function arity | Calls with too few or too many arguments return a clear compile-time error; valid recursion and nested calls remain green | P0 (implemented) |
| WP-001.2 | Add value-level VM assertions | Tests verify runtime errors, `Main()` return behavior, and frame/local cleanup rather than only `is_ok()` | P0 (implemented) |
| WP-001.3 | Lock down module boundary | Every supported namespace/function is listed in one reference; missing imports and unsupported functions have tested diagnostics | P1 (implemented) |
| WP-001.4 | Make scope limitation explicit | Documentation states that `STATIC`, `PRIVATE`, and `PUBLIC` use shared global storage in this release | P1 (implemented) |
| WP-001.5 | Make database stubs explicit | Database examples/reference label operations as no-op stubs and tests prevent accidental claims of persistence | P1 (implemented) |
| WP-001.6 | Curate release validation | Non-interactive examples run through a documented CI command; dependency/cache requirements are documented | P1 (implemented) |
| WP-001.7 | Reconcile status docs | README, roadmap, next-phase, checklist, and test-results pages use the same Phase 3 status and deferment list | P1 (implemented) |

## Dependencies and sequencing

1. Complete WP-001.1 and WP-001.2 before changing the Phase 3 status.
2. Complete WP-001.3–WP-001.5 alongside the documentation reconciliation.
3. Complete WP-001.6 in CI; local runs may require a pre-populated Cargo cache.

## Not included

Database engines, SQL/NoSQL drivers, HTTP, async/concurrency, macro
preprocessing, package management, DAP functionality, and broad LSP features
are separate future work.

## Exit checklist

- [x] Function arity errors are deterministic and tested.
- [x] VM tests assert observable values and frame state.
- [x] Module and scope references match source behavior.
- [x] Database operations are visibly stubs.
- [x] Release examples and CI test command are documented.
- [x] Status pages link this work package and ADR.

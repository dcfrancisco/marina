# ADR-002: Sequence the next Marina features after Phase 3

- **Status:** Accepted for planning
- **Date:** 2026-08-30
- **Decision owners:** Marina maintainers
- **Depends on:** [ADR-001](ADR-001-phase3-runtime-stabilization.md)

## Context

Marina already has a usable compiler/VM core, arrays, functions, imports, and a
small built-in library. The feature checklist contains substantially larger
ideas—exceptions, collections, packages, databases, macros, and IDE tooling—
but these features have different design dependencies and risk levels. Starting
them in parallel would expand the language contract before the runtime contract
is stable.

## Decision

After Phase 3 exits, implement features in the following order:

1. **Language safety and diagnostics** — function arity checks, structured
   runtime errors, stack traces, and value-level regression coverage.
2. **Small standard-library expansion** — collection operations for arrays,
   predictable string/date/file APIs, and documented error behavior.
3. **Module and package design** — replace the current built-in-only import
   validation with a versioned module boundary and a project/package layout.
4. **Tooling depth** — improve the LSP and implement a debugger only after
   bytecode locations, errors, and call frames are stable enough to expose.

The corresponding planning packages are:

- [WP-006 — Language safety and diagnostics](../work-packages/WP-006-language-safety-and-diagnostics.md)
- [WP-007 — Standard-library expansion](../work-packages/WP-007-standard-library-expansion.md)
- [WP-008 — Module and package design](../work-packages/WP-008-module-package-design.md)
- [WP-009 — Tooling depth](../work-packages/WP-009-tooling-depth.md)

Language-specific compatibility rules are defined in
[ADR-003](ADR-003-language-feature-evolution.md).

Database scope and prerequisites are defined in
[ADR-004](ADR-004-database-feature-boundary.md).

Database engines, async/concurrency, macro preprocessing, `.ch` compatibility,
and `@ SAY/GET` remain separate proposals. They do not enter the implementation
queue merely because they appear in the feature checklist.

## Rationale

The first two steps reduce risk for every later feature: they establish how
errors are represented and how values move through the VM. Modules and tooling
then have stable contracts to consume. Databases and macros require independent
compatibility and persistence decisions, so they need their own ADRs and work
packages rather than being folded into a general “Phase 4” task.

## Constraints for future proposals

- Every new opcode or runtime value must include a compatibility and migration
  note.
- New standard-library functions must specify arity, accepted types, return
  values, and error behavior.
- A module proposal must define discovery, versioning, isolation, and import
  failure semantics before implementation.
- A macro or database proposal must identify its compatibility target and test
  fixtures before claiming support.

## Consequences

The roadmap remains intentionally conservative: language reliability and a
small library lead; package/tooling work follows; database, macro, and async
features remain opt-in design work. This prevents future documentation from
implying that a stub or experimental binary is production-ready.

## Exit criteria

ADR-002 can be replaced by feature-specific ADRs when each proposed feature has
an owner, a work package, acceptance tests, and an explicit compatibility plan.

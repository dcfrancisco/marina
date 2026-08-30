# ADR-003: Evolve the language in compatibility-sized increments

- **Status:** Accepted for planning
- **Date:** 2026-08-30
- **Decision owners:** Marina maintainers
- **Related:** [ADR-001](ADR-001-phase3-runtime-stabilization.md), [ADR-002](ADR-002-post-phase3-feature-sequencing.md)
- **Execution package:** [WP-010](../work-packages/WP-010-language-compatibility-increments.md)

## Review of the current language

The implemented language surface is expression-based and procedural: scalar
values, arrays, variables, arithmetic and boolean operators, `IF`/`ELSEIF`,
loops, `CASE`, functions/procedures, returns, indexed assignment, and a small
namespaced built-in set. The parser and VM also support recursion and a `Main()`
entrypoint.

The feature checklist still contains broad proposals that are not language
gaps for the next release: classes, async/await, concurrency, macros, package
registries, maps, sets, and advanced functional abstractions. Implementing
these before value/error semantics settle would create incompatible contracts.

## Decision

The next language work should be delivered in three compatibility-sized waves:

1. **Safety and control-flow ergonomics:** constants, `CONTINUE`, and a clear
   `FOR EACH` design only after iterator semantics are specified.
2. **Core data model:** a map/dictionary value and collection helpers, with
   explicit equality, indexing, mutation, and serialization rules.
3. **First-class functions:** function values, closures, and higher-order calls
   only after call frames, captures, and lifetime/ownership behavior have a
   written design.

Every wave requires parser, compiler, VM, reference documentation, and negative
tests. Existing syntax and bytecode behavior remain backward compatible unless
a feature-specific ADR approves a migration.

## Explicitly deferred language proposals

- Classes, inheritance, and object access
- Async/await, threads, channels, and actors
- Macro preprocessing, `.ch`, `#command`, `#translate`, `#include`, and
  `@ SAY/GET` compatibility
- Exceptions/`TRY`/`CATCH` until structured diagnostics and unwinding are
  designed (tracked first by WP-006)
- Tuples, sets, pattern matching, destructuring, and advanced operators until
  the collection/value model is stable

## Acceptance gates for a new language feature

- Syntax is unambiguous and documented with examples.
- The AST and bytecode impact is recorded, including version compatibility.
- Type/coercion, scope, mutation, and error behavior are specified.
- At least one positive, one boundary, and one invalid-input test exists.
- The feature has a work package and does not silently expand Phase 3 scope.

## Consequences

This keeps Marina's language coherent as it grows: small control-flow and data
model improvements can ship independently, while features that change execution
or lifetime semantics are designed before implementation.

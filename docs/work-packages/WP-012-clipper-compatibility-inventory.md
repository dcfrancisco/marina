# WP-012: Clipper 5.2/5.3 compatibility inventory

- **Status:** Inventory implemented — conformance fixtures pending
- **Priority:** P2 after WP-006 and WP-010
- **Related:** [ADR-005](../adr/ADR-005-clipper-compatibility-boundary.md)

## Goal

Turn the compatibility ambition into a tested, versioned subset rather than a
general claim of Clipper 5.2/5.3 support.

## Scope

- Inventory syntax and macros from `include/STD.CH`, `SIMPLEIO.CH`, and
  `COMMON.CH`.
- Classify each item as supported, translatable in a future preprocessor,
  intentionally replaced, or unsupported.
- Build 5.2 fixtures for the selected subset and record any 5.3 differences.
- Define diagnostics for unsupported commands, macros, `.CH` files, and
  workarea/RDD constructs.
- Update compatibility tables and examples only from passing fixtures.

## Acceptance criteria

- A versioned compatibility matrix exists with no ambiguous “supported” labels.
- Every claimed feature has a positive and negative fixture.
- 5.3 claims cite a concrete specification or fixture and a documented delta.
- Macro/preprocessor and database work remain gated by ADR-004 and separate
  implementation packages.

The initial inventory is published in the
[Clipper compatibility matrix](../compatibility/CLIPPER_COMPATIBILITY_MATRIX.md).

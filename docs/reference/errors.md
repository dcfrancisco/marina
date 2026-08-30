# Error behavior reference

This is the current Phase 3 error contract. Errors are returned as strings by
the compiler/VM API; structured categories and source-span diagnostics are
tracked by WP-006.

## Compile-time errors

The compiler rejects unsupported modules and namespace members, calls to
user-defined functions with the wrong arity, missing imports for namespaced
calls, and general member expressions. Diagnostics include the relevant name
and expected/received values where applicable.

## Runtime errors

The VM reports deterministic errors for division by zero, invalid array/string
indexes, numeric type violations, stack underflow, and unknown functions.
Database opcodes are not errors today: they are explicit diagnostic no-op stubs
and must not be interpreted as persistence.

## Compatibility note

Error text is currently stable enough for tests but is not yet a versioned
machine-readable API. Consumers should avoid parsing error strings until WP-006
defines error categories and source locations.


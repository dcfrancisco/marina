# Next Phase

This file tracks the remaining work needed to finish **Phase 3: Runtime & Module Stabilization**.

Phase 3 is complete when Marina has a stable execution core and a narrowly scoped module plan that matches the codebase.

## Exit Criteria

### 1. VM behavior is predictable

Required:

- Review current opcode behavior for edge cases
- Remove release-blocking runtime inconsistencies
- Keep `cargo test` green

### 2. Function semantics are settled

Required:

- Confirm argument ordering and return behavior
- Confirm top-level plus `Main()` execution flow
- Decide whether any call-frame cleanup issues remain

### 3. Bytecode surface is stable enough for v1.0-rc1

Required:

- Treat current opcodes as the release baseline
- Avoid adding non-core instructions
- Document intentionally stubbed opcodes clearly

### 4. The module story is honest and minimal

Required:

- Replace speculative module documentation with an implementation plan
- Define the smallest viable import/module boundary for post-Phase-3 work
- Do not claim lazy loading before it exists

### 5. Core language gaps are resolved or explicitly deferred

Required:

- Address ignored nested `CASE`
- Decide whether `ELSEIF` is supported or deferred
- Document the current meaning of `STATIC`, `PRIVATE`, and `PUBLIC`

### 6. Examples and docs support release evaluation

Required:

- Keep a small set of reliable core examples
- Ensure README, roadmap, and architecture docs agree
- Separate current behavior from long-range design notes

## Not Part of This Phase

- Database engines
- SQL or NoSQL drivers
- HTTP clients
- Web frameworks
- Async runtime
- Package ecosystem
- Full LSP productization
- DAP implementation

## Recommended Work Order

1. Close runtime correctness gaps (`CASE`, `ELSEIF`, variable-scope semantics).
2. Freeze and document the bytecode/runtime baseline.
3. Keep modules at the architecture-definition level unless implementation is intentionally started.
4. Prepare `v1.0-rc1` only after docs, tests, and examples describe the same system.

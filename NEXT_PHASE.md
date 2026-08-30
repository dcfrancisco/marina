# Next Phase

Phase 3 runtime and module stabilization is complete locally and is
release-candidate ready. The remaining external gate is hosted CI confirmation.

Phase 3 is complete when Marina has a stable execution core and a narrowly scoped module plan that matches the codebase.

## Exit Criteria — locally satisfied

### 1. VM behavior is predictable

Verified:

- Review current opcode behavior for edge cases
- Remove release-blocking runtime inconsistencies
- Keep `cargo test` green

### 2. Function semantics are settled

Verified:

- Confirm argument ordering and return behavior
- Confirm top-level plus `Main()` execution flow
- Decide whether any call-frame cleanup issues remain

### 3. Bytecode surface is stable enough for v1.0-rc1

Verified:

- Treat current opcodes as the release baseline
- Avoid adding non-core instructions
- Document intentionally stubbed opcodes clearly

### 4. The module story is honest and minimal

Verified:

- Replace speculative module documentation with an implementation plan
- Define the smallest viable import/module boundary for post-Phase-3 work
- Do not claim lazy loading before it exists

### 5. Core language gaps are resolved or explicitly deferred

Verified:

- Nested `CASE` and `ELSEIF` are implemented and covered by tests
- Document the current meaning of `STATIC`, `PRIVATE`, and `PUBLIC`

### 6. Examples and docs support release evaluation

Verified:

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

## Remaining release action

1. Confirm the first hosted GitHub Actions run after these changes land.
2. Cut `v1.0-rc1` if hosted CI passes.

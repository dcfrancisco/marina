# Release validation

Phase 3 validation consists of two commands, run from the repository root:

```bash
cargo test --all-targets
./scripts/run_examples_noninteractive.sh
```

The example runner skips programs containing interactive input primitives and
fails if any non-interactive example exits unsuccessfully. Use
`--dry-run --no-build` to inspect classification when a release binary is
already present; use `--verbose` to retain program output.

CI must provide a writable Cargo registry/cache (or network access) for the
dependency resolution step. A local failure caused by an empty or read-only
Cargo cache is an environment failure, not evidence that an example passed.

Database examples are not part of this validation: database opcodes are still
no-op stubs and are tracked separately from release behavior.


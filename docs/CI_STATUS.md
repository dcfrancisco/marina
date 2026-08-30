# CI status

Automatic GitHub Actions CI is disabled and moved to the backlog. The workflow
in `.github/workflows/ci.yml` can be run manually with `workflow_dispatch`.

CI uses the repository-pinned stable toolchain from `rust-toolchain.toml` and
the Rust dependency/build cache action.

Local verification on 2026-08-31 passed:

- `cargo test --all-targets --no-fail-fast`: 100 passed
- `./scripts/run_examples_noninteractive.sh --no-build`: 28 ran, 13 skipped, 0 failed

Hosted verification previously passed in GitHub Actions run `33336798038` on
the `main` branch. Future CI reactivation should preserve that test/example
coverage and address any action-maintenance warnings.

## Backlog

- Decide when automatic push/pull-request CI should be reactivated.
- Add branch protection and a maintainer-owned failure response process before
  treating CI as a release gate again.

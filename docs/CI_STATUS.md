# CI status

The GitHub Actions workflow is enabled for pushes and pull requests in
`.github/workflows/ci.yml`.

CI uses the repository-pinned stable toolchain from `rust-toolchain.toml` and
the Rust dependency/build cache action.

Local verification on 2026-08-31 passed:

- `cargo test --all-targets --no-fail-fast`: 100 passed
- `./scripts/run_examples_noninteractive.sh --no-build`: 28 ran, 13 skipped, 0 failed

The only remaining release action is to confirm the first hosted workflow run
after these changes land on the remote repository. It cannot be verified from
the local checkout.

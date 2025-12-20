#!/usr/bin/env bash
set -euo pipefail

# Runs all example .prg programs that appear to be non-interactive.
# Skips files that contain common input/prompt primitives.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLES_DIR="$ROOT_DIR/examples"
CLIPPER_BIN="$ROOT_DIR/target/release/clipper"

dry_run=false
verbose=false
no_build=false

usage() {
  cat <<'USAGE'
Usage: ./scripts/run_examples_noninteractive.sh [options]

Options:
  --dry-run    Print what would run/skip, but do not execute
  --verbose    Do not redirect program output to /dev/null
  --no-build   Do not auto-build if clipper is missing
  -h, --help   Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run) dry_run=true; shift ;;
    --verbose) verbose=true; shift ;;
    --no-build) no_build=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ ! -x "$CLIPPER_BIN" ]]; then
  if [[ "$no_build" == true ]]; then
    echo "error: $CLIPPER_BIN not found or not executable" >&2
    echo "hint: build it first: cargo build --release" >&2
    exit 1
  fi

  echo "Building clipper (release)..."
  (cd "$ROOT_DIR" && cargo build --release --bin clipper)
fi

skip_regex='GetInput\(|GetSecret\(|Inkey\(|ACCEPT|INPUT|ReadKey\(|WaitKey\('

shopt -s nullglob

ran=0
skipped=0
failed=0

echo "Running non-interactive examples..."
for file in "$EXAMPLES_DIR"/*.prg; do
  if grep -Eq "$skip_regex" "$file"; then
    echo "SKIP  $(basename "$file") (interactive)"
    skipped=$((skipped + 1))
    continue
  fi

  echo "RUN   $(basename "$file")"
  if [[ "$dry_run" == true ]]; then
    ran=$((ran + 1))
    continue
  fi

  if [[ "$verbose" == true ]]; then
    if "$CLIPPER_BIN" "$file"; then
      ran=$((ran + 1))
    else
      echo "FAIL  $(basename "$file")"
      failed=$((failed + 1))
    fi
  else
    if "$CLIPPER_BIN" "$file" >/dev/null; then
      ran=$((ran + 1))
    else
      echo "FAIL  $(basename "$file")"
      failed=$((failed + 1))
    fi
  fi
done

echo
echo "Summary: ran=$ran skipped=$skipped failed=$failed"

if [[ "$failed" -ne 0 ]]; then
  exit 1
fi

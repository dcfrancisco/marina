#!/usr/bin/env bash
set -euo pipefail

# Creates the tiny DBF fixture used by examples/database_demo.prg.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT_DIR/examples"
printf '%s' 'AwAAAAIAAABhAA8AAAAAAAAAAAAAAAAAAAAAAAAAAABJRAAAAAAAAAAAAEMAAAAABAAAAAAAAAAAAAAAAAAAAE5BTUUAAAAAAAAAQwAAAAAKAAAAAAAAAAAAAAAAAAAADSAwMDAxT0xEICAgICAgICAwMDAyU0VDT05EICAgIBo=' | base64 --decode > "$ROOT_DIR/examples/database_demo.dbf"
echo "Created $ROOT_DIR/examples/database_demo.dbf"

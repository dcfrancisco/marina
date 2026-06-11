#!/usr/bin/env bash

set -e

INSTALL_DIR="${INSTALL_DIR:-$HOME/.marina/bin}"
BINS=(
  clipper
  marina-lsp
  marina-fmt
  marina-dap
  marina-docs
)

mkdir -p "$INSTALL_DIR"

echo "Installing Marina (development mode)"
echo "Building Marina..."
cargo build

echo "Linking binaries to $INSTALL_DIR..."
for bin in "${BINS[@]}"; do
  if [ -f "target/debug/$bin" ]; then
    ln -sf "$(pwd)/target/debug/$bin" "$INSTALL_DIR/$bin"
    echo "Linked $bin to $INSTALL_DIR/$bin"
  else
    echo "Warning: target/debug/$bin not found, skipping."
  fi
done

echo
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "Add this to your shell config if needed:"
  echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo
echo "Done. Try: clipper --help"

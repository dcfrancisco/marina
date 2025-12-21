# Install binaries to ~/.marina/bin by default, or to a custom location if specified
INSTALL_DIR="${INSTALL_DIR:-$HOME/.marina/bin}"

# Ensure the install directory exists
mkdir -p "$INSTALL_DIR"
echo "Installing binaries to $INSTALL_DIR..."
for bin in clipper marina-lsp marina-fmt marina-dap; do
  if [ -f "target/debug/$bin" ]; then
    ln -sf "$(pwd)/target/debug/$bin" "$INSTALL_DIR/$bin"
    echo "Linked $bin to $INSTALL_DIR."
  else
    echo "Warning: target/debug/$bin not found, skipping."
  fi
done
echo "Installing Marina (development mode)"
set -e

echo "Installing Marina (development mode)"

# Build
echo "Building Marina..."
cargo build

echo "Marina installed to $MARINA_BIN/marina"
echo

# PATH reminder
if [[ ":$PATH:" != *":$MARINA_BIN:"* ]]; then
  echo "Add this to your shell config if needed:"
  echo "  export PATH=\"\$PATH:$MARINA_BIN\""
fi

echo
echo "Done. Try: marina --help"

#!/bin/bash
# Marina Handbook PDF Builder
# Combines all handbook markdown files into a single PDF

set -e

HANDBOOK_DIR="handbook"
OUTPUT_FILE="Marina_Clipper2025_Handbook.pdf"

echo "Building Marina Handbook PDF..."

# Combine all markdown files in reading order
pandoc \
  --from markdown+smart \
  --to pdf \
  --pdf-engine=xelatex \
  --toc \
  --toc-depth=3 \
  --number-sections \
  --highlight-style=tango \
  --metadata title="Marina (Clipper-2025) Handbook" \
  --metadata author="Danny Francisco" \
  --metadata date="$(date +'%B %Y')" \
  -V geometry:margin=1in \
  -V fontsize=11pt \
  -V documentclass=report \
  -V linkcolor=blue \
  -V urlcolor=blue \
  -o "$OUTPUT_FILE" \
  "$HANDBOOK_DIR/introduction.md" \
  "$HANDBOOK_DIR/language/design_philosophy.md" \
  "$HANDBOOK_DIR/language/syntax.md" \
  "$HANDBOOK_DIR/architecture/compiler.md" \
  "$HANDBOOK_DIR/architecture/vm.md" \
  "$HANDBOOK_DIR/architecture/modules.md" \
  "$HANDBOOK_DIR/architecture/macros.md" \
  "$HANDBOOK_DIR/database/dbf_engine.md" \
  "$HANDBOOK_DIR/database/modern_backends.md" \
  "$HANDBOOK_DIR/vision/lost_visions.md" \
  "$HANDBOOK_DIR/vision/roadmap.md" \
  "$HANDBOOK_DIR/reference/grammar.md" \
  "$HANDBOOK_DIR/reference/bytecode.md" \
  "$HANDBOOK_DIR/reference/stdlib.md"

echo "✅ PDF created: $OUTPUT_FILE"

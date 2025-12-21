#!/usr/bin/env bash
# build_pdf.sh - Assemble and build Marina compiler internals documentation as a PDF
# Requires: pandoc, optionally wkhtmltopdf or latex for PDF output

set -e

DOCS_DIR="$(dirname "$0")"
OUTFILE="${DOCS_DIR}/Marina_Compiler_Internals.pdf"

# List of markdown files in order
INPUT_FILES=(
  "${DOCS_DIR}/README.md"
  "${DOCS_DIR}/overview.md"
  "${DOCS_DIR}/lexer.md"
  "${DOCS_DIR}/parser.md"
  "${DOCS_DIR}/ast.md"
  "${DOCS_DIR}/bytecode.md"
  "${DOCS_DIR}/vm.md"
  "${DOCS_DIR}/errors.md"
  "${DOCS_DIR}/extending.md"
)

# Assemble and convert to PDF
  # Assemble and convert to PDF with enhanced formatting and metadata
  pandoc \
    --from markdown+smart \
    --to pdf \
    --pdf-engine=xelatex \
    --toc \
    --toc-depth=3 \
    --number-sections \
    --highlight-style=tango \
    --metadata title="Marina Compiler Internals" \
    --metadata author="Danny Francisco" \
    --metadata date="$(date +'%B %Y')" \
    -V geometry:margin=1in \
    -V fontsize=11pt \
    -V documentclass=report \
    -V linkcolor=blue \
    -V urlcolor=blue \
    -o "$OUTFILE" \
    "${DOCS_DIR}/README.md" \
    "${DOCS_DIR}/overview.md" \
    "${DOCS_DIR}/lexer.md" \
    "${DOCS_DIR}/parser.md" \
    "${DOCS_DIR}/ast.md" \
    "${DOCS_DIR}/bytecode.md" \
    "${DOCS_DIR}/vm.md" \
    "${DOCS_DIR}/errors.md" \
    "${DOCS_DIR}/extending.md"

echo "PDF built at $OUTFILE"

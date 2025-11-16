# Marina Documentation Build

This directory contains scripts to generate different formats of the Marina documentation.

## Prerequisites

### Install Pandoc and LaTeX

**macOS:**
```bash
brew install pandoc
brew install --cask basictex  # Lightweight (100MB)
# or
brew install --cask mactex    # Full distribution (4GB)

# Update LaTeX packages (required)
sudo tlmgr update --self
sudo tlmgr install collection-fontsrecommended
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install pandoc texlive-xetex texlive-fonts-recommended
```

**Windows:**
```bash
# Install Chocolatey first, then:
choco install pandoc miktex
```

## Build Commands

### Quick Build (Shell Script)

```bash
cd docs
chmod +x build_pdf.sh
./build_pdf.sh
```

This creates `Marina_Clipper2025_Handbook.pdf` in the `docs/` directory.

### Using Makefile (Recommended)

```bash
cd docs

# Build PDF (default)
make pdf

# Build EPUB (e-reader format)
make epub

# Build standalone HTML
make html

# Build all formats
make all

# Clean generated files
make clean
```

## Output Files

- **Marina_Handbook.pdf** - Full handbook in PDF format (~50-100 pages)
- **Marina_Handbook.epub** - E-reader format (Kindle, Apple Books, etc.)
- **Marina_Handbook.html** - Single-page HTML with embedded CSS

## File Order

The handbook is assembled in this reading order:

1. Introduction
2. Language (design philosophy, syntax)
3. Architecture (compiler, VM, modules, macros)
4. Database (DBF engine, modern backends)
5. Vision (lost visions, roadmap)
6. Reference (grammar, bytecode, stdlib)

## Customization

### Change Fonts

Edit the Makefile or build_pdf.sh:

```bash
-V mainfont="Arial" \
-V monofont="Courier New" \
```

### Adjust Margins

```bash
-V geometry:margin=0.75in \
```

### Change Syntax Highlighting

Available styles: `tango`, `pygments`, `kate`, `monochrome`, `espresso`, `zenburn`, `haddock`

```bash
--highlight-style=zenburn \
```

### Add Cover Page

Create `docs/cover.md`:

```markdown
---
title: Marina (Clipper-2025) Handbook
subtitle: Modern Evolution of the Clipper Language
author: Danny Francisco
date: November 2025
---

\newpage
```

Then add it as the first file in the Makefile.

## Alternative: mdBook

For web-based documentation with search and navigation:

```bash
cargo install mdbook

# Create mdbook structure
cd docs
mdbook init --title "Marina Handbook"

# Edit book.toml to include handbook files
# Then build
mdbook build
mdbook serve  # Live preview at http://localhost:3000
```

## Alternative: Docusaurus

For a full documentation website:

```bash
npx create-docusaurus@latest marina-docs classic
# Copy handbook files to docs/
npm start
```

## Troubleshooting

**Error: `xelatex not found`**
- Install LaTeX: `brew install --cask basictex`
- Update PATH: `eval "$(/usr/libexec/path_helper)"`

**Error: Missing fonts**
```bash
sudo tlmgr install collection-fontsrecommended
```

**Error: Unicode characters broken**
- Use `xelatex` instead of `pdflatex`: `--pdf-engine=xelatex`

**PDF too large**
- Compress images before building
- Or use `--pdf-engine=lualatex` with compression

## CI/CD Integration

Add to `.github/workflows/docs.yml`:

```yaml
name: Build Documentation

on:
  push:
    paths:
      - 'docs/**'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y pandoc texlive-xetex texlive-fonts-recommended
      - name: Build PDF
        run: |
          cd docs
          make pdf
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: Marina-Handbook
          path: docs/Marina_Handbook.pdf
```

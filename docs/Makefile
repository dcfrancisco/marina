# Marina Documentation Builder
# Supports PDF, EPUB, HTML generation

.PHONY: all pdf epub html clean

HANDBOOK_FILES := \
	handbook/introduction.md \
	handbook/language/design_philosophy.md \
	handbook/language/syntax.md \
	handbook/architecture/compiler.md \
	handbook/architecture/vm.md \
	handbook/architecture/modules.md \
	handbook/architecture/macros.md \
	handbook/database/dbf_engine.md \
	handbook/database/modern_backends.md \
	handbook/vision/lost_visions.md \
	handbook/vision/roadmap.md \
	handbook/reference/grammar.md \
	handbook/reference/bytecode.md \
	handbook/reference/stdlib.md

METADATA := \
	--metadata title="Marina (Clipper-2025) Handbook" \
	--metadata author="Danny Francisco" \
	--metadata date="$(shell date +'%B %Y')"

all: pdf

pdf: Marina_Handbook.pdf

epub: Marina_Handbook.epub

html: Marina_Handbook.html

Marina_Handbook.pdf: $(HANDBOOK_FILES)
	@echo "📚 Building PDF handbook..."
	pandoc \
		--from markdown+smart \
		--to pdf \
		--pdf-engine=xelatex \
		--toc \
		--toc-depth=3 \
		--number-sections \
		--highlight-style=tango \
		$(METADATA) \
		-V geometry:margin=1in \
		-V fontsize=11pt \
		-V documentclass=report \
		-V linkcolor=blue \
		-V urlcolor=blue \
		-o $@ \
		$(HANDBOOK_FILES)
	@echo "✅ PDF created: $@"

Marina_Handbook.epub: $(HANDBOOK_FILES)
	@echo "📱 Building EPUB handbook..."
	pandoc \
		--from markdown+smart \
		--to epub \
		--toc \
		--toc-depth=3 \
		--number-sections \
		--highlight-style=tango \
		$(METADATA) \
		-o $@ \
		$(HANDBOOK_FILES)
	@echo "✅ EPUB created: $@"

Marina_Handbook.html: $(HANDBOOK_FILES)
	@echo "🌐 Building HTML handbook..."
	pandoc \
		--from markdown+smart \
		--to html5 \
		--standalone \
		--toc \
		--toc-depth=3 \
		--number-sections \
		--highlight-style=tango \
		--css=handbook.css \
		$(METADATA) \
		-o $@ \
		$(HANDBOOK_FILES)
	@echo "✅ HTML created: $@"

clean:
	@echo "🧹 Cleaning generated files..."
	rm -f Marina_Handbook.pdf Marina_Handbook.epub Marina_Handbook.html
	@echo "✅ Clean complete"

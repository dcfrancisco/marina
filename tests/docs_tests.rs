use marina::docs::markdown::{Block, parse_markdown};
use marina::docs::{self, DocsConfig};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn parse_markdown_recognizes_core_blocks() {
    let source = "# Title\n\nParagraph text.\n\n- first\n- second\n\n1. alpha\n2. beta\n\n> quote\n\n```\ncode\n```\n";
    let document = parse_markdown(source, "Fallback");

    assert_eq!(document.title, "Title");
    assert!(matches!(
        document.blocks[0],
        Block::Heading { level: 1, .. }
    ));
    assert!(
        document
            .blocks
            .iter()
            .any(|block| matches!(block, Block::Paragraph(_)))
    );
    assert!(
        document
            .blocks
            .iter()
            .any(|block| matches!(block, Block::UnorderedList(_)))
    );
    assert!(
        document
            .blocks
            .iter()
            .any(|block| matches!(block, Block::OrderedList(_)))
    );
    assert!(
        document
            .blocks
            .iter()
            .any(|block| matches!(block, Block::Quote(_)))
    );
    assert!(
        document
            .blocks
            .iter()
            .any(|block| matches!(block, Block::CodeBlock(_)))
    );
}

#[test]
fn render_html_writes_expected_output() {
    let dir = temp_test_dir("docs_html");
    let input = dir.join("guide.md");
    fs::write(&input, "# Guide\n\nHello Marina.\n").unwrap();

    let output = docs::render_html(&input, None, &DocsConfig::default()).unwrap();
    let html = fs::read_to_string(output).unwrap();

    assert!(html.contains("<h1>Guide</h1>"));
    assert!(html.contains("Hello Marina."));
}

#[test]
fn render_pdf_writes_pdf_header() {
    let dir = temp_test_dir("docs_pdf");
    let input = dir.join("guide.md");
    let output = dir.join("guide.pdf");
    fs::write(&input, "# Guide\n\nHello Marina.\n").unwrap();

    let path = docs::render_pdf(&input, Some(&output), &DocsConfig::default()).unwrap();
    let pdf = fs::read(path).unwrap();

    assert!(pdf.starts_with(b"%PDF-1.4"));
    assert!(String::from_utf8_lossy(&pdf).contains("Marina marina-docs"));
}

#[test]
fn render_directory_prefers_readme_first() {
    let dir = temp_test_dir("docs_dir");
    fs::write(dir.join("README.md"), "# Start\n\nFirst.\n").unwrap();
    fs::write(dir.join("appendix.md"), "# Appendix\n\nSecond.\n").unwrap();

    let output = docs::render_html(&dir, None, &DocsConfig::default()).unwrap();
    let html = fs::read_to_string(output).unwrap();

    let start = html.find("Start").unwrap();
    let appendix = html.find("Appendix").unwrap();
    assert!(start < appendix);
}

fn temp_test_dir(prefix: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("marina_{prefix}_{suffix}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}

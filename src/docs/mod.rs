pub mod config;
pub mod html;
pub mod markdown;
pub mod pdf;
pub mod themes;

use std::fs;
use std::path::{Path, PathBuf};

pub use config::DocsConfig;
pub use markdown::{Block, Document};

pub fn render_html(
    input: &Path,
    output: Option<&Path>,
    config: &DocsConfig,
) -> Result<PathBuf, String> {
    let title = config.resolve_title(input);
    let document = markdown::load_document(input, Some(&title))?;
    let html = html::render_document(&document, config);
    let output_path = output
        .map(PathBuf::from)
        .unwrap_or_else(|| config.default_output_path("html", input));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create '{}': {}", parent.display(), e))?;
    }

    fs::write(&output_path, html)
        .map_err(|e| format!("Failed to write '{}': {}", output_path.display(), e))?;
    Ok(output_path)
}

pub fn render_pdf(
    input: &Path,
    output: Option<&Path>,
    config: &DocsConfig,
) -> Result<PathBuf, String> {
    let title = config.resolve_title(input);
    let document = markdown::load_document(input, Some(&title))?;
    let pdf = pdf::render_document(&document, config)?;
    let output_path = output
        .map(PathBuf::from)
        .unwrap_or_else(|| config.default_output_path("pdf", input));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create '{}': {}", parent.display(), e))?;
    }

    fs::write(&output_path, pdf)
        .map_err(|e| format!("Failed to write '{}': {}", output_path.display(), e))?;
    Ok(output_path)
}

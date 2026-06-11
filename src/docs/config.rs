use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DocsConfig {
    pub title: Option<String>,
    pub theme: String,
}

impl Default for DocsConfig {
    fn default() -> Self {
        Self {
            title: None,
            theme: "marina".to_string(),
        }
    }
}

impl DocsConfig {
    pub fn resolve_title(&self, input: &Path) -> String {
        if let Some(title) = &self.title {
            return title.clone();
        }

        input
            .file_stem()
            .or_else(|| input.file_name())
            .map(|value| value.to_string_lossy().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "Marina Document".to_string())
    }

    pub fn default_output_path(&self, format: &str, input: &Path) -> PathBuf {
        if input.is_dir() {
            return match format {
                "html" => input.join("index.html"),
                "pdf" => {
                    let name = input
                        .file_name()
                        .map(|value| value.to_string_lossy().to_string())
                        .filter(|value| !value.is_empty())
                        .unwrap_or_else(|| "document".to_string());
                    input.join(format!("{}.pdf", name))
                }
                _ => input.join(format!("document.{}", format)),
            };
        }

        input.with_extension(format)
    }
}

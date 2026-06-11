use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Document {
    pub title: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    UnorderedList(Vec<String>),
    OrderedList(Vec<String>),
    CodeBlock(String),
    Quote(String),
    HorizontalRule,
}

pub fn load_document(input: &Path, explicit_title: Option<&str>) -> Result<Document, String> {
    if input.is_dir() {
        load_directory_document(input, explicit_title)
    } else {
        let source = fs::read_to_string(input)
            .map_err(|e| format!("Failed to read '{}': {}", input.display(), e))?;
        let fallback = fallback_title(input);
        let title = explicit_title.unwrap_or(&fallback);
        Ok(parse_markdown(&source, title))
    }
}

pub fn parse_markdown(source: &str, fallback_title: &str) -> Document {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let lines: Vec<&str> = normalized.lines().collect();
    let mut idx = 0usize;
    let mut blocks = Vec::new();

    while idx < lines.len() {
        let line = lines[idx];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            idx += 1;
            continue;
        }

        if let Some((level, text)) = parse_heading(trimmed) {
            blocks.push(Block::Heading { level, text });
            idx += 1;
            continue;
        }

        if is_horizontal_rule(trimmed) {
            blocks.push(Block::HorizontalRule);
            idx += 1;
            continue;
        }

        if is_code_fence(trimmed) {
            idx += 1;
            let mut code_lines = Vec::new();
            while idx < lines.len() && !is_code_fence(lines[idx].trim()) {
                code_lines.push(lines[idx]);
                idx += 1;
            }
            if idx < lines.len() {
                idx += 1;
            }
            blocks.push(Block::CodeBlock(code_lines.join("\n")));
            continue;
        }

        if let Some(first) = parse_unordered_item(trimmed) {
            let mut items = vec![first];
            idx += 1;
            while idx < lines.len() {
                if let Some(item) = parse_unordered_item(lines[idx].trim()) {
                    items.push(item);
                    idx += 1;
                } else if lines[idx].trim().is_empty() {
                    idx += 1;
                    break;
                } else {
                    break;
                }
            }
            blocks.push(Block::UnorderedList(items));
            continue;
        }

        if let Some(first) = parse_ordered_item(trimmed) {
            let mut items = vec![first];
            idx += 1;
            while idx < lines.len() {
                if let Some(item) = parse_ordered_item(lines[idx].trim()) {
                    items.push(item);
                    idx += 1;
                } else if lines[idx].trim().is_empty() {
                    idx += 1;
                    break;
                } else {
                    break;
                }
            }
            blocks.push(Block::OrderedList(items));
            continue;
        }

        if let Some(first) = trimmed.strip_prefix('>') {
            let mut quote_lines = vec![first.trim().to_string()];
            idx += 1;
            while idx < lines.len() {
                let trimmed = lines[idx].trim();
                if let Some(next) = trimmed.strip_prefix('>') {
                    quote_lines.push(next.trim().to_string());
                    idx += 1;
                } else if trimmed.is_empty() {
                    idx += 1;
                    break;
                } else {
                    break;
                }
            }
            blocks.push(Block::Quote(quote_lines.join(" ")));
            continue;
        }

        let mut paragraph_lines = vec![trimmed.to_string()];
        idx += 1;
        while idx < lines.len() {
            let trimmed = lines[idx].trim();
            if trimmed.is_empty()
                || parse_heading(trimmed).is_some()
                || is_horizontal_rule(trimmed)
                || is_code_fence(trimmed)
                || parse_unordered_item(trimmed).is_some()
                || parse_ordered_item(trimmed).is_some()
                || trimmed.starts_with('>')
            {
                break;
            }
            paragraph_lines.push(trimmed.to_string());
            idx += 1;
        }
        blocks.push(Block::Paragraph(paragraph_lines.join(" ")));
    }

    let title = blocks
        .iter()
        .find_map(|block| match block {
            Block::Heading { level: 1, text } => Some(text.clone()),
            _ => None,
        })
        .unwrap_or_else(|| fallback_title.to_string());

    Document { title, blocks }
}

fn load_directory_document(input: &Path, explicit_title: Option<&str>) -> Result<Document, String> {
    let files = collect_markdown_files(input)?;
    if files.is_empty() {
        return Err(format!(
            "No markdown files found in directory '{}'",
            input.display()
        ));
    }

    let fallback = fallback_title(input);
    let title = explicit_title.unwrap_or(&fallback).to_string();
    let mut blocks = Vec::new();

    for (index, file) in files.iter().enumerate() {
        let source = fs::read_to_string(file)
            .map_err(|e| format!("Failed to read '{}': {}", file.display(), e))?;
        let file_title = fallback_title(file);
        let mut document = parse_markdown(&source, &file_title);

        if index > 0 && !blocks.is_empty() {
            blocks.push(Block::HorizontalRule);
        }

        blocks.append(&mut document.blocks);
    }

    Ok(Document { title, blocks })
}

fn collect_markdown_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_markdown_files_recursive(root, &mut files)?;
    files.sort_by(|left, right| sort_key(root, left).cmp(&sort_key(root, right)));
    Ok(files)
}

fn collect_markdown_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory '{}': {}", dir.display(), e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to read directory '{}': {}", dir.display(), e))?;

    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files_recursive(&path, files)?;
            continue;
        }

        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase());

        if matches!(extension.as_deref(), Some("md") | Some("markdown")) {
            files.push(path);
        }
    }

    Ok(())
}

fn sort_key(root: &Path, path: &Path) -> (u8, String) {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let name = relative.to_string_lossy().replace('\\', "/");
    let lower = name.to_ascii_lowercase();
    let is_readme = lower.ends_with("readme.md") || lower.ends_with("readme.markdown");
    (if is_readme { 0 } else { 1 }, lower)
}

fn fallback_title(path: &Path) -> String {
    path.file_stem()
        .or_else(|| path.file_name())
        .map(|value| value.to_string_lossy().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Marina Document".to_string())
}

fn parse_heading(line: &str) -> Option<(u8, String)> {
    let hashes = line.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }

    let text = line.get(hashes..)?.trim();
    if text.is_empty() {
        return None;
    }

    Some((hashes as u8, text.to_string()))
}

fn is_horizontal_rule(line: &str) -> bool {
    let chars: Vec<char> = line.chars().filter(|ch| !ch.is_whitespace()).collect();
    chars.len() >= 3 && chars.iter().all(|ch| *ch == '-' || *ch == '*')
}

fn is_code_fence(line: &str) -> bool {
    line.starts_with("```")
}

fn parse_unordered_item(line: &str) -> Option<String> {
    ["- ", "* ", "+ "].iter().find_map(|prefix| {
        line.strip_prefix(prefix)
            .map(|value| value.trim().to_string())
    })
}

fn parse_ordered_item(line: &str) -> Option<String> {
    let digits = line.chars().take_while(|ch| ch.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }

    let rest = line.get(digits..)?;
    let text = rest.strip_prefix(". ")?;
    Some(text.trim().to_string())
}

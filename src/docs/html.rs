use crate::docs::config::DocsConfig;
use crate::docs::markdown::{Block, Document};
use crate::docs::themes;

pub fn render_document(document: &Document, config: &DocsConfig) -> String {
    let mut body = String::new();
    body.push_str("<main>\n");

    for block in &document.blocks {
        match block {
            Block::Heading { level, text } => {
                let level = (*level).clamp(1, 6);
                body.push_str(&format!("<h{level}>{}</h{level}>\n", escape_html(text)));
            }
            Block::Paragraph(text) => {
                body.push_str(&format!("<p>{}</p>\n", escape_html(text)));
            }
            Block::UnorderedList(items) => {
                body.push_str("<ul>\n");
                for item in items {
                    body.push_str(&format!("<li>{}</li>\n", escape_html(item)));
                }
                body.push_str("</ul>\n");
            }
            Block::OrderedList(items) => {
                body.push_str("<ol>\n");
                for item in items {
                    body.push_str(&format!("<li>{}</li>\n", escape_html(item)));
                }
                body.push_str("</ol>\n");
            }
            Block::CodeBlock(code) => {
                body.push_str(&format!("<pre><code>{}</code></pre>\n", escape_html(code)));
            }
            Block::Quote(text) => {
                body.push_str(&format!("<blockquote>{}</blockquote>\n", escape_html(text)));
            }
            Block::HorizontalRule => body.push_str("<hr />\n"),
        }
    }

    body.push_str("</main>\n");

    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\" />\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n<title>{}</title>\n<style>\n{}\n</style>\n</head>\n<body>\n{}</body>\n</html>\n",
        escape_html(&document.title),
        themes::stylesheet(&config.theme),
        body
    )
}

fn escape_html(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => ch.to_string(),
        })
        .collect()
}

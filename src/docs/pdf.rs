use crate::docs::config::DocsConfig;
use crate::docs::markdown::{Block, Document};

const PAGE_WIDTH: f32 = 595.0;
const PAGE_HEIGHT: f32 = 842.0;
const MARGIN_LEFT: f32 = 54.0;
const MARGIN_RIGHT: f32 = 54.0;
const MARGIN_TOP: f32 = 54.0;
const MARGIN_BOTTOM: f32 = 54.0;

pub fn render_document(document: &Document, _config: &DocsConfig) -> Result<Vec<u8>, String> {
    let mut renderer = PdfRenderer::new();
    renderer.render(document);
    renderer.build(document)
}

#[derive(Debug, Clone, Copy)]
enum Font {
    Body,
    Bold,
    Mono,
}

impl Font {
    fn resource_name(self) -> &'static str {
        match self {
            Font::Body => "F1",
            Font::Bold => "F2",
            Font::Mono => "F3",
        }
    }
}

struct PdfRenderer {
    pages: Vec<String>,
    current_page: String,
    y: f32,
}

impl PdfRenderer {
    fn new() -> Self {
        Self {
            pages: Vec::new(),
            current_page: String::new(),
            y: PAGE_HEIGHT - MARGIN_TOP,
        }
    }

    fn render(&mut self, document: &Document) {
        for block in &document.blocks {
            match block {
                Block::Heading { level, text } => self.render_heading(*level, text),
                Block::Paragraph(text) => self.render_paragraph(text),
                Block::UnorderedList(items) => self.render_unordered_list(items),
                Block::OrderedList(items) => self.render_ordered_list(items),
                Block::CodeBlock(code) => self.render_code_block(code),
                Block::Quote(text) => self.render_quote(text),
                Block::HorizontalRule => self.render_horizontal_rule(),
            }
        }
    }

    fn render_heading(&mut self, level: u8, text: &str) {
        let (size, before, after) = match level {
            1 => (24.0, 8.0, 14.0),
            2 => (18.0, 12.0, 10.0),
            3 => (15.0, 10.0, 8.0),
            _ => (13.0, 8.0, 6.0),
        };

        self.advance(before);
        self.render_wrapped_lines(text, Font::Bold, size, MARGIN_LEFT, 0.0);
        self.advance(after);
    }

    fn render_paragraph(&mut self, text: &str) {
        self.render_wrapped_lines(text, Font::Body, 12.0, MARGIN_LEFT, 0.0);
        self.advance(10.0);
    }

    fn render_unordered_list(&mut self, items: &[String]) {
        for item in items {
            self.render_wrapped_lines(item, Font::Body, 12.0, MARGIN_LEFT + 16.0, 14.0);
            self.draw_text(Font::Body, 12.0, MARGIN_LEFT, self.y, "-");
            self.advance(6.0);
        }
        self.advance(4.0);
    }

    fn render_ordered_list(&mut self, items: &[String]) {
        for (index, item) in items.iter().enumerate() {
            let marker = format!("{}.", index + 1);
            self.render_wrapped_lines(item, Font::Body, 12.0, MARGIN_LEFT + 20.0, 18.0);
            self.draw_text(Font::Body, 12.0, MARGIN_LEFT, self.y, &marker);
            self.advance(6.0);
        }
        self.advance(4.0);
    }

    fn render_code_block(&mut self, code: &str) {
        for line in code.lines() {
            self.render_wrapped_lines(line, Font::Mono, 10.0, MARGIN_LEFT + 10.0, 0.0);
        }
        self.advance(12.0);
    }

    fn render_quote(&mut self, text: &str) {
        self.render_wrapped_lines(text, Font::Body, 12.0, MARGIN_LEFT + 14.0, 0.0);
        self.draw_vertical_rule(MARGIN_LEFT, self.y + 2.0, self.y + 34.0);
        self.advance(10.0);
    }

    fn render_horizontal_rule(&mut self) {
        self.ensure_space(14.0);
        let y = self.y - 8.0;
        self.current_page.push_str(&format!(
            "0.75 w\n{} {} m\n{} {} l\nS\n",
            format_number(MARGIN_LEFT),
            format_number(y),
            format_number(PAGE_WIDTH - MARGIN_RIGHT),
            format_number(y)
        ));
        self.advance(18.0);
    }

    fn render_wrapped_lines(
        &mut self,
        text: &str,
        font: Font,
        size: f32,
        x: f32,
        extra_indent: f32,
    ) {
        let max_width = PAGE_WIDTH - MARGIN_RIGHT - x;
        let lines = wrap_text(text, max_width, size);
        let line_height = size * 1.4;

        for (index, line) in lines.iter().enumerate() {
            self.ensure_space(line_height);
            let indent = if index == 0 { 0.0 } else { extra_indent };
            self.draw_text(font, size, x + indent, self.y, line);
            self.advance(line_height);
        }
    }

    fn draw_text(&mut self, font: Font, size: f32, x: f32, y: f32, text: &str) {
        self.current_page.push_str(&format!(
            "BT /{} {} Tf 1 0 0 1 {} {} Tm ({}) Tj ET\n",
            font.resource_name(),
            format_number(size),
            format_number(x),
            format_number(y),
            escape_pdf_text(text)
        ));
    }

    fn draw_vertical_rule(&mut self, x: f32, y1: f32, y2: f32) {
        self.current_page.push_str(&format!(
            "1 w\n{} {} m\n{} {} l\nS\n",
            format_number(x),
            format_number(y1),
            format_number(x),
            format_number(y2)
        ));
    }

    fn ensure_space(&mut self, height: f32) {
        if self.y - height < MARGIN_BOTTOM {
            self.finish_page();
        }
    }

    fn advance(&mut self, height: f32) {
        self.y -= height;
    }

    fn finish_page(&mut self) {
        if !self.current_page.is_empty() {
            self.pages.push(std::mem::take(&mut self.current_page));
        }
        self.y = PAGE_HEIGHT - MARGIN_TOP;
    }

    fn build(mut self, document: &Document) -> Result<Vec<u8>, String> {
        self.finish_page();

        if self.pages.is_empty() {
            self.pages.push(String::new());
        }

        let font_body = 3usize;
        let font_bold = 4usize;
        let font_mono = 5usize;
        let first_page_obj = 6usize;
        let info_obj = first_page_obj + self.pages.len() * 2;

        let mut objects = Vec::new();
        objects.push("<< /Type /Catalog /Pages 2 0 R >>".to_string());

        let mut kids = String::new();
        for index in 0..self.pages.len() {
            let page_obj = first_page_obj + index * 2;
            kids.push_str(&format!("{} 0 R ", page_obj));
        }
        objects.push(format!(
            "<< /Type /Pages /Kids [{}] /Count {} >>",
            kids.trim_end(),
            self.pages.len()
        ));
        objects.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string());
        objects.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>".to_string());
        objects.push("<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>".to_string());

        for (index, page_content) in self.pages.iter().enumerate() {
            let page_obj = first_page_obj + index * 2;
            let content_obj = page_obj + 1;

            let page = format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 {} 0 R /F2 {} 0 R /F3 {} 0 R >> >> /Contents {} 0 R >>",
                format_number(PAGE_WIDTH),
                format_number(PAGE_HEIGHT),
                font_body,
                font_bold,
                font_mono,
                content_obj
            );

            let content = format!(
                "<< /Length {} >>\nstream\n{}\nendstream",
                page_content.len(),
                page_content
            );

            if objects.len() + 1 != page_obj {
                return Err("Internal PDF object numbering error".to_string());
            }

            objects.push(page);
            objects.push(content);
        }

        objects.push(format!(
            "<< /Title ({}) /Producer (Marina marina-docs) >>",
            escape_pdf_text(&document.title)
        ));

        if objects.len() != info_obj {
            return Err("Internal PDF trailer numbering error".to_string());
        }

        Ok(serialize_pdf(objects, info_obj))
    }
}

fn serialize_pdf(objects: Vec<String>, info_obj: usize) -> Vec<u8> {
    let mut pdf = Vec::new();
    pdf.extend_from_slice(b"%PDF-1.4\n%\xC7\xEC\x8F\xA2\n");

    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", index + 1, object).as_bytes());
    }

    let xref_offset = pdf.len();
    pdf.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
    pdf.extend_from_slice(b"0000000000 65535 f \n");

    for offset in offsets {
        pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }

    pdf.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R /Info {} 0 R >>\nstartxref\n{}\n%%EOF\n",
            objects.len() + 1,
            info_obj,
            xref_offset
        )
        .as_bytes(),
    );

    pdf
}

fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    if text.trim().is_empty() {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };

        if estimate_text_width(&candidate, font_size) <= max_width || current.is_empty() {
            current = candidate;
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn estimate_text_width(text: &str, font_size: f32) -> f32 {
    text.chars()
        .map(|ch| match ch {
            'i' | 'l' | '!' | '|' | '.' | ',' | ';' | ':' => 0.25,
            'm' | 'w' | 'M' | 'W' => 0.9,
            ' ' => 0.28,
            '0'..='9' => 0.56,
            'A'..='Z' => 0.68,
            _ if ch.is_ascii_punctuation() => 0.35,
            _ => 0.58,
        })
        .sum::<f32>()
        * font_size
}

fn escape_pdf_text(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '\\' => "\\\\".to_string(),
            '(' => "\\(".to_string(),
            ')' => "\\)".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => String::new(),
            _ if ch.is_ascii() => ch.to_string(),
            _ => "?".to_string(),
        })
        .collect()
}

fn format_number(value: f32) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if rounded.fract() == 0.0 {
        format!("{rounded:.0}")
    } else {
        format!("{rounded:.2}")
    }
}

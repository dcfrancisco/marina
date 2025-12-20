#[derive(Debug, Clone, Copy)]
pub struct FormatOptions {
    pub indent_size: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self { indent_size: 4 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockKind {
    If,
    While,
    For,
    Loop,
    Case,
    Function,
    Procedure,
    DoWhile,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    kind: BlockKind,
    base_indent: usize,
    case_in_label_body: bool,
}

/// Format Clipper-2025 source.
///
/// Goals (MVP):
/// - Preserve comments and blank lines
/// - Normalize indentation (default 4 spaces)
/// - Uppercase keywords outside strings
pub fn format_source(source: &str, options: FormatOptions) -> String {
    // Normalize line endings to \n.
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = String::new();

    let lines: Vec<&str> = normalized.split('\n').collect();
    let total_lines = lines.len();

    let mut indent_level: usize = 0;
    let mut stack: Vec<Block> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let line = *line;

        let (code_part, comment_part) = split_line_comment(line);
        let code_trimmed = code_part.trim();

        // Preserve empty lines exactly as empty.
        if code_trimmed.is_empty() {
            if comment_part.is_empty() {
                if idx + 1 < total_lines {
                    out.push('\n');
                }
                continue;
            }

            // Comment-only line: indent at current level.
            out.push_str(&" ".repeat(indent_level * options.indent_size));
            out.push_str(comment_part.trim_start());
            if idx + 1 < total_lines {
                out.push('\n');
            }
            continue;
        }

        let first_token = first_token_upper(code_trimmed);

        // Pre-dedent adjustments (closing tokens, etc.)
        match first_token.as_deref() {
            Some("ENDIF") => {
                if indent_level > 0 {
                    indent_level -= 1;
                }
                pop_if_kind(&mut stack, BlockKind::If);
            }
            Some("ENDDO") => {
                if indent_level > 0 {
                    indent_level -= 1;
                }
                pop_if_kind(&mut stack, BlockKind::While);
            }
            Some("NEXT") => {
                if indent_level > 0 {
                    indent_level -= 1;
                }
                pop_if_kind(&mut stack, BlockKind::For);
            }
            Some("ENDLOOP") => {
                if indent_level > 0 {
                    indent_level -= 1;
                }
                pop_if_kind(&mut stack, BlockKind::Loop);
            }
            Some("ENDCASE") => {
                // Exit any active case label body first.
                if let Some(top) = stack.last_mut() {
                    if top.kind == BlockKind::Case && top.case_in_label_body {
                        if indent_level > 0 {
                            indent_level -= 1;
                        }
                        top.case_in_label_body = false;
                    }
                }

                if indent_level > 0 {
                    indent_level -= 1;
                }
                pop_if_kind(&mut stack, BlockKind::Case);
            }
            Some("ELSE") | Some("ELSEIF") => {
                if indent_level > 0 {
                    indent_level -= 1;
                }
            }
            Some("WHILE") => {
                // Could be DO..WHILE terminator if currently inside DoWhile.
                if matches!(stack.last().map(|b| b.kind), Some(BlockKind::DoWhile)) {
                    if indent_level > 0 {
                        indent_level -= 1;
                    }
                    stack.pop();
                }
            }
            Some("CASE") | Some("OTHERWISE") => {
                // Could be a CASE label inside a CASE block.
                if matches!(stack.last().map(|b| b.kind), Some(BlockKind::Case)) {
                    if let Some(top) = stack.last_mut() {
                        if top.case_in_label_body {
                            if indent_level > 0 {
                                indent_level -= 1;
                            }
                            top.case_in_label_body = false;
                        }
                    }
                }
            }
            Some("RETURN") => {
                // Heuristic: if we are *only* inside a function/procedure block,
                // treat RETURN as the final line and close it.
                if let Some(top) = stack.last() {
                    if (top.kind == BlockKind::Function || top.kind == BlockKind::Procedure)
                        && stack.len() == 1
                    {
                        indent_level = top.base_indent;
                    }
                }
            }
            _ => {}
        }

        // Format the code portion (uppercase keywords outside strings).
        let formatted_code = uppercase_keywords_outside_strings(code_trimmed);

        out.push_str(&" ".repeat(indent_level * options.indent_size));
        out.push_str(&formatted_code);

        if !comment_part.is_empty() {
            // Preserve comment, ensuring a single space before it if code exists.
            out.push(' ');
            out.push_str(comment_part.trim_start());
        }

        // Post-indent adjustments (opening tokens, etc.)
        match first_token.as_deref() {
            Some("IF") => {
                stack.push(Block {
                    kind: BlockKind::If,
                    base_indent: indent_level,
                    case_in_label_body: false,
                });
                indent_level += 1;
            }
            Some("WHILE") => {
                // Only push if it's a normal WHILE loop start (not a DO..WHILE terminator)
                if !matches!(stack.last().map(|b| b.kind), Some(BlockKind::DoWhile))
                    && !matches!(stack.last().map(|b| b.kind), Some(BlockKind::DoWhile))
                {
                    // If it was a terminator, we already popped above.
                    if !matches!(stack.last().map(|b| b.kind), Some(BlockKind::While)) {
                        stack.push(Block {
                            kind: BlockKind::While,
                            base_indent: indent_level,
                            case_in_label_body: false,
                        });
                        indent_level += 1;
                    }
                }
            }
            Some("FOR") => {
                stack.push(Block {
                    kind: BlockKind::For,
                    base_indent: indent_level,
                    case_in_label_body: false,
                });
                indent_level += 1;
            }
            Some("LOOP") => {
                stack.push(Block {
                    kind: BlockKind::Loop,
                    base_indent: indent_level,
                    case_in_label_body: false,
                });
                indent_level += 1;
            }
            Some("DO") => {
                stack.push(Block {
                    kind: BlockKind::DoWhile,
                    base_indent: indent_level,
                    case_in_label_body: false,
                });
                indent_level += 1;
            }
            Some("CASE") => {
                if matches!(stack.last().map(|b| b.kind), Some(BlockKind::Case)) {
                    // CASE label inside CASE statement: start label body.
                    if let Some(top) = stack.last_mut() {
                        top.case_in_label_body = true;
                    }
                    indent_level += 1;
                } else {
                    // CASE statement start
                    stack.push(Block {
                        kind: BlockKind::Case,
                        base_indent: indent_level,
                        case_in_label_body: false,
                    });
                    indent_level += 1;
                }
            }
            Some("OTHERWISE") => {
                if matches!(stack.last().map(|b| b.kind), Some(BlockKind::Case)) {
                    if let Some(top) = stack.last_mut() {
                        top.case_in_label_body = true;
                    }
                    indent_level += 1;
                }
            }
            Some("ELSE") | Some("ELSEIF") => {
                indent_level += 1;
            }
            Some("FUNCTION") => {
                stack.push(Block {
                    kind: BlockKind::Function,
                    base_indent: indent_level,
                    case_in_label_body: false,
                });
                indent_level += 1;
            }
            Some("PROCEDURE") => {
                stack.push(Block {
                    kind: BlockKind::Procedure,
                    base_indent: indent_level,
                    case_in_label_body: false,
                });
                indent_level += 1;
            }
            Some("RETURN") => {
                // If we used the heuristic to place RETURN at base indent, close the function block.
                if let Some(top) = stack.last() {
                    if (top.kind == BlockKind::Function || top.kind == BlockKind::Procedure)
                        && stack.len() == 1
                    {
                        stack.pop();
                        indent_level = 0;
                    }
                }
            }
            _ => {}
        }

        if idx + 1 < total_lines {
            out.push('\n');
        }
    }

    // Ensure we keep trailing newline if the original had one.
    if normalized.ends_with('\n') && !out.ends_with('\n') {
        out.push('\n');
    }

    out
}

fn pop_if_kind(stack: &mut Vec<Block>, kind: BlockKind) {
    if matches!(stack.last().map(|b| b.kind), Some(k) if k == kind) {
        stack.pop();
    }
}

fn first_token_upper(s: &str) -> Option<String> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    if s.starts_with("??") {
        return Some("??".to_string());
    }
    if s.starts_with('?') {
        return Some("?".to_string());
    }

    let mut end = 0;
    for (i, ch) in s.char_indices() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            end = i + ch.len_utf8();
        } else {
            break;
        }
    }

    if end == 0 {
        return None;
    }

    Some(s[..end].to_uppercase())
}

fn split_line_comment(line: &str) -> (&str, &str) {
    // Find // outside of string literals.
    let mut in_string = false;
    let mut prev = '\0';

    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        let c = bytes[i] as char;
        let n = bytes[i + 1] as char;

        if c == '"' && prev != '\\' {
            in_string = !in_string;
        }

        if !in_string && c == '/' && n == '/' {
            return (&line[..i], &line[i..]);
        }

        prev = c;
        i += 1;
    }

    (line, "")
}

fn uppercase_keywords_outside_strings(code: &str) -> String {
    // Keywords list is intentionally small + conservative for MVP.
    const KEYWORDS: &[&str] = &[
        "FUNCTION",
        "PROCEDURE",
        "RETURN",
        "LOCAL",
        "STATIC",
        "PRIVATE",
        "PUBLIC",
        "IF",
        "ELSE",
        "ELSEIF",
        "ENDIF",
        "DO",
        "WHILE",
        "ENDDO",
        "FOR",
        "TO",
        "STEP",
        "NEXT",
        "LOOP",
        "ENDLOOP",
        "EXIT",
        "CASE",
        "OTHERWISE",
        "ENDCASE",
        "TRUE",
        "FALSE",
        "NIL",
        "USE",
        "DBSKIP",
        "DBGOTOP",
        "DBGOBOTTOM",
        "DBSEEK",
        "REPLACE",
        "AND",
        "OR",
        "NOT",
    ];

    let mut out = String::with_capacity(code.len());
    let mut in_string = false;
    let mut prev = '\0';

    let mut word = String::new();

    let flush_word = |out: &mut String, word: &mut String| {
        if word.is_empty() {
            return;
        }
        let upper = word.to_uppercase();
        if KEYWORDS.iter().any(|k| *k == upper) {
            out.push_str(&upper);
        } else {
            out.push_str(word);
        }
        word.clear();
    };

    for ch in code.chars() {
        if ch == '"' && prev != '\\' {
            flush_word(&mut out, &mut word);
            in_string = !in_string;
            out.push(ch);
            prev = ch;
            continue;
        }

        if in_string {
            out.push(ch);
            prev = ch;
            continue;
        }

        if ch.is_ascii_alphanumeric() || ch == '_' {
            word.push(ch);
        } else {
            flush_word(&mut out, &mut word);
            out.push(ch);
        }
        prev = ch;
    }

    // Flush trailing.
    if !word.is_empty() {
        let upper = word.to_uppercase();
        if KEYWORDS.iter().any(|k| *k == upper) {
            out.push_str(&upper);
        } else {
            out.push_str(&word);
        }
    }

    out.trim_end().to_string()
}

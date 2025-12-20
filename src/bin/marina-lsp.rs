// Marina Language Server Protocol (LSP) implementation
// Provides IntelliSense, go-to-definition, diagnostics, and other editor features

#[cfg(feature = "lsp")]
use tower_lsp::jsonrpc::Result;
#[cfg(feature = "lsp")]
use tower_lsp::lsp_types::*;
#[cfg(feature = "lsp")]
use tower_lsp::{Client, LanguageServer, LspService, Server};
#[cfg(feature = "lsp")]
use marina::{Lexer, Parser};
#[cfg(feature = "lsp")]
use marina::diagnostics::{Diagnostic as MarinaDiagnostic, Severity as MarinaSeverity, Span as MarinaSpan};
#[cfg(feature = "lsp")]
use std::collections::HashMap;
#[cfg(feature = "lsp")]
use tokio::sync::RwLock;

#[cfg(feature = "lsp")]
#[derive(Debug)]
struct MarinaLanguageServer {
    client: Client,
    documents: RwLock<HashMap<Url, String>>,
}

#[cfg(feature = "lsp")]
#[tower_lsp::async_trait]
impl LanguageServer for MarinaLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Marina Language Server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    ..Default::default()
                })),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Marina Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Document opened")
            .await;

        {
            let mut docs = self.documents.write().await;
            docs.insert(params.text_document.uri.clone(), params.text_document.text.clone());
        }

        self.diagnose_document(&params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        let updated_text = {
            let mut docs = self.documents.write().await;
            let doc = docs.entry(uri.clone()).or_insert_with(String::new);
            for change in &params.content_changes {
                apply_change(doc, change);
            }
            doc.clone()
        };

        self.diagnose_document(&uri, &updated_text).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        {
            let mut docs = self.documents.write().await;
            docs.remove(&params.text_document.uri);
        }

        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let keywords: &[(&str, &str)] = &[
            ("function", "Define a function"),
            ("procedure", "Define a procedure"),
            ("return", "Return from function/procedure"),
            ("local", "Declare local variable"),
            ("static", "Declare static variable"),
            ("private", "Declare private variable"),
            ("public", "Declare public variable"),

            ("if", "Conditional statement"),
            ("elseif", "Conditional branch"),
            ("else", "Conditional branch"),
            ("endif", "End IF block"),

            ("do", "Begin DO block"),
            ("while", "While loop / DO..WHILE terminator"),
            ("enddo", "End WHILE block"),

            ("for", "For loop"),
            ("to", "For loop range"),
            ("step", "For loop step"),
            ("next", "End FOR block"),
            ("exit", "Exit loop"),

            ("loop", "Begin LOOP block"),
            ("endloop", "End LOOP block"),

            ("case", "Begin CASE block / CASE clause"),
            ("otherwise", "Default CASE clause"),
            ("endcase", "End CASE block"),

            ("use", "Open database/file"),
            ("select", "Select work area"),
            ("dbcreate", "Create DBF"),
            ("dbappend", "Append record"),
            ("dbskip", "Skip record"),
            ("dbgotop", "Go to first record"),
            ("dbgobottom", "Go to last record"),
            ("dbseek", "Seek record"),
            ("index", "Create/set index"),
            ("close", "Close database"),
            ("replace", "Replace field"),
            ("delete", "Mark record deleted"),
            ("recall", "Recall record"),

            ("and", "Logical AND"),
            ("or", "Logical OR"),
            ("not", "Logical NOT"),
            ("true", "Boolean literal"),
            ("false", "Boolean literal"),
            ("nil", "Nil literal"),
        ];

        let mut items = keywords
            .iter()
            .map(|(label, detail)| CompletionItem {
                label: (*label).to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some((*detail).to_string()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        // Built-in functions (now case-insensitive in the VM).
        let builtins: &[(&str, &str)] = &[
            ("clearScreen", "Clear the terminal screen"),
            ("setPos", "Move cursor (row, col)"),
            ("devPos", "Alias of setPos (row, col)"),
            ("gotoXY", "Move cursor (col, row)"),
            ("outStd", "Output text"),
            ("setColor", "Set text color"),
            ("setCursor", "Show/hide cursor"),
            ("savePos", "Save cursor position"),
            ("restorePos", "Restore cursor position"),
            ("sleep", "Sleep (milliseconds)"),
            ("getInput", "Read input with editing"),
            ("getSecret", "Read hidden input"),
            ("replicate", "Repeat string"),
            ("space", "String of spaces"),
            ("len", "Length of string/array"),
            ("subStr", "Substring"),
            ("trim", "Trim both sides"),
            ("allTrim", "Alias of trim"),
            ("lTrim", "Trim left"),
            ("rTrim", "Trim right"),
            ("chr", "ASCII code to char"),
            ("asc", "Char to ASCII code"),
            ("inkey", "Read single key"),
            ("val", "String to number"),
            ("str", "Value to string"),
            ("abs", "Absolute value"),
            ("sqrt", "Square root"),
            ("round", "Round number"),
            ("int", "Truncate number"),
            ("min", "Minimum"),
            ("max", "Maximum"),
            ("sin", "Sine"),
            ("cos", "Cosine"),
            ("tan", "Tangent"),
        ];

        items.extend(builtins.iter().map(|(label, detail)| CompletionItem {
            label: (*label).to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some((*detail).to_string()),
            ..Default::default()
        }));

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "Marina - Clipper-2025 Language".to_string(),
            )),
            range: None,
        }))
    }
}

#[cfg(feature = "lsp")]
fn apply_change(text: &mut String, change: &TextDocumentContentChangeEvent) {
    match change.range {
        None => {
            *text = change.text.clone();
        }
        Some(range) => {
            let start = position_to_offset_utf16(text, range.start);
            let end = position_to_offset_utf16(text, range.end);
            let (start, end) = if start <= end { (start, end) } else { (end, start) };
            if start <= text.len() && end <= text.len() {
                text.replace_range(start..end, &change.text);
            } else {
                // If the client gives us an out-of-range edit, fall back to replacing all text.
                *text = change.text.clone();
            }
        }
    }
}

#[cfg(feature = "lsp")]
fn position_to_offset_utf16(text: &str, position: Position) -> usize {
    let target_line = position.line as usize;
    let target_col_utf16 = position.character as usize;

    // Find the start byte offset of the requested line.
    let mut current_line = 0usize;
    let mut line_start = 0usize;
    for (idx, b) in text.as_bytes().iter().enumerate() {
        if current_line == target_line {
            break;
        }
        if *b == b'\n' {
            current_line += 1;
            line_start = idx + 1;
        }
    }

    // If the requested line is past EOF, clamp to end.
    if current_line < target_line {
        return text.len();
    }

    // Find line end.
    let mut line_end = text.len();
    for (idx, b) in text.as_bytes()[line_start..].iter().enumerate() {
        if *b == b'\n' {
            line_end = line_start + idx;
            break;
        }
    }

    let line = &text[line_start..line_end];
    let mut utf16_units = 0usize;
    let mut byte_in_line = 0usize;
    for ch in line.chars() {
        if utf16_units >= target_col_utf16 {
            break;
        }
        let next = utf16_units + ch.len_utf16();
        if next > target_col_utf16 {
            // Position points into the middle of a surrogate pair; clamp to char start.
            break;
        }
        utf16_units = next;
        byte_in_line += ch.len_utf8();
    }

    (line_start + byte_in_line).min(text.len())
}

#[cfg(feature = "lsp")]
impl MarinaLanguageServer {
    async fn diagnose_document(&self, uri: &Url, text: &str) {
        let diagnostics = self.get_diagnostics(text);
        
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    fn get_diagnostics(&self, text: &str) -> Vec<Diagnostic> {
        let mut out = Vec::new();

        // Lex (collecting multiple lexer diagnostics)
        let mut lexer = Lexer::new(text.to_string());
        let lex_result = lexer.scan_tokens_with_diagnostics();
        for diag in lex_result.diagnostics {
            out.push(to_lsp_diagnostic("Lexer", diag));
        }

        // Parse (collecting multiple parser diagnostics)
        let mut parser = Parser::new(lex_result.tokens);
        let parse_result = parser.parse_with_diagnostics();
        for diag in parse_result.diagnostics {
            out.push(to_lsp_diagnostic("Parser", diag));
        }

        out
    }
}

#[cfg(feature = "lsp")]
fn to_lsp_diagnostic(kind: &str, diag: MarinaDiagnostic) -> Diagnostic {
    let severity = match diag.severity {
        MarinaSeverity::Error => DiagnosticSeverity::ERROR,
        MarinaSeverity::Warning => DiagnosticSeverity::WARNING,
    };

    Diagnostic {
        range: to_lsp_range(diag.span),
        severity: Some(severity),
        source: Some("marina-lsp".to_string()),
        message: format!("{} error: {}", kind, diag.message),
        ..Default::default()
    }
}

#[cfg(feature = "lsp")]
fn to_lsp_range(span: MarinaSpan) -> Range {
    let line0 = span.line.saturating_sub(1) as u32;
    let col0 = span.column.saturating_sub(1) as u32;
    let len = span.len.max(1) as u32;

    Range::new(
        Position::new(line0, col0),
        Position::new(line0, col0.saturating_add(len.saturating_sub(1))),
    )
}

#[cfg(feature = "lsp")]
#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| MarinaLanguageServer {
        client,
        documents: RwLock::new(HashMap::new()),
    });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(not(feature = "lsp"))]
fn main() {
    eprintln!("Error: marina-lsp requires the 'lsp' feature to be enabled");
    eprintln!("Build with: cargo build --bin marina-lsp --features lsp");
    std::process::exit(1);
}

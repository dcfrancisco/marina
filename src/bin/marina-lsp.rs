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
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
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
        if let Some(change) = params.content_changes.first() {
            {
                let mut docs = self.documents.write().await;
                docs.insert(params.text_document.uri.clone(), change.text.clone());
            }
            self.diagnose_document(&params.text_document.uri, &change.text)
                .await;
        }
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
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem {
                label: "FUNCTION".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Define a function".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "LOCAL".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Declare local variable".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "IF".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Conditional statement".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "WHILE".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("While loop".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "FOR".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("For loop".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "RETURN".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Return from function".to_string()),
                ..Default::default()
            },
        ])))
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
impl MarinaLanguageServer {
    async fn diagnose_document(&self, uri: &Url, text: &str) {
        let diagnostics = self.get_diagnostics(text);
        
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    fn get_diagnostics(&self, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Try to lex the source
        let mut lexer = Lexer::new(text.to_string());
        let tokens = match lexer.scan_tokens() {
            Ok(tokens) => tokens,
            Err(e) => {
                let (line, col) = extract_line_col(&e).unwrap_or((1, 1));
                diagnostics.push(Diagnostic {
                    range: single_char_range(line, col),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("marina-lsp".to_string()),
                    message: format!("Lexer error: {}", e),
                    ..Default::default()
                });
                return diagnostics;
            }
        };

        // Try to parse
        let mut parser = Parser::new(tokens);
        if let Err(e) = parser.parse() {
            let (line, col) = extract_line_col(&e).unwrap_or((1, 1));
            diagnostics.push(Diagnostic {
                range: single_char_range(line, col),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("marina-lsp".to_string()),
                message: format!("Parser error: {}", e),
                ..Default::default()
            });
        }

        diagnostics
    }
}

#[cfg(feature = "lsp")]
fn extract_line_col(message: &str) -> Option<(u32, u32)> {
    // Supports messages like:
    // - "... at line 3, column 14"
    // - "... at line 3"
    let line = extract_number_after(message, "at line ")?;
    let col = extract_number_after(message, "column ").unwrap_or(1);
    Some((line, col))
}

#[cfg(feature = "lsp")]
fn extract_number_after(message: &str, needle: &str) -> Option<u32> {
    let start = message.find(needle)? + needle.len();
    let digits: String = message[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<u32>().ok()
}

#[cfg(feature = "lsp")]
fn single_char_range(line_1_based: u32, col_1_based: u32) -> Range {
    let line0 = line_1_based.saturating_sub(1);
    let col0 = col_1_based.saturating_sub(1);
    Range::new(Position::new(line0, col0), Position::new(line0, col0.saturating_add(1)))
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

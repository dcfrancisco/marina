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
#[derive(Debug)]
struct MarinaLanguageServer {
    client: Client,
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
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("marina-lsp".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                    },
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
        
        self.diagnose_document(&params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.first() {
            self.diagnose_document(&params.text_document.uri, &change.text)
                .await;
        }
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
        if let Err(e) = lexer.scan_tokens() {
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Lexer error: {}", e),
                ..Default::default()
            });
            return diagnostics;
        }

        // Try to parse
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        if let Err(e) = parser.parse() {
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Parser error: {}", e),
                ..Default::default()
            });
        }

        diagnostics
    }
}

#[cfg(feature = "lsp")]
#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| MarinaLanguageServer { client });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(not(feature = "lsp"))]
fn main() {
    eprintln!("Error: marina-lsp requires the 'lsp' feature to be enabled");
    eprintln!("Build with: cargo build --bin marina-lsp --features lsp");
    std::process::exit(1);
}

mod document;
mod diagnostics;
mod lexer;
mod parser;
mod semantic;

use document::DocumentStore;
use parser::Parser;
use semantic::SemanticAnalyzer;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// The main LSP server implementation for QBHD BASIC.
///
/// Provides IDE features (completion, hover, definition, references, rename)
/// by combining a custom lexer/parser/semantic analyzer with external
/// compiler diagnostics from `qbhd --json --check`.
#[derive(Debug)]
struct QbhdLsp {
    /// LSP client for sending notifications to the editor.
    client: Client,
    /// In-memory store of open document texts and versions.
    documents: Arc<Mutex<DocumentStore>>,
    /// Symbol table and semantic analysis engine.
    analyzer: Arc<Mutex<SemanticAnalyzer>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for QbhdLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        " ".to_string(),
                        ".".to_string(),
                        "$".to_string(),
                        "(".to_string(),
                    ]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions::default(),
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "qbhd-lsp".to_string(),
                version: Some("0.2.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "QBHD LSP server initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text.clone();
        let version = params.text_document.version;

        self.documents
            .lock()
            .await
            .open(uri.clone(), text.clone(), version);

        self.reanalyze(&uri, &text).await;
        self.publish_diagnostics(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text.clone();
            self.documents
                .lock()
                .await
                .change(&uri, text.clone(), version);

            self.reanalyze(&uri, &text).await;
            self.publish_diagnostics(uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.lock().await.close(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        // Get word prefix for filtering
        let doc = self.documents.lock().await;
        let prefix = if let Some(document) = doc.get(&uri) {
            document
                .text
                .lines()
                .nth(pos.line as usize)
                .and_then(|line| {
                    let chars: Vec<char> = line.chars().collect();
                    let mut start = pos.character as usize;
                    while start > 0
                        && (chars[start - 1].is_ascii_alphanumeric()
                            || chars[start - 1] == '_'
                            || chars[start - 1] == '$')
                    {
                        start -= 1;
                    }
                    Some(chars[start..pos.character as usize].iter().collect::<String>())
                })
                .unwrap_or_default()
        } else {
            String::new()
        };
        drop(doc);

        let items = self
            .analyzer
            .lock()
            .await
            .get_completions()
            .into_iter()
            .filter(|label| {
                if prefix.is_empty() {
                    return true;
                }
                label.to_uppercase().starts_with(&prefix.to_uppercase())
            })
            .map(|label| {
                let kind = if label.starts_with('_') {
                    CompletionItemKind::FUNCTION
                } else if label.to_uppercase() == label && label.chars().all(|c| c.is_ascii_alphabetic() || c == '$') {
                    CompletionItemKind::KEYWORD
                } else {
                    CompletionItemKind::VARIABLE
                };
                CompletionItem {
                    label: label.clone(),
                    kind: Some(kind),
                    detail: get_completion_detail(&label),
                    insert_text: Some(label),
                    ..Default::default()
                }
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let doc = self.documents.lock().await;
        let Some(document) = doc.get(&uri) else {
            return Ok(None);
        };

        let word = semantic::SemanticAnalyzer::new()
            .get_symbol_at_position(&document.text, pos.line, pos.character);
        drop(doc);

        let Some(word) = word else {
            return Ok(None);
        };

        let hover_text = self.analyzer.lock().await.get_hover(&word);

        Ok(hover_text.map(|text| Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: text,
            }),
            range: None,
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let doc = self.documents.lock().await;
        let Some(document) = doc.get(&uri) else {
            return Ok(None);
        };

        let word = semantic::SemanticAnalyzer::new()
            .get_symbol_at_position(&document.text, pos.line, pos.character);
        drop(doc);

        let Some(word) = word else {
            return Ok(None);
        };

        let analyzer = self.analyzer.lock().await;
        let mut location = analyzer.find_definition(&word);
        drop(analyzer);

        // Override the URI to the actual document
        if let Some(ref mut loc) = location {
            loc.uri = uri;
        }

        Ok(location.map(GotoDefinitionResponse::Scalar))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let doc = self.documents.lock().await;
        let Some(document) = doc.get(&uri) else {
            return Ok(None);
        };

        let word = semantic::SemanticAnalyzer::new()
            .get_symbol_at_position(&document.text, pos.line, pos.character);
        drop(doc);

        let Some(word) = word else {
            return Ok(None);
        };

        let analyzer = self.analyzer.lock().await;
        let mut refs = analyzer.find_references(&word);
        drop(analyzer);

        // Override URIs to the actual document
        for r in &mut refs {
            r.uri = uri.clone();
        }

        Ok(Some(refs))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let new_name = params.new_name;

        let doc = self.documents.lock().await;
        let Some(document) = doc.get(&uri) else {
            return Ok(None);
        };

        let word = semantic::SemanticAnalyzer::new()
            .get_symbol_at_position(&document.text, pos.line, pos.character);
        drop(doc);

        let Some(word) = word else {
            return Ok(None);
        };

        let analyzer = self.analyzer.lock().await;
        let refs = analyzer.find_references(&word);
        drop(analyzer);

        let text_edits: Vec<TextEdit> = refs
            .into_iter()
            .map(|loc| TextEdit {
                range: loc.range,
                new_text: new_name.clone(),
            })
            .collect();

        let mut changes = std::collections::HashMap::new();
        changes.insert(uri, text_edits);

        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }))
    }

    async fn code_action(&self, _: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        // Basic code actions - can be expanded later
        Ok(Some(vec![]))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl QbhdLsp {
    async fn reanalyze(&self, _uri: &Url, text: &str) {
        let mut parser = Parser::new(text);
        let stmts = parser.parse();
        self.analyzer.lock().await.analyze(&stmts);
    }

    async fn publish_diagnostics(&self, uri: Url) {
        let file_path = uri.to_file_path().ok();
        let Some(path) = file_path else { return };
        let Some(path_str) = path.to_str() else { return };

        let diagnostics = diagnostics::get_diagnostics(path_str);

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

fn get_completion_detail(label: &str) -> Option<String> {
    let upper = label.to_uppercase();
    match upper.as_str() {
        "PRINT" => Some("Output text to screen".to_string()),
        "INPUT" => Some("Read user input".to_string()),
        "IF" => Some("Conditional statement".to_string()),
        "FOR" => Some("Counter loop".to_string()),
        "WHILE" => Some("Condition loop".to_string()),
        "DO" => Some("Loop construct".to_string()),
        "SELECT" => Some("Multi-way branch".to_string()),
        "DIM" => Some("Declare variable".to_string()),
        "SUB" => Some("Subroutine definition".to_string()),
        "FUNCTION" => Some("Function definition".to_string()),
        "GOTO" => Some("Unconditional jump".to_string()),
        "GOSUB" => Some("Subroutine call".to_string()),
        "RETURN" => Some("Return from subroutine".to_string()),
        "END" => Some("End program/block".to_string()),
        "CLS" => Some("Clear screen".to_string()),
        "SCREEN" => Some("Set screen mode".to_string()),
        "COLOR" => Some("Set text color".to_string()),
        "LINE" => Some("Draw line/box".to_string()),
        "CIRCLE" => Some("Draw circle".to_string()),
        "OPEN" => Some("Open file".to_string()),
        "CLOSE" => Some("Close file".to_string()),
        "AND" => Some("Logical AND".to_string()),
        "OR" => Some("Logical OR".to_string()),
        "NOT" => Some("Logical NOT".to_string()),
        "MOD" => Some("Modulo operator".to_string()),
        "_RGB" => Some("Create RGB color".to_string()),
        "_NEWIMAGE" => Some("Create image buffer".to_string()),
        _ => None,
    }
}

/// Entry point for the QBHD LSP server.
///
/// Starts the language server on stdin/stdout using the LSP protocol.
/// Logging is directed to stderr to avoid corrupting the JSON-RPC stream.
#[tokio::main]
async fn main() {
    // IMPORTANT: Log to stderr, not stdout. stdout is used for LSP JSON-RPC messages.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| QbhdLsp {
        client,
        documents: Arc::new(Mutex::new(DocumentStore::new())),
        analyzer: Arc::new(Mutex::new(SemanticAnalyzer::new())),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

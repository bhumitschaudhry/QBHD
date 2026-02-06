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

#[derive(Debug)]
struct QbhdLsp {
    client: Client,
    documents: Arc<Mutex<DocumentStore>>,
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
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions::default(),
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "QBHD LSP initialized")
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

        let mut parser = Parser::new(&text);
        let stmts = parser.parse();
        self.analyzer.lock().await.analyze(&stmts);

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

            let mut parser = Parser::new(&text);
            let stmts = parser.parse();
            self.analyzer.lock().await.analyze(&stmts);

            self.publish_diagnostics(uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.lock().await.close(&params.text_document.uri);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let items = self
            .analyzer
            .lock()
            .await
            .get_completions()
            .into_iter()
            .map(|label| CompletionItem {
                label,
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
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

        let line = document.text.lines().nth(pos.line as usize)?;
        let word = line
            .split_whitespace()
            .find(|_| true)
            .unwrap_or("");

        let hover_text = self.analyzer.lock().await.get_hover(word);

        Ok(hover_text.map(|text| Hover {
            contents: HoverContents::Scalar(MarkedString::String(text)),
            range: None,
        }))
    }

    async fn goto_definition(
        &self,
        _: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }

    async fn references(&self, _: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl QbhdLsp {
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| QbhdLsp {
        client,
        documents: Arc::new(Mutex::new(DocumentStore::new())),
        analyzer: Arc::new(Mutex::new(SemanticAnalyzer::new())),
    });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}

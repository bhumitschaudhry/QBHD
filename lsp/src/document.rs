//! In-memory document store for the LSP server.
//!
//! Tracks open documents by URI with their current text content and version numbers.

use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

/// A single open document.
#[derive(Debug, Clone)]
pub struct Document {
    /// The document's URI (typically a file:// URL)
    pub uri: Url,
    /// The full text content of the document
    pub text: String,
    /// The document version number (incremented on each change)
    pub version: i32,
}

/// Thread-safe store for all open documents.
///
/// Documents are keyed by URI. The store supports open, change, close, and get operations.
/// Thread safety is provided by the caller (typically via `Arc<Mutex<DocumentStore>>`).
#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: HashMap<Url, Document>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, uri: Url, text: String, version: i32) {
        self.documents.insert(
            uri.clone(),
            Document { uri, text, version },
        );
    }

    pub fn change(&mut self, uri: &Url, text: String, version: i32) {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.text = text;
            doc.version = version;
        }
    }

    pub fn close(&mut self, uri: &Url) {
        self.documents.remove(uri);
    }

    pub fn get(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }
}

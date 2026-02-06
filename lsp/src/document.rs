use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

#[derive(Debug, Clone)]
pub struct Document {
    pub uri: Url,
    pub text: String,
    pub version: i32,
}

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

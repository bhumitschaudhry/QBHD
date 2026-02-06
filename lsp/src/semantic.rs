use crate::parser::{Stmt, Expr};
use std::collections::HashMap;

pub struct SemanticAnalyzer {
    symbols: HashMap<String, SymbolInfo>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable,
    Function,
    Sub,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Dim(name) => {
                self.symbols.insert(
                    name.clone(),
                    SymbolInfo {
                        name: name.clone(),
                        kind: SymbolKind::Variable,
                        line: 0,
                    },
                );
            }
            Stmt::Assignment(name, _) => {
                if !self.symbols.contains_key(name) {
                    self.symbols.insert(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Variable,
                            line: 0,
                        },
                    );
                }
            }
            _ => {}
        }
    }

    pub fn get_completions(&self) -> Vec<String> {
        let mut items = vec![
            "PRINT", "DIM", "IF", "THEN", "ELSE", "FOR", "NEXT", "SUB", "FUNCTION", "END",
            "_RGB", "_NEWIMAGE", "SCREEN", "CLS", "CIRCLE", "LINE",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        
        items.extend(self.symbols.keys().cloned());
        items
    }

    pub fn get_hover(&self, symbol: &str) -> Option<String> {
        if let Some(info) = self.symbols.get(symbol) {
            Some(format!("{:?}: {}", info.kind, info.name))
        } else {
            match symbol.to_uppercase().as_str() {
                "PRINT" => Some("PRINT - Output text to screen".to_string()),
                "_RGB" => Some("_RGB(r, g, b) - Create RGB color value".to_string()),
                _ => None,
            }
        }
    }
}

use std::process::Command;
use serde::Deserialize;
use tower_lsp::lsp_types::*;

#[derive(Debug, Deserialize)]
struct QbhdDiagnostic {
    file: String,
    line: i32,
    column: i32,
    severity: String,
    message: String,
}

pub fn get_diagnostics(file_path: &str) -> Vec<Diagnostic> {
    let output = Command::new("./qbhd")
        .args(&["--json", "--check", file_path])
        .output();

    let Ok(output) = output else {
        return vec![];
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Ok(diags): Result<Vec<QbhdDiagnostic>, _> = serde_json::from_str(&stdout) else {
        return vec![];
    };

    diags
        .into_iter()
        .map(|d| Diagnostic {
            range: Range {
                start: Position {
                    line: (d.line - 1).max(0) as u32,
                    character: (d.column - 1).max(0) as u32,
                },
                end: Position {
                    line: (d.line - 1).max(0) as u32,
                    character: d.column.max(0) as u32 + 10,
                },
            },
            severity: Some(match d.severity.as_str() {
                "error" => DiagnosticSeverity::ERROR,
                "warning" => DiagnosticSeverity::WARNING,
                _ => DiagnosticSeverity::INFORMATION,
            }),
            message: d.message,
            ..Default::default()
        })
        .collect()
}

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
    // Try to find qbhd binary in common locations
    let qbhd_cmd = find_qbhd_binary();

    let output = Command::new(&qbhd_cmd)
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
                    character: (d.column - 1).max(0) as u32 + 10,
                },
            },
            severity: Some(match d.severity.as_str() {
                "error" => DiagnosticSeverity::ERROR,
                "warning" => DiagnosticSeverity::WARNING,
                _ => DiagnosticSeverity::INFORMATION,
            }),
            source: Some("qbhd".to_string()),
            message: d.message,
            ..Default::default()
        })
        .collect()
}

fn find_qbhd_binary() -> String {
    // Check environment variable first
    if let Ok(path) = std::env::var("QBHD_PATH") {
        return path;
    }

    // Check if qbhd is in PATH
    if Command::new("qbhd").arg("--version").output().is_ok() {
        return "qbhd".to_string();
    }

    // Check common locations
    let candidates = if cfg!(target_os = "windows") {
        vec![
            "qbhd.exe",
            "./qbhd.exe",
            "../qbhd.exe",
            "../../qbhd.exe",
        ]
    } else {
        vec![
            "qbhd",
            "./qbhd",
            "../qbhd",
            "../../qbhd",
            "/usr/local/bin/qbhd",
        ]
    };

    for candidate in &candidates {
        if Command::new(candidate).arg("--version").output().is_ok() {
            return candidate.to_string();
        }
    }

    // Fallback
    "qbhd".to_string()
}

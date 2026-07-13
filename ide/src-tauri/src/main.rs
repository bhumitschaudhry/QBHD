#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    output: Mutex<String>,
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {}", path, e))
}

#[tauri::command]
fn save_file(path: String, contents: String) -> Result<(), String> {
    fs::write(&path, &contents).map_err(|e| format!("Failed to save {}: {}", path, e))
}

#[tauri::command]
fn compile_file(path: String) -> Result<String, String> {
    let output = Command::new("qbhd")
        .arg(&path)
        .output()
        .map_err(|e| format!("Failed to run qbhd: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(format!("{}\nBuild succeeded.", stdout.trim()))
    } else {
        Err(format!("{}\n{}", stdout.trim(), stderr.trim()))
    }
}

#[tauri::command]
fn check_file(path: String) -> Result<String, String> {
    let output = Command::new("qbhd")
        .args(&["--json", "--check", &path])
        .output()
        .map_err(|e| format!("Failed to run qbhd: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        // Return stdout anyway -- check mode outputs JSON diagnostics even on warnings
        if !stdout.trim().is_empty() {
            Ok(stdout)
        } else {
            Err(stderr)
        }
    }
}

#[tauri::command]
fn run_file(path: String, state: State<AppState>) -> Result<String, String> {
    let output = Command::new(&path)
        .output()
        .map_err(|e| format!("Failed to run {}: {}", path, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let result = if output.status.success() {
        stdout
    } else {
        format!("{}\n{}", stdout, stderr)
    };

    *state.output.lock().unwrap() = result.clone();
    Ok(result)
}

#[tauri::command]
fn list_directory(path: String) -> Result<Vec<DirEntry>, String> {
    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory {}: {}", path, e))?;

    let mut result: Vec<DirEntry> = entries
        .filter_map(|e| e.ok())
        .map(|e| {
            let metadata = e.metadata().ok();
            DirEntry {
                name: e.file_name().to_string_lossy().to_string(),
                path: e.path().to_string_lossy().to_string(),
                is_dir: metadata.map(|m| m.is_dir()).unwrap_or(false),
            }
        })
        .collect();

    result.sort_by(|a, b| {
        // Directories first, then by name
        b.is_dir.cmp(&a.is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(result)
}

#[derive(serde::Serialize)]
struct DirEntry {
    name: String,
    path: String,
    is_dir: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            output: Mutex::new(String::new()),
        })
        .invoke_handler(tauri::generate_handler![
            read_file,
            save_file,
            compile_file,
            check_file,
            run_file,
            list_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

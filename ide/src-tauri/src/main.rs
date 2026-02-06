#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::Manager;

#[tauri::command]
fn compile_file(path: String) -> Result<String, String> {
    let output = Command::new("qbhd")
        .arg(&path)
        .output()
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn check_file(path: String) -> Result<String, String> {
    let output = Command::new("qbhd")
        .args(&["--json", "--check", &path])
        .output()
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn run_file(path: String) -> Result<(), String> {
    Command::new(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile_file, check_file, run_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

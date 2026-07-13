# Security Review

## Current Security Posture

The QBHD project has minimal security measures. This document identifies known issues and recommendations.

## Critical Issues

### 1. Arbitrary Command Execution in IDE Backend

**File:** `ide/src-tauri/src/main.rs:63`
**Severity:** Critical

The `run_file` command executes arbitrary file paths with no validation:

```rust
Command::new(&path)
    .spawn()
    .map_err(|e| format!("Failed to run {}: {}", path, e))?;
```

A malicious path could execute any program on the system.

**Recommendation:** Validate that the path is a compiled QBHD binary (check extension, location, or signature).

### 2. Unrestricted Filesystem Access

**File:** `ide/src-tauri/tauri.conf.json:34`
**Severity:** High

The filesystem scope is set to `["**"]`, granting the IDE access to the entire filesystem:

```json
"fs": {
    "all": true,
    "scope": ["**"]
}
```

**Recommendation:** Restrict scope to project directories and common BASIC file locations.

### 3. Shell Scope Bypass

**File:** `ide/src-tauri/src/main.rs`
**Severity:** High

The Tauri shell scope restricts execution to `qbhd` and `qbhd-run` commands, but the backend uses `std::process::Command` directly, bypassing the scope entirely.

**Recommendation:** Use Tauri's shell API (`tauri::api::shell`) instead of `std::process::Command`.

### 4. LSP Tracing to stdout

**File:** `lsp/src/main.rs:353`
**Severity:** Medium

The tracing subscriber writes to stdout, which is also used for LSP JSON-RPC communication:

```rust
tracing_subscriber::fmt().init();
```

This can corrupt the LSP protocol stream and cause data leakage.

**Recommendation:** Write tracing output to stderr or a file:
```rust
tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .init();
```

## Medium Issues

### 5. No Input Validation on File Paths

**File:** `ide/src-tauri/src/main.rs` (all commands)
**Severity:** Medium

File paths from the frontend are passed directly to filesystem operations with no validation. Path traversal attacks could access files outside the project.

**Recommendation:** Validate paths are within the project directory before operations.

### 6. No Content Security Policy

**File:** `ide/src-tauri/tauri.conf.json`
**Severity:** Medium

No CSP is configured for the Tauri webview. This allows inline scripts and external resource loading.

**Recommendation:** Add CSP headers to restrict script sources.

### 7. Blocking Subprocess Calls

**File:** `ide/src-tauri/src/main.rs` (compile_file, check_file, run_file)
**Severity:** Medium (availability)

All subprocess calls block the Tauri main thread, causing UI freezes during compilation.

**Recommendation:** Use async subprocess execution via `tokio::process::Command` or Tauri's async command support.

## Low Issues

### 8. No Rate Limiting

**Severity:** Low

The IDE has no rate limiting on compilation or file operations. Rapid button clicks could spawn many concurrent processes.

### 9. No Authentication

**Severity:** Low (desktop app)

As a desktop application, authentication is not typically required. However, if the IDE were to support remote collaboration, authentication would be needed.

### 10. Unused Dependencies

**File:** `ide/src-tauri/Cargo.toml`, `lsp/Cargo.toml`
**Severity:** Low

Unused dependencies (`serde_json` in IDE, `anyhow`/`thiserror` in LSP) increase the attack surface unnecessarily.

## Dependency Analysis

### Rust Dependencies (LSP Server)

| Crate | Version | Known CVEs | Notes |
|-------|---------|------------|-------|
| tower-lsp | 0.20 | None known | LSP framework |
| tokio | 1 | None known | Async runtime |
| serde | 1 | None known | Serialization |
| serde_json | 1 | None known | JSON parsing |
| tracing | 0.1 | None known | Logging |
| tracing-subscriber | 0.3 | None known | Log output |

### JavaScript Dependencies (IDE Frontend)

| Package | Version | Known CVEs | Notes |
|---------|---------|------------|-------|
| react | ^18.2.0 | None known | UI framework |
| react-dom | ^18.2.0 | None known | DOM rendering |
| @tauri-apps/api | ^1.5.0 | None known | Tauri IPC |
| @monaco-editor/react | ^4.6.0 | None known | Code editor |
| vite | ^5.0.0 | None known | Build tool |

### C/C++ Dependencies (Runtime)

Inherited from QB64 upstream. These are well-established libraries:
- FreeGLUT - OpenGL windowing
- FreeType - Font rendering
- miniaudio - Audio
- STB libraries - Image/audio
- zlib - Compression

## Recommendations Summary

### Immediate (Critical)

1. Validate file paths before executing commands
2. Restrict filesystem scope to project directories
3. Fix tracing to write to stderr instead of stdout

### Short-term (High)

4. Use Tauri's shell API instead of std::process::Command
5. Add CSP headers to the webview
6. Add async subprocess execution

### Medium-term (Medium)

7. Add rate limiting for compilation
8. Remove unused dependencies
9. Add path validation for all file operations

### Long-term (Low)

10. Add authentication for remote collaboration features
11. Implement sandboxing for compiled program execution
12. Add integrity checks for the compiler binary

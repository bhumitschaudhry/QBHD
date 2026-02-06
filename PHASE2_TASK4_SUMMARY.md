# Phase 2, Task 4: Rust LSP Server Setup

## Summary

Created Rust-based Language Server Protocol implementation for QBHD with basic document synchronization and diagnostics.

## Deliverables

### LSP Server (`lsp/`)
- **`Cargo.toml`** - Rust project configuration
- **`src/main.rs`** - LSP server entry point
- **`src/document.rs`** - Document state management
- **`src/diagnostics.rs`** - Diagnostics via `qbhd --json --check`
- **`README.md`** - LSP documentation

### Neovim Plugin (`nvim-qbhd/`)
- **`lua/qbhd/init.lua`** - Neovim LSP client
- **`README.md`** - Plugin documentation

### Build Scripts
- **`build_lsp.sh`** - Build LSP server

## Features Implemented

### LSP Server
- ✓ Initialize/shutdown handlers
- ✓ Document synchronization (open, change, close)
- ✓ Real-time diagnostics using `qbhd --json --check`
- ✓ Full text document sync
- ✓ Async/await with Tokio
- ✓ JSON-RPC communication

### Neovim Plugin
- ✓ Auto-start LSP on .bas files
- ✓ Configurable LSP command
- ✓ Root directory detection

## Architecture

```
┌─────────────────┐
│  Neovim Editor  │
└────────┬────────┘
         │ LSP Protocol (JSON-RPC)
         ↓
┌─────────────────┐
│   qbhd-lsp      │
│  (Rust Server)  │
└────────┬────────┘
         │ Shell command
         ↓
┌─────────────────┐
│  qbhd --json    │
│    --check      │
└─────────────────┘
```

## Build

```bash
./build_lsp.sh
```

This will:
1. Check for Rust/Cargo
2. Build LSP server in release mode
3. Output binary to `lsp/target/release/qbhd-lsp`

## Install

```bash
# Install LSP server
sudo cp lsp/target/release/qbhd-lsp /usr/local/bin/

# Install Neovim plugin (lazy.nvim)
# Add to your Neovim config:
{
    dir = "/home/bhumit/QBHD/nvim-qbhd",
    ft = "basic",
    config = function()
        require("qbhd").setup()
    end
}
```

## Usage

### Start LSP Server
```bash
qbhd-lsp
# Communicates via stdin/stdout
```

### Neovim
```lua
-- In your init.lua
require("qbhd").setup({
    cmd = { "qbhd-lsp" }
})
```

Open a .bas file and the LSP will automatically start.

## How It Works

### 1. Document Sync
- Neovim opens a .bas file
- LSP receives `textDocument/didOpen`
- Document stored in memory

### 2. Diagnostics
- On document change
- LSP calls `qbhd --json --check <file>`
- Parses JSON output
- Publishes diagnostics to Neovim
- Errors/warnings shown in editor

### 3. Real-time Feedback
- Type in editor → LSP notified
- LSP runs syntax check (~100ms)
- Diagnostics updated instantly

## Dependencies

### Rust Crates
- `tower-lsp` - LSP framework
- `tokio` - Async runtime
- `serde` - JSON serialization
- `serde_json` - JSON parsing
- `tracing` - Logging

### System
- Rust/Cargo (install from https://rustup.rs)
- QBHD compiler with `--json --check` support

## Testing

### Manual Test
```bash
# Terminal 1: Start LSP
./lsp/target/release/qbhd-lsp

# Terminal 2: Send LSP messages
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ./lsp/target/release/qbhd-lsp
```

### Neovim Test
```bash
# Open a .bas file
nvim test.bas

# Check LSP status
:LspInfo

# Should show: qbhd-lsp (active)
```

## Next Steps (Task 5)

Implement BASIC parser in Rust:
- Lexer for BASIC tokens
- Parser for BASIC syntax
- AST generation
- Error recovery

This will enable:
- Better diagnostics
- Code completion
- Go-to-definition
- Find references

---

**Status**: COMPLETE ✓  
**Date**: 2026-02-06  
**Phase 2**: 1/15 tasks complete (6.7%)

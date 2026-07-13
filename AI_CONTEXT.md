# AI Context

This file enables another AI agent to become productive in this repository with minimal additional context.

## Repository Summary

QBHD is a modernized fork of QB64, a BASIC-to-C++ compiler. The project strips the original QB64 text-mode IDE and adds three modern frontends: a CLI compiler, a Rust LSP server, and a Tauri GUI IDE. The compiler backend (26,000 lines of BASIC) is unchanged from upstream QB64.

## Coding Conventions

### BASIC (source/)
- Inherited from QB64 upstream
- Uses `$INCLUDE` for modular includes
- `DEFLNG A-Z` (default long integer)
- Constants in `source/global/constants.bas`
- Delimiter characters: `CHR$(13)`, `CHR$(10)`, `CHR$(26)`

### Rust (lsp/, ide/src-tauri/)
- Edition 2021
- No async runtime preference (tokio for LSP, tauri's runtime for IDE)
- Error handling via `Result<T, String>` for Tauri commands
- LSP uses `tower_lsp::jsonrpc::Result`
- No custom error types (anyhow/thiserror imported but unused)

### JavaScript (ide/src/)
- Plain JSX (no TypeScript)
- React 18 functional components with hooks
- No state management library (useState/useCallback only)
- Monaco Editor for code editing
- CSS modules not used (single App.css file)

### Lua (nvim-qbhd/)
- Standard Neovim Lua plugin pattern
- Module table `M` with `M.setup(opts)` function
- Uses `vim.lsp`, `vim.diagnostic`, `vim.api`

### Shell (build scripts, .ci/)
- Bash for Linux/macOS
- Batch for Windows
- `set -e` for error handling

## Folder Purposes

| Folder | Purpose |
|--------|---------|
| `source/` | QB64/QBHD BASIC compiler source |
| `source/global/` | Global constants, settings, version |
| `source/utilities/` | File, string, config, INI utilities |
| `source/ide/` | IDE code (stripped by `strip_ide.py`) |
| `source/subs_functions/` | BASIC subroutine/function implementations |
| `lsp/` | Rust LSP server |
| `lsp/src/` | LSP implementation (lexer, parser, semantic, diagnostics) |
| `nvim-qbhd/` | Neovim plugin |
| `nvim-qbhd/lua/qbhd/` | Plugin Lua code |
| `nvim-qbhd/syntax/` | Vim syntax highlighting |
| `nvim-qbhd/ftdetect/` | Filetype detection |
| `ide/` | Tauri GUI IDE |
| `ide/src/` | React frontend |
| `ide/src/components/` | React components |
| `ide/src-tauri/` | Rust Tauri backend |
| `internal/` | QB64 runtime libraries (C/C++) |
| `internal/c/` | C/C++ runtime source |
| `internal/c/libqb/` | Core QB64 runtime |
| `internal/c/parts/` | Modular components (audio, video, input, etc.) |
| `internal/help/` | BASIC keyword help documentation (90+ files) |
| `.ci/` | CI/CD bootstrap and build scripts |
| `.github/workflows/` | GitHub Actions workflows |
| `licenses/` | Third-party license files |

## Common Workflows

### Building the project
```bash
./build.sh            # Full build (compiler + LSP + IDE deps)
./build.sh --lsp      # LSP server only
./build.sh --ide      # IDE dependencies only
./build.sh --test     # Run tests
```

### Running the IDE
```bash
cd ide
npm install           # First time
npm run tauri dev     # Development mode
npm run tauri build   # Production build
```

### Running the LSP server
```bash
cd lsp
cargo build --release
# Binary at lsp/target/release/qbhd-lsp
```

### Compiling a BASIC program
```bash
./qbhd myprogram.bas
./qbhd --json --check myprogram.bas  # For LSP diagnostics
./qbhd --version
```

## Architecture Decisions

1. **Stripped compiler, not forked**: Rather than maintaining a separate compiler, QBHD strips IDE code from the original QB64 source. This preserves compatibility but creates a dependency on the upstream source structure.

2. **LSP for editor integration**: Using the Language Server Protocol allows any LSP-compatible editor (Neovim, VS Code, etc.) to get IDE features without custom integrations.

3. **Tauri for GUI IDE**: Tauri provides a lightweight desktop app using web technologies (React) with a Rust backend. Smaller binary size than Electron.

4. **Monaco Editor**: The same editor that powers VS Code, embedded in the Tauri IDE. Provides familiar editing experience.

5. **External diagnostics**: The LSP server shells out to `qbhd --json --check` for diagnostics rather than implementing its own type checker. This ensures diagnostics match the compiler exactly.

6. **Pratt parsing for expressions**: The parser uses Pratt parsing (top-down operator precedence) for expression parsing, which handles operator precedence cleanly.

## Important Files

| File | Why It Matters |
|------|---------------|
| `source/qb64.bas` | Original QB64 source (26,345 lines) |
| `source/qbhd_compiler.bas` | Compiler that gets built (26,348 lines) |
| `lsp/src/main.rs` | LSP protocol implementation |
| `lsp/src/parser.rs` | BASIC parser (830 lines) |
| `lsp/src/semantic.rs` | Symbol table and hover docs |
| `ide/src/App.jsx` | IDE state management |
| `ide/src/components/Editor.jsx` | Monaco BASIC language definition |
| `ide/src-tauri/src/main.rs` | Tauri IPC commands |
| `nvim-qbhd/lua/qbhd/init.lua` | Neovim plugin setup |
| `build.sh` | Unified build script |
| `strip_ide.py` | IDE code removal |

## Entry Points

- **CLI compiler**: `source/qbhd_compiler.bas` (compiled to `qbhd` binary)
- **LSP server**: `lsp/src/main.rs` (compiled to `qbhd-lsp` binary)
- **IDE frontend**: `ide/src/main.jsx` → `ide/src/App.jsx`
- **IDE backend**: `ide/src-tauri/src/main.rs`
- **Neovim plugin**: `nvim-qbhd/lua/qbhd/init.lua` → `M.setup()`
- **Build system**: `build.sh`

## Common Commands

```bash
# Build everything
./build.sh

# Build just the compiler
./build.sh && ./qbhd --version

# Build just the LSP
./build.sh --lsp

# Run the IDE in dev mode
cd ide && npm run tauri dev

# Check a BASIC file
./qbhd --json --check myfile.bas

# Run a BASIC program
./qbhd myfile.bas && ./myfile
```

## Build Process

1. `install_deps()` - Install platform dependencies (g++, mesa, etc.)
2. `build_qb64()` - Build the QB64 compiler from source if not present
3. `build_qbhd()` - Run `strip_ide.py` then compile `qbhd_compiler.bas`
4. `apply_enhancements()` - Patch CLI flags into compiler source
5. `build_lsp()` - `cargo build --release` in `lsp/`
6. `build_ide()` - `npm install` in `ide/`

## Testing Process

```bash
./build.sh --test
# Runs test_cli_enhancements.sh and test_suite.py
# Tests: --version, --help, --check, --json, --verbose, --output, --optimize, --debug
```

Note: Test files (`test_hello.bas`, `test_suite.py`) may not be present in the repository.

## Known Limitations

1. **No tests in repo**: Test files are referenced but not present
2. **Blocking subprocess calls**: Tauri commands and LSP diagnostics block the UI
3. **No TypeScript**: All JavaScript is plain JSX with no type checking
4. **No linting**: No ESLint, Prettier, or clippy configuration
5. **Tracing to stdout**: LSP server's tracing can corrupt JSON-RPC protocol
6. **Approximate line numbers**: Semantic analyzer uses statement index, not source lines
7. **No scope tracking**: All symbols in a flat namespace
8. **Single-line IF**: Parser requires `END IF`
9. **Dead dependencies**: `anyhow`, `thiserror`, `serde_json` (IDE) are unused
10. **Hardcoded paths**: Some docs reference `/home/bhumit/QBHD`

## Glossary

| Term | Meaning |
|------|---------|
| QB64 | Original BASIC-to-C++ compiler (upstream) |
| QBHD | This project - modernized QB64 fork |
| LSP | Language Server Protocol |
| Monarch | Monaco Editor's tokenizer DSL |
| Pratt parsing | Top-down operator precedence parsing |
| libqb | QB64's core runtime library (C) |
| FreeGLUT | OpenGL Utility Toolkit (windowing) |
| Tauri | Desktop app framework (Rust + web frontend) |
| `$INCLUDE` | BASIC file inclusion directive |
| `NoIDEMode` | QB64 variable that disables the built-in IDE |
| `ConsoleMode` | QB64 variable that enables console output |

# Contributing to QBHD

## Getting Started

### Prerequisites

- **Linux**: `g++`, `mesa-common-dev`, `libglu1-mesa-dev`, `libasound2-dev`, `libx11-dev`
- **macOS**: Xcode Command Line Tools
- **Windows**: MinGW/GCC or Visual Studio Build Tools, Python 3
- **Optional**: Rust/Cargo (for LSP server), Node.js/npm (for IDE)

### Building

```bash
# Full build
./build.sh

# Just the compiler
./build.sh

# Just the LSP server
./build.sh --lsp

# Just the IDE dependencies
./build.sh --ide
```

### Running

```bash
# CLI compiler
./qbhd myprogram.bas

# IDE
cd ide && npm run tauri dev

# LSP server (after building)
lsp/target/release/qbhd-lsp
```

## Project Structure

See `REPOSITORY_INDEX.md` for a complete file listing.

Key directories:
- `source/` - BASIC compiler source
- `lsp/` - Rust LSP server
- `ide/` - Tauri GUI IDE
- `nvim-qbhd/` - Neovim plugin
- `.ci/` - CI/CD scripts

## Making Changes

### Compiler Changes

The compiler source is `source/qbhd_compiler.bas` (generated from `source/qb64.bas` by `strip_ide.py`). To modify the compiler:

1. Edit `source/qb64.bas` (the upstream source)
2. Run `python3 strip_ide.py` to regenerate `qbhd_compiler.bas`
3. Test with `./qbhd --version` and `./qbhd --check test.bas`

To add CLI flags:
1. Edit `apply_cli_enhancements.sh`
2. Run `./build.sh` to rebuild

### LSP Server Changes

The LSP server is in `lsp/src/`. Key files:
- `main.rs` - LSP protocol handler
- `lexer.rs` - Tokenizer
- `parser.rs` - Parser
- `semantic.rs` - Symbol table and analysis
- `diagnostics.rs` - Compiler integration

To modify:
1. Edit files in `lsp/src/`
2. Run `cd lsp && cargo build --release`
3. Test with an LSP client (Neovim, VS Code)

### IDE Changes

The IDE is split between React frontend (`ide/src/`) and Rust backend (`ide/src-tauri/src/`).

Frontend:
1. Edit JSX files in `ide/src/`
2. Run `cd ide && npm run tauri dev`
3. Changes hot-reload automatically

Backend:
1. Edit `ide/src-tauri/src/main.rs`
2. Run `cd ide && npm run tauri dev`
3. Tauri restarts the backend automatically

### Neovim Plugin Changes

The plugin is in `nvim-qbhd/lua/qbhd/init.lua`.

1. Edit the Lua file
2. Restart Neovim or run `:luafile %`
3. Test with `:QBHDCheck` or `:QBHDCompile`

## Code Style

### Rust
- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting

### JavaScript/JSX
- No TypeScript (plain JSX)
- Use functional components with hooks
- Follow existing code style (no Prettier configured)

### BASIC
- Follow QB64 conventions
- Use `DEFLNG A-Z`
- Comments with `'` or `REM`

### Lua
- Follow Neovim Lua conventions
- Use `vim.api` for Neovim APIs

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Commit with descriptive messages
6. Push and create a pull request

### Commit Messages

Follow conventional commit format:
```
type: short description

Longer description if needed.
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Reporting Issues

Use the GitHub issue templates:
- Bug reports: Include QBHD version, OS, reproduction steps
- Feature requests: Describe the problem and desired solution

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT).

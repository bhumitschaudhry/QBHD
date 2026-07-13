# QBHD - Modern QB64 Compiler & IDE

QBHD is a modernized fork of QB64 that replaces the built-in text-mode IDE with modern tooling: a command-line compiler, a Rust-based Language Server Protocol (LSP) server, a Neovim plugin, and a Tauri-based GUI IDE.

## Quick Start

### Prerequisites

- **Linux**: `g++`, `mesa`, `libglu`, `alsa-lib`, `libx11`
- **macOS**: Xcode Command Line Tools
- **Windows**: MinGW/GCC or Visual Studio Build Tools, Python 3

### Build Everything

```bash
./build.sh
```

This will:
1. Install platform dependencies
2. Build the QB64 compiler backend
3. Strip IDE code to create the QBHD compiler
4. Apply CLI enhancements (`--json`, `--check`, etc.)
5. Build the Rust LSP server (if Cargo is installed)
6. Install IDE dependencies (if Node.js is installed)

### Build Individual Components

```bash
# Just the compiler
./build.sh

# Just the LSP server
./build.sh --lsp

# Just the IDE
./build.sh --ide

# Run tests
./build.sh --test
```

## Components

### 1. QBHD Compiler

The core compiler transpiles BASIC to C++ and compiles to native executables.

```bash
# Compile a program
./qbhd myprogram.bas

# Check for errors (JSON output for editor integration)
./qbhd --json --check myprogram.bas

# Show version
./qbhd --version

# Compile with custom output
./qbhd myprogram.bas -o myapp
```

**CLI Flags:**
| Flag | Description |
|------|-------------|
| `--json` | Output diagnostics as JSON |
| `--check` | Check for errors without compiling |
| `--output <file>` | Set output filename |
| `--optimize` | Enable optimizations |
| `--debug` | Include debug info |
| `--verbose` | Verbose output |
| `--version` | Show version |

### 2. LSP Server (`lsp/`)

A Rust-based Language Server Protocol implementation for BASIC.

**Features:**
- Real-time diagnostics (shells out to `qbhd --json --check`)
- Code completion (50+ BASIC/QB64 keywords + user symbols)
- Hover documentation for keywords and functions
- Go-to-definition
- Find references
- Rename symbol
- Syntax-aware parsing (IF, FOR, WHILE, DO, SELECT CASE, SUB, FUNCTION, etc.)

**Building:**
```bash
cd lsp
cargo build --release
```

**Usage with Neovim:**
Install the `nvim-qbhd` plugin (see below).

### 3. Neovim Plugin (`nvim-qbhd/`)

A Lua plugin for Neovim with LSP integration and BASIC syntax highlighting.

**Installation (lazy.nvim):**
```lua
{
  "your-username/qbhd",
  ft = "basic",
  config = function()
    require("qbhd").setup({
      cmd = { "qbhd-lsp" },  -- or path to the binary
    })
  end,
}
```

**Features:**
- Auto-start LSP on `.bas` and `.bi` files
- Syntax highlighting (keywords, types, QB64 extensions, comments, strings, numbers)
- Diagnostic signs and virtual text
- Keybindings: `gd` (definition), `K` (hover), `gr` (references), `<leader>rn` (rename), `<leader>ca` (code action)
- Commands: `:QBHDCompile`, `:QBHDRun`, `:QBHDCheck`, `:QBHDFormat`, `:QBHDInfo`

### 4. GUI IDE (`ide/`)

A Tauri-based desktop IDE with Monaco Editor.

**Tech Stack:**
- **Frontend**: React 18, Monaco Editor, Vite 5
- **Backend**: Rust (Tauri 1.5)

**Features:**
- Monaco Editor with BASIC syntax highlighting
- File explorer with directory tree
- Integrated output terminal
- Check/Build/Run toolbar
- Save with Ctrl+S
- Status bar with cursor position
- Open folder dialog
- Dark VS Code-inspired theme

**Running:**
```bash
cd ide
npm install
npm run tauri dev
```

**Building:**
```bash
cd ide
npm run tauri build
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  Frontend                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │
│  │ Neovim   │  │ CLI      │  │ Tauri IDE    │   │
│  │ Plugin   │  │          │  │ (Monaco)     │   │
│  └────┬─────┘  └────┬─────┘  └──────┬───────┘   │
│       │              │               │            │
│  ┌────▼──────────────▼───────────────▼───────┐   │
│  │          QBHD LSP Server (Rust)           │   │
│  │  Lexer → Parser → Semantic Analyzer       │   │
│  └─────────────────┬─────────────────────────┘   │
│                    │                              │
│  ┌─────────────────▼─────────────────────────┐   │
│  │       QBHD Compiler (BASIC → C++)         │   │
│  └─────────────────┬─────────────────────────┘   │
│                    │                              │
│  ┌─────────────────▼─────────────────────────┐   │
│  │     Native Compiler (g++/clang++)         │   │
│  └───────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## Supported BASIC Syntax

The LSP parser understands:
- `PRINT`, `INPUT`, `DIM`, `REDIM`
- `IF...THEN...ELSE...END IF`
- `FOR...NEXT` (with `STEP`)
- `WHILE...WEND`
- `DO...LOOP` (with `WHILE`/`UNTIL`)
- `SELECT CASE...END SELECT`
- `SUB...END SUB`, `FUNCTION...END FUNCTION`
- `CALL`, `GOTO`, `GOSUB`, `RETURN`
- `EXIT SUB/FUNCTION/FOR/DO`
- `OPEN`, `CLOSE`, `GET`, `PUT`
- `CLS`, `SCREEN`, `COLOR`, `LOCATE`
- `LINE`, `CIRCLE`, `PSET`, `PAINT`, `DRAW`
- All QB64 underscore keywords (`_RGB`, `_NEWIMAGE`, etc.)
- Comments (`'` and `REM`)
- Binary operators with precedence (`+`, `-`, `*`, `/`, `\`, `^`, `AND`, `OR`, `NOT`, `MOD`, `XOR`)

## Differences from QB64

QBHD removes the built-in text-mode IDE and replaces it with:
- Modern CLI with structured output
- LSP server for editor integration
- Neovim plugin with full IDE features
- Tauri GUI IDE with Monaco Editor

The compiler backend is unchanged from QB64 - full compatibility is maintained.

## License

Same as QB64 - MIT License

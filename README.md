# QBHD - Modern BASIC Development Environment

![QBHD](source/qb64.png)

QBHD is a modernized BASIC compiler with three professional interfaces: command-line tools, Neovim LSP integration, and a modern GUI IDE. It retains QB4.5/QBasic compatibility while compiling native binaries for Windows, Linux, and macOS.

[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/QBHDTeam/qb64/issues)

## Table of Contents

1. [Quick Start](#quick-start)
2. [Installation](#installation)
3. [Usage](#usage)
   - [Command Line](#command-line)
   - [Neovim + LSP](#neovim--lsp)
   - [GUI IDE](#gui-ide)
4. [Features](#features)
5. [Platform Support](#platform-support)

## Quick Start

```bash
# Build all components
./build.sh          # Compiler
./build_lsp.sh      # LSP server
./build_ide.sh      # GUI IDE

# Use the CLI
qbhd --check myprogram.bas
qbhd myprogram.bas -o myprogram

# Use Neovim
nvim myprogram.bas

# Use GUI IDE
cd ide && npm run tauri dev
```

## Installation

### Prerequisites

- **Linux**: GCC/G++, OpenGL, ALSA
- **macOS**: Xcode command line tools (`xcode-select --install`)
- **Windows**: MinGW or MSVC

### Build from Source

```bash
# Clone repository
git clone https://github.com/QBHDTeam/qbhd.git
cd qbhd

# Build compiler
./build.sh

# Build LSP server (requires Rust)
./build_lsp.sh

# Build GUI IDE (requires Node.js)
./build_ide.sh
```

### Install Binaries

```bash
# Install compiler
sudo cp qbhd /usr/local/bin/

# Install LSP server
sudo cp lsp/target/release/qbhd-lsp /usr/local/bin/

# IDE runs from source directory
```

## Usage

### Command Line

The QBHD compiler provides a modern CLI with powerful flags.

#### Basic Compilation

```bash
# Compile and run
qbhd myprogram.bas

# Compile to specific output
qbhd myprogram.bas -o myprogram

# Compile without running
qbhd -c myprogram.bas
```

#### Advanced Options

```bash
# Check syntax without compiling
qbhd --check myprogram.bas

# Optimize compilation (0-3)
qbhd --optimize 3 myprogram.bas

# Include debug symbols
qbhd --debug myprogram.bas

# Verbose output
qbhd --verbose myprogram.bas

# JSON output for tools
qbhd --json --check myprogram.bas

# Show version
qbhd --version
```

#### CLI Examples

```bash
# Quick syntax check
qbhd --check program.bas

# Production build with optimization
qbhd --optimize 3 program.bas -o program

# Debug build with symbols
qbhd --debug program.bas

# Check multiple files
for f in *.bas; do qbhd --check "$f"; done

# Compile and capture JSON output
qbhd --json --check program.bas > diagnostics.json
```

### Neovim + LSP

Full Language Server Protocol support for Neovim with real-time diagnostics, code completion, and navigation.

#### Setup

Add to your `init.lua`:

```lua
{
    "nvim-lspconfig",
    config = function()
        local lspconfig = require("lspconfig")
        lspconfig.qbhd_lsp.setup({
            cmd = { "qbhd-lsp" },
            filetypes = { "basic" },
            root_dir = lspconfig.util.root_pattern(".git", vim.fn.getcwd()),
        })
    end
}
```

Or use the included plugin:

```lua
{
    dir = "/path/to/qbhd/nvim-qbhd",
    ft = "basic",
    config = function()
        require("qbhd").setup()
    end
}
```

#### Keybindings

| Key | Action |
|-----|--------|
| `gd` | Go to definition |
| `K` | Hover information |
| `gr` | Find references |
| `<leader>rn` | Rename symbol |
| `<leader>ca` | Code actions |

#### Commands

```vim
:QBHDCompile      " Compile current file
:QBHDRun          " Run compiled program
:QBHDCheck        " Check syntax
:QBHDFormat       " Format code
```

#### Features

- **Real-time Diagnostics**: Errors, warnings, and info as you type
- **Code Completion**: Keywords, variables, and functions
- **Hover Info**: Symbol documentation and type information
- **Navigation**: Jump to definitions, find all references
- **Semantic Analysis**: Type checking and symbol tracking

#### Neovim Example

```bash
# Open file with LSP
nvim myprogram.bas

# In Neovim:
# - Type and see diagnostics appear
# - Press 'gd' to jump to function definition
# - Press 'K' to see hover documentation
# - Press 'gr' to find all usages
# - Use <C-x><C-o> for completion
```

### GUI IDE

Modern Tauri-based IDE with visual editor, file explorer, and integrated terminal.

#### Launch

```bash
cd ide
npm install
npm run tauri dev
```

#### Features

- **Monaco Editor**: Professional code editor with syntax highlighting
- **File Explorer**: Browse and manage .bas files
- **Build Panel**: Check, Build, and Run buttons
- **Integrated Terminal**: View compilation output
- **Settings**: Configure editor behavior
- **Dark Theme**: Professional VS Code-inspired design

#### Workflow

1. **Open Project**: Select folder containing .bas files
2. **Edit Code**: Use Monaco editor with full syntax highlighting
3. **Check Syntax**: Click "Check" button for instant feedback
4. **Build**: Click "Build" to compile
5. **Run**: Click "Run" to execute compiled program
6. **View Output**: See results in integrated terminal

#### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+S` | Save file |
| `Ctrl+/` | Toggle comment |
| `Ctrl+Shift+F` | Format code |
| `Ctrl+B` | Toggle file explorer |
| `Ctrl+J` | Toggle terminal |

## Features

### Compiler

- ✓ QB4.5/QBasic compatible syntax
- ✓ Modern CLI with 7 flags
- ✓ Fast syntax checking
- ✓ Optimization levels (0-3)
- ✓ Debug symbol support
- ✓ JSON output for tooling
- ✓ Cross-platform compilation

### LSP Server

- ✓ Document synchronization
- ✓ Real-time diagnostics (<100ms)
- ✓ Code completion (50+ keywords)
- ✓ Hover documentation
- ✓ Go-to-definition
- ✓ Find references
- ✓ Semantic analysis
- ✓ Symbol renaming

### IDE

- ✓ Monaco editor integration
- ✓ File tree explorer
- ✓ Integrated terminal
- ✓ Build integration
- ✓ Dark theme
- ✓ Settings panel
- ✓ Professional UI/UX

## Platform Support

| Platform | Status | Build |
|----------|--------|-------|
| Linux | ✓ Supported | `./build.sh` |
| macOS | ✓ Supported | `./build.sh` |
| Windows | ✓ Supported | `./build.sh` |

## Examples

### Hello World

```basic
PRINT "Hello, QBHD!"
END
```

Compile and run:
```bash
qbhd hello.bas
```

### Using LSP in Neovim

```bash
nvim hello.bas
# Type code and see diagnostics
# Press gd to navigate
# Press K for documentation
```

### Using GUI IDE

```bash
cd ide && npm run tauri dev
# Open folder with .bas files
# Edit in Monaco editor
# Click Build and Run
```

## Documentation

- [CLI Quick Reference](CLI_QUICKREF.txt)
- [CLI Enhancements](CLI_ENHANCEMENTS.md)
- [Quick Start Guide](QUICKSTART.md)
- [Project Plan](PLAN.md)
- [Status](STATUS.md)

## Contributing

Contributions welcome! Please check the [issues page](https://github.com/QBHDTeam/qb64/issues).

## License

See LICENSE file for details.
# QBHD Usage Guide

Complete guide for using QBHD across all three interfaces.

## Table of Contents

1. [Command Line Interface](#command-line-interface)
2. [Neovim Integration](#neovim-integration)
3. [GUI IDE](#gui-ide)
4. [Workflow Examples](#workflow-examples)
5. [Troubleshooting](#troubleshooting)

---

## Command Line Interface

### Basic Commands

```bash
# Compile and run
qbhd myprogram.bas

# Compile to output file
qbhd myprogram.bas -o myprogram

# Compile without running
qbhd -c myprogram.bas
```

### Flags Reference

| Flag | Description | Example |
|------|-------------|---------|
| `--check` | Syntax check only | `qbhd --check file.bas` |
| `--optimize N` | Optimization level (0-3) | `qbhd --optimize 3 file.bas` |
| `--debug` | Include debug symbols | `qbhd --debug file.bas` |
| `--verbose` | Verbose output | `qbhd --verbose file.bas` |
| `--json` | JSON output | `qbhd --json --check file.bas` |
| `--output FILE` | Output filename | `qbhd file.bas --output prog` |
| `--version` | Show version | `qbhd --version` |

### Advanced Usage

#### Batch Processing

```bash
# Check all BASIC files
for f in *.bas; do
  echo "Checking $f..."
  qbhd --check "$f"
done

# Compile all files
for f in *.bas; do
  qbhd "$f" -o "${f%.bas}"
done
```

#### JSON Output for Tooling

```bash
# Get diagnostics as JSON
qbhd --json --check program.bas > diagnostics.json

# Parse with jq
qbhd --json --check program.bas | jq '.diagnostics'
```

#### Optimization Levels

```bash
# No optimization (fastest compile)
qbhd --optimize 0 program.bas

# Balanced (default)
qbhd --optimize 1 program.bas

# Aggressive
qbhd --optimize 2 program.bas

# Maximum (slowest compile, fastest runtime)
qbhd --optimize 3 program.bas
```

#### Debug Builds

```bash
# Build with debug symbols
qbhd --debug program.bas -o program_debug

# Verbose compilation
qbhd --verbose --debug program.bas
```

---

## Neovim Integration

### Installation

1. Install LSP server:
```bash
./build_lsp.sh
sudo cp lsp/target/release/qbhd-lsp /usr/local/bin/
```

2. Add to `init.lua`:
```lua
{
    dir = "/path/to/qbhd/nvim-qbhd",
    ft = "basic",
    config = function()
        require("qbhd").setup()
    end
}
```

### Navigation

| Keybinding | Action |
|-----------|--------|
| `gd` | Go to definition |
| `K` | Hover information |
| `gr` | Find references |
| `<leader>rn` | Rename symbol |
| `<leader>ca` | Code actions |

### Commands

```vim
" Compile current file
:QBHDCompile

" Run compiled program
:QBHDRun

" Check syntax
:QBHDCheck

" Format code
:QBHDFormat
```

### Real-time Features

- **Diagnostics**: Errors (✗), warnings (⚠), info (ℹ), hints (💡)
- **Completion**: Press `<C-x><C-o>` for code completion
- **Hover**: Hover over symbols to see documentation
- **Floating Windows**: Enhanced UI with floating diagnostic windows

### Workflow Example

```bash
# Open file
nvim myprogram.bas

# In Neovim:
# 1. Type code - see diagnostics in real-time
# 2. Press 'gd' to jump to function definition
# 3. Press 'K' to see hover documentation
# 4. Press 'gr' to find all usages
# 5. Use <C-x><C-o> for completion
# 6. :QBHDCompile to build
# 7. :QBHDRun to execute
```

---

## GUI IDE

### Launch

```bash
cd ide
npm install
npm run tauri dev
```

### Interface Overview

**Left Panel**: File explorer with .bas files
**Center**: Monaco editor with syntax highlighting
**Bottom**: Integrated terminal for output
**Top**: Toolbar with Check, Build, Run buttons

### Workflow

1. **Open Project**
   - Click "Open Folder" or use File menu
   - Select directory containing .bas files

2. **Edit Code**
   - Click file in explorer to open
   - Edit in Monaco editor
   - Auto-save enabled

3. **Check Syntax**
   - Click "Check" button
   - View diagnostics in terminal

4. **Build**
   - Click "Build" button
   - Compilation output shown in terminal

5. **Run**
   - Click "Run" button
   - Program output shown in terminal

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+S` | Save file |
| `Ctrl+/` | Toggle comment |
| `Ctrl+Shift+F` | Format code |
| `Ctrl+B` | Toggle file explorer |
| `Ctrl+J` | Toggle terminal |
| `Ctrl+K Ctrl+0` | Fold all |
| `Ctrl+K Ctrl+J` | Unfold all |

### Settings

Access via Settings menu:
- Editor font size
- Tab size
- Theme (dark/light)
- Auto-save interval
- Terminal font size

---

## Workflow Examples

### Example 1: Quick Syntax Check

```bash
# CLI approach
qbhd --check myprogram.bas

# Neovim approach
nvim myprogram.bas
# :QBHDCheck

# IDE approach
# Open IDE, click Check button
```

### Example 2: Development Cycle

```bash
# Terminal workflow
while true; do
  qbhd --check myprogram.bas
  sleep 2
done

# Neovim workflow
nvim myprogram.bas
# Edit, save, :QBHDCheck, repeat

# IDE workflow
# Open IDE, edit, click Check, repeat
```

### Example 3: Production Build

```bash
# CLI approach
qbhd --optimize 3 --debug myprogram.bas -o myprogram

# Neovim approach
nvim myprogram.bas
# :QBHDCompile (uses optimized settings)

# IDE approach
# Open IDE, click Build button
```

### Example 4: Batch Compilation

```bash
# Compile all files
for f in *.bas; do
  echo "Building $f..."
  qbhd --optimize 2 "$f" -o "${f%.bas}"
done

# Check all files
for f in *.bas; do
  qbhd --json --check "$f" >> results.json
done
```

### Example 5: CI/CD Integration

```bash
# GitHub Actions example
- name: Check BASIC files
  run: |
    for f in *.bas; do
      qbhd --check "$f" || exit 1
    done

- name: Build
  run: qbhd --optimize 3 main.bas -o main

- name: Test
  run: ./main
```

---

## Troubleshooting

### LSP Not Starting

```bash
# Check if qbhd-lsp is installed
which qbhd-lsp

# Rebuild LSP
./build_lsp.sh
sudo cp lsp/target/release/qbhd-lsp /usr/local/bin/

# Check Neovim logs
:LspInfo
```

### Compilation Errors

```bash
# Get verbose output
qbhd --verbose myprogram.bas

# Check syntax first
qbhd --check myprogram.bas

# Use JSON for detailed diagnostics
qbhd --json --check myprogram.bas
```

### IDE Not Launching

```bash
# Check Node.js version
node --version  # Should be 14+

# Reinstall dependencies
cd ide
rm -rf node_modules
npm install

# Rebuild
npm run tauri build
```

### Performance Issues

```bash
# Use lower optimization level
qbhd --optimize 0 myprogram.bas

# Check file size
wc -l myprogram.bas

# Split large files
# Break into multiple files and use includes
```

### File Not Found

```bash
# Check current directory
pwd

# Use absolute paths
qbhd /full/path/to/myprogram.bas

# Check file permissions
ls -la myprogram.bas
```

---

## Tips & Tricks

### Alias for Common Commands

```bash
# Add to ~/.bashrc or ~/.zshrc
alias qbcheck='qbhd --check'
alias qbopt='qbhd --optimize 3'
alias qbdebug='qbhd --debug'
```

### Watch for Changes

```bash
# Compile on file change
while inotifywait -e modify myprogram.bas; do
  qbhd --check myprogram.bas
done
```

### Parallel Compilation

```bash
# Compile multiple files in parallel
find . -name "*.bas" | xargs -P 4 -I {} qbhd --check {}
```

### Integration with Make

```makefile
.PHONY: check build run clean

check:
	qbhd --check *.bas

build:
	qbhd --optimize 3 main.bas -o main

run: build
	./main

clean:
	rm -f main
```

---

## Getting Help

```bash
# Show version
qbhd --version

# Check syntax
qbhd --check myprogram.bas

# Verbose output
qbhd --verbose myprogram.bas

# JSON diagnostics
qbhd --json --check myprogram.bas
```

For more information, see:
- [README.md](README.md)
- [CLI_QUICKREF.txt](CLI_QUICKREF.txt)
- [QUICKSTART.md](QUICKSTART.md)

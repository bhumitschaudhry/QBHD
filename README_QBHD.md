# QBHD - Modern QB64 Compiler

QBHD is a modernized version of QB64 focused on being a powerful command-line compiler with modern editor integration.

## Quick Start

### Build QBHD

```bash
./build_qbhd.sh
```

This will:
1. Build QB64 if not already built
2. Strip IDE code to create compiler-only version
3. Build the QBHD compiler

### Usage

```bash
# Show help
./qbhd --help

# Compile a program
./qbhd -c myprogram.bas

# Compile with custom output name
./qbhd -c myprogram.bas -o myapp

# Compile and show output
./qbhd -x myprogram.bas

# Show warnings
./qbhd -c -w myprogram.bas
```

### Test

```bash
# Compile and run test program
./qbhd -x test_hello.bas
```

## Project Status

### Phase 1: Compiler Core ✓ (In Progress)
- [x] Task 1: Strip IDE code - **COMPLETE**
- [ ] Task 2: Enhance CLI (--json, --check flags)
- [ ] Task 3: Update build system

### Phase 2: LSP & Neovim (Not Started)
- [ ] Tasks 4-13: Language Server and Neovim plugin

### Phase 3: Tauri IDE (Not Started)
- [ ] Tasks 14-23: Full IDE implementation

## Architecture

```
QBHD Compiler (BASIC → C++)
    ↓
Native Compiler (g++/clang++)
    ↓
Executable
```

Future architecture will include:
- Rust-based Language Server Protocol (LSP) implementation
- Neovim plugin for editing
- Tauri-based GUI IDE

## Development

### Files
- `source/qbhd_compiler.bas` - Compiler-only source (generated)
- `strip_ide.py` - Script to remove IDE code
- `build_qbhd.sh` - Complete build script
- `PLAN.md` - Full implementation plan

### Building from Source
See `PLAN.md` for the complete implementation roadmap.

## Differences from QB64

QBHD removes the built-in text-mode IDE and focuses on:
- Command-line compilation
- Modern editor integration (Neovim, VS Code, etc.)
- Language Server Protocol support
- Optional GUI IDE (Tauri-based)

## License

Same as QB64 - MIT License

## Contributing

See `PLAN.md` for the implementation plan and current status.

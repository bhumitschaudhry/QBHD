# QBHD Project File Structure

## Root Directory
```
QBHD/
├── README.md                      # Original QB64 README
├── README_QBHD.md                 # QBHD user guide
├── CHANGELOG.md                   # Version history
├── .gitignore                     # Git ignore rules
│
├── PLAN.md                        # Complete implementation plan (34 tasks)
├── STATUS.md                      # Current status and quick start
├── IMPLEMENTATION_SUMMARY.md      # Overall progress summary
│
├── PHASE1_TASK1_SUMMARY.md        # Task 1: Strip IDE (COMPLETE)
├── PHASE1_TASK2_SUMMARY.md        # Task 2: Enhanced CLI (COMPLETE)
├── TASK2_COMPLETE.md              # Task 2 completion checklist
│
├── CLI_ENHANCEMENTS.md            # CLI user guide
├── CLI_QUICKREF.txt               # Quick reference card
│
├── test_hello.bas                 # Test program
│
├── strip_ide.py                   # Remove IDE code from qb64.bas
├── build_qbhd.sh                  # Build QBHD compiler
├── build_compiler_only.sh         # Alternative build script
├── apply_cli_enhancements.sh      # Apply CLI enhancements
├── test_cli_enhancements.sh       # Test CLI flags
├── add_json_support.bas           # JSON output helpers
│
├── setup_lnx.sh                   # Linux setup (original QB64)
├── setup_osx.command              # macOS setup (original QB64)
├── setup_win.cmd                  # Windows setup (original QB64)
│
├── qb64                           # QB64 compiler (if built)
├── qbhd                           # QBHD compiler (if built)
│
└── source/
    ├── qb64.bas                   # Original QB64 source (~1.1MB)
    ├── qbhd_compiler.bas          # QBHD compiler source (generated, ~600KB)
    ├── qbhd.png                   # Icon
    ├── qbhd.ico                   # Icon
    ├── icon.rc                    # Resource file
    │
    ├── global/                    # Global includes
    ├── ide/                       # IDE code (removed in QBHD)
    ├── subs_functions/            # Built-in functions
    └── utilities/                 # Utility code
```

## Documentation Files

### User Documentation
- **README_QBHD.md** - Getting started with QBHD
- **CLI_ENHANCEMENTS.md** - Complete CLI guide with examples
- **CLI_QUICKREF.txt** - Quick reference card

### Planning & Status
- **PLAN.md** - Complete 34-task implementation plan
- **STATUS.md** - Current status and next steps
- **IMPLEMENTATION_SUMMARY.md** - Overall progress and achievements

### Task Documentation
- **PHASE1_TASK1_SUMMARY.md** - Strip IDE implementation
- **PHASE1_TASK2_SUMMARY.md** - Enhanced CLI implementation
- **TASK2_COMPLETE.md** - Task 2 completion checklist

## Scripts

### Build Scripts
- **build_qbhd.sh** - Main build script for QBHD
  - Installs dependencies
  - Builds QB64 first
  - Strips IDE code
  - Builds QBHD compiler

- **strip_ide.py** - Removes IDE code from qb64.bas
  - Removes IDE includes
  - Forces NoIDEMode
  - Updates branding

- **build_compiler_only.sh** - Alternative build approach

### Enhancement Scripts
- **apply_cli_enhancements.sh** - Adds new CLI flags
  - Adds variables for new flags
  - Adds case handlers
  - Updates help text

- **test_cli_enhancements.sh** - Tests all CLI flags
  - Tests --version, --help
  - Tests --check, --json
  - Tests --optimize, --debug, --verbose

### Original QB64 Setup
- **setup_lnx.sh** - Linux setup
- **setup_osx.command** - macOS setup
- **setup_win.cmd** - Windows setup

## Source Files

### Main Source
- **source/qb64.bas** - Original QB64 source (1,142,120 bytes)
  - Includes IDE code
  - Full QB64 functionality

- **source/qbhd_compiler.bas** - QBHD compiler source (generated)
  - IDE code removed
  - Compiler-only
  - ~500KB smaller

### Helper Code
- **add_json_support.bas** - JSON output functions
  - OutputJSONDiagnostic()
  - EscapeJSON$()
  - StartJSONOutput()
  - EndJSONOutput()

## Generated Files (After Build)

```
QBHD/
├── qbhd                           # QBHD compiler executable
├── source/qbhd_compiler.bas       # Generated compiler source
├── internal/
│   ├── c/
│   │   └── libqb/                 # Compiled C++ objects
│   └── temp/                      # Temporary build files
```

## File Sizes

| File | Size | Description |
|------|------|-------------|
| source/qb64.bas | ~1.1 MB | Original QB64 with IDE |
| source/qbhd_compiler.bas | ~600 KB | QBHD compiler-only |
| qb64 (binary) | ~2.5 MB | QB64 with IDE |
| qbhd (binary) | ~2.0 MB | QBHD compiler-only |

## Key Directories

### `/source/`
Contains all BASIC source code for the compiler.

### `/internal/`
Contains C++ code, libraries, and build artifacts.

### `/internal/help/`
Contains help documentation (markdown files).

### `/internal/c/`
Contains C++ runtime library and build files.

## Workflow

### Initial Setup
```bash
1. Clone repository
2. Run ./build_qbhd.sh
3. Test with ./qbhd --version
```

### Apply CLI Enhancements
```bash
1. Run ./apply_cli_enhancements.sh
2. Run ./build_qbhd.sh
3. Test with ./test_cli_enhancements.sh
```

### Development
```bash
1. Edit source/qbhd_compiler.bas
2. Run ./build_qbhd.sh
3. Test changes
```

## Next Phase Files (Upcoming)

### Phase 2: LSP & Neovim
```
QBHD/
├── lsp/                           # Rust LSP server
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── parser.rs
│   │   └── semantic.rs
│   └── tests/
│
└── nvim-qbhd/                     # Neovim plugin
    ├── lua/
    │   └── qbhd/
    │       ├── init.lua
    │       └── lsp.lua
    └── plugin/
        └── qbhd.vim
```

### Phase 3: Tauri IDE
```
QBHD/
└── ide/                           # Tauri IDE
    ├── src-tauri/                 # Rust backend
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs
    └── src/                       # Frontend
        ├── App.tsx
        ├── components/
        └── styles/
```

---

**Last Updated**: 2026-02-06  
**Current Files**: Phase 1 complete  
**Next**: Phase 2 & 3 file structure

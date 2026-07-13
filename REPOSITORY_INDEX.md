# Repository Index

Every important file in the QBHD repository with a one-line description.

## Root Files

| File | Purpose |
|------|---------|
| `build.sh` | Unified cross-platform build script (compiler, LSP, IDE) |
| `build_qbhd.sh` | Original build script for QBHD compiler only |
| `build_compiler_only.sh` | Alternative build using sed instead of Python |
| `apply_cli_enhancements.sh` | Patches CLI flags (--json, --check, etc.) into compiler source |
| `strip_ide.py` | Removes IDE code from QB64 source to create QBHD compiler |
| `test_suite.py` | Python test suite for CLI flags |
| `test_cli_enhancements.sh` | Bash test script for CLI enhancements |
| `Makefile` | GNU Make targets (build, clean, test, install) |
| `README_QBHD.md` | Main project documentation |
| `STATUS.md` | Project status dashboard |
| `ARCHITECTURE.md` | Architecture documentation with diagrams |
| `AI_CONTEXT.md` | Context file for AI agents |
| `REPOSITORY_INDEX.md` | This file |

## Documentation Files

| File | Purpose |
|------|---------|
| `QUICKSTART.md` | Quick start guide for new users |
| `FEATURES.md` | Comprehensive feature reference |
| `USAGE_GUIDE.md` | Detailed usage instructions |
| `CLI_ENHANCEMENTS.md` | CLI flag documentation |
| `IMPLEMENTATION_SUMMARY.md` | High-level implementation summary |
| `PHASE1_COMPLETE.md` | Phase 1 completion report |
| `PHASE2_COMPLETE.md` | Phase 2 completion report |
| `PHASE3_COMPLETE.md` | Phase 3 completion report |
| `PROJECT_COMPLETE.md` | Project completion celebration |
| `PHASE1_TASK1_SUMMARY.md` | IDE stripping task details |
| `PHASE1_TASK2_SUMMARY.md` | CLI enhancements task details |
| `PHASE1_TASK3_SUMMARY.md` | Build system task details |
| `PHASE2_TASK4_SUMMARY.md` | LSP server setup details |
| `TASK2_COMPLETE.md` | Task 2 completion announcement |
| `DOCUMENTATION_UPDATE.md` | Documentation changelog |
| `CHANGELOG.md` | QB64 upstream changelog (inherited) |

## Source: BASIC Compiler (`source/`)

| File | Lines | Purpose |
|------|-------|---------|
| `source/qb64.bas` | 26,345 | Original QB64 compiler + IDE source |
| `source/qbhd_compiler.bas` | 26,348 | Stripped compiler-only version (target for build) |

### Source: Global (`source/global/`)

| File | Lines | Purpose |
|------|-------|---------|
| `source/global/version.bas` | 15 | Version string ("2.1.1") and build channel |
| `source/global/settings.bas` | 3 | Debug constant (`CONST Debug = 0`) |
| `source/global/constants.bas` | 61 | String delimiters, ASCII codes, key constants |
| `source/global/IDEsettings.bas` | 529 | IDE configuration variables and INI persistence |

### Source: Utilities (`source/utilities/`)

| File | Lines | Purpose |
|------|-------|---------|
| `source/utilities/build.bas` | 38 | Build helpers (purge temp, find make) |
| `source/utilities/config.bas` | 83 | Config read/write wrappers for INI manager |
| `source/utilities/file.bas` | 77 | File copy, path manipulation, extension handling |
| `source/utilities/strings.bas` | 40 | String remove, replace, add quotes |
| `source/utilities/ini-manager/ini.bi` | 21 | INI manager header (global variable declarations) |
| `source/utilities/ini-manager/ini.bm` | 501 | INI manager implementation (read/write INI files) |
| `source/utilities/ini-manager/readme.txt` | 2 | Notes on INI manager origin |

## LSP Server (`lsp/`)

| File | Lines | Purpose |
|------|-------|---------|
| `lsp/Cargo.toml` | 17 | Rust crate manifest with dependencies |
| `lsp/src/main.rs` | 365 | LSP protocol handler (initialize, completion, hover, definition, references, rename) |
| `lsp/src/lexer.rs` | 475 | BASIC tokenizer (130+ keywords, comments, strings, hex/octal numbers) |
| `lsp/src/parser.rs` | 830 | Recursive descent parser with Pratt expression parsing |
| `lsp/src/semantic.rs` | 343 | Symbol table, completions, hover docs, go-to-definition |
| `lsp/src/diagnostics.rs` | 93 | Bridges `qbhd --json --check` to LSP diagnostics |
| `lsp/src/document.rs` | 42 | In-memory document store by URI |

## Neovim Plugin (`nvim-qbhd/`)

| File | Lines | Purpose |
|------|-------|---------|
| `nvim-qbhd/lua/qbhd/init.lua` | 117 | Plugin setup: LSP config, keybindings, commands |
| `nvim-qbhd/syntax/basic.vim` | 131 | Vim syntax highlighting for BASIC |
| `nvim-qbhd/ftdetect/basic.vim` | 2 | Filetype detection for .bas and .bi files |

## Tauri GUI IDE (`ide/`)

### Frontend

| File | Lines | Purpose |
|------|-------|---------|
| `ide/package.json` | 21 | NPM manifest (React, Monaco, Tauri, Vite) |
| `ide/vite.config.js` | 11 | Vite build config (React plugin, port 1420) |
| `ide/index.html` | 21 | HTML entry point with dark background |
| `ide/src/main.jsx` | 9 | React entry point (createRoot + StrictMode) |
| `ide/src/App.jsx` | 106 | Root component: state management, file ops, layout |
| `ide/src/App.css` | 325 | VS Code-inspired dark theme with CSS variables |
| `ide/src/components/Editor.jsx` | 178 | Monaco Editor with BASIC language definition |
| `ide/src/components/FileTree.jsx` | 135 | Directory tree with expand/collapse |
| `ide/src/components/Toolbar.jsx` | 128 | New/Save/Check/Build/Run buttons |
| `ide/src/components/Terminal.jsx` | 28 | Output display with auto-scroll |
| `ide/src/components/StatusBar.jsx` | 25 | File name, cursor position, modification status |

### Backend

| File | Lines | Purpose |
|------|-------|---------|
| `ide/src-tauri/Cargo.toml` | 13 | Rust crate manifest (tauri, serde) |
| `ide/src-tauri/tauri.conf.json` | 52 | Tauri config (shell scope, filesystem, window) |
| `ide/src-tauri/src/main.rs` | 127 | IPC commands: read/write files, compile, check, run |

## CI/CD (`.ci/`)

| File | Lines | Purpose |
|------|-------|---------|
| `.ci/bootstrap.sh` | 40 | Linux/macOS bootstrap compiler build |
| `.ci/bootstrap.bat` | 73 | Windows x64 bootstrap (downloads MinGW) |
| `.ci/bootstrap32.bat` | 64 | Windows x86 bootstrap |
| `.ci/compile.sh` | 12 | Linux/macOS final compile with cleanup |
| `.ci/build.sh` | 13 | Linux/macOS final compile (incomplete cleanup) |
| `.ci/compile.bat` | 24 | Windows final compile with cleanup |
| `.ci/build.bat` | 25 | Windows final compile (AppVeyor variant) |
| `.ci/deploy.sh` | 16 | Linux/macOS release archive creation |
| `.ci/compile.ps1` | 12 | Windows release archive creation |
| `.ci/update-source.sh` | 9 | Git auto-commit helper |
| `.ci/common-exclusion.list` | 14 | Files excluded from release archives |
| `.ci/lnx-exclusion.list` | 3 | Linux-specific exclusions |
| `.ci/osx-exclusion.list` | 2 | macOS-specific exclusions |
| `.ci/win-exclusion.list` | 4 | Windows-specific exclusions |

## GitHub (`.github/`)

| File | Lines | Purpose |
|------|-------|---------|
| `.github/workflows/build.yml` | 90 | Multi-platform CI build (Linux, macOS, Windows) |
| `.github/workflows/push.yml` | 18 | CI on push to master |
| `.github/workflows/pull_request.yml` | 18 | CI on PRs to master |
| `.github/workflows/release.yml` | 87 | Manual release workflow |
| `.github/ISSUE_TEMPLATE/bug_report.md` | 32 | Bug report template |
| `.github/ISSUE_TEMPLATE/feature_request.md` | 20 | Feature request template |

## Licenses (`licenses/`)

| File | Purpose |
|------|---------|
| `licenses/COPYING.TXT` | Master license document listing all components |
| `licenses/license_freeglut.txt` | FreeGLUT (MIT-like) |
| `licenses/license_freetype_ftl.txt` | FreeType (FTL, BSD-like) |
| `licenses/license_gnu_gpl_2.txt` | GPL v2 |
| `licenses/license_gnu_gpl_3.txt` | GPL v3 |
| `licenses/license_gnu_lgpl_2.txt` | LGPL v2 |
| `licenses/license_gnu_lgpl_2_1.txt` | LGPL v2.1 |
| `licenses/license_libxmp-lite.txt` | Libxmp-lite (MIT) |
| `licenses/license_miniaudio.txt` | miniaudio (Public Domain / MIT) |
| `licenses/license_stb_image.txt` | STB Image (MIT / Public Domain) |
| `licenses/license_stb_vorbis.txt` | STB Vorbis (MIT / Public Domain) |
| `licenses/license_tinysoundfont.txt` | TinySoundFont (MIT) |
| `licenses/license_zlib.txt` | zlib license |

## Runtime Libraries (`internal/`)

The `internal/` directory contains the QB64 runtime libraries in C/C++. These are inherited from upstream QB64 and are not modified by QBHD.

| Directory | Purpose |
|-----------|---------|
| `internal/c/libqb/` | Core QB64 runtime library |
| `internal/c/parts/audio/` | Audio subsystem (miniaudio, vorbis, xmp) |
| `internal/c/parts/core/` | Core components (GLEW, zlib) |
| `internal/c/parts/input/` | Input handling (gamepad, keyboard) |
| `internal/c/parts/network/` | Networking |
| `internal/c/parts/video/` | Video subsystem (FreeGLUT, FreeType, stb_image) |
| `internal/help/` | 90+ markdown help files for BASIC keywords |
| `internal/temp/` | Build artifact directory (empty) |

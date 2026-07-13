# Development Guide

## Prerequisites

### Required

| Tool | Version | Purpose |
|------|---------|---------|
| g++ (Linux) | 7+ | C++ compiler for QB64 runtime |
| Python 3 | 3.6+ | strip_ide.py, test_suite.py |
| Bash | 4+ | Build scripts |

### Optional

| Tool | Version | Purpose |
|------|---------|---------|
| Rust/Cargo | 1.65+ | LSP server |
| Node.js/npm | 18+ | IDE frontend |
| Git | 2.30+ | Version control |

### Platform-Specific Dependencies

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get install g++ mesa-common-dev libglu1-mesa-dev libasound2-dev libx11-dev
```

#### Linux (Fedora)
```bash
sudo dnf install gcc-c++ mesa-libGL-devel mesa-libGLU-devel alsa-lib-devel libX11-devel
```

#### Linux (Arch)
```bash
sudo pacman -S gcc mesa glu alsa-lib libx11
```

#### macOS
```bash
xcode-select --install
```

#### Windows
- MinGW/GCC (g++) in PATH, or Visual Studio Build Tools
- Python 3 in PATH
- Git Bash or MSYS2

## Local Development

### Building the Compiler

```bash
# Full build (compiler + LSP + IDE deps)
./build.sh

# Just the compiler
./build.sh
```

The build process:
1. Installs platform dependencies
2. Builds QB64 compiler (if not present)
3. Runs `strip_ide.py` to create `qbhd_compiler.bas`
4. Compiles `qbhd_compiler.bas` into `qbhd` binary
5. Applies CLI enhancements
6. Rebuilds with enhancements

### Developing the LSP Server

```bash
cd lsp
cargo build            # Debug build
cargo build --release  # Release build
cargo test             # Run tests (none currently)
cargo clippy           # Lint
cargo fmt              # Format
```

The LSP server binary is at `lsp/target/release/qbhd-lsp`.

To test with Neovim:
1. Build the LSP server
2. Install the `nvim-qbhd` plugin
3. Open a `.bas` file
4. Check `:LspInfo` for connection status

### Developing the IDE

```bash
cd ide
npm install           # Install dependencies
npm run dev           # Start Vite dev server only
npm run tauri dev     # Start Tauri dev (Vite + Rust)
npm run build         # Build frontend
npm run tauri build   # Build full application
```

The dev server runs on `http://localhost:1420`.

Changes to React files hot-reload automatically. Changes to Rust files trigger a Tauri restart.

### Developing the Neovim Plugin

Edit files in `nvim-qbhd/`:
- `lua/qbhd/init.lua` - Plugin logic
- `syntax/basic.vim` - Syntax highlighting
- `ftdetect/basic.vim` - Filetype detection

To test:
1. Add the plugin to your Neovim config
2. Restart Neovim
3. Open a `.bas` file
4. Check `:QBHDInfo` for LSP status

## Build System

### Unified Build Script (`build.sh`)

```bash
./build.sh              # Full build
./build.sh --deps-only  # Install dependencies only
./build.sh --lsp        # Build LSP server only
./build.sh --ide        # Install IDE dependencies only
./build.sh --test       # Run tests
```

### Makefile

```bash
make          # Full build
make clean    # Remove build artifacts
make test     # Build and run tests
make install  # Install to /usr/local/bin
make help     # Show available targets
```

### Individual Build Scripts

```bash
# Original build scripts (may be outdated)
./build_qbhd.sh
./build_compiler_only.sh
./apply_cli_enhancements.sh
```

## Testing

### CLI Tests

```bash
# Python test suite
python3 test_suite.py

# Bash test script
./test_cli_enhancements.sh
```

Tests verify:
- `--version` flag
- `--help` flag
- `--check` flag
- `--json` flag
- Basic compilation
- `--optimize` flag
- `--output` flag

### Manual Testing

```bash
# Test compiler
./qbhd --version
./qbhd --help
./qbhd --check myfile.bas
./qbhd --json --check myfile.bas
./qbhd myfile.bas

# Test LSP
cd lsp && cargo test

# Test IDE
cd ide && npm run tauri dev
# Open a .bas file, edit, save, check, build, run
```

## Linting

### Rust
```bash
cd lsp
cargo clippy -- -D warnings
cargo fmt -- --check
```

### JavaScript
No linting tools configured. Consider adding ESLint:
```bash
cd ide
npm install --save-dev eslint @eslint/js
npx eslint --init
```

### BASIC
No linting tools available for BASIC.

## Formatting

### Rust
```bash
cd lsp
cargo fmt
```

### JavaScript
No formatter configured. Consider adding Prettier:
```bash
cd ide
npm install --save-dev prettier
npx prettier --write "src/**/*.{jsx,js,css}"
```

## Deployment

### Building Release Binaries

```bash
# Compiler
./build.sh
# Binary: ./qbhd (Linux/macOS) or ./qbhd.exe (Windows)

# LSP server
cd lsp && cargo build --release
# Binary: lsp/target/release/qbhd-lsp

# IDE
cd ide && npm run tauri build
# Binary: ide/src-tauri/target/release/qbhd-ide
```

### CI/CD

GitHub Actions workflows:
- `build.yml` - Multi-platform CI (Linux, macOS, Windows)
- `push.yml` - Build on push to master
- `pull_request.yml` - Build on PRs to master
- `release.yml` - Manual release workflow

The CI pipeline:
1. Checks out code
2. Installs platform dependencies
3. Bootstraps the compiler
4. Builds QBHD
5. Applies CLI enhancements
6. Runs tests
7. Uploads artifacts

## Debugging

### Compiler Debugging

Set `Debug = 1` in `source/global/settings.bas` to change delimiter characters to visible ones for debugging output.

### LSP Server Debugging

The LSP server uses `tracing` for logging. To enable:
```bash
RUST_LOG=debug qbhd-lsp
```

**Warning**: The tracing subscriber writes to stdout by default, which can corrupt the LSP JSON-RPC protocol. Use stderr or a file for logging in production.

### IDE Debugging

Open the Tauri dev tools (right-click → Inspect) to access browser developer tools for debugging React components and network requests.

## Common Issues

### "QB64 not found"
Run `./build.sh` to build the compiler.

### "Permission denied"
```bash
chmod +x *.sh *.py
```

### Build fails on Linux
Install dependencies:
```bash
sudo apt-get install g++ mesa-common-dev libglu1-mesa-dev libasound2-dev libx11-dev
```

### IDE won't start
```bash
cd ide
rm -rf node_modules
npm install
npm run tauri dev
```

### LSP server not connecting
1. Check `qbhd-lsp` is in PATH
2. Check `:LspInfo` in Neovim
3. Check `:QBHDInfo` for connection status

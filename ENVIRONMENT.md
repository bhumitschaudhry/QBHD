# Environment Configuration

## Environment Variables

### QBHD_PATH

**Type:** Optional
**Default:** Auto-detected
**Purpose:** Path to the `qbhd` compiler binary.

Used by the LSP server's diagnostics module (`lsp/src/diagnostics.rs`) to find the compiler.

```bash
export QBHD_PATH=/usr/local/bin/qbhd
```

### QBHD_LSP_LOG

**Type:** Optional
**Default:** None
**Purpose:** Enable LSP server logging.

```bash
export QBHD_LSP_LOG=debug
```

**Note:** The LSP server currently uses `tracing_subscriber::fmt().init()` which reads `RUST_LOG`:

```bash
export RUST_LOG=debug
```

**Warning:** Logging writes to stdout by default, which corrupts the LSP JSON-RPC protocol. Redirect to stderr or a file.

### RUST_LOG

**Type:** Optional
**Default:** None
**Purpose:** Rust logging level for the LSP server.

Values: `trace`, `debug`, `info`, `warn`, `error`

```bash
export RUST_LOG=debug
```

## Build Configuration

### Compiler Source

| Variable | Default | Purpose |
|----------|---------|---------|
| `QB64_SOURCE` | `source/qb64.bas` | Original QB64 source file |
| `QBHD_SOURCE` | `source/qbhd_compiler.bas` | Generated QBHD compiler source |

### Build Targets

| Target | Command | Output |
|--------|---------|--------|
| Compiler | `./build.sh` | `./qbhd` (or `./qbhd.exe`) |
| LSP Server | `./build.sh --lsp` | `lsp/target/release/qbhd-lsp` |
| IDE | `./build.sh --ide` | `ide/node_modules/` (dev mode) |

## IDE Configuration

### Tauri Configuration

**File:** `ide/src-tauri/tauri.conf.json`

| Setting | Value | Purpose |
|---------|-------|---------|
| `devPath` | `http://localhost:1420` | Vite dev server URL |
| `distDir` | `../dist` | Built frontend directory |
| `window.width` | 1200 | Initial window width |
| `window.height` | 800 | Initial window height |
| `window.minWidth` | 800 | Minimum window width |
| `window.minHeight` | 600 | Minimum window height |

### Vite Configuration

**File:** `ide/vite.config.js`

| Setting | Value | Purpose |
|---------|-------|---------|
| `server.port` | 1420 | Dev server port |
| `server.strictPort` | true | Fail if port in use |
| `clearScreen` | false | Keep terminal output |

### IDE Runtime Settings

The IDE has no runtime configuration system. All settings are hardcoded:

| Setting | Value | Location |
|---------|-------|----------|
| Theme | VS Code dark | `ide/src/App.css` |
| Font | Consolas, Courier New | `ide/src/components/Editor.jsx` |
| Tab size | 4 | `ide/src/components/Editor.jsx` |
| Insert spaces | false (use tabs) | `ide/src/components/Editor.jsx` |
| Minimap | enabled | `ide/src/components/Editor.jsx` |
| Terminal height | 200px | `ide/src/App.css` |
| Sidebar width | 260px | `ide/src/App.css` |

## LSP Server Configuration

### Capabilities

The LSP server advertises these capabilities:

| Capability | Value |
|------------|-------|
| Text sync | FULL (entire document on change) |
| Completion | Yes (with trigger characters) |
| Hover | Yes |
| Definition | Yes |
| References | Yes |
| Rename | Yes |
| Code action | Yes (stub) |
| Diagnostics | Yes |

### Trigger Characters

Completion is triggered by:
- Space (` `)
- Dot (`.`)
- Dollar (`$`)
- Open paren (`(`)

## Neovim Plugin Configuration

### Setup

```lua
require("qbhd").setup({
    cmd = { "qbhd-lsp" },  -- Path to LSP binary
})
```

### Keybindings

| Key | Mode | Action |
|-----|------|--------|
| `gd` | Normal | Go to definition |
| `K` | Normal | Hover documentation |
| `gr` | Normal | Find references |
| `<leader>ca` | Normal | Code action |
| `<leader>rn` | Normal | Rename symbol |
| `<C-Space>` | Insert | Trigger completion |

### Commands

| Command | Description |
|---------|-------------|
| `:QBHDCompile` | Compile current file |
| `:QBHDRun` | Compile and run current file |
| `:QBHDCheck` | Check for errors |
| `:QBHDFormat` | Basic formatting (trim whitespace) |
| `:QBHDInfo` | Show LSP connection status |

## Compiler Configuration

### INI File

The QB64/QBHD compiler uses `internal/config.ini` for configuration, managed by the INI Manager (`source/utilities/ini-manager/`).

### CLI Flags

| Flag | Description |
|------|-------------|
| `--version` | Show version information |
| `--help` | Show help message |
| `--json` | Output diagnostics as JSON |
| `--check` | Check for errors without compiling |
| `--output <file>` | Set output filename |
| `--optimize <level>` | Optimization level (0-3) |
| `--debug` | Include debug info |
| `--verbose` | Verbose output |

## CI/CD Configuration

### GitHub Actions

**File:** `.github/workflows/build.yml`

| Job | OS | Trigger |
|-----|----|---------|
| build-linux | ubuntu-latest | push to main/develop, PR to main |
| build-macos | macos-latest | push to main/develop, PR to main |
| build-windows | windows-latest | push to main/develop, PR to main |

**Note:** Branch targets are inconsistent across workflows. `build.yml` targets `main`/`develop`, while `push.yml` and `pull_request.yml` target `master`.

### Release Configuration

**File:** `.github/workflows/release.yml`

Manual workflow (`workflow_dispatch`) that builds for:
- Linux (x64)
- macOS (x64)
- Windows (x86)
- Windows (x64)

## Secrets

No secrets are required for building or running QBHD.

The CI/CD pipeline does not use any secrets. The release workflow has commented-out S3 upload logic that would require AWS credentials.

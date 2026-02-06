# QBHD Enhanced CLI

Modern command-line interface for the QBHD compiler with LSP integration support.

## Installation

```bash
# Apply CLI enhancements
./apply_cli_enhancements.sh

# Rebuild compiler
./build_qbhd.sh

# Test
./qbhd --version
```

## New Features

### 🚀 Quick Syntax Checking
```bash
# Fast syntax validation without compilation
./qbhd --check myprogram.bas
```

### 📊 JSON Output for LSP
```bash
# Get structured diagnostics for editor integration
./qbhd --json --check myprogram.bas
```

### 🎯 Optimization Levels
```bash
# Development build (fast compile)
./qbhd --optimize 0 myprogram.bas

# Release build (optimized)
./qbhd --optimize 3 myprogram.bas
```

### 🐛 Debug Support
```bash
# Include debug symbols for GDB/LLDB
./qbhd --debug myprogram.bas
gdb ./myprogram
```

### 📝 Verbose Output
```bash
# See detailed compilation steps
./qbhd --verbose myprogram.bas
```

## Complete Flag Reference

| Flag | Description | Example |
|------|-------------|---------|
| `--version` | Show version and exit | `./qbhd --version` |
| `--help` | Show help message | `./qbhd --help` |
| `--check` | Syntax check only | `./qbhd --check file.bas` |
| `--json` | JSON output format | `./qbhd --json --check file.bas` |
| `--output <file>` | Specify output file | `./qbhd --output myapp file.bas` |
| `--optimize <0-3>` | Set optimization level | `./qbhd --optimize 2 file.bas` |
| `--debug` | Include debug symbols | `./qbhd --debug file.bas` |
| `--verbose` | Verbose output | `./qbhd --verbose file.bas` |
| `-c` | Compile mode | `./qbhd -c file.bas` |
| `-x` | Console compile | `./qbhd -x file.bas` |
| `-w` | Show warnings | `./qbhd -w file.bas` |
| `-q` | Quiet mode | `./qbhd -q file.bas` |
| `-m` | Monochrome output | `./qbhd -m file.bas` |

## Usage Examples

### Development Workflow
```bash
# Edit-compile-test cycle
while true; do
    # Quick syntax check
    ./qbhd --check game.bas && \
    # Compile with debug info
    ./qbhd --debug --output game_debug game.bas && \
    # Run
    ./game_debug
done
```

### CI/CD Integration
```bash
# Validate all source files
for file in src/*.bas; do
    echo "Checking $file..."
    ./qbhd --json --check "$file" || exit 1
done

# Build release
./qbhd --optimize 3 --output bin/release src/main.bas
```

### Editor Integration (Neovim/LSP)
```lua
-- In your LSP config
local function qbhd_diagnostics(file)
    local cmd = string.format("qbhd --json --check %s", file)
    local output = vim.fn.system(cmd)
    return vim.fn.json_decode(output)
end
```

## JSON Output Format

When using `--json`, diagnostics are output as:

```json
[
  {
    "file": "program.bas",
    "line": 10,
    "column": 5,
    "severity": "error",
    "message": "Expected END IF"
  }
]
```

**Severity levels:**
- `error` - Compilation error (prevents build)
- `warning` - Warning message (build continues)
- `info` - Informational message

## Exit Codes

- `0` - Success
- `1` - Compilation error
- `2` - System error (missing files, etc.)

## Performance Tips

### Fast Iteration
```bash
# Use --check for instant feedback
./qbhd --check myprogram.bas  # ~100ms

# Full compile when ready
./qbhd myprogram.bas  # ~3-5s
```

### Optimization Levels
```bash
# Development (fastest compile)
./qbhd --optimize 0 myprogram.bas  # ~3s

# Release (best performance)
./qbhd --optimize 3 myprogram.bas  # ~10s
```

## Troubleshooting

### "Unknown flag" error
Make sure you've applied the enhancements:
```bash
./apply_cli_enhancements.sh
./build_qbhd.sh
```

### JSON output not working
Check that `--json` is combined with `--check`:
```bash
./qbhd --json --check file.bas  # ✓ Correct
./qbhd --json file.bas          # ✗ Wrong
```

### Debug symbols not included
Ensure you're using `--debug`:
```bash
./qbhd --debug myprogram.bas
file ./myprogram  # Should show "not stripped"
```

## What's Next

These CLI enhancements enable:
- **Phase 2**: LSP server implementation (uses `--json --check`)
- **Phase 3**: Tauri IDE integration
- **Better tooling**: Editor plugins, CI/CD, automated testing

## Contributing

See `PHASE1_TASK2_SUMMARY.md` for implementation details.

## License

Same as QBHD/QB64 (MIT)

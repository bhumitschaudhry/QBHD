# QBHD Quick Start Guide

## What is QBHD?

QBHD is a modernized version of QB64 with:
- **No built-in IDE** - Use your favorite editor (Neovim, VS Code, etc.)
- **Modern CLI** - JSON output, fast syntax checking, optimization levels
- **LSP support** (coming in Phase 2) - Real-time diagnostics and autocomplete
- **Modern GUI IDE** (coming in Phase 3) - Tauri-based visual IDE

## Installation

### Step 1: Build QBHD
```bash
cd /home/bhumit/QBHD
./build_qbhd.sh
```

This will:
- Install dependencies (if needed)
- Build QB64 (~3-5 minutes)
- Strip IDE code
- Build QBHD compiler (~1 minute)

### Step 2: Apply CLI Enhancements
```bash
./apply_cli_enhancements.sh
./build_qbhd.sh
```

This adds modern CLI flags like `--json`, `--check`, `--optimize`, etc.

### Step 3: Test
```bash
./qbhd --version
./qbhd --help
./test_cli_enhancements.sh
```

## Basic Usage

### Compile a Program
```bash
./qbhd myprogram.bas
./myprogram
```

### Quick Syntax Check
```bash
./qbhd --check myprogram.bas
```
Fast validation without compilation (~100ms vs ~3s)

### Compile with Optimization
```bash
./qbhd --optimize 2 myprogram.bas
```
Levels: 0 (none), 1 (basic), 2 (moderate), 3 (max)

### Debug Build
```bash
./qbhd --debug myprogram.bas
gdb ./myprogram
```

### Specify Output Name
```bash
./qbhd --output mygame game.bas
```

## Editor Integration

### For LSP/Neovim (Phase 2 - Coming Soon)
```bash
# Get JSON diagnostics
./qbhd --json --check myprogram.bas
```

Output:
```json
[
  {
    "file": "myprogram.bas",
    "line": 10,
    "column": 5,
    "severity": "error",
    "message": "Expected END IF"
  }
]
```

### For Manual Workflow
```bash
# Edit in your favorite editor
vim myprogram.bas

# Quick check
./qbhd --check myprogram.bas

# Compile when ready
./qbhd myprogram.bas

# Run
./myprogram
```

## Common Workflows

### Development Cycle
```bash
# Fast iteration
while true; do
    vim myprogram.bas
    ./qbhd --check myprogram.bas && \
    ./qbhd --debug myprogram.bas && \
    ./myprogram
done
```

### Release Build
```bash
# Maximum optimization
./qbhd --optimize 3 --output release/myapp myprogram.bas
```

### CI/CD
```bash
# Validate all files
for file in src/*.bas; do
    ./qbhd --json --check "$file" || exit 1
done

# Build
./qbhd --optimize 3 --output bin/myapp src/main.bas
```

## All CLI Flags

| Flag | Description |
|------|-------------|
| `--version` | Show version |
| `--help` | Show help |
| `--check` | Syntax check only |
| `--json` | JSON output |
| `--output <file>` | Output file |
| `--optimize <0-3>` | Optimization |
| `--debug` | Debug symbols |
| `--verbose` | Verbose output |
| `-c` | Compile mode |
| `-x` | Console compile |
| `-w` | Show warnings |
| `-q` | Quiet mode |

## Examples

### Hello World
```basic
' hello.bas
PRINT "Hello, QBHD!"
```

```bash
./qbhd hello.bas
./hello
```

### With Graphics
```basic
' graphics.bas
SCREEN _NEWIMAGE(800, 600, 32)
CIRCLE (400, 300), 100, _RGB(255, 0, 0)
SLEEP
```

```bash
./qbhd graphics.bas
./graphics
```

### Game Loop
```basic
' game.bas
SCREEN _NEWIMAGE(800, 600, 32)
DO
    CLS
    PRINT "FPS:"; INT(1 / _FRAMETIME)
    _DISPLAY
    _LIMIT 60
LOOP UNTIL _KEYHIT = 27 ' ESC to quit
```

```bash
./qbhd --optimize 2 game.bas
./game
```

## Troubleshooting

### "qbhd: command not found"
```bash
./qbhd --version  # Use ./ prefix
# Or add to PATH:
export PATH=$PATH:/home/bhumit/QBHD
```

### "internal folder not found"
Make sure you're in the QBHD directory:
```bash
cd /home/bhumit/QBHD
./qbhd --version
```

### Build fails
Install dependencies:
```bash
sudo apt-get install g++ mesa-common-dev libglu1-mesa-dev libasound2-dev
```

### New flags don't work
Apply enhancements:
```bash
./apply_cli_enhancements.sh
./build_qbhd.sh
```

## What's Next?

### Phase 2: LSP & Neovim (Coming Soon)
- Real-time diagnostics
- Autocomplete
- Go-to-definition
- Enhanced UI

### Phase 3: Tauri IDE (Coming Later)
- Visual IDE
- Debugger
- Project management
- Modern UI

## Documentation

- **CLI_ENHANCEMENTS.md** - Complete CLI guide
- **CLI_QUICKREF.txt** - Quick reference
- **IMPLEMENTATION_SUMMARY.md** - Project status
- **FILE_STRUCTURE.md** - Project organization

## Get Help

- Check `./qbhd --help`
- Read `CLI_ENHANCEMENTS.md`
- See examples in `test_hello.bas`

## Contributing

See `PLAN.md` for the complete implementation plan.

---

**Version**: QBHD V2.1  
**Based on**: QB64  
**License**: MIT

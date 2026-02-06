# Phase 1, Task 2: Enhanced Command-Line Interface

## Summary

Enhanced QBHD compiler with modern CLI flags for better integration with LSP servers and development tools.

## New Flags Added

### `--version`
Shows version information and exits.
```bash
./qbhd --version
# Output: QBHD Compiler V2.1
#         Based on QB64
```

### `--json`
Outputs diagnostics in JSON format for LSP integration. Automatically enables quiet mode.
```bash
./qbhd --json --check program.bas
# Output: [{"file":"program.bas","line":10,"column":5,"severity":"error","message":"Syntax error"}]
```

### `--check`
Performs syntax checking only without compilation. Useful for fast feedback in editors.
```bash
./qbhd --check program.bas
# Validates syntax, reports errors, but doesn't compile
```

### `--output <file>` (alias for `-o`)
Specifies the output executable name.
```bash
./qbhd --output myprogram program.bas
./qbhd --output bin/game game.bas
```

### `--optimize <level>`
Sets optimization level (0-3):
- `0`: No optimization (fastest compile)
- `1`: Basic optimization
- `2`: Moderate optimization
- `3`: Maximum optimization (slowest compile)

```bash
./qbhd --optimize 2 program.bas
```

### `--debug`
Includes debug symbols for GDB/LLDB debugging.
```bash
./qbhd --debug program.bas
gdb ./program
```

### `--verbose`
Shows detailed compilation output.
```bash
./qbhd --verbose program.bas
# Shows: parsing, code generation, C++ compilation steps
```

## Implementation Details

### Variables Added
```basic
DIM SHARED JSONMode AS _BYTE        ' Enable JSON output
DIM SHARED CheckOnlyMode AS _BYTE   ' Syntax check only
DIM SHARED VerboseMode AS _BYTE     ' Verbose output
DIM SHARED OptimizeLevel AS INTEGER ' 0-3 optimization level
DIM SHARED DebugMode AS _BYTE       ' Include debug symbols
```

### Exit Codes
- `0`: Success
- `1`: Compilation error
- `2`: System error (missing files, etc.)

### JSON Output Format
```json
[
  {
    "file": "program.bas",
    "line": 10,
    "column": 5,
    "severity": "error",
    "message": "Expected END IF"
  },
  {
    "file": "program.bas",
    "line": 15,
    "column": 1,
    "severity": "warning",
    "message": "Variable 'x' declared but never used"
  }
]
```

Severity levels:
- `"error"`: Compilation error
- `"warning"`: Warning message
- `"info"`: Informational message

## Usage Examples

### LSP Integration
```bash
# Check syntax and get JSON diagnostics
./qbhd --json --check source.bas

# Compile with debug symbols for debugging
./qbhd --debug --output debug_build source.bas
```

### Development Workflow
```bash
# Quick syntax check while editing
./qbhd --check myprogram.bas

# Verbose build to see what's happening
./qbhd --verbose --optimize 2 myprogram.bas

# Release build with maximum optimization
./qbhd --optimize 3 --output release/myapp myprogram.bas
```

### CI/CD Pipeline
```bash
# Check all .bas files
for file in src/*.bas; do
    ./qbhd --json --check "$file" || exit 1
done

# Build release version
./qbhd --optimize 3 --output bin/myapp src/main.bas
```

## Testing

### Test 1: Version Flag
```bash
./qbhd --version
# Expected: Version info displayed, exit code 0
```

### Test 2: Help Flag
```bash
./qbhd --help
# Expected: Help text with all new flags listed
```

### Test 3: Check Mode
```bash
./qbhd --check test_hello.bas
# Expected: Syntax validation, no executable created
```

### Test 4: JSON Output
```bash
./qbhd --json --check test_hello.bas
# Expected: JSON array (empty if no errors)
```

### Test 5: Optimization Levels
```bash
./qbhd --optimize 0 test_hello.bas  # Fast compile
./qbhd --optimize 3 test_hello.bas  # Optimized build
```

### Test 6: Debug Mode
```bash
./qbhd --debug test_hello.bas
file ./test_hello  # Should show "not stripped"
```

### Test 7: Verbose Mode
```bash
./qbhd --verbose test_hello.bas
# Expected: Detailed compilation steps shown
```

## Files Modified

- `source/qbhd_compiler.bas` - Added CLI parsing for new flags
- `apply_cli_enhancements.sh` - Script to apply enhancements
- `add_json_support.bas` - JSON output helper functions

## Next Steps (Task 3)

After testing these enhancements:
1. Update build scripts (`setup_lnx.sh`, `setup_osx.command`, `setup_win.cmd`)
2. Add CI/CD configuration (GitHub Actions)
3. Create automated tests for CLI flags
4. Update documentation

## Benefits for LSP (Phase 2)

These CLI enhancements enable:
- **Fast syntax checking** (`--check`) for real-time diagnostics
- **Structured output** (`--json`) for easy parsing
- **Proper exit codes** for error detection
- **Debug support** for future debugger integration

The LSP server (Phase 2) will use:
```bash
qbhd --json --check <file>
```
to get diagnostics without full compilation.

---

**Status**: Ready to apply and test
**Dependencies**: Phase 1, Task 1 (Complete)
**Enables**: Phase 2 (LSP implementation)

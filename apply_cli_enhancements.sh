#!/bin/bash
# Apply CLI enhancements to QBHD compiler
# Phase 1, Task 2: Enhance Command-Line Interface

set -e

echo "=== QBHD CLI Enhancement ==="
echo "Phase 1, Task 2: Enhance Command-Line Interface"
echo

# Check if qbhd_compiler.bas exists
if [ ! -f "source/qbhd_compiler.bas" ]; then
    echo "Error: source/qbhd_compiler.bas not found"
    echo "Run ./build_qbhd.sh first to generate the compiler source"
    exit 1
fi

# Backup original
echo "→ Creating backup..."
cp source/qbhd_compiler.bas source/qbhd_compiler.bas.backup

# Validation helper: check that a pattern exists before sed-ing it
require_pattern() {
    local pattern="$1"
    local description="$2"
    if ! grep -q "$pattern" source/qbhd_compiler.bas; then
        echo "ERROR: Expected pattern not found: $description"
        echo "  Pattern: $pattern"
        echo "  The compiler source may have changed. Aborting to prevent corruption."
        # Restore backup
        cp source/qbhd_compiler.bas.backup source/qbhd_compiler.bas
        exit 1
    fi
}

# Apply enhancements using sed
echo "→ Adding new CLI variables..."
require_pattern "DIM SHARED MonochromeLoggingMode AS _BYTE" "MonochromeLoggingMode declaration"
sed -i '/DIM SHARED MonochromeLoggingMode AS _BYTE/a\
DIM SHARED JSONMode AS _BYTE, CheckOnlyMode AS _BYTE, VerboseMode AS _BYTE\
DIM SHARED OptimizeLevel AS INTEGER, DebugMode AS _BYTE' source/qbhd_compiler.bas

echo "→ Adding --version handler..."
require_pattern 'CASE "-?"' "Help case (-?) handler"
sed -i '/CASE "-?" .*Command-line help/,/SYSTEM/{
    /SYSTEM/a\
            CASE "--version"\
                _DEST _CONSOLE\
                PRINT "QBHD Compiler V" + Version$\
                PRINT "Based on QB64"\
                SYSTEM
}' source/qbhd_compiler.bas

echo "→ Adding --json handler..."
require_pattern 'CASE "-e"' "Option Explicit case (-e)"
sed -i '/CASE "-e" .*Option Explicit/i\
            CASE "--json"\
                JSONMode = -1\
                QuietMode = -1\
                cmdlineswitch = -1' source/qbhd_compiler.bas

echo "→ Adding --check handler..."
require_pattern 'CASE "--json"' "JSON mode case (--json)"
sed -i '/CASE "--json"/a\
            CASE "--check"\
                CheckOnlyMode = -1\
                NoIDEMode = 1\
                cmdlineswitch = -1' source/qbhd_compiler.bas

echo "→ Adding --verbose handler..."
require_pattern 'CASE "--check"' "Check mode case (--check)"
sed -i '/CASE "--check"/a\
            CASE "--verbose"\
                VerboseMode = -1\
                cmdlineswitch = -1' source/qbhd_compiler.bas

echo "→ Adding --debug handler..."
require_pattern 'CASE "--verbose"' "Verbose mode case (--verbose)"
sed -i '/CASE "--verbose"/a\
            CASE "--debug"\
                DebugMode = -1\
                idedebuginfo = 1\
                Include_GDB_Debugging_Info = 1\
                cmdlineswitch = -1' source/qbhd_compiler.bas

echo "→ Adding --output handler..."
require_pattern 'CASE "-o"' "Output file case (-o)"
sed -i '/CASE "-o" .*Specify an output file/a\
            CASE "--output"\
                IF LEN(COMMAND$(i + 1)) > 0 THEN outputfile_cmd$ = COMMAND$(i + 1): i = i + 1\
                cmdlineswitch = -1' source/qbhd_compiler.bas

echo "→ Adding --optimize handler..."
require_pattern 'CASE "--output"' "Output case (--output)"
sed -i '/CASE "--output"/a\
            CASE "--optimize"\
                IF LEN(COMMAND$(i + 1)) > 0 THEN\
                    OptimizeLevel = VAL(COMMAND$(i + 1))\
                    IF OptimizeLevel < 0 THEN OptimizeLevel = 0\
                    IF OptimizeLevel > 3 THEN OptimizeLevel = 3\
                    i = i + 1\
                END IF\
                cmdlineswitch = -1' source/qbhd_compiler.bas

echo "→ Updating help text..."
require_pattern 'PRINT "  -c                      Compile instead of edit"' "Help text for -c flag"
sed -i '/PRINT "  -c                      Compile instead of edit"/a\
                PRINT "  --check                 Syntax check only (no compilation)"\
                PRINT "  --json                  Output diagnostics in JSON format (for LSP)"\
                PRINT "  --output <file>         Write output executable to <file>"\
                PRINT "  --optimize <0-3>        Optimization level (0=none, 3=max)"\
                PRINT "  --debug                 Include debug symbols (GDB/LLDB)"\
                PRINT "  --verbose               Verbose compilation output"\
                PRINT "  --version               Show version and exit"' source/qbhd_compiler.bas

# Verify enhancements were applied
echo
echo "→ Verifying enhancements..."
ENHANCEMENTS=("--version" "--json" "--check" "--output" "--optimize" "--debug" "--verbose")
MISSING=0
for flag in "${ENHANCEMENTS[@]}"; do
    if ! grep -q "CASE \"$flag\"" source/qbhd_compiler.bas; then
        echo "  WARNING: $flag handler may not have been inserted correctly"
        MISSING=$((MISSING + 1))
    fi
done

if [ $MISSING -eq 0 ]; then
    echo "  All enhancements verified successfully"
else
    echo "  WARNING: $MISSING enhancement(s) may be missing"
fi

echo
echo "✓ CLI enhancements applied successfully!"
echo
echo "New flags added:"
echo "  --version      Show version and exit"
echo "  --json         JSON output for LSP"
echo "  --check        Syntax check only"
echo "  --output       Specify output file"
echo "  --optimize     Set optimization level"
echo "  --debug        Include debug symbols"
echo "  --verbose      Verbose output"
echo
echo "Next steps:"
echo "  1. Rebuild QBHD: ./build_qbhd.sh"
echo "  2. Test new flags: ./qbhd --version"
echo "  3. Test JSON mode: ./qbhd --json --check test_hello.bas"

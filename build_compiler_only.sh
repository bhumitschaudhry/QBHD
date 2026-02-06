#!/bin/bash
# QBHD Compiler-Only Build Script
# Phase 1, Task 1: Strip IDE and build compiler-only version

set -e

echo "=== QBHD Compiler-Only Build ==="
echo "Removing IDE components and building compiler..."

# Create backup
if [ ! -f "source/qb64.bas.original" ]; then
    echo "Creating backup of original qb64.bas..."
    cp source/qb64.bas source/qb64.bas.original
fi

# Create compiler-only version
echo "Creating compiler-only version..."
cp source/qb64.bas.original source/qbhd_compiler.bas

# Remove IDE includes
sed -i "s/'\$INCLUDE:'ide\\\\ide_global.bas'/REM IDE REMOVED/g" source/qbhd_compiler.bas
sed -i "s/'\$INCLUDE:'ide\\\\ide_methods.bas'/REM IDE REMOVED/g" source/qbhd_compiler.bas

# Remove $SCREENHIDE (not needed for console-only)
sed -i 's/\$SCREENHIDE/REM \$SCREENHIDE/g' source/qbhd_compiler.bas

# Force NoIDEMode and ConsoleMode
# We'll add this right after the DIM SHARED declarations
sed -i '/^DIM SHARED ConsoleMode, No_C_Compile_Mode, NoIDEMode/a NoIDEMode = 1: ConsoleMode = 1' source/qbhd_compiler.bas

# Update version info
sed -i 's/QB64 IDE and Compiler/QBHD Compiler/g' source/qbhd_compiler.bas
sed -i 's/QB64/QBHD/g' source/qbhd_compiler.bas | head -30

echo "Modified source created: source/qbhd_compiler.bas"
echo ""
echo "Building QBHD compiler..."

# Build using existing QB64
if [ -f "./qb64" ]; then
    ./qb64 -x source/qbhd_compiler.bas -o qbhd
    echo ""
    echo "=== Build Complete ==="
    echo "QBHD compiler created: ./qbhd"
    echo ""
    echo "Test with: ./qbhd --help"
else
    echo "ERROR: qb64 executable not found. Please run setup first:"
    echo "  ./setup_lnx.sh"
    exit 1
fi

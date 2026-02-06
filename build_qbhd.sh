#!/bin/bash
# QBHD Phase 1 Complete Build Script
# Builds QB64 if needed, then creates QBHD compiler-only version

set -e

echo "======================================"
echo "  QBHD Phase 1: Compiler Extraction"
echo "======================================"
echo ""

# Step 1: Check if QB64 is built
if [ ! -f "./qb64" ]; then
    echo "[1/3] QB64 not found. Building QB64 first..."
    echo "This may take several minutes..."
    ./setup_lnx.sh
    echo "✓ QB64 built successfully"
else
    echo "[1/3] ✓ QB64 already built"
fi

echo ""

# Step 2: Strip IDE code
echo "[2/3] Stripping IDE code from QB64..."
python3 strip_ide.py
echo "✓ IDE code removed"

echo ""

# Step 3: Build QBHD compiler
echo "[3/3] Building QBHD compiler..."
./qb64 -x source/qbhd_compiler.bas -o qbhd

if [ -f "./qbhd" ]; then
    echo ""
    echo "======================================"
    echo "  ✓ QBHD Compiler Built Successfully!"
    echo "======================================"
    echo ""
    echo "Test it:"
    echo "  ./qbhd --help"
    echo "  ./qbhd --version"
    echo ""
    echo "Compile a program:"
    echo "  ./qbhd -c myprogram.bas"
    echo ""
else
    echo "ERROR: Build failed"
    exit 1
fi

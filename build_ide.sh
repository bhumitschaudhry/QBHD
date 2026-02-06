#!/bin/bash
# Build QBHD IDE

set -e

echo "=== Building QBHD IDE ==="
echo

# Check for Node.js
if ! command -v npm &> /dev/null; then
    echo "Error: Node.js/npm not found"
    echo "Install from: https://nodejs.org"
    exit 1
fi

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found"
    echo "Install from: https://rustup.rs"
    exit 1
fi

cd ide

echo "→ Installing dependencies..."
npm install

echo "→ Building IDE..."
npm run tauri build

echo
echo "✓ Build complete!"
echo
echo "Binary: ide/src-tauri/target/release/qbhd-ide"
echo
echo "Run:"
echo "  cd ide && npm run tauri dev"

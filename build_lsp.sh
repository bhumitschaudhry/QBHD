#!/bin/bash
# Build QBHD LSP server

set -e

echo "=== Building QBHD LSP Server ==="
echo

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found"
    echo "Install from: https://rustup.rs"
    exit 1
fi

cd lsp

echo "→ Building LSP server..."
cargo build --release

echo
echo "✓ Build complete!"
echo
echo "Binary: lsp/target/release/qbhd-lsp"
echo
echo "Install:"
echo "  sudo cp lsp/target/release/qbhd-lsp /usr/local/bin/"
echo
echo "Test:"
echo "  ./lsp/target/release/qbhd-lsp"

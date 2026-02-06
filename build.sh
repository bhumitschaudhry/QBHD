#!/bin/bash
# Unified build script for QBHD - all platforms

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== QBHD Unified Build Script ==="
echo

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

echo "Detected OS: $OS"
echo

# Install dependencies
install_deps() {
    echo "→ Installing dependencies..."
    if [ "$OS" = "linux" ]; then
        if command -v apt-get &> /dev/null; then
            sudo apt-get update
            sudo apt-get install -y g++ mesa-common-dev libglu1-mesa-dev libasound2-dev libx11-dev
        elif command -v dnf &> /dev/null; then
            sudo dnf install -y gcc-c++ mesa-libGL-devel mesa-libGLU-devel alsa-lib-devel libX11-devel
        elif command -v pacman &> /dev/null; then
            sudo pacman -S --noconfirm gcc mesa glu alsa-lib libx11
        fi
    elif [ "$OS" = "macos" ]; then
        xcode-select --install 2>/dev/null || true
    fi
}

# Build QB64 if needed
build_qb64() {
    if [ ! -f "./qb64" ]; then
        echo "→ Building QB64..."
        if [ "$OS" = "linux" ]; then
            ./setup_lnx.sh
        elif [ "$OS" = "macos" ]; then
            ./setup_osx.command
        fi
    else
        echo "✓ QB64 already built"
    fi
}

# Strip IDE and build QBHD
build_qbhd() {
    echo "→ Stripping IDE code..."
    python3 strip_ide.py
    
    echo "→ Building QBHD compiler..."
    ./qb64 -x source/qbhd_compiler.bas -o qbhd
    
    if [ -f "./qbhd" ]; then
        echo "✓ QBHD built successfully"
    else
        echo "✗ QBHD build failed"
        exit 1
    fi
}

# Apply CLI enhancements
apply_enhancements() {
    echo "→ Applying CLI enhancements..."
    ./apply_cli_enhancements.sh
    
    echo "→ Rebuilding with enhancements..."
    ./qb64 -x source/qbhd_compiler.bas -o qbhd
}

# Run tests
run_tests() {
    echo "→ Running tests..."
    ./test_cli_enhancements.sh
}

# Main build flow
main() {
    install_deps
    build_qb64
    build_qbhd
    apply_enhancements
    
    echo
    echo "=== Build Complete ==="
    echo
    ./qbhd --version
    echo
    echo "Run './qbhd --help' for usage"
}

# Parse arguments
case "${1:-}" in
    --deps-only)
        install_deps
        ;;
    --test)
        run_tests
        ;;
    *)
        main
        ;;
esac

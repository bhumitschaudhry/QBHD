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
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "win32" ]]; then
    OS="windows"
else
    echo "Unsupported OS: $OSTYPE"
    echo "Supported: linux, macos, windows (Git Bash/MSYS2)"
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
    elif [ "$OS" = "windows" ]; then
        echo "  Windows build requires:"
        echo "  - MinGW/GCC (g++) in PATH"
        echo "  - Or Visual Studio Build Tools"
        echo "  - Python 3 in PATH"
        if ! command -v g++ &> /dev/null && ! command -v cl &> /dev/null; then
            echo "  WARNING: No C++ compiler found in PATH"
        fi
    fi
}

# Build QB64 if needed
build_qb64() {
    if [ "$OS" = "windows" ]; then
        if [ ! -f "./qb64.exe" ]; then
            echo "→ Building QB64 (Windows)..."
            if [ -f "setup_win.bat" ]; then
                cmd //c setup_win.bat
            else
                echo "  Run setup_win.bat manually or install from qb64.org"
                exit 1
            fi
        else
            echo "✓ QB64 already built"
        fi
    else
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
    fi
}

# Strip IDE and build QBHD
build_qbhd() {
    echo "→ Stripping IDE code..."
    python3 strip_ide.py || python strip_ide.py

    echo "→ Building QBHD compiler..."
    if [ "$OS" = "windows" ]; then
        ./qb64.exe -x source/qbhd_compiler.bas -o qbhd.exe
        if [ -f "./qbhd.exe" ]; then
            echo "✓ QBHD built successfully (qbhd.exe)"
        else
            echo "✗ QBHD build failed"
            exit 1
        fi
    else
        ./qb64 -x source/qbhd_compiler.bas -o qbhd
        if [ -f "./qbhd" ]; then
            echo "✓ QBHD built successfully"
        else
            echo "✗ QBHD build failed"
            exit 1
        fi
    fi
}

# Apply CLI enhancements
apply_enhancements() {
    echo "→ Applying CLI enhancements..."
    if [ -f "apply_cli_enhancements.sh" ]; then
        bash apply_cli_enhancements.sh
    fi

    echo "→ Rebuilding with enhancements..."
    if [ "$OS" = "windows" ]; then
        ./qb64.exe -x source/qbhd_compiler.bas -o qbhd.exe
    else
        ./qb64 -x source/qbhd_compiler.bas -o qbhd
    fi
}

# Build LSP server
build_lsp() {
    echo "→ Building LSP server..."
    if command -v cargo &> /dev/null; then
        cd lsp
        cargo build --release
        cd ..
        echo "✓ LSP server built"
    else
        echo "  Skipping LSP build (Rust/Cargo not found)"
    fi
}

# Build IDE (if Node.js available)
build_ide() {
    echo "→ Building IDE..."
    if command -v npm &> /dev/null; then
        cd ide
        npm install
        cd ..
        echo "✓ IDE dependencies installed"
        echo "  Run 'cd ide && npm run tauri dev' to launch the IDE"
    else
        echo "  Skipping IDE build (Node.js not found)"
    fi
}

# Run tests
run_tests() {
    echo "→ Running tests..."
    if [ -f "test_cli_enhancements.sh" ]; then
        bash test_cli_enhancements.sh
    fi
    if [ -f "test_suite.py" ]; then
        python3 test_suite.py || python test_suite.py
    fi
}

# Main build flow
main() {
    install_deps
    build_qb64
    build_qbhd
    apply_enhancements
    build_lsp
    build_ide

    echo
    echo "=== Build Complete ==="
    echo
    if [ "$OS" = "windows" ]; then
        ./qbhd.exe --version
        echo
        echo "Run './qbhd.exe --help' for usage"
    else
        ./qbhd --version
        echo
        echo "Run './qbhd --help' for usage"
    fi
}

# Parse arguments
case "${1:-}" in
    --deps-only)
        install_deps
        ;;
    --test)
        run_tests
        ;;
    --lsp)
        build_lsp
        ;;
    --ide)
        build_ide
        ;;
    *)
        main
        ;;
esac

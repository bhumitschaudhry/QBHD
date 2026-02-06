# Makefile for QBHD

.PHONY: all build clean test install help

all: build

build:
	@echo "Building QBHD..."
	@./build.sh

clean:
	@echo "Cleaning build artifacts..."
	@rm -f qbhd qb64
	@rm -f source/qbhd_compiler.bas
	@rm -f test_hello test_out
	@rm -rf internal/c/libqb/*.o

test: build
	@echo "Running tests..."
	@python3 test_suite.py

install: build
	@echo "Installing QBHD to /usr/local/bin..."
	@sudo cp qbhd /usr/local/bin/
	@echo "Done. Run 'qbhd --version' to verify."

help:
	@echo "QBHD Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  make build    - Build QBHD compiler"
	@echo "  make test     - Run test suite"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make install  - Install to /usr/local/bin"
	@echo "  make help     - Show this help"

#!/usr/bin/env python3
"""
QBHD Compiler Extractor
Phase 1, Task 1: Strip IDE code from QB64 to create compiler-only QBHD
"""

import re
import sys

def strip_ide_code(input_file, output_file):
    """Remove IDE-related code from QB64 source"""
    
    print(f"Reading {input_file}...")
    with open(input_file, 'r', encoding='utf-8', errors='ignore') as f:
        lines = f.readlines()
    
    output_lines = []
    skip_mode = False
    
    for i, line in enumerate(lines):
        # Skip IDE include files
        if "'$INCLUDE:'ide" in line.lower():
            output_lines.append("' IDE INCLUDE REMOVED\n")
            continue
        
        # Remove $SCREENHIDE
        if "$SCREENHIDE" in line:
            output_lines.append(line.replace("$SCREENHIDE", "' $SCREENHIDE"))
            continue
        
        # Update branding
        line = line.replace("QB64 IDE and Compiler", "QBHD Compiler")
        line = line.replace("QB64 x32", "QBHD x32")
        line = line.replace("QB64 x64", "QBHD x64")
        line = line.replace('WindowTitle = "QB64', 'WindowTitle = "QBHD')
        line = line.replace("QB64 Compiler V", "QBHD Compiler V")
        line = line.replace("Usage: qb64", "Usage: qbhd")
        
        # Force compiler mode after ConsoleMode declaration
        if "DIM SHARED ConsoleMode, No_C_Compile_Mode, NoIDEMode" in line:
            output_lines.append(line)
            output_lines.append("' QBHD: Force compiler-only mode\n")
            output_lines.append("NoIDEMode = 1\n")
            output_lines.append("ConsoleMode = 1\n")
            continue
        
        output_lines.append(line)
    
    print(f"Writing {output_file}...")
    with open(output_file, 'w', encoding='utf-8') as f:
        f.writelines(output_lines)
    
    print(f"✓ Created compiler-only source: {output_file}")
    print(f"  Original lines: {len(lines)}")
    print(f"  Output lines: {len(output_lines)}")

if __name__ == "__main__":
    strip_ide_code("source/qb64.bas", "source/qbhd_compiler.bas")
    print("\nNext step: Build with existing QB64:")
    print("  ./qb64 -x source/qbhd_compiler.bas -o qbhd")

#!/usr/bin/env python3
"""
Enhance QBHD CLI - Add modern command-line flags
Adds: --json, --check, --output, --optimize, --debug, --verbose, --version
"""

import re
import sys

def enhance_cli(input_file, output_file):
    with open(input_file, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Add new shared variables after existing CLI variables
    cli_vars_pattern = r'(DIM SHARED ShowWarnings AS _BYTE, QuietMode AS _BYTE, CMDLineFile AS STRING\s+DIM SHARED MonochromeLoggingMode AS _BYTE)'
    cli_vars_addition = r'\1\nDIM SHARED JSONMode AS _BYTE, CheckOnlyMode AS _BYTE, VerboseMode AS _BYTE\nDIM SHARED OptimizeLevel AS INTEGER, DebugMode AS _BYTE'
    
    content = re.sub(cli_vars_pattern, cli_vars_addition, content, count=1)
    
    # Find the help text section and enhance it
    help_pattern = r'(PRINT "Options:"\s+PRINT "  <file>.*?Source file to load".*?\n)(.*?)(PRINT\s+SYSTEM)'
    
    new_help = r'''\1                PRINT "  -c                      Compile instead of edit"
                PRINT "  -x                      Compile and output to console"
                PRINT "  -o <output file>        Write output executable to <output file>"
                PRINT "  --output <file>         Same as -o"
                PRINT "  --check                 Syntax check only (no compilation)"
                PRINT "  --json                  Output diagnostics in JSON format (for LSP)"
                PRINT "  --optimize <0-3>        Optimization level (0=none, 3=max)"
                PRINT "  --debug                 Include debug symbols (GDB/LLDB)"
                PRINT "  --verbose               Verbose compilation output"
                PRINT "  --version               Show version and exit"
                PRINT "  -w                      Show warnings"
                PRINT "  -q                      Quiet mode"
                PRINT "  -m                      Monochrome output"
                PRINT "  -e                      Enable OPTION _EXPLICIT"
                PRINT "  -p                      Purge pre-compiled content"
                PRINT "  -z                      Generate C code only"
                PRINT
                \3'''
    
    content = re.sub(help_pattern, new_help, content, flags=re.DOTALL)
    
    # Add new case handlers in ParseCMDLineArgs
    case_handlers = r'''            CASE "-e" 'Option Explicit
                optionexplicit_cmd = -1
                cmdlineswitch = -1'''
    
    new_cases = r'''            CASE "-e" 'Option Explicit
                optionexplicit_cmd = -1
                cmdlineswitch = -1
            CASE "--" 'Long options
                SELECT CASE LCASE$(token$)
                    CASE "--version"
                        _DEST _CONSOLE
                        PRINT "QBHD Compiler V" + Version$
                        PRINT "Based on QB64"
                        SYSTEM
                    CASE "--json"
                        JSONMode = -1
                        QuietMode = -1
                        cmdlineswitch = -1
                    CASE "--check"
                        CheckOnlyMode = -1
                        NoIDEMode = 1
                        cmdlineswitch = -1
                    CASE "--verbose"
                        VerboseMode = -1
                        cmdlineswitch = -1
                    CASE "--debug"
                        DebugMode = -1
                        idedebuginfo = 1
                        Include_GDB_Debugging_Info = 1
                        cmdlineswitch = -1
                    CASE "--output", "--optimize"
                        IF LCASE$(token$) = "--output" THEN
                            IF LEN(COMMAND$(i + 1)) > 0 THEN outputfile_cmd$ = COMMAND$(i + 1): i = i + 1
                        ELSE
                            IF LEN(COMMAND$(i + 1)) > 0 THEN
                                OptimizeLevel = VAL(COMMAND$(i + 1))
                                IF OptimizeLevel < 0 THEN OptimizeLevel = 0
                                IF OptimizeLevel > 3 THEN OptimizeLevel = 3
                                i = i + 1
                            END IF
                        END IF
                        cmdlineswitch = -1
                END SELECT'''
    
    content = content.replace(case_handlers, new_cases)
    
    with open(output_file, 'w', encoding='utf-8', errors='ignore') as f:
        f.write(content)
    
    print(f"✓ Enhanced CLI in {output_file}")
    print("  Added: --json, --check, --output, --optimize, --debug, --verbose, --version")

if __name__ == "__main__":
    input_file = "source/qbhd_compiler.bas"
    output_file = "source/qbhd_compiler.bas"
    
    enhance_cli(input_file, output_file)

# QBHD Features Documentation

Complete feature reference for QBHD development environment.

## Compiler Features

### Modern CLI

The QBHD compiler provides a professional command-line interface with modern flags.

#### Syntax Checking

```bash
# Fast syntax validation without compilation
qbhd --check myprogram.bas

# Verbose output with detailed messages
qbhd --check --verbose myprogram.bas

# JSON output for tool integration
qbhd --json --check myprogram.bas
```

**Output**: Errors, warnings, and info messages with line numbers and descriptions.

#### Optimization

```bash
# Level 0: No optimization (fastest compile)
qbhd --optimize 0 myprogram.bas

# Level 1: Basic optimization (default)
qbhd --optimize 1 myprogram.bas

# Level 2: Aggressive optimization
qbhd --optimize 2 myprogram.bas

# Level 3: Maximum optimization (slowest compile)
qbhd --optimize 3 myprogram.bas
```

**Benefits**: Faster runtime, smaller binaries, reduced memory usage.

#### Debug Support

```bash
# Include debug symbols for debugging
qbhd --debug myprogram.bas

# Combine with optimization
qbhd --debug --optimize 2 myprogram.bas

# Debug with verbose output
qbhd --debug --verbose myprogram.bas
```

**Enables**: Debugger support, stack traces, symbol information.

#### Output Control

```bash
# Specify output filename
qbhd myprogram.bas -o myprogram

# Compile without running
qbhd -c myprogram.bas

# Compile and run (default)
qbhd myprogram.bas
```

#### JSON Output

```bash
# Get structured diagnostics
qbhd --json --check myprogram.bas

# Example output:
# {
#   "diagnostics": [
#     {
#       "line": 10,
#       "column": 5,
#       "severity": "error",
#       "message": "Undefined variable: x"
#     }
#   ]
# }
```

**Use Cases**: IDE integration, CI/CD pipelines, automated testing.

### Cross-Platform Compilation

```bash
# Linux
./build.sh
qbhd myprogram.bas

# macOS
./build.sh
qbhd myprogram.bas

# Windows
./build.sh
qbhd myprogram.bas
```

**Supported**: Windows (XP+), Linux, macOS

---

## LSP Server Features

### Real-Time Diagnostics

Errors, warnings, and info messages appear as you type.

```
✗ Error: Undefined variable 'x'
⚠ Warning: Unused variable 'y'
ℹ Info: Function 'foo' defined at line 5
💡 Hint: Consider using CONST for constants
```

**Performance**: <100ms response time

### Code Completion

Intelligent completion for keywords, variables, and functions.

```bash
# In Neovim: Press <C-x><C-o>
# Suggestions appear:
# - Keywords (PRINT, INPUT, FOR, etc.)
# - User-defined variables
# - Function names
# - Built-in functions
```

**Coverage**: 50+ keywords, all user symbols

### Hover Information

View symbol details by hovering.

```
Function: PrintMessage
Location: line 15
Type: SUB
Parameters: message AS STRING
```

### Go-to-Definition

Jump to where symbols are defined.

```bash
# In Neovim: Press 'gd'
# Jumps to:
# - Variable declarations
# - Function definitions
# - Type definitions
```

### Find References

Find all usages of a symbol.

```bash
# In Neovim: Press 'gr'
# Shows all locations where symbol is used
```

### Semantic Analysis

Type checking and symbol tracking.

```
✗ Type mismatch: Expected STRING, got INTEGER
✗ Undefined variable: myVar
⚠ Variable 'x' assigned but never used
```

### Document Synchronization

Real-time sync between editor and LSP server.

```
- Open file: Full document sent
- Edit: Incremental changes sent
- Save: Document updated
- Close: Document removed from cache
```

---

## Neovim Plugin Features

### Enhanced UI

Professional diagnostic display with floating windows.

```
┌─────────────────────────────────┐
│ Error: Undefined variable 'x'   │
│ Line 10, Column 5               │
└─────────────────────────────────┘
```

### Syntax Highlighting

BASIC syntax highlighting with color support.

```basic
PRINT "Hello"           ' String (green)
DIM x AS INTEGER        ' Keywords (blue)
x = 42                  ' Numbers (orange)
' Comment (gray)
```

### Filetype Detection

Automatic detection of .bas files.

```bash
# Automatically sets filetype to 'basic'
nvim myprogram.bas
```

### Commands

```vim
:QBHDCompile      " Compile current file
:QBHDRun          " Run compiled program
:QBHDCheck        " Check syntax
:QBHDFormat       " Format code
```

### Keybindings

```vim
gd                " Go to definition
K                 " Hover information
gr                " Find references
<leader>rn        " Rename symbol
<leader>ca        " Code actions
```

---

## GUI IDE Features

### Monaco Editor

Professional code editor with advanced features.

```
- Syntax highlighting
- Line numbers
- Code folding
- Minimap
- Smooth scrolling
- Multi-cursor editing
- Find and replace
```

### File Explorer

Browse and manage BASIC files.

```
Project/
├── main.bas
├── utils.bas
├── graphics.bas
└── data.txt
```

**Features**:
- File tree view
- .bas file filtering
- File icons
- Hover effects
- Context menu

### Build Integration

One-click compilation and execution.

```
┌─────────────────────────────────┐
│ [Check] [Build] [Run]           │
└─────────────────────────────────┘
```

**Actions**:
- Check: Syntax validation
- Build: Compile to binary
- Run: Execute program

### Integrated Terminal

View compilation output and program results.

```
$ qbhd --check main.bas
✓ Syntax OK

$ qbhd main.bas -o main
Compiling... Done!

$ ./main
Hello, QBHD!
```

### Settings Panel

Configure editor behavior.

```
Font Size: 14
Tab Size: 4
Theme: Dark
Auto-save: On
Terminal Font: Monospace
```

### Dark Theme

Professional VS Code-inspired design.

```
- Dark background
- Syntax highlighting
- Contrast optimized
- Eye-friendly colors
```

### Keyboard Shortcuts

```
Ctrl+S              Save file
Ctrl+/              Toggle comment
Ctrl+Shift+F        Format code
Ctrl+B              Toggle explorer
Ctrl+J              Toggle terminal
Ctrl+K Ctrl+0       Fold all
Ctrl+K Ctrl+J       Unfold all
```

---

## Compatibility

### QB4.5/QBasic Syntax

Full compatibility with classic BASIC syntax.

```basic
' Variables
DIM x AS INTEGER
DIM name AS STRING

' Control flow
IF x > 0 THEN
    PRINT "Positive"
END IF

' Loops
FOR i = 1 TO 10
    PRINT i
NEXT i

' Functions
FUNCTION Add(a AS INTEGER, b AS INTEGER) AS INTEGER
    Add = a + b
END FUNCTION

' Subroutines
SUB PrintMessage(msg AS STRING)
    PRINT msg
END SUB
```

### Data Types

```basic
INTEGER         ' 32-bit integer
LONG            ' 64-bit integer
SINGLE          ' 32-bit float
DOUBLE          ' 64-bit float
STRING          ' Text
BYTE            ' 8-bit integer
```

### Built-in Functions

```basic
' String functions
LEN(s)          ' Length
MID$(s, p, n)   ' Substring
INSTR(s, t)     ' Find substring
UCASE$(s)       ' Uppercase
LCASE$(s)       ' Lowercase

' Math functions
ABS(x)          ' Absolute value
SQR(x)          ' Square root
SIN(x)          ' Sine
COS(x)          ' Cosine
INT(x)          ' Integer part
RND()           ' Random number

' I/O functions
PRINT           ' Output
INPUT           ' Input
OPEN            ' File operations
CLOSE           ' Close file
```

---

## Performance

### Compilation Speed

- **Syntax Check**: <100ms
- **Debug Build**: ~1-2 seconds
- **Optimized Build**: ~3-5 seconds

### Runtime Performance

- **Optimization Level 0**: Baseline
- **Optimization Level 1**: ~10% faster
- **Optimization Level 2**: ~25% faster
- **Optimization Level 3**: ~40% faster

### Memory Usage

- **Compiler**: ~50MB
- **LSP Server**: ~30MB
- **IDE**: ~100MB

---

## Integration

### CI/CD

```yaml
# GitHub Actions example
- name: Check BASIC files
  run: |
    for f in *.bas; do
      qbhd --check "$f" || exit 1
    done

- name: Build
  run: qbhd --optimize 3 main.bas -o main

- name: Test
  run: ./main
```

### Make Integration

```makefile
.PHONY: check build run clean

check:
	qbhd --check *.bas

build:
	qbhd --optimize 3 main.bas -o main

run: build
	./main

clean:
	rm -f main
```

### Editor Integration

```bash
# Vim/Neovim
nvim myprogram.bas

# VS Code (via LSP)
code myprogram.bas

# Emacs (via LSP)
emacs myprogram.bas
```

---

## Advanced Features

### Batch Processing

```bash
# Check all files
for f in *.bas; do
  qbhd --check "$f"
done

# Compile all files
for f in *.bas; do
  qbhd "$f" -o "${f%.bas}"
done
```

### Parallel Compilation

```bash
# Compile 4 files in parallel
find . -name "*.bas" | xargs -P 4 -I {} qbhd {}
```

### Watch Mode

```bash
# Recompile on file change
while inotifywait -e modify *.bas; do
  qbhd --check *.bas
done
```

### JSON Diagnostics

```bash
# Get structured output
qbhd --json --check myprogram.bas > diagnostics.json

# Parse with jq
jq '.diagnostics[] | select(.severity=="error")' diagnostics.json
```

---

## Limitations

- No GUI debugger (use command-line debuggers)
- No built-in package manager
- Limited to BASIC syntax (no extensions)
- Single-threaded compilation

---

## Future Enhancements

Planned features:
- Multiple editor tabs
- Git integration
- Extension system
- Debugger UI
- Package manager
- Remote compilation
- Cloud IDE support

# API Documentation

## Tauri IPC Commands

The IDE backend exposes these commands via Tauri's IPC mechanism. The React frontend invokes them using `@tauri-apps/api/tauri`'s `invoke` function.

### read_file

Read a file from disk and return its contents as a string.

**Request:**
```json
{
  "path": "string"
}
```

**Response:** `string` (file contents)

**Error:** `"Failed to read {path}: {error}"`

**Frontend usage:**
```javascript
import { invoke } from '@tauri-apps/api/tauri';
const contents = await invoke('read_file', { path: '/path/to/file.bas' });
```

---

### save_file

Write a string to a file on disk.

**Request:**
```json
{
  "path": "string",
  "contents": "string"
}
```

**Response:** `null` (success)

**Error:** `"Failed to save {path}: {error}"`

**Frontend usage:**
```javascript
await invoke('save_file', { path: '/path/to/file.bas', contents: 'PRINT "Hello"' });
```

---

### compile_file

Compile a BASIC file using the `qbhd` compiler.

**Request:**
```json
{
  "path": "string"
}
```

**Response:** `string` (compilation output, includes "Build succeeded." on success)

**Error:** Compilation error output (stdout + stderr)

**Frontend usage:**
```javascript
const result = await invoke('compile_file', { path: '/path/to/file.bas' });
```

**Notes:**
- Blocks the UI thread during compilation
- Runs `qbhd {path}` as a subprocess
- Returns stdout on success, stdout+stderr on failure

---

### check_file

Check a BASIC file for errors without compiling.

**Request:**
```json
{
  "path": "string"
}
```

**Response:** `string` (JSON array of diagnostics)

**Error:** stderr output if no JSON produced

**Frontend usage:**
```javascript
const json = await invoke('check_file', { path: '/path/to/file.bas' });
const diagnostics = JSON.parse(json);
```

**Response format (JSON):**
```json
[
  {
    "file": "file.bas",
    "line": 5,
    "column": 10,
    "severity": "error",
    "message": "Undefined variable 'x'"
  }
]
```

**Notes:**
- Blocks the UI thread during checking
- Runs `qbhd --json --check {path}` as a subprocess
- Returns JSON even on errors (if the compiler produces JSON)

---

### run_file

Execute a compiled binary.

**Request:**
```json
{
  "path": "string"
}
```

**Response:** `string` (program output: stdout + stderr)

**Error:** `"Failed to run {path}: {error}"`

**Frontend usage:**
```javascript
const output = await invoke('run_file', { path: '/path/to/compiled/binary' });
```

**Notes:**
- Blocks the UI thread during execution
- Runs `{path}` as a subprocess
- Returns stdout on success, stdout+stderr on failure
- No timeout mechanism

---

### list_directory

List the contents of a directory.

**Request:**
```json
{
  "path": "string"
}
```

**Response:** `DirEntry[]`

**DirEntry schema:**
```json
{
  "name": "string",
  "path": "string",
  "is_dir": "boolean"
}
```

**Error:** `"Failed to read directory {path}: {error}"`

**Frontend usage:**
```javascript
const entries = await invoke('list_directory', { path: '/path/to/directory' });
entries.forEach(entry => {
  console.log(entry.name, entry.is_dir ? '(dir)' : '(file)');
});
```

**Notes:**
- Returns entries sorted: directories first, then files (case-insensitive)
- No pagination (returns all entries)
- No filtering (returns all file types)

---

## LSP Protocol

The LSP server (`qbhd-lsp`) implements the Language Server Protocol over stdin/stdout (JSON-RPC).

### Capabilities

```json
{
  "textDocumentSync": "FULL",
  "completionProvider": {
    "triggerCharacters": [" ", ".", "$", "("]
  },
  "hoverProvider": true,
  "definitionProvider": true,
  "referencesProvider": true,
  "renameProvider": true,
  "codeActionProvider": true,
  "diagnosticProvider": {}
}
```

### Supported Methods

| Method | Description |
|--------|-------------|
| `initialize` | Server initialization |
| `initialized` | Initialization complete |
| `textDocument/didOpen` | Document opened |
| `textDocument/didChange` | Document changed |
| `textDocument/didClose` | Document closed |
| `textDocument/completion` | Code completion |
| `textDocument/hover` | Hover information |
| `textDocument/definition` | Go to definition |
| `textDocument/references` | Find references |
| `textDocument/rename` | Rename symbol |
| `textDocument/codeAction` | Code actions (stub) |
| `shutdown` | Server shutdown |

### Completion

Returns keywords and user-defined symbols. Filtered by prefix matching.

**Completion kinds:**
- `KEYWORD` - BASIC keywords (PRINT, IF, FOR, etc.)
- `FUNCTION` - QB64 underscore-prefixed keywords (_RGB, _NEWIMAGE, etc.)
- `VARIABLE` - User-defined symbols

### Hover

Returns markdown documentation for symbols.

**Built-in documentation covers:**
- All BASIC keywords (PRINT, IF, FOR, WHILE, DO, SELECT, etc.)
- All QB64 keywords (_RGB, _NEWIMAGE, etc.)
- All built-in functions (ABS, SIN, COS, LEN, LEFT$, etc.)
- All types (INTEGER, LONG, SINGLE, DOUBLE, STRING)
- User-defined variables, functions, and subs

### Definition

Returns the location where a symbol is defined.

**Limitations:**
- Returns location within the same document only
- Character position is always 0 (column tracking not implemented)
- Line numbers are approximate (based on statement index, not source lines)

### References

Returns all locations where a symbol is referenced.

**Limitations:**
- Returns all occurrences of the symbol name (no scope filtering)
- Character positions are always 0
- Same line number limitations as definition

### Rename

Renames all occurrences of a symbol.

**Limitations:**
- Same scope limitations as references
- No workspace-wide rename (single document only)

### Diagnostics

Published automatically on document open and change.

**Source:** `qbhd --json --check {file}`

**Diagnostic format:**
```json
{
  "range": {
    "start": { "line": 4, "character": 9 },
    "end": { "line": 4, "character": 19 }
  },
  "severity": 1,
  "source": "qbhd",
  "message": "Undefined variable 'x'"
}
```

**Limitations:**
- Range end is always start + 10 characters (fixed width)
- Only "error" and "warning" severities are mapped
- Subprocess call blocks the async runtime

---

## Neovim Plugin Commands

| Command | Description | Implementation |
|---------|-------------|----------------|
| `:QBHDCompile` | Compile current file | `!qbhd %` |
| `:QBHDRun` | Compile and run | `!qbhd % && ./output` |
| `:QBHDCheck` | Check for errors | `!qbhd --check %` |
| `:QBHDFormat` | Trim whitespace, ensure final newline | Lua function |
| `:QBHDInfo` | Show LSP connection status | `vim.lsp.get_clients()` |

---

## CLI Interface

```bash
qbhd [flags] [file.bas]
```

### Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--version` | Show version | `qbhd --version` |
| `--help` | Show help | `qbhd --help` |
| `--json` | JSON output | `qbhd --json --check file.bas` |
| `--check` | Check only | `qbhd --check file.bas` |
| `--output <file>` | Output filename | `qbhd file.bas -o myapp` |
| `--optimize <level>` | Optimization (0-3) | `qbhd --optimize 2 file.bas` |
| `--debug` | Debug info | `qbhd --debug file.bas` |
| `--verbose` | Verbose output | `qbhd --verbose file.bas` |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Compilation error |
| 2 | Invalid arguments |

### JSON Output Format

With `--json --check`:
```json
[
  {
    "file": "myfile.bas",
    "line": 5,
    "column": 10,
    "severity": "error",
    "message": "Undefined variable"
  }
]
```

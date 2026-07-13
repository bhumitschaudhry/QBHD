# Architecture Decision Log

This document describes the key architectural decisions made in the QBHD project, their rationale, and tradeoffs.

## Decision 1: Strip IDE from QB64 rather than fork

**Date:** 2026-02-06
**Status:** Accepted

### Context

QB64 is a 26,000-line BASIC compiler with a built-in text-mode IDE. The goal is to create a modern development environment while preserving the compiler's compatibility.

### Decision

Strip the IDE code from the QB64 source using a Python script (`strip_ide.py`) rather than maintaining a separate fork.

### Rationale

- Preserves exact compiler compatibility with QB64
- Reduces maintenance burden (upstream changes can be merged)
- Simple transformation (comment out includes, force compiler mode)
- No risk of introducing compiler bugs through manual editing

### Tradeoffs

- **Pro**: Minimal changes to compiler source
- **Pro**: Easy to update when QB64 releases new versions
- **Con**: Stripping is fragile (depends on specific text patterns in source)
- **Con**: CLI enhancements are applied via `sed` patching, which is brittle
- **Con**: The "stripped" file is only 3 lines different from the original

## Decision 2: LSP for editor integration

**Date:** 2026-02-06
**Status:** Accepted

### Context

The project needs IDE features (diagnostics, completion, hover, navigation) in multiple editors (Neovim, GUI IDE).

### Decision

Implement a Language Server Protocol (LSP) server in Rust that provides all IDE features.

### Rationale

- LSP is the industry standard for editor integration
- One server works with any LSP-compatible editor (Neovim, VS Code, Emacs, etc.)
- Rust provides good performance and safety for a language server
- The `tower-lsp` crate provides a solid LSP framework

### Tradeoffs

- **Pro**: Single implementation works with all editors
- **Pro**: Rust is performant for parsing and analysis
- **Pro**: LSP protocol handles editor communication
- **Con**: Rust has a steeper learning curve than Python/TypeScript
- **Con**: LSP protocol is complex (many capabilities to implement)
- **Con**: Diagnostics shell out to the compiler (blocking subprocess)

## Decision 3: Tauri for GUI IDE

**Date:** 2026-02-06
**Status:** Accepted

### Context

The project needs a modern GUI IDE to replace the stripped QB64 text-mode IDE.

### Decision

Use Tauri (Rust backend + web frontend) with React and Monaco Editor.

### Rationale

- Tauri produces smaller binaries than Electron
- Rust backend integrates naturally with the LSP server
- Monaco Editor provides VS Code-quality editing
- React is a well-established UI framework
- Web technologies allow rapid UI development

### Tradeoffs

- **Pro**: Small binary size (~10MB vs ~100MB for Electron)
- **Pro**: Rust backend is fast and safe
- **Pro**: Monaco Editor has excellent syntax highlighting
- **Pro**: React ecosystem is mature
- **Con**: Tauri requires Rust knowledge for backend changes
- **Con**: Monaco Editor is large (~5MB)
- **Con**: Web technologies add complexity vs native UI

## Decision 4: Monaco Editor for code editing

**Date:** 2026-02-06
**Status:** Accepted

### Context

The IDE needs a code editor with syntax highlighting, line numbers, minimap, and other features.

### Decision

Use Monaco Editor (the editor that powers VS Code) via `@monaco-editor/react`.

### Rationale

- Monaco is the most feature-rich web-based code editor
- Provides syntax highlighting via Monarch tokenizer
- Built-in minimap, line numbers, search/replace
- Familiar editing experience for VS Code users
- Well-maintained by Microsoft

### Tradeoffs

- **Pro**: Feature-rich out of the box
- **Pro**: Excellent syntax highlighting support
- **Pro**: Familiar to VS Code users
- **Con**: Large bundle size (~5MB)
- **Con**: No built-in BASIC language (custom Monarch definition needed)
- **Con**: No LSP integration in the IDE (uses only syntax highlighting)

## Decision 5: External diagnostics via subprocess

**Date:** 2026-02-06
**Status:** Accepted

### Context

The LSP server needs to provide diagnostics (errors, warnings) for BASIC code.

### Decision

Shell out to `qbhd --json --check` to get diagnostics rather than implementing a type checker in the LSP server.

### Rationale

- Ensures diagnostics match the compiler exactly
- No need to maintain a separate type checker
- The compiler already has all the error checking logic
- JSON output format is easy to parse

### Tradeoffs

- **Pro**: Diagnostics are always accurate (same as compiler)
- **Pro**: No duplicate implementation
- **Pro**: Easy to maintain
- **Con**: Subprocess call is blocking (freezes LSP during check)
- **Con**: Binary must be found on PATH or configured
- **Con**: No incremental checking (full file on every change)

## Decision 6: Pratt parsing for expressions

**Date:** 2026-02-06
**Status:** Accepted

### Context

The BASIC parser needs to handle expressions with operator precedence (e.g., `a + b * c`).

### Decision

Use Pratt parsing (top-down operator precedence) for expression parsing.

### Rationale

- Clean handling of operator precedence
- Easy to extend with new operators
- Well-understood algorithm
- Works well with recursive descent for statements

### Tradeoffs

- **Pro**: Clean precedence handling
- **Pro**: Easy to add new operators
- **Pro**: Works well with recursive descent
- **Con**: More complex than simple recursive descent
- **Con**: Requires precedence table maintenance

## Decision 7: Full document sync for LSP

**Date:** 2026-02-06
**Status:** Accepted

### Context

The LSP server needs to stay in sync with the editor's document content.

### Decision

Use `TextDocumentSyncKind::FULL` (send entire document on every change) rather than incremental sync.

### Rationale

- Simpler implementation
- No need to track document changes
- Works reliably with all editors
- BASIC files are typically small (<10,000 lines)

### Tradeoffs

- **Pro**: Simple implementation
- **Pro**: No change tracking bugs
- **Pro**: Works with all editors
- **Con**: More bandwidth for large files
- **Con**: Full re-analysis on every keystroke

## Decision 8: Flat symbol table (no scoping)

**Date:** 2026-02-06
**Status:** Accepted

### Context

The semantic analyzer needs to track symbol definitions (variables, functions, subs).

### Decision

Use a single flat `HashMap<String, Vec<SymbolInfo>>` for all symbols, without scope tracking.

### Rationale

- Simpler implementation
- Works for basic features (completion, hover, go-to-definition)
- BASIC has limited scoping rules (no block scope)
- Can be extended later if needed

### Tradeoffs

- **Pro**: Simple implementation
- **Pro**: Fast lookups
- **Pro**: Easy to understand
- **Con**: No scope-aware features (unused variable detection, etc.)
- **Con**: Variables in different SUBs/FUNCTIONs are not distinguished
- **Con**: Rename may affect unrelated symbols with the same name

## Decision 9: JSX without TypeScript

**Date:** 2026-02-06
**Status:** Accepted

### Context

The IDE frontend needs to be written in JavaScript/TypeScript with React.

### Decision

Use plain JSX without TypeScript.

### Rationale

- Faster development (no type annotations)
- Simpler build configuration
- Monaco Editor types are complex
- Small codebase (<500 lines of JSX)

### Tradeoffs

- **Pro**: Faster initial development
- **Pro**: Simpler configuration
- **Pro**: No TypeScript compilation step
- **Con**: No compile-time type checking
- **Con**: Harder to maintain as codebase grows
- **Con**: No IDE autocomplete for props

## Decision 10: Single CSS file (no modules)

**Date:** 2026-02-06
**Status:** Accepted

### Context

The IDE needs styling for the dark theme, layout, and components.

### Decision

Use a single `App.css` file with global class names, rather than CSS modules or styled-components.

### Rationale

- Simple implementation
- No additional dependencies
- Easy to understand
- Small component count (5 components)

### Tradeoffs

- **Pro**: Simple implementation
- **Pro**: No additional dependencies
- **Pro**: Easy to debug
- **Con**: Global namespace (class name collisions possible)
- **Con**: No component-level style encapsulation
- **Con**: Harder to maintain as components grow

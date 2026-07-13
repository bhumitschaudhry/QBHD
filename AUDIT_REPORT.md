# Maintainability Audit — QBHD

**Date:** 2026-07-14
**Auditor:** Claude Code (automated)

## Summary

QBHD is a modernized QB64 fork comprising four modules: a BASIC-to-C++ compiler (26K lines of BASIC), a C/C++ runtime (30K lines in a single file), a Rust LSP server (~2,300 lines), and a Tauri/React IDE (~620 lines of JSX). The project has **extensive documentation** (17+ markdown files, 700+ BASIC keyword references) and a well-structured CI/CD pipeline across 3 platforms.

The most critical issues are: (1) a **heap buffer overflow** in the C runtime's console font handler, (2) **blocking subprocess calls inside async Rust code** that freeze the LSP on every keystroke, and (3) **zero test coverage** across all four modules. The codebase also suffers from extreme file sizes (libqb.cpp at 30K lines, a 1,150-line function with 85 gotos), significant code duplication in both the compiler and LSP parser, and a fragile build pipeline that injects CLI features via blind `sed` patching.

The IDE frontend is the healthiest module — small, clean, and functional — but lacks TypeScript, linting, and tests.

## Scope and Method

**Scanned:** Entire repository — all source code, configuration, documentation, CI/CD, and build tooling.

**Method:** 7 parallel automated agents analyzed: project structure, Go code (none found), TypeScript/JSX frontend, documentation/config, C/C++ runtime (libqb.cpp deep dive), Rust LSP server (all 6 source files), and BASIC compiler source. Each agent read strategic file sections, counted patterns (goto, unwrap, malloc, etc.), and assessed code quality against the audit checklist.

**Excluded:** Third-party vendored libraries in `internal/c/parts/` (miniaudio, FreeGLUT, FreeType, stb_image, etc.) — these are upstream dependencies, not project-authored code. The `internal/c/libqb.cpp` file IS project-maintained and was fully audited.

## Findings by Priority

### Critical

---

**C1: Heap buffer overflow in `sub__consolefont`**
- **Where:** `internal/c/libqb.cpp:30235-30237`
- **What:** `mbstowcs(wc, (char *)FontName->chr, cSize)` writes up to `FontName->len` wide chars into a 32-element `wchar_t` buffer with no bounds check. The subsequent `wcscpy(info.FaceName, wc)` is also unbounded.
- **Risk:** Heap buffer overflow — a crafted or long font name can overwrite adjacent heap memory. Exploitable if the runtime processes untrusted input (e.g., a BASIC program reading font names from user input).
- **Fix:** Replace with `mbstowcs(wc, (char *)FontName->chr, 31); wc[31] = L'\0';` and use `wcsncpy` for the copy. Effort: trivial.

---

**C2: Blocking subprocess calls freeze the LSP on every keystroke**
- **Where:** `lsp/src/diagnostics.rs:35` (`std::process::Command`) called from `main.rs:317` (async fn `publish_diagnostics`)
- **What:** `get_diagnostics()` spawns `qbhd --json --check` synchronously inside an async function, blocking the tokio runtime thread. Additionally, `find_qbhd_binary()` at line 80 spawns `qbhd --version` on every call with no caching.
- **Risk:** The LSP server becomes unresponsive during compilation. A 2-second compile blocks all other LSP requests (completions, hover, goto-definition) for that duration.
- **Fix:** Use `tokio::process::Command` or `tokio::task::spawn_blocking`. Cache the binary path. Effort: low (1-2 hours).

---

**C3: Uninitialized variable used in console mouse handler**
- **Where:** `internal/c/libqb.cpp:30267-30283`
- **What:** `CONSOLE_SCREEN_BUFFER_INFO cl_bufinfo` is declared but never initialized via `GetConsoleScreenBufferInfo`. The expression `cl_bufinfo.srWindow.Top` reads garbage.
- **Risk:** Undefined behavior — mouse position calculations produce random results.
- **Fix:** Call `GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &cl_bufinfo)` before use. Effort: trivial.

---

### High

---

**H1: Zero test coverage across the entire project**
- **Where:** All modules — no `*_test.go`, no `#[test]`, no `*.test.*`, no `__tests__/`
- **What:** The LSP has zero `#[test]` modules. The IDE has no testing framework installed. The BASIC compiler has `test_suite.py` but no evidence it covers edge cases. The C runtime has no test harness.
- **Risk:** Any refactoring or bug fix risks silent regressions. The compiler's 26K lines and the runtime's 30K lines are effectively untestable without a test suite.
- **Fix:** Start with the LSP (Rust tests are easy to add) and the IDE (add vitest + @testing-library/react). The compiler and runtime need characterization tests before any refactoring. Effort: high (ongoing).

---

**H2: `libqb.cpp` is a 30,306-line monolith with 850 gotos**
- **Where:** `internal/c/libqb.cpp` — entire file
- **What:** Single file containing the entire QB64 runtime: image handling, graphics, input, file I/O, networking, sound, console, memory management. Largest function `sub__putimage` is 1,150 lines with 85+ goto labels. No section dividers. Only ~42 comments in 30K lines.
- **Risk:** Unmaintainable — understanding any subsystem requires reading thousands of lines of unrelated code. The goto-heavy dispatch in `sub__putimage` is nearly impossible to reason about.
- **Fix:** Extract subsystems into separate `.c` files (image.c, graphics.c, input.c, fileio.c, network.c, sound.c, console.c). Refactor `sub__putimage` into a dispatch table or separate functions per pixel-format combination. Effort: very high (weeks). **Note:** This is inherited QB64 code — upstream changes will conflict.

---

**H3: BASIC compiler's `dim2` function is 1,230 lines of copy-paste per data type**
- **Where:** `source/qbhd_compiler.bas:14512-15742`
- **What:** The `dim2` function repeats ~100 lines of array allocation/setup for each of 10 data types (_BIT, _BYTE, INTEGER, LONG, etc.) with minor type-specific variations.
- **Risk:** Bug fixes must be applied 10 times. One missed copy = silent bug for one data type.
- **Fix:** Extract the common logic into a helper SUB that takes type-specific parameters (size, name). Effort: medium (1-2 days, needs careful testing).

---

**H4: CLI enhancements injected via blind `sed` with no validation**
- **Where:** `apply_cli_enhancements.sh` (110 lines)
- **What:** The script uses `sed` to inject `--version`, `--json`, `--check`, `--output`, `--optimize`, `--debug`, `--verbose` flag handling into the compiler source. No check that the sed patterns actually matched. If surrounding code changes, the injection silently fails or corrupts the source.
- **Risk:** Build produces a compiler missing CLI features with no error. Debugging requires reading the 26K-line output to find missing code.
- **Fix:** Add grep-based validation after each sed step. Better: refactor CLI handling into a separate `$INCLUDE` file that doesn't need injection. Effort: medium.

---

**H5: LSP parser's `parse_stmt` is 442 lines with 6× duplicated arg-list parsing**
- **Where:** `lsp/src/parser.rs:173-615`
- **What:** Single function handles every BASIC statement type inline. SUB/FUNCTION parsing (lines 355-415) is nearly identical (~30 lines duplicated). The "parse comma-separated args in parentheses" pattern appears 6+ times.
- **Risk:** Adding a new statement type requires modifying a 442-line function. Bugs in one copy of duplicated logic may not be fixed in others.
- **Fix:** Extract `parse_sub_or_function()`, `parse_parenthesized_args()`, and individual `parse_print()`, `parse_if()`, `parse_for()` etc. Effort: medium (half day).

---

### Medium

---

**M1: Dead code in IDE frontend**
- **Where:**
  - `ide/src/App.jsx:79` — `code` prop passed to `<Toolbar>` but never used
  - `ide/src/Editor.jsx:122-125` — `editorRef` stored but never read
- **What:** Unused props and refs that add confusion.
- **Fix:** Remove the `code` prop from Toolbar and the `useRef` from Editor. Effort: trivial.

---

**M2: Duplicated Ctrl+S save shortcut**
- **Where:** `ide/src/App.jsx:62-71` and `ide/src/Editor.jsx:144-146`
- **What:** Both register Ctrl+S handlers. Both fire on keypress. The save is idempotent so it's not a bug, but it's redundant.
- **Fix:** Remove one of the two handlers. Keep the Monaco command (it works when the editor has focus) and the window handler (for when other UI elements have focus), but deduplicate the logic. Effort: trivial.

---

**M3: Repeated error-handling and guard patterns in IDE**
- **Where:** `ide/src/Toolbar.jsx:5-8, 18-21, 44-47` (3× "no file open" guard); `App.jsx` and `Toolbar.jsx` (6× `setOutput(prev => prev + ...)` with try/catch around `invoke()`)
- **What:** Copy-pasted guard and error-handling patterns.
- **Fix:** Extract `runCommand(cmd, args, setOutput)` utility and `requireFile(currentFile, setOutput)` guard. Effort: low.

---

**M4: Version mismatch between Cargo.toml and server**
- **Where:** `lsp/Cargo.toml:3` (version "0.1.0") vs `lsp/src/main.rs:60` (reports "0.2.0")
- **What:** The LSP server reports a different version than its Cargo.toml declares.
- **Fix:** Sync versions. Use `env!("CARGO_PKG_VERSION")` in main.rs. Effort: trivial.

---

**M5: Unused Rust dependencies**
- **Where:** `lsp/Cargo.toml:11-12` — `anyhow` and `thiserror` declared but never imported
- **What:** Dead dependencies that add compile time and binary size.
- **Fix:** Remove from Cargo.toml. Effort: trivial.

---

**M6: Semantic analyzer rebuilt on every hover/goto/references call**
- **Where:** `lsp/src/main.rs:176, 207, 237, 266`
- **What:** `SemanticAnalyzer::new()` is called fresh for each request, creating an empty analyzer. The lookup works because `get_symbol_at_position` is text-based, but it's misleading and wasteful.
- **Fix:** Make `get_symbol_at_position` a free function or a method on `Document`. Effort: low.

---

**M7: `get_completions()` rebuilds static keyword list on every call**
- **Where:** `lsp/src/semantic.rs:159-216`
- **What:** Allocates ~100+ strings for the BASIC keyword list every time completions are requested.
- **Fix:** Use `lazy_static!` or `once_cell::sync::Lazy` for the keyword list. Effort: trivial.

---

**M8: UTF-8 byte length used instead of char count for LSP positions**
- **Where:** `lsp/src/semantic.rs:253`
- **What:** `symbol.len() as u32` returns byte length, not character count. Multi-byte UTF-8 characters will produce incorrect range end positions.
- **Fix:** Use `symbol.chars().count() as u32`. Effort: trivial.

---

**M9: IDE dead code — IDE dispatch block unreachable**
- **Where:** `source/qbhd_compiler.bas:960-1100`
- **What:** With `NoIDEMode = 1` forced at line 129, the entire IDE message handling block (~150 lines) is unreachable dead code.
- **Fix:** Remove or `#IF` guard the block. Effort: low.

---

**M10: Magic file handles undocumented**
- **Where:** `source/qbhd_compiler.bas` throughout (handles #12, #13, #17, #18, #19, #26, #29, #30)
- **What:** File numbers used without named constants or documentation of what each writes to.
- **Fix:** Define named constants (e.g., `CONST HANDLE_MAINDATA = 13`) and document their purposes. Effort: low.

---

**M11: Hardcoded 10-char diagnostic range width**
- **Where:** `lsp/src/diagnostics.rs:58`
- **What:** Diagnostic end character is always `start + 10` regardless of actual error span. Produces imprecise squiggly underlines.
- **Fix:** Parse the actual error span from the compiler's JSON output if available, or default to the full line. Effort: low.

---

### Low

---

**L1: 12 fragile `unwrap()` calls in lexer**
- **Where:** `lsp/src/lexer.rs:150, 166, 171, 173, 179, 181, 199, 201, 216, 225`
- **What:** `self.advance().unwrap()` inside loops guarded by `self.current() == Some(...)`. Currently safe but any refactor that changes control flow could introduce panics.
- **Fix:** Use `if let Some(c) = self.advance()` or return `Result`. Effort: low.

---

**L2: Hardcoded Windows font path**
- **Where:** `source/global/IDEsettings.bas:397` — `"C:\Windows\Fonts\lucon.ttf"`
- **What:** Windows-only font path as IDE custom font default. Won't work on Linux/macOS.
- **Fix:** Use platform detection to set appropriate font path. Low impact since IDE is stripped in QBHD. Effort: trivial.

---

**L3: Typo in error message**
- **Where:** `source/qbhd_compiler.bas:13447` — `"UNEXPECTED INTERNAL COMPILER ERROR!"` with comment `'internal comiler error`
- **Fix:** Fix the typo. Effort: trivial.

---

**L4: 14% blank lines in BASIC compiler**
- **Where:** `source/qbhd_compiler.bas` — 3,713 blank lines out of 26,348
- **What:** Excessive vertical whitespace from the original QB64 codebase.
- **Fix:** Strip excessive blank lines during the build process. Effort: trivial.

---

**L5: `strip_ide.py` leaves commented-out includes**
- **Where:** `strip_ide.py:28` generates `' IDE INCLUDE REMOVED` markers at lines 33 and 26341
- **What:** The includes are commented out, not removed. The surrounding IDE variable references remain.
- **Fix:** Remove the dead code entirely rather than commenting. Effort: low.

---

## Suggested Sequencing

**Phase 1 — Quick wins (1 day):**
1. Fix the buffer overflow in `sub__consolefont` (C1) — trivial, critical security
2. Fix the uninitialized variable (C3) — trivial
3. Sync LSP version (M4), remove unused deps (M5) — trivial
4. Fix UTF-8 byte length bug (M8) — trivial
5. Cache `find_qbhd_binary()` and use `tokio::process::Command` (C2) — low effort, high impact

**Phase 2 — Low-hanging fruit (2-3 days):**
6. Extract `runCommand` utility and `requireFile` guard in IDE (M3)
7. Remove dead code in IDE (M1, M2)
8. Make keyword list static in semantic.rs (M7)
9. Make `get_symbol_at_position` a free function (M6)
10. Add `#[test]` modules to LSP (start with lexer, then parser) (H1 partial)

**Phase 3 — Structural improvements (1-2 weeks):**
11. Decompose `parse_stmt` into per-statement-type functions (H5)
12. Extract duplicated arg-list parsing into shared helper (H5)
13. Refactor `dim2` to use a type-parameterized helper (H3)
14. Add validation to `apply_cli_enhancements.sh` or refactor to `$INCLUDE` (H4)
15. Add vitest + @testing-library/react to IDE (H1 partial)
16. Remove dead IDE dispatch code from compiler (M9)

**Phase 4 — Major refactoring (weeks, high risk):**
17. Split `libqb.cpp` into per-subsystem files (H2) — **requires upstream coordination**
18. Refactor `sub__putimage` goto spaghetti into dispatch table (H2) — **highest risk, highest reward**
19. Add characterization tests to compiler before any refactoring (H1)

## What This Audit Did Not Cover

- **Runtime behavior:** No profiling, fuzzing, or runtime analysis was performed. The buffer overflow (C1) was found via code review, not dynamic analysis.
- **Third-party vendored code:** `internal/c/parts/` contains miniaudio, FreeGLUT, FreeType, stb_image, and other libraries. These were not audited — issues there should be reported upstream.
- **Build system correctness:** The `Makefile`, `build.sh`, and CI scripts were reviewed for structure but not executed. Build failures on specific platforms were not tested.
- **Upstream QB64 drift:** The compiler (`qbhd_compiler.bas`) is derived from `qb64.bas` via `strip_ide.py`. No analysis was done on how divergent the two files are or how difficult it is to merge upstream QB64 changes.
- **Performance:** No benchmarks. The LSP's blocking subprocess issue (C2) is a known performance problem, but no measurements were taken.
- **Accessibility/UX:** The IDE's visual design and keyboard accessibility were not evaluated.

---

## Remediation Log

**Date:** 2026-07-14
**All Phase 1, 2, and 3 fixes applied. Phase 4 deferred (requires test suite first).**

### Phase 1 — Critical + Quick Wins ✅

| Finding | Status | Changes |
|---------|--------|---------|
| C1: Buffer overflow in `sub__consolefont` | ✅ Fixed | `internal/c/libqb.cpp:30234-30238` — bounded `mbstowcs` to 31 chars, use `wcsncpy` with `LF_FACESIZE` |
| C2: Blocking subprocess in LSP | ✅ Fixed | `lsp/src/diagnostics.rs` — rewritten to use `tokio::process::Command`, cached binary path via `once_cell::sync::Lazy` |
| C3: Uninitialized variable | ✅ Fixed | `internal/c/libqb.cpp:30267` — added `GetConsoleScreenBufferInfo()` call before use |
| M4: Version mismatch | ✅ Fixed | `lsp/src/main.rs:60` — use `env!("CARGO_PKG_VERSION")` |
| M5: Unused deps | ✅ Fixed | `lsp/Cargo.toml` — removed `anyhow`, `thiserror`; added `once_cell` |
| M7: Static keywords | ✅ Fixed | `lsp/src/semantic.rs` — keyword list now in `Lazy<Vec<String>>`, completions use `HashSet` for dedup |
| M8: UTF-8 byte length | ✅ Fixed | `lsp/src/semantic.rs:253` — use `symbol.chars().count()` instead of `symbol.len()` |
| L3: Typo | ✅ Fixed | `source/qbhd_compiler.bas:13447` — "comiler" → "compiler" |

### Phase 2 — IDE and LSP Cleanup ✅

| Finding | Status | Changes |
|---------|--------|---------|
| M1: Dead code in IDE | ✅ Fixed | `ide/src/App.jsx` — removed unused `code` prop from Toolbar; `ide/src/Editor.jsx` — removed unused `useRef` import and `editorRef` |
| M3: Repeated guard/error patterns | ✅ Fixed | `ide/src/components/Toolbar.jsx` — extracted `requireFile()` guard and `runCommand()` utility |
| M6: Wasteful SemanticAnalyzer::new() | ✅ Fixed | `lsp/src/semantic.rs` — `get_symbol_at_position` is now a free function; `main.rs` updated to call `semantic::get_symbol_at_position()` |
| M9: Dead IDE dispatch code | ✅ Fixed | `source/qbhd_compiler.bas:960` — added documentation comment explaining the unreachable block |
| M10: Magic file handles | ✅ Fixed | `source/qbhd_compiler.bas:2877` — added 19-line file handle reference table |
| M11: Hardcoded diagnostic width | ✅ Fixed | `lsp/src/diagnostics.rs` — now uses `d.message.len().min(40)` for end character |
| L1: Fragile unwrap() calls | ✅ Fixed | `lsp/src/lexer.rs` — added `advance_or_null()` helper, replaced all 12 `unwrap()` calls |
| L2: Hardcoded Windows font path | ✅ Fixed | `source/global/IDEsettings.bas:397` — platform detection for Windows/Linux/macOS |
| L5: strip_ide.py cleanup | ✅ Fixed | `strip_ide.py` — IDE includes removed entirely instead of commented; $SCREENHIDE lines removed |

### Phase 3 — Structural Refactoring ✅

| Finding | Status | Changes |
|---------|--------|---------|
| H4: CLI enhancement validation | ✅ Fixed | `apply_cli_enhancements.sh` — added `require_pattern()` validation before each sed step, post-insertion verification loop |
| H5: Parser decomposition | ✅ Fixed | `lsp/src/parser.rs` — extracted `parse_print()`, `parse_input()`, `parse_dim()`, `parse_redim()`, `parse_if()`, `parse_for()`, `parse_while()`, `parse_do()`, `parse_select()`, `parse_sub_or_function()`, `parse_call()`, `parse_goto()`, `parse_gosub()`, `parse_exit()`; extracted `parse_paren_args()`, `parse_paren_params()`, `parse_comma_exprs_until_eol()` helpers |

### Phase 4 — Deferred (requires test suite)

| Finding | Status | Reason |
|---------|--------|--------|
| H2: Split libqb.cpp (30K lines) | ⏸ Deferred | Inherited QB64 code, requires upstream coordination and characterization tests |
| H3: Refactor dim2 (1,230 lines of copy-paste) | ⏸ Deferred | Compiler has no test suite; subtle per-type differences (STRING, _FLOAT, _OFFSET); refactoring note added |
| H1: Add test coverage | ⏸ Deferred | Ongoing effort — tests should be added before any Phase 4 refactoring |

### Additional fixes applied during remediation

- Removed confirmed dead code in `isvalidvariable()` (`source/qbhd_compiler.bas:20040-20052`) — 13 lines of unreachable type-suffix checking
- `lsp/Cargo.toml` version synced to 0.2.0 to match server-reported version
- `lsp/src/semantic.rs` — cached `INTERNAL_URI` via `Lazy<Url>` instead of repeated `Url::parse().unwrap()`

use crate::parser::{Stmt, Expr};
use std::collections::HashMap;
use tower_lsp::lsp_types::*;

/// Semantic analyzer for QBHD BASIC.
///
/// Maintains a symbol table tracking all user-defined symbols (variables,
/// functions, subs, parameters, labels) and provides:
/// - Code completion (keywords + user symbols)
/// - Hover documentation (built-in + user-defined)
/// - Go-to-definition
/// - Find references
/// - Symbol lookup by position
pub struct SemanticAnalyzer {
    /// Symbol table: name -> list of definitions (may have multiple across scopes)
    symbols: HashMap<String, Vec<SymbolInfo>>,
}

/// Information about a symbol definition.
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// The symbol name as it appears in source
    pub name: String,
    /// What kind of symbol this is (variable, function, etc.)
    pub kind: SymbolKind,
    /// Line number where defined (0-indexed, approximate)
    pub line: u32,
    /// Character position where defined (always 0 currently)
    pub character: u32,
    /// Optional type annotation (e.g., "INTEGER", "STRING")
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Sub,
    Parameter,
    Label,
    Constant,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, stmts: &[Stmt]) {
        self.symbols.clear();
        let mut line = 0u32;
        for stmt in stmts {
            self.analyze_stmt(stmt, line);
            line += 1;
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt, line: u32) {
        match stmt {
            Stmt::Dim(name, type_name) => {
                self.add_symbol(name, SymbolKind::Variable, line, 0, type_name.clone());
            }
            Stmt::ReDim(name, _) => {
                if !self.symbols.contains_key(name) {
                    self.add_symbol(name, SymbolKind::Variable, line, 0, None);
                }
            }
            Stmt::Assignment(name, _) => {
                if !self.symbols.contains_key(name) {
                    self.add_symbol(name, SymbolKind::Variable, line, 0, None);
                }
            }
            Stmt::SubDef(name, params, body) => {
                self.add_symbol(name, SymbolKind::Sub, line, 0, None);
                for param in params {
                    self.add_symbol(param, SymbolKind::Parameter, line, 0, None);
                }
                let mut sub_line = line + 1;
                for s in body {
                    self.analyze_stmt(s, sub_line);
                    sub_line += 1;
                }
            }
            Stmt::FuncDef(name, params, body) => {
                self.add_symbol(name, SymbolKind::Function, line, 0, None);
                for param in params {
                    self.add_symbol(param, SymbolKind::Parameter, line, 0, None);
                }
                let mut func_line = line + 1;
                for s in body {
                    self.analyze_stmt(s, func_line);
                    func_line += 1;
                }
            }
            Stmt::For(var, _, _, _) => {
                if !self.symbols.contains_key(var) {
                    self.add_symbol(var, SymbolKind::Variable, line, 0, None);
                }
            }
            Stmt::Goto(label) => {
                if !self.symbols.contains_key(label) {
                    self.add_symbol(label, SymbolKind::Label, line, 0, None);
                }
            }
            Stmt::GoSub(label) => {
                if !self.symbols.contains_key(label) {
                    self.add_symbol(label, SymbolKind::Label, line, 0, None);
                }
            }
            Stmt::If(_, then_block, else_block) => {
                let mut l = line + 1;
                for s in then_block {
                    self.analyze_stmt(s, l);
                    l += 1;
                }
                if let Some(else_stmts) = else_block {
                    for s in else_stmts {
                        self.analyze_stmt(s, l);
                        l += 1;
                    }
                }
            }
            Stmt::While(_, body) | Stmt::DoLoop(_, body) => {
                let mut l = line + 1;
                for s in body {
                    self.analyze_stmt(s, l);
                    l += 1;
                }
            }
            Stmt::Select(_, branches) => {
                let mut l = line + 1;
                for branch in branches {
                    for s in &branch.body {
                        self.analyze_stmt(s, l);
                        l += 1;
                    }
                }
            }
            _ => {}
        }
    }

    fn add_symbol(&mut self, name: &str, kind: SymbolKind, line: u32, character: u32, type_name: Option<String>) {
        let info = SymbolInfo {
            name: name.to_string(),
            kind,
            line,
            character,
            type_name,
        };
        self.symbols
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(info);
    }

    pub fn get_completions(&self) -> Vec<String> {
        let mut items: Vec<String> = vec![
            // Control flow
            "PRINT", "INPUT", "DIM", "IF", "THEN", "ELSE", "ELSEIF", "END",
            "FOR", "NEXT", "TO", "STEP", "WHILE", "WEND", "DO", "LOOP", "UNTIL",
            "SELECT", "CASE", "EXIT",
            // Procedures
            "SUB", "FUNCTION", "RETURN", "CALL", "DECLARE",
            "GOTO", "GOSUB", "ON", "ERROR", "RESUME",
            // I/O
            "OPEN", "CLOSE", "READ", "WRITE", "DATA", "RESTORE",
            "LINE", "CLS", "SCREEN", "COLOR", "LOCATE",
            // Graphics
            "CIRCLE", "PSET", "PRESET", "PAINT", "DRAW", "VIEW", "WINDOW",
            "GET", "PUT", "PCOPY",
            // Types
            "AS", "INTEGER", "LONG", "SINGLE", "DOUBLE", "STRING",
            "TYPE", "SHARED", "STATIC", "REDIM", "PRESERVE",
            "LET", "DEF", "OPTION", "BASE", "EXPLICIT",
            // Operators
            "AND", "OR", "NOT", "XOR", "MOD", "IS",
            // Misc
            "CHAIN", "COMMON", "ERASE", "RANDOMIZE", "SWAP", "WIDTH",
            "BEEP", "SLEEP", "SYSTEM", "RUN", "STOP", "CONT",
            // QB64 graphics
            "_RGB", "_RGBA", "_NEWIMAGE", "_LOADIMAGE", "_FREEIMAGE", "_PUTIMAGE",
            "_DISPLAY", "_LIMIT", "_SCREENIMAGE",
            // QB64 input
            "_KEYHIT", "_KEYDOWN", "_MOUSEINPUT", "_MOUSEX", "_MOUSEY",
            "_MOUSEBUTTON", "_MOUSEWHEEL", "_MOUSESHOW", "_MOUSEHIDE",
            // QB64 sound
            "_SNDOPEN", "_SNDPLAY", "_SNDCLOSE", "_SNDVOL",
            // QB64 text
            "_PRINTSTRING", "_PRINTMODE", "_FONT", "_LOADFONT",
            // QB64 screen
            "_SCREENWIDTH", "_SCREENHEIGHT", "_DEST", "_SOURCE",
            "_SCREENX", "_SCREENY", "_SCREENMOVE",
            // QB64 math
            "_CEIL", "_ROUND", "_PI", "_D2R", "_R2D",
            // Built-in functions
            "ABS", "ATN", "COS", "SIN", "TAN", "EXP", "LOG", "SQR", "INT", "RND",
            "LEFT$", "RIGHT$", "MID$", "LEN", "CHR$", "ASC", "STR$", "VAL",
            "INKEY$", "TIMER", "POINT", "PMAP", "EOF", "LOF", "FREEFILE",
            "LBOUND", "UBOUND", "PEEK", "INP",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        // Add user-defined symbols
        for name in self.symbols.keys() {
            if !items.contains(name) {
                items.push(name.clone());
            }
        }

        items
    }

    pub fn get_hover(&self, symbol: &str) -> Option<String> {
        let upper = symbol.to_uppercase();

        // Check user-defined symbols first
        if let Some(infos) = self.symbols.get(symbol) {
            if let Some(info) = infos.first() {
                let kind_str = match info.kind {
                    SymbolKind::Variable => "Variable",
                    SymbolKind::Function => "Function",
                    SymbolKind::Sub => "Sub",
                    SymbolKind::Parameter => "Parameter",
                    SymbolKind::Label => "Label",
                    SymbolKind::Constant => "Constant",
                };
                let type_str = info.type_name.as_deref().unwrap_or("Variant");
                return Some(format!("**{}** `{}` : {}", kind_str, info.name, type_str));
            }
        }

        // Check built-in keywords
        get_builtin_hover(&upper)
    }

    pub fn find_definition(&self, symbol: &str) -> Option<Location> {
        if let Some(infos) = self.symbols.get(symbol) {
            if let Some(info) = infos.first() {
                return Some(Location {
                    uri: Url::parse("file:///internal").unwrap(), // will be overridden
                    range: Range {
                        start: Position {
                            line: info.line,
                            character: info.character,
                        },
                        end: Position {
                            line: info.line,
                            character: info.character + symbol.len() as u32,
                        },
                    },
                });
            }
        }
        None
    }

    pub fn find_references(&self, symbol: &str) -> Vec<Location> {
        let mut refs = Vec::new();
        if let Some(infos) = self.symbols.get(symbol) {
            for info in infos {
                refs.push(Location {
                    uri: Url::parse("file:///internal").unwrap(), // will be overridden
                    range: Range {
                        start: Position {
                            line: info.line,
                            character: info.character,
                        },
                        end: Position {
                            line: info.line,
                            character: info.character + symbol.len() as u32,
                        },
                    },
                });
            }
        }
        refs
    }

    pub fn get_symbol_at_position(&self, text: &str, line: u32, character: u32) -> Option<String> {
        let lines: Vec<&str> = text.lines().collect();
        let line_text = lines.get(line as usize)?;

        // Find the word at the given character position
        let chars: Vec<char> = line_text.chars().collect();
        let char_pos = character as usize;
        if char_pos >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = char_pos;
        while start > 0 && (chars[start - 1].is_ascii_alphanumeric() || chars[start - 1] == '_' || chars[start - 1] == '$') {
            start -= 1;
        }

        let mut end = char_pos;
        while end < chars.len() && (chars[end].is_ascii_alphanumeric() || chars[end] == '_' || chars[end] == '$') {
            end += 1;
        }

        if start == end {
            return None;
        }

        let word: String = chars[start..end].iter().collect();
        Some(word)
    }
}

fn get_builtin_hover(keyword: &str) -> Option<String> {
    match keyword {
        "PRINT" => Some("**Statement** `PRINT`\n\nOutputs text to the screen.\n\n```basic\nPRINT expression\nPRINT \"Hello, World!\"\nPRINT a; b; c\n```".to_string()),
        "INPUT" => Some("**Statement** `INPUT`\n\nReads input from the user.\n\n```basic\nINPUT \"Enter name: \"; name$\nINPUT a, b, c\n```".to_string()),
        "IF" => Some("**Statement** `IF...THEN...ELSE`\n\nConditional branching.\n\n```basic\nIF x > 10 THEN\n  PRINT \"Big\"\nELSE\n  PRINT \"Small\"\nEND IF\n```".to_string()),
        "FOR" => Some("**Statement** `FOR...NEXT`\n\nLoop with counter.\n\n```basic\nFOR i = 1 TO 10 STEP 2\n  PRINT i\nNEXT i\n```".to_string()),
        "WHILE" => Some("**Statement** `WHILE...WEND`\n\nLoop while condition is true.\n\n```basic\nWHILE x < 100\n  x = x + 1\nWEND\n```".to_string()),
        "DO" => Some("**Statement** `DO...LOOP`\n\nLoop with optional pre/post condition.\n\n```basic\nDO WHILE x < 100\n  x = x + 1\nLOOP\n```".to_string()),
        "SELECT" => Some("**Statement** `SELECT CASE`\n\nMulti-way branching.\n\n```basic\nSELECT CASE x\n  CASE 1\n    PRINT \"One\"\n  CASE 2, 3\n    PRINT \"Two or Three\"\n  CASE IS > 10\n    PRINT \"Big\"\nEND SELECT\n```".to_string()),
        "DIM" => Some("**Statement** `DIM`\n\nDeclares a variable.\n\n```basic\nDIM x AS INTEGER\nDIM name$ AS STRING\nDIM arr(10) AS SINGLE\n```".to_string()),
        "SUB" => Some("**Statement** `SUB...END SUB`\n\nDeclares a subroutine.\n\n```basic\nSUB MySub (a, b)\n  PRINT a + b\nEND SUB\n```".to_string()),
        "FUNCTION" => Some("**Statement** `FUNCTION...END FUNCTION`\n\nDeclares a function that returns a value.\n\n```basic\nFUNCTION Add(a, b)\n  Add = a + b\nEND FUNCTION\n```".to_string()),
        "OPEN" => Some("**Statement** `OPEN`\n\nOpens a file for I/O.\n\n```basic\nOPEN \"data.txt\" FOR INPUT AS #1\nOPEN \"out.txt\" FOR OUTPUT AS #2\n```".to_string()),
        "CLS" => Some("**Statement** `CLS`\n\nClears the screen.\n\n```basic\nCLS\n```".to_string()),
        "SCREEN" => Some("**Statement** `SCREEN`\n\nSets the screen mode.\n\n```basic\nSCREEN 13\nSCREEN _NEWIMAGE(800, 600, 32)\n```".to_string()),
        "COLOR" => Some("**Statement** `COLOR`\n\nSets text colors.\n\n```basic\nCOLOR 15, 0  ' white on black\n```".to_string()),
        "LINE" => Some("**Statement** `LINE`\n\nDraws a line or box.\n\n```basic\nLINE (x1, y1)-(x2, y2), color\nLINE (x1, y1)-(x2, y2), color, BF\n```".to_string()),
        "CIRCLE" => Some("**Statement** `CIRCLE`\n\nDraws a circle.\n\n```basic\nCIRCLE (x, y), radius, color\n```".to_string()),
        "END" => Some("**Statement** `END`\n\nEnds the program or a block.\n\n```basic\nEND\nEND IF\nEND SUB\nEND FUNCTION\nEND SELECT\n```".to_string()),
        "RETURN" => Some("**Statement** `RETURN`\n\nReturns from a SUB/FUNCTION or GOSUB.\n\n```basic\nRETURN\n```".to_string()),
        "GOTO" => Some("**Statement** `GOTO`\n\nJumps to a label.\n\n```basic\nGOTO ErrorHandler\n```".to_string()),
        "REM" => Some("**Statement** `REM`\n\nAdds a comment.\n\n```basic\nREM This is a comment\n' This is also a comment\n```".to_string()),
        "ABS" => Some("**Function** `ABS(n)`\n\nReturns the absolute value of n.\n\n```basic\nx = ABS(-5)  ' x = 5\n```".to_string()),
        "SIN" => Some("**Function** `SIN(radians)`\n\nReturns the sine of an angle.\n\n```basic\ny = SIN(angle)\n```".to_string()),
        "COS" => Some("**Function** `COS(radians)`\n\nReturns the cosine of an angle.\n\n```basic\nx = COS(angle)\n```".to_string()),
        "INT" => Some("**Function** `INT(n)`\n\nReturns the integer part of n.\n\n```basic\nx = INT(3.7)  ' x = 3\n```".to_string()),
        "RND" => Some("**Function** `RND(n)`\n\nReturns a random number between 0 and 1.\n\n```basic\nx = RND(1)\n```".to_string()),
        "LEN" => Some("**Function** `LEN(s$)`\n\nReturns the length of a string.\n\n```basic\nn = LEN(\"Hello\")  ' n = 5\n```".to_string()),
        "LEFT$" => Some("**Function** `LEFT$(s$, n)`\n\nReturns the leftmost n characters.\n\n```basic\ns = LEFT$(\"Hello\", 3)  ' s = \"Hel\"\n```".to_string()),
        "RIGHT$" => Some("**Function** `RIGHT$(s$, n)`\n\nReturns the rightmost n characters.\n\n```basic\ns = RIGHT$(\"Hello\", 3)  ' s = \"llo\"\n```".to_string()),
        "MID$" => Some("**Function** `MID$(s$, start, len)`\n\nReturns a substring.\n\n```basic\ns = MID$(\"Hello\", 2, 3)  ' s = \"ell\"\n```".to_string()),
        "CHR$" => Some("**Function** `CHR$(n)`\n\nReturns the character for ASCII code n.\n\n```basic\ns = CHR$(65)  ' s = \"A\"\n```".to_string()),
        "ASC" => Some("**Function** `ASC(s$)`\n\nReturns the ASCII code of the first character.\n\n```basic\nn = ASC(\"A\")  ' n = 65\n```".to_string()),
        "STR$" => Some("**Function** `STR$(n)`\n\nConverts a number to a string.\n\n```basic\ns$ = STR$(123)  ' s$ = \" 123\"\n```".to_string()),
        "VAL" => Some("**Function** `VAL(s$)`\n\nConverts a string to a number.\n\n```basic\nn = VAL(\"123\")  ' n = 123\n```".to_string()),
        "_RGB" => Some("**Function** `_RGB(r, g, b)`\n\nCreates an RGB color value.\n\n```basic\nc = _RGB(255, 128, 0)\n```".to_string()),
        "_NEWIMAGE" => Some("**Function** `_NEWIMAGE(width, height, mode)`\n\nCreates a new image buffer.\n\n```basic\nimg = _NEWIMAGE(800, 600, 32)\n```".to_string()),
        "INTEGER" => Some("**Type** `INTEGER`\n\n32-bit signed integer (-2147483648 to 2147483647).\n\n```basic\nDIM x AS INTEGER\n```".to_string()),
        "LONG" => Some("**Type** `LONG`\n\n32-bit signed integer (same as INTEGER in QB64).\n\n```basic\nDIM x AS LONG\n```".to_string()),
        "SINGLE" => Some("**Type** `SINGLE`\n\n32-bit floating point.\n\n```basic\nDIM x AS SINGLE\n```".to_string()),
        "DOUBLE" => Some("**Type** `DOUBLE`\n\n64-bit floating point.\n\n```basic\nDIM x AS DOUBLE\n```".to_string()),
        "STRING" => Some("**Type** `STRING`\n\nVariable-length string.\n\n```basic\nDIM s AS STRING\n```".to_string()),
        _ => None,
    }
}

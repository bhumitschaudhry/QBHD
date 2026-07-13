#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Operator(String),
    Comment(String),
    Colon,
    Newline,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        Lexer {
            input,
            chars,
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.current();
        self.pos += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current() {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('\r') => {
                self.advance();
                if self.current() == Some('\n') {
                    self.advance();
                }
                Token::Newline
            }
            Some(':') => {
                self.advance();
                Token::Colon
            }
            Some('"') => self.read_string(),
            Some('\'') => self.read_single_quote_comment(),
            Some(c) if c.is_ascii_digit() => self.read_number(),
            Some('&') => self.read_hex_octal(),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.read_identifier_or_keyword(),
            Some(c) => self.read_operator(c),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening "
        let mut s = String::new();
        while let Some(c) = self.current() {
            if c == '"' {
                self.advance();
                // Handle doubled quotes ("") as escaped quote
                if self.current() == Some('"') {
                    s.push('"');
                    self.advance();
                } else {
                    break;
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        Token::String(s)
    }

    fn read_single_quote_comment(&mut self) -> Token {
        self.advance(); // skip '
        let mut comment = String::new();
        while let Some(c) = self.current() {
            if c == '\n' || c == '\r' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        Token::Comment(comment)
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                num.push(c);
                self.advance();
            } else if c == '.' && !has_dot && !has_exp {
                has_dot = true;
                num.push(c);
                self.advance();
            } else if (c == 'e' || c == 'E') && !has_exp {
                has_exp = true;
                num.push(c);
                self.advance();
                // Handle optional sign in exponent
                if self.current() == Some('+') || self.current() == Some('-') {
                    num.push(self.advance().unwrap());
                }
            } else if c == '#' || c == '!' || c == '%' || c == '&' {
                // Type suffixes for numbers
                num.push(c);
                self.advance();
                break;
            } else {
                break;
            }
        }
        Token::Number(num)
    }

    fn read_hex_octal(&mut self) -> Token {
        let mut num = String::new();
        num.push(self.advance().unwrap()); // &

        match self.current() {
            Some('h') | Some('H') => {
                num.push(self.advance().unwrap());
                while let Some(c) = self.current() {
                    if c.is_ascii_hexdigit() {
                        num.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
            }
            Some('o') | Some('O') => {
                num.push(self.advance().unwrap());
                while let Some(c) = self.current() {
                    if c >= '0' && c <= '7' {
                        num.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }
        Token::Number(num)
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut id = String::new();

        // Check for QB64 underscore-prefixed keywords
        if self.current() == Some('_') {
            id.push(self.advance().unwrap());
            while let Some(c) = self.current() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    id.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
            let upper = id.to_uppercase();
            if is_keyword(&upper) || is_qb64_keyword(&upper) {
                return Token::Keyword(upper);
            }
            return Token::Identifier(id);
        }

        while let Some(c) = self.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                id.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // Check for type suffix
        if let Some(suffix) = self.current() {
            if matches!(suffix, '$' | '%' | '!' | '#' | '&') {
                id.push(self.advance().unwrap());
            }
        }

        let upper = id.to_uppercase();
        if is_keyword(&upper) {
            Token::Keyword(upper)
        } else if is_qb64_keyword(&upper) {
            Token::Keyword(upper)
        } else if upper == "REM" {
            // REM comment - rest of line is comment
            let mut comment = String::new();
            while let Some(c) = self.current() {
                if c == '\n' || c == '\r' {
                    break;
                }
                comment.push(c);
                self.advance();
            }
            Token::Comment(comment)
        } else {
            Token::Identifier(id)
        }
    }

    fn read_operator(&mut self, c: char) -> Token {
        self.advance();
        match c {
            '<' => {
                if self.current() == Some('=') {
                    self.advance();
                    Token::Operator("<=".to_string())
                } else if self.current() == Some('>') {
                    self.advance();
                    Token::Operator("<>".to_string())
                } else {
                    Token::Operator("<".to_string())
                }
            }
            '>' => {
                if self.current() == Some('=') {
                    self.advance();
                    Token::Operator(">=".to_string())
                } else {
                    Token::Operator(">".to_string())
                }
            }
            '=' => Token::Operator("=".to_string()),
            '+' => Token::Operator("+".to_string()),
            '-' => Token::Operator("-".to_string()),
            '*' => {
                if self.current() == Some('*') {
                    self.advance();
                    Token::Operator("**".to_string())
                } else {
                    Token::Operator("*".to_string())
                }
            }
            '/' => Token::Operator("/".to_string()),
            '\\' => Token::Operator("\\".to_string()),
            '^' => Token::Operator("^".to_string()),
            '&' => Token::Operator("&".to_string()),
            '(' => Token::Operator("(".to_string()),
            ')' => Token::Operator(")".to_string()),
            ',' => Token::Operator(",".to_string()),
            ';' => Token::Operator(";".to_string()),
            '.' => Token::Operator(".".to_string()),
            '!' => Token::Operator("!".to_string()),
            '#' => Token::Operator("#".to_string()),
            _ => Token::Operator(c.to_string()),
        }
    }

    /// Tokenize the entire input into a Vec of tokens (excluding Eof)
    pub fn tokenize_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "PRINT"
            | "INPUT"
            | "IF"
            | "THEN"
            | "ELSE"
            | "ELSEIF"
            | "END"
            | "FOR"
            | "NEXT"
            | "TO"
            | "STEP"
            | "WHILE"
            | "WEND"
            | "DO"
            | "LOOP"
            | "UNTIL"
            | "SELECT"
            | "CASE"
            | "SUB"
            | "FUNCTION"
            | "RETURN"
            | "CALL"
            | "GOTO"
            | "GOSUB"
            | "ON"
            | "ERROR"
            | "RESUME"
            | "DIM"
            | "REDIM"
            | "PRESERVE"
            | "AS"
            | "INTEGER"
            | "LONG"
            | "SINGLE"
            | "DOUBLE"
            | "STRING"
            | "TYPE"
            | "DECLARE"
            | "LET"
            | "LINE"
            | "CLS"
            | "SCREEN"
            | "COLOR"
            | "LOCATE"
            | "OPEN"
            | "CLOSE"
            | "READ"
            | "WRITE"
            | "DATA"
            | "RESTORE"
            | "AND"
            | "OR"
            | "NOT"
            | "XOR"
            | "MOD"
            | "SHARED"
            | "STATIC"
            | "EXIT"
            | "DEF"
            | "IS"
            | "TAB"
            | "SPC"
            | "OPTION"
            | "BASE"
            | "EXPLICIT"
            | "CHAIN"
            | "COMMON"
            | "ERASE"
            | "RANDOMIZE"
            | "SWAP"
            | "WIDTH"
            | "BEEP"
            | "SLEEP"
            | "SYSTEM"
            | "RUN"
            | "STOP"
            | "CONT"
            | "TRON"
            | "TROFF"
            | "PAINT"
            | "DRAW"
            | "VIEW"
            | "WINDOW"
            | "GET"
            | "PUT"
            | "CIRCLE"
            | "PSET"
            | "PRESET"
            | "PCOPY"
            | "SOUND"
            | "PLAY"
            | "TIMER"
            | "ON"
            | "OFF"
            | "KEY"
            | "MID$"
            | "LSET"
            | "RSET"
            | "FIELD"
            | "MKI$"
            | "MKS$"
            | "MKD$"
            | "MKL$"
            | "CVI"
            | "CVS"
            | "CVD"
            | "CVL"
    )
}

fn is_qb64_keyword(s: &str) -> bool {
    matches!(
        s,
        "_RGB"
            | "_RGBA"
            | "_NEWIMAGE"
            | "_LOADIMAGE"
            | "_FREEIMAGE"
            | "_PUTIMAGE"
            | "_DISPLAY"
            | "_LIMIT"
            | "_KEYHIT"
            | "_KEYDOWN"
            | "_MOUSEINPUT"
            | "_SCREENWIDTH"
            | "_SCREENHEIGHT"
            | "_DEST"
            | "_SOURCE"
            | "_PRINTSTRING"
            | "_PRINTMODE"
            | "_FONT"
            | "_LOADFONT"
            | "_SNDOPEN"
            | "_SNDPLAY"
            | "_SNDCLOSE"
            | "_SNDVOL"
            | "_MOUSEX"
            | "_MOUSEY"
            | "_MOUSEBUTTON"
            | "_MOUSEWHEEL"
            | "_CEIL"
            | "_ROUND"
            | "_MAPUNICODE"
            | "_D2R"
            | "_R2D"
            | "_PI"
            | "_SCREENIMAGE"
            | "_CLIPBOARD$"
            | "_TITLE"
            | "_RESIZE"
            | "_MOUSESHOW"
            | "_MOUSEHIDE"
            | "_AXIS"
            | "_BUTTON"
            | "_DEVICE$"
            | "_NUMDEVICES"
            | "_DEVICES"
            | "_STICK"
            | "_STRIG"
            | "_SCREENX"
            | "_SCREENY"
            | "_SCREENMOVE"
            | "_GLRENDER"
            | "_DELAY"
            | "_MEM"
            | "_MEMNEW"
            | "_MEMFREE"
            | "_MEMGET"
            | "_MEMPUT"
            | "_MEMFILL"
            | "_MEMCOPY"
            | "_MEMELEMENT"
            | "_MEMEXISTS"
    )
}

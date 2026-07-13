use crate::lexer::{Lexer, Token};

/// Statement AST nodes for the BASIC parser.
///
/// Represents all supported BASIC statement types. Each variant contains
/// the statement's operands and nested blocks where applicable.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// `PRINT expr1; expr2, expr3` - Output to screen
    Print(Vec<Expr>),
    /// `INPUT "prompt"; var1, var2` - Read user input
    Input(Option<Expr>, Vec<String>),
    /// `DIM name AS type` - Variable declaration
    Dim(String, Option<String>),
    /// `REDIM [PRESERVE] name` - Array redeclaration
    ReDim(String, bool),
    /// `name = expr` - Variable assignment
    Assignment(String, Expr),
    /// `IF cond THEN ... ELSE ... END IF` - Conditional
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    /// `FOR var = from TO [STEP] ... NEXT` - Counter loop
    For(String, Expr, Expr, Option<Expr>),
    /// `WHILE cond ... WEND` - Pre-condition loop
    While(Expr, Vec<Stmt>),
    /// `DO [WHILE|UNTIL] cond ... LOOP` - Flexible loop
    DoLoop(LoopCondition, Vec<Stmt>),
    /// `SELECT CASE expr ... CASE ... END SELECT` - Multi-way branch
    Select(Expr, Vec<CaseBranch>),
    /// `SUB name(params) ... END SUB` - Subroutine definition
    SubDef(String, Vec<String>, Vec<Stmt>),
    /// `FUNCTION name(params) ... END FUNCTION` - Function definition
    FuncDef(String, Vec<String>, Vec<Stmt>),
    /// `CALL name(args)` or `name(args)` - Subroutine call
    Call(String, Vec<Expr>),
    /// `RETURN` - Return from GOSUB or procedure
    Return,
    /// `GOTO label` - Unconditional jump
    Goto(String),
    /// `GOSUB label` - Subroutine call (legacy)
    GoSub(String),
    /// `EXIT SUB|FUNCTION|FOR|DO` - Early exit from block
    Exit(String),
    /// `'comment` or `REM comment` - Comment text
    Rem(String),
    /// `CLS` - Clear screen
    Cls,
    /// `END` - End program
    End,
    /// `LINE (x1,y1)-(x2,y2), color [,BF]` - Draw line/box
    Line(Vec<Expr>),
    /// `CIRCLE (x,y), radius [,color]` - Draw circle
    Circle(Vec<Expr>),
    /// `PSET (x,y), color` - Set pixel
    Pset(Vec<Expr>),
    /// `SCREEN mode` - Set screen mode
    Screen(Vec<Expr>),
    /// `COLOR fg [,bg]` - Set text colors
    Color(Vec<Expr>),
    /// `LOCATE row, col` - Set cursor position
    Locate(Vec<Expr>),
    /// `OPEN file FOR mode AS #n` - Open file
    Open(Vec<Expr>),
    /// `CLOSE #n` - Close file
    Close(Vec<Expr>),
    /// `GET #n, pos, var` - Read from file
    Get(Vec<Expr>),
    /// `PUT #n, pos, var` - Write to file
    Put(Vec<Expr>),
    /// Standalone expression (e.g., function call as statement)
    Expression(Expr),
    /// Fallback for unrecognized statements
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CaseBranch {
    pub values: Vec<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum LoopCondition {
    While(Expr),
    Until(Expr),
    None,
}

/// Expression AST nodes for the BASIC parser.
///
/// Represents all supported expression types. The Pratt parsing algorithm
/// handles operator precedence for binary expressions.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric literal (42, 3.14, &HFF, &O77)
    Number(String),
    /// String literal ("hello")
    String(String),
    /// Variable reference (x, name$)
    Var(String),
    /// Binary operation (left op right), e.g., a + b, x AND y
    Binary(Box<Expr>, String, Box<Expr>),
    /// Unary operation (op expr), e.g., -x, NOT flag
    Unary(String, Box<Expr>),
    /// Function call (name(args)), e.g., ABS(x), SIN(angle)
    Call(String, Vec<Expr>),
    /// Parenthesized expression ((expr))
    Paren(Box<Expr>),
    /// Array element access (name(indices)), e.g., arr(i, j)
    ArrayAccess(String, Vec<Expr>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn skip_newlines(&mut self) {
        while self.current == Token::Newline {
            self.advance();
        }
    }

    fn expect_keyword(&mut self, kw: &str) -> bool {
        if let Token::Keyword(ref k) = self.current {
            if k == kw {
                self.advance();
                return true;
            }
        }
        false
    }

    fn current_is_keyword(&self, kw: &str) -> bool {
        if let Token::Keyword(ref k) = self.current {
            k == kw
        } else {
            false
        }
    }

    fn current_is_operator(&self, op: &str) -> bool {
        if let Token::Operator(ref o) = self.current {
            o == op
        } else {
            false
        }
    }

    fn is_line_end(&self) -> bool {
        matches!(
            self.current,
            Token::Newline | Token::Eof | Token::Comment(_)
        )
    }

    // === Shared parsing helpers ===

    /// Parse comma-separated expressions until a closing `)`.
    /// Caller must have already consumed the opening `(`.
    fn parse_paren_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        while !self.current_is_operator(")") && self.current != Token::Eof {
            if let Some(expr) = self.parse_expr() {
                args.push(expr);
            }
            if self.current_is_operator(",") {
                self.advance();
            }
        }
        if self.current_is_operator(")") {
            self.advance();
        }
        args
    }

    /// Parse comma-separated expressions until end of line.
    fn parse_comma_exprs_until_eol(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        while !self.is_line_end() {
            if let Some(expr) = self.parse_expr() {
                args.push(expr);
            }
            if self.current_is_operator(",") || self.current_is_operator(";") {
                self.advance();
            }
        }
        args
    }

    /// Parse comma-separated identifier names until closing `)`.
    /// Caller must have already consumed the opening `(`.
    fn parse_paren_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        while !self.current_is_operator(")") && self.current != Token::Eof {
            if let Token::Identifier(p) = &self.current.clone() {
                params.push(p.clone());
                self.advance();
            }
            if self.current_is_operator(",") {
                self.advance();
            }
        }
        if self.current_is_operator(")") {
            self.advance();
        }
        params
    }

    // === Statement parsing (decomposed) ===

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while self.current != Token::Eof {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.skip_newlines();
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match &self.current.clone() {
            Token::Comment(text) => {
                let t = text.clone();
                self.advance();
                Some(Stmt::Rem(t))
            }
            Token::Keyword(kw) => match kw.as_str() {
                "PRINT" | "?" => self.parse_print(),
                "INPUT" => self.parse_input(),
                "DIM" => self.parse_dim(),
                "REDIM" => self.parse_redim(),
                "IF" => self.parse_if(),
                "FOR" => self.parse_for(),
                "WHILE" => self.parse_while(),
                "DO" => self.parse_do(),
                "SELECT" => self.parse_select(),
                "SUB" => self.parse_sub_or_function(StmtEndKeyword::SUB),
                "FUNCTION" => self.parse_sub_or_function(StmtEndKeyword::FUNCTION),
                "CALL" => self.parse_call(),
                "RETURN" => { self.advance(); Some(Stmt::Return) }
                "GOTO" => self.parse_goto(),
                "GOSUB" => self.parse_gosub(),
                "EXIT" => self.parse_exit(),
                "END" => { self.advance(); Some(Stmt::End) }
                "CLS" => { self.advance(); Some(Stmt::Cls) }
                "SCREEN" => { self.advance(); Some(Stmt::Screen(self.parse_comma_exprs_until_eol())) }
                "COLOR" => { self.advance(); Some(Stmt::Color(self.parse_comma_exprs_until_eol())) }
                "LOCATE" => { self.advance(); Some(Stmt::Locate(self.parse_comma_exprs_until_eol())) }
                "LINE" => { self.advance(); Some(Stmt::Line(self.parse_comma_exprs_until_eol())) }
                "CIRCLE" => { self.advance(); Some(Stmt::Circle(self.parse_comma_exprs_until_eol())) }
                "PSET" | "PRESET" => { self.advance(); Some(Stmt::Pset(self.parse_comma_exprs_until_eol())) }
                "OPEN" => { self.advance(); Some(Stmt::Open(self.parse_comma_exprs_until_eol())) }
                "CLOSE" => { self.advance(); Some(Stmt::Close(self.parse_comma_exprs_until_eol())) }
                "GET" => { self.advance(); Some(Stmt::Get(self.parse_comma_exprs_until_eol())) }
                "PUT" => { self.advance(); Some(Stmt::Put(self.parse_comma_exprs_until_eol())) }
                "LET" => { self.advance(); self.parse_assignment_or_call() }
                "STOP" => { self.advance(); Some(Stmt::End) }
                _ => { self.advance(); Some(Stmt::Unknown) }
            },
            Token::Identifier(_) => self.parse_assignment_or_call(),
            Token::Newline => { self.advance(); None }
            _ => { self.advance(); Some(Stmt::Unknown) }
        }
    }

    fn parse_print(&mut self) -> Option<Stmt> {
        self.advance();
        let mut args = Vec::new();
        if !self.is_line_end() {
            args.push(self.parse_expr()?);
            while self.current_is_operator(",") || self.current_is_operator(";") {
                self.advance();
                if !self.is_line_end() {
                    args.push(self.parse_expr()?);
                }
            }
        }
        Some(Stmt::Print(args))
    }

    fn parse_input(&mut self) -> Option<Stmt> {
        self.advance();
        let prompt = if self.current_is_operator(";") || self.current_is_operator(",") {
            None
        } else {
            Some(self.parse_expr()?)
        };
        if self.current_is_operator(";") || self.current_is_operator(",") {
            self.advance();
        }
        let mut vars = Vec::new();
        if let Token::Identifier(name) = &self.current.clone() {
            vars.push(name.clone());
            self.advance();
            while self.current_is_operator(",") {
                self.advance();
                if let Token::Identifier(n) = &self.current.clone() {
                    vars.push(n.clone());
                    self.advance();
                }
            }
        }
        Some(Stmt::Input(prompt, vars))
    }

    fn parse_dim(&mut self) -> Option<Stmt> {
        self.advance();
        if let Token::Identifier(name) = &self.current.clone() {
            let n = name.clone();
            self.advance();
            let mut type_name = None;
            if self.current_is_keyword("AS") {
                self.advance();
                if let Token::Keyword(t) = &self.current.clone() {
                    type_name = Some(t.clone());
                    self.advance();
                }
            }
            Some(Stmt::Dim(n, type_name))
        } else {
            None
        }
    }

    fn parse_redim(&mut self) -> Option<Stmt> {
        self.advance();
        let preserve = self.current_is_keyword("PRESERVE");
        if preserve {
            self.advance();
        }
        if let Token::Identifier(name) = &self.current.clone() {
            let n = name.clone();
            self.advance();
            Some(Stmt::ReDim(n, preserve))
        } else {
            None
        }
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        self.advance();
        let condition = self.parse_expr()?;
        let _ = self.expect_keyword("THEN");
        self.skip_newlines();
        let then_block = self.parse_block_until_else_or_end();
        let else_block = if self.current_is_keyword("ELSE") {
            self.advance();
            self.skip_newlines();
            Some(self.parse_block_until_end_if())
        } else {
            None
        };
        let _ = self.expect_keyword("END");
        let _ = self.expect_keyword("IF");
        Some(Stmt::If(condition, then_block, else_block))
    }

    fn parse_for(&mut self) -> Option<Stmt> {
        self.advance();
        let var = if let Token::Identifier(name) = &self.current.clone() {
            let n = name.clone();
            self.advance();
            n
        } else {
            return None;
        };
        let _ = self.expect_keyword("TO") || self.current_is_operator("=");
        if self.current_is_operator("=") {
            self.advance();
        }
        let from = self.parse_expr()?;
        let _ = self.expect_keyword("TO");
        let to = self.parse_expr()?;
        let step = if self.current_is_keyword("STEP") {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.skip_newlines();
        let body = self.parse_block_until("NEXT");
        let _ = self.expect_keyword("NEXT");
        if let Token::Identifier(_) = &self.current {
            self.advance();
        }
        Some(Stmt::For(var, from, to, step))
    }

    fn parse_while(&mut self) -> Option<Stmt> {
        self.advance();
        let condition = self.parse_expr()?;
        self.skip_newlines();
        let body = self.parse_block_until("WEND");
        let _ = self.expect_keyword("WEND");
        Some(Stmt::While(condition, body))
    }

    fn parse_do(&mut self) -> Option<Stmt> {
        self.advance();
        let condition = if self.current_is_keyword("WHILE") {
            self.advance();
            LoopCondition::While(self.parse_expr()?)
        } else if self.current_is_keyword("UNTIL") {
            self.advance();
            LoopCondition::Until(self.parse_expr()?)
        } else {
            LoopCondition::None
        };
        self.skip_newlines();
        let body = self.parse_block_until("LOOP");
        let _ = self.expect_keyword("LOOP");
        let _ = if self.current_is_keyword("WHILE") {
            self.advance();
            Some(LoopCondition::While(self.parse_expr()?))
        } else if self.current_is_keyword("UNTIL") {
            self.advance();
            Some(LoopCondition::Until(self.parse_expr()?))
        } else {
            None
        };
        Some(Stmt::DoLoop(condition, body))
    }

    fn parse_select(&mut self) -> Option<Stmt> {
        self.advance();
        let _ = self.expect_keyword("CASE");
        let expr = self.parse_expr()?;
        self.skip_newlines();
        let mut branches = Vec::new();
        while self.current_is_keyword("CASE") {
            self.advance();
            let mut values = Vec::new();
            values.push(self.parse_expr()?);
            while self.current_is_operator(",") {
                self.advance();
                values.push(self.parse_expr()?);
            }
            self.skip_newlines();
            let body = self.parse_block_until_select_case();
            branches.push(CaseBranch { values, body });
        }
        let _ = self.expect_keyword("END");
        let _ = self.expect_keyword("SELECT");
        Some(Stmt::Select(expr, branches))
    }

    /// Parse SUB or FUNCTION definition (shared logic).
    fn parse_sub_or_function(&mut self, kind: StmtEndKeyword) -> Option<Stmt> {
        self.advance();
        let name = if let Token::Identifier(n) = &self.current.clone() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        let params = if self.current_is_operator("(") {
            self.advance();
            self.parse_paren_params()
        } else {
            Vec::new()
        };
        self.skip_newlines();
        let end_kw = match kind {
            StmtEndKeyword::SUB => "SUB",
            StmtEndKeyword::FUNCTION => "FUNCTION",
        };
        let body = self.parse_block_until("END");
        let _ = self.expect_keyword("END");
        let _ = self.expect_keyword(end_kw);
        match kind {
            StmtEndKeyword::SUB => Some(Stmt::SubDef(name, params, body)),
            StmtEndKeyword::FUNCTION => Some(Stmt::FuncDef(name, params, body)),
        }
    }

    fn parse_call(&mut self) -> Option<Stmt> {
        self.advance();
        if let Token::Identifier(name) = &self.current.clone() {
            let n = name.clone();
            self.advance();
            let args = if self.current_is_operator("(") {
                self.advance();
                self.parse_paren_args()
            } else {
                Vec::new()
            };
            Some(Stmt::Call(n, args))
        } else {
            None
        }
    }

    fn parse_goto(&mut self) -> Option<Stmt> {
        self.advance();
        if let Token::Identifier(label) = &self.current.clone() {
            let l = label.clone();
            self.advance();
            Some(Stmt::Goto(l))
        } else {
            None
        }
    }

    fn parse_gosub(&mut self) -> Option<Stmt> {
        self.advance();
        if let Token::Identifier(label) = &self.current.clone() {
            let l = label.clone();
            self.advance();
            Some(Stmt::GoSub(l))
        } else {
            None
        }
    }

    fn parse_exit(&mut self) -> Option<Stmt> {
        self.advance();
        if let Token::Keyword(kw) = &self.current.clone() {
            let what = kw.clone();
            self.advance();
            Some(Stmt::Exit(what))
        } else {
            Some(Stmt::Exit(String::new()))
        }
    }

    fn parse_assignment_or_call(&mut self) -> Option<Stmt> {
        if let Token::Identifier(name) = &self.current.clone() {
            let n = name.clone();
            self.advance();

            if self.current_is_operator("(") {
                self.advance();
                let indices = self.parse_paren_args();
                if self.current_is_operator("=") {
                    self.advance();
                    let value = self.parse_expr()?;
                    Some(Stmt::Assignment(n, value))
                } else {
                    Some(Stmt::Expression(Expr::ArrayAccess(n, indices)))
                }
            } else if self.current_is_operator("=") {
                self.advance();
                let value = self.parse_expr()?;
                Some(Stmt::Assignment(n, value))
            } else {
                let mut args = Vec::new();
                if self.current_is_operator("(") {
                    self.advance();
                    args = self.parse_paren_args();
                }
                if args.is_empty() {
                    Some(Stmt::Expression(Expr::Var(n)))
                } else {
                    Some(Stmt::Call(n, args))
                }
            }
        } else {
            self.advance();
            Some(Stmt::Unknown)
        }
    }

    // === Block parsing ===

    fn parse_block_until_else_or_end(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.current_is_keyword("ELSE")
            && !self.current_is_keyword("ELSEIF")
            && !self.current_is_keyword("END")
            && self.current != Token::Eof
        {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.skip_newlines();
        }
        stmts
    }

    fn parse_block_until_end_if(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.current_is_keyword("END") && self.current != Token::Eof {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.skip_newlines();
        }
        stmts
    }

    fn parse_block_until(&mut self, keyword: &str) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.current_is_keyword(keyword) && self.current != Token::Eof {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.skip_newlines();
        }
        stmts
    }

    fn parse_block_until_select_case(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.current_is_keyword("CASE") && !self.current_is_keyword("END") && self.current != Token::Eof {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.skip_newlines();
        }
        stmts
    }

    // === Expression parsing (Pratt parser) ===

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Option<Expr> {
        let mut left = self.parse_unary()?;

        loop {
            let prec = self.get_operator_precedence();
            if prec < min_prec || prec == 0 {
                break;
            }

            let op = match &self.current {
                Token::Operator(o) => o.clone(),
                Token::Keyword(k) if k == "AND" || k == "OR" || k == "XOR" || k == "MOD" => k.clone(),
                Token::Keyword(k) if k == "IS" => {
                    self.advance();
                    let right = self.parse_binary_expr(prec + 1)?;
                    left = Expr::Binary(Box::new(left), "IS".to_string(), Box::new(right));
                    continue;
                }
                _ => break,
            };
            self.advance();

            let right = self.parse_binary_expr(prec + 1)?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }

        Some(left)
    }

    fn get_operator_precedence(&self) -> u8 {
        match &self.current {
            Token::Keyword(k) if k == "OR" || k == "XOR" => 1,
            Token::Keyword(k) if k == "AND" => 2,
            Token::Keyword(k) if k == "NOT" => 3,
            Token::Operator(o) if o == "=" || o == "<>" || o == "<" || o == ">" || o == "<=" || o == ">=" => 4,
            Token::Keyword(k) if k == "IS" => 4,
            Token::Operator(o) if o == "+" || o == "-" || o == "&" => 5,
            Token::Operator(o) if o == "*" || o == "/" || o == "\\" => 6,
            Token::Keyword(k) if k == "MOD" => 6,
            Token::Operator(o) if o == "^" => 7,
            _ => 0,
        }
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        match &self.current.clone() {
            Token::Operator(o) if o == "-" || o == "+" => {
                let op = o.clone();
                self.advance();
                let expr = self.parse_unary()?;
                Some(Expr::Unary(op, Box::new(expr)))
            }
            Token::Keyword(k) if k == "NOT" => {
                self.advance();
                let expr = self.parse_unary()?;
                Some(Expr::Unary("NOT".to_string(), Box::new(expr)))
            }
            _ => self.parse_atom(),
        }
    }

    fn parse_atom(&mut self) -> Option<Expr> {
        match &self.current.clone() {
            Token::Number(n) => {
                let num = n.clone();
                self.advance();
                Some(Expr::Number(num))
            }
            Token::String(s) => {
                let str = s.clone();
                self.advance();
                Some(Expr::String(str))
            }
            Token::Operator(o) if o == "(" => {
                self.advance();
                let expr = self.parse_expr()?;
                if self.current_is_operator(")") {
                    self.advance();
                }
                Some(Expr::Paren(Box::new(expr)))
            }
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                if self.current_is_operator("(") {
                    self.advance();
                    let args = self.parse_paren_args();
                    Some(Expr::Call(name, args))
                } else {
                    Some(Expr::Var(name))
                }
            }
            Token::Keyword(k) if is_builtin_function(k) => {
                let name = k.clone();
                self.advance();
                if self.current_is_operator("(") {
                    self.advance();
                    let args = self.parse_paren_args();
                    Some(Expr::Call(name, args))
                } else {
                    Some(Expr::Var(name))
                }
            }
            _ => None,
        }
    }
}

/// Helper for distinguishing SUB vs FUNCTION in shared parsing.
enum StmtEndKeyword {
    SUB,
    FUNCTION,
}

fn is_builtin_function(kw: &str) -> bool {
    matches!(
        kw,
        "ABS" | "ATN" | "COS" | "SIN" | "TAN" | "EXP" | "LOG" | "SQR" | "INT" | "RND"
            | "LEN" | "ASC" | "VAL" | "TIMER" | "POINT" | "PMAP"
            | "EOF" | "LOF" | "FREEFILE"
            | "LBOUND" | "UBOUND"
            | "PEEK" | "INP"
            | "SCREEN"
    )
}

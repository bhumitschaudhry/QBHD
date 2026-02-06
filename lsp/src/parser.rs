use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Vec<Expr>),
    Dim(String),
    Assignment(String, Expr),
    If(Expr, Vec<Stmt>),
    For(String, Expr, Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(String),
    String(String),
    Var(String),
    Binary(Box<Expr>, String, Box<Expr>),
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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::Eof {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            if self.current == Token::Newline {
                self.advance();
            }
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match &self.current {
            Token::Keyword(kw) if kw == "PRINT" => {
                self.advance();
                Some(Stmt::Print(vec![self.parse_expr()?]))
            }
            Token::Keyword(kw) if kw == "DIM" => {
                self.advance();
                if let Token::Identifier(name) = &self.current {
                    let n = name.clone();
                    self.advance();
                    Some(Stmt::Dim(n))
                } else {
                    None
                }
            }
            Token::Identifier(name) => {
                let n = name.clone();
                self.advance();
                if let Token::Operator(op) = &self.current {
                    if op == "=" {
                        self.advance();
                        Some(Stmt::Assignment(n, self.parse_expr()?))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
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
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                Some(Expr::Var(name))
            }
            _ => None,
        }
    }
}

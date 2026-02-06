use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Operator(String),
    Newline,
    Eof,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current = self.input.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('"') => self.read_string(),
            Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),
            Some(c) if c.is_numeric() => self.read_number(),
            Some(c) => {
                let op = c.to_string();
                self.advance();
                Token::Operator(op)
            }
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening "
        let mut s = String::new();
        while let Some(c) = self.current {
            if c == '"' {
                self.advance();
                break;
            }
            s.push(c);
            self.advance();
        }
        Token::String(s)
    }

    fn read_identifier(&mut self) -> Token {
        let mut id = String::new();
        while let Some(c) = self.current {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        let upper = id.to_uppercase();
        if matches!(upper.as_str(), "PRINT" | "IF" | "THEN" | "ELSE" | "FOR" | "NEXT" | "DIM" | "SUB" | "FUNCTION" | "END") {
            Token::Keyword(upper)
        } else {
            Token::Identifier(id)
        }
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.current {
            if c.is_numeric() || c == '.' {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(num)
    }
}

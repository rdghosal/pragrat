use crate::types::{Literal, Token, TokenType};

struct Tokenizer {
    current: usize,
    line: usize,
    source: String,
    start: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(source: String) -> Self {
        Self {
            // TODO: make pre-formatter for source.
            source: source.replace("\t", "    "),
            line: 1,
            current: 0,
            start: 0,
            tokens: Vec::new(),
        }
    }
    fn scan_tokens(&mut self) {
        while !self.reached_eof() {
            self.start = self.current;
            self.scan_token();
        }
    }
    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(TokenType::LParen, None),
                ')' => self.add_token(TokenType::RParen, None),
                '}' => self.add_token(TokenType::LBrace, None),
                '{' => self.add_token(TokenType::RBrace, None),
                '[' => self.add_token(TokenType::LBracket, None),
                ']' => self.add_token(TokenType::RBracket, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                ':' => self.add_token(TokenType::Colon, None),
                '\t' => self.add_token(TokenType::Indent, None),
                '!' => {
                    if self.matches('=') {
                        self.add_token(TokenType::BangEqual, None)
                    } else {
                        self.add_token(TokenType::Bang, None)
                    }
                }
                '*' => {
                    if self.matches('=') {
                        self.add_token(TokenType::StarEqual, None)
                    } else if self.matches('*') {
                        self.add_token(TokenType::DoubleStar, None)
                    } else {
                        self.add_token(TokenType::Star, None)
                    }
                }
                '/' => {
                    if self.matches('=') {
                        self.add_token(TokenType::SlashEqual, None)
                    } else if self.matches('/') {
                        self.add_token(TokenType::DoubleSlash, None)
                    } else {
                        self.add_token(TokenType::Slash, None)
                    }
                }
                '-' => {
                    if self.matches('=') {
                        self.add_token(TokenType::MinusEqual, None)
                    } else {
                        self.add_token(TokenType::Minus, None)
                    }
                }
                '+' => {
                    if self.matches('=') {
                        self.add_token(TokenType::PlusEqual, None)
                    } else {
                        self.add_token(TokenType::Plus, None)
                    }
                }
                _ => panic!("Invalid token {c:?}"),
            }
        }
    }
    fn matches(&mut self, expected: char) -> bool {
        let c = self.get_char();
        if c.is_some() && c.unwrap() == expected {
            self.current += 1;
            return true;
        } else {
            return false;
        }
    }
    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let content = self.source[self.start..=self.current].to_string();
        self.tokens.push(Token {
            type_: token_type,
            literal,
            line: self.line,
            content,
        })
    }
    fn get_char(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn advance(&mut self) -> Option<char> {
        let c = self.get_char();
        self.current += 1;
        c
    }
    fn reached_eof(&self) -> bool {
        self.current >= self.source.len()
    }
}

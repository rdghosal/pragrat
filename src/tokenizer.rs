use std::collections::HashMap;

use phf::phf_map;

use crate::types::{Literal, Token, TokenType};

pub struct Tokenizer {
    current: usize,
    line: usize,
    source: String,
    start: usize,
    tokens: Vec<Token>,
}

trait PySource {
    fn is_snakecase(&self, is_first: bool) -> bool;
}
impl PySource for char {
    fn is_snakecase(&self, is_first: bool) -> bool {
        if is_first {
            return self.is_ascii_alphabetic() || self == &'_';
        }
        return self.is_ascii_alphanumeric() || self == &'_';
    }
}

const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "as" => TokenType::As,
    "async" => TokenType::Async,
    "await" => TokenType::Await,
    "assert" => TokenType::Assert,
    "break" => TokenType::Break,
    "class" => TokenType::Class,
    "continue" => TokenType::Continue,
    "def" => TokenType::Def,
    "del" => TokenType::Del,
    "elif" => TokenType::Elif,
    "else" => TokenType::Else,
    "except" => TokenType::Except,
    "false" => TokenType::False,
    "finally" => TokenType::Finally,
    "for" => TokenType::For,
    "global" => TokenType::Global,
    "if" => TokenType::If,
    "import" => TokenType::Import,
    "in" => TokenType::In,
    "is" => TokenType::Is,
    "lambda" => TokenType::Lambda,
    "none" => TokenType::None,
    "nonlocal" => TokenType::Nonlocal,
    "not" => TokenType::Not,
    "or" => TokenType::Or,
    "pass" => TokenType::Pass,
    "raise" => TokenType::Raise,
    "return" => TokenType::Return,
    "true" => TokenType::True,
    "try" => TokenType::Try,
    "while" => TokenType::While,
    "with" => TokenType::With,
    "yield" => TokenType::Yield,
};

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
                '(' => self.add_token(TokenType::LPar, None),
                ')' => self.add_token(TokenType::RPar, None),
                '}' => self.add_token(TokenType::LBrace, None),
                '{' => self.add_token(TokenType::RBrace, None),
                '[' => self.add_token(TokenType::LSqB, None),
                ']' => self.add_token(TokenType::RSqB, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                ';' => self.add_token(TokenType::Semi, None),
                ':' => self.add_token(TokenType::Colon, None),
                '!' => {
                    if self.matches('=') {
                        self.add_token(TokenType::NotEqual, None)
                    } else {
                        self.add_token(TokenType::Exclamation, None)
                    }
                }
                '*' => {
                    if self.matches('=') {
                        self.add_token(TokenType::StarEqual, None)
                    } else if self.matches('*') {
                        if self.matches('=') {
                            self.add_token(TokenType::DoubleStarEqual, None)
                        }
                        self.add_token(TokenType::DoubleStar, None)
                    } else {
                        self.add_token(TokenType::Star, None)
                    }
                }
                '/' => {
                    if self.matches('=') {
                        self.add_token(TokenType::SlashEqual, None)
                    } else if self.matches('/') {
                        if self.matches('=') {
                            self.add_token(TokenType::DoubleSlashEqual, None)
                        }
                        self.add_token(TokenType::DoubleSlash, None)
                    } else {
                        self.add_token(TokenType::Slash, None)
                    }
                }
                '-' => {
                    if self.matches('=') {
                        self.add_token(TokenType::MinEqual, None)
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
                '#' => {
                    while !self.reached_eof() && self.peek(1).unwrap() != '\n' {
                        self.advance();
                    }
                }
                '\n' => {
                    if let Some(last) = self.tokens.last() {
                        if last.token_type == TokenType::Newline {
                            self.add_token(TokenType::NL, None);
                            self.line += 1
                        }
                    }
                    self.add_token(TokenType::Newline, None);
                    self.line += 1
                }
                ' ' | '\r' => (),
                '\'' => {
                    let c_first = self.peek(1);
                    let c_second = self.peek(2);
                    if c_first.is_some()
                        && c_second.is_some()
                        && c_first.unwrap() == '\''
                        && c_second.unwrap() == '\''
                    {
                        // Found docstring.
                        self.advance();
                        // Find terminating triple quotes.
                        while !(self.get_char().unwrap() == '\''
                            && self.peek(1).unwrap() == '\''
                            && self.peek(2).unwrap() == '\'')
                        {
                            let c = self.advance();
                            if self.reached_eof() {
                                panic!("Unterminated docstring.")
                            } else if c.unwrap() == '\n' {
                                self.line += 1;
                            }
                        }
                    } else {
                        // Handle string.
                        while let Some(next) = self.peek(1) {
                            self.advance();
                            if next == '\'' {
                                break;
                            }
                        }
                        self.advance();
                        let str = &self.source[self.start + 1..self.current - 1];
                        self.add_token(TokenType::String, Some(Literal::String(str.to_string())))
                    }
                }
                _ => {
                    if c.is_ascii_digit() {
                        while let Some(next) = self.peek(1) {
                            if next.is_ascii_digit() || next == '.' {
                                self.advance();
                                continue;
                            }
                            break;
                        }
                        let num = &self.source[self.start..=self.current];
                        self.add_token(TokenType::Number, Some(Literal::Number(num.to_string())))
                    } else if c.is_snakecase(true) {
                        while let Some(next) = self.peek(1) {
                            if next.is_snakecase(false) {
                                self.advance();
                                continue;
                            }
                            break;
                        }
                        self.advance();
                        let text = &self.source[self.start..self.current];
                        if let Some(t) = KEYWORDS.get(text) {
                            self.add_token(t.clone(), None)
                        } else {
                            self.add_token(TokenType::Name, None)
                        }
                    } else {
                        panic!("Unexpected character {c:?}")
                    }
                }
            }
        }
    }
    fn peek(&self, offset: usize) -> Option<char> {
        self.source.chars().nth(self.current + offset)
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
        let value = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            literal,
            line: self.line,
            value,
        });
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

#[test]
fn test_simple() {
    let source = r"
    def my_func():
        print('hello world!')
    ";
    let expected = [
        Token {
            token_type: TokenType::Newline,
            value: "\n".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: TokenType::Def,
            value: "def".to_string(),
            literal: None,
            line: 2,
        },
        Token {
            token_type: TokenType::Name,
            value: "my_func".to_string(),
            literal: None,
            line: 2,
        },
        Token {
            token_type: TokenType::LPar,
            value: "(".to_string(),
            literal: None,
            line: 2,
        },
        Token {
            token_type: TokenType::RPar,
            value: ")".to_string(),
            literal: None,
            line: 2,
        },
        Token {
            token_type: TokenType::Colon,
            value: ":".to_string(),
            literal: None,
            line: 2,
        },
        Token {
            token_type: TokenType::Newline,
            value: "\n".to_string(),
            literal: None,
            line: 2,
        },
        Token {
            token_type: TokenType::Name,
            value: "print".to_string(),
            literal: None,
            line: 3,
        },
        Token {
            token_type: TokenType::LPar,
            value: "(".to_string(),
            literal: None,
            line: 3,
        },
        Token {
            token_type: TokenType::String,
            value: "'hello world!'".to_string(),
            literal: Some(Literal::String("hello world!".to_string())),
            line: 3,
        },
        Token {
            token_type: TokenType::RPar,
            value: ")".to_string(),
            literal: None,
            line: 3,
        },
        Token {
            token_type: TokenType::Newline,
            value: "\n".to_string(),
            literal: None,
            line: 3,
        },
    ];
    let mut t = Tokenizer::new(source.to_string());
    t.scan_tokens();
    for (i, token) in t.tokens.iter().enumerate() {
        assert_eq!(expected[i], *token);
    }
}

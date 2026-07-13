// tokenizer.rs

#![allow(dead_code)]
#![allow(unused)]

use crate::lexer::token::{Token, TokenType};

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1
        }
    }

    fn check_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn check_punctuation(&mut self, tokens: &mut Vec<Token>) {
        match self.input[self.position] {
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    line: self.line,
                    column: self.column,
                    value: None
                });
                self.position += 1;
                self.column += 1;
            },
            ':' => {
                tokens.push(Token {
                    token_type: TokenType::Colon,
                    line: self.line,
                    column: self.column,
                    value: None
                });
                self.position += 1;
                self.column += 1;
            },
            '"' => {
                tokens.push(Token {
                    token_type: TokenType::DoubleQuote,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;

                let mut string: String = String::new();
                while self.input[self.position] != '"' {
                    string.push(self.input[self.position]);
                    self.position += 1;
                    self.column += 1;
                }
                tokens.push(Token {
                    token_type: TokenType::Content(string.clone()),
                    line: self.line,
                    column: self.column - string.len(),
                    value: Some(string)
                });

                tokens.push(Token {
                    token_type: TokenType::DoubleQuote,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;
            },
            '=' => {
                tokens.push(Token {
                    token_type: TokenType::Equal,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;
            },
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::LParen,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;
            },
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RParen,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;
            },
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;
            },
            '-' => {
                tokens.push(Token {
                    token_type: TokenType::Minus,
                    line: self.line,
                    column: self.column,
                    value: None
                });

                self.position += 1;
                self.column += 1;
            },
            '*' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '*' {
                    tokens.push(Token {
                        token_type: TokenType::DoubleAsterisk,
                        line: self.line,
                        column: self.column,
                        value: None
                    });
        
                    self.position += 2;
                    self.column += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Asterisk,
                        line: self.line,
                        column: self.column,
                        value: None
                    });
                    self.position += 1;
                    self.column += 1;
                }
            },
            '/' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '/' {
                    tokens.push(Token {
                        token_type: TokenType::DoubleSlash,
                        line: self.line,
                        column: self.column,
                        value: None
                    });

                    self.position += 2;
                    self.column += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Slash,
                        line: self.line,
                        column: self.column,
                        value: None
                    });

                    self.position += 1;
                    self.column += 1;
                }
            },
            _ => {
                self.position += 1;
                self.column += 1;
            }
        }
    }

    fn check_keyword(&mut self, tokens: &mut Vec<Token>) {
        let mut keyword = String::new();
        let start_column = self.column;

        if self.input[self.position].is_alphabetic() {
            while (
                self.position < self.input.len() && (
                    self.input[self.position].is_alphanumeric() ||
                    self.input[self.position] == '?'
                )
            ) {
                keyword.push(self.input[self.position]);
                self.position += 1;
                self.column += 1;
            }

            match keyword.as_str() {
                "print?" => {
                    tokens.push(Token {
                        token_type: TokenType::Print,
                        line: self.line,
                        column: start_column,
                        value: None
                    });
                },
                "println?" => {
                    tokens.push(Token {
                        token_type: TokenType::Println,
                        line: self.line,
                        column: start_column,
                        value: None
                    });
                },
                "let" => {
                    tokens.push(Token {
                        token_type: TokenType::Let,
                        line: self.line,
                        column: start_column,
                        value: None
                    });
                },
                "mod" => {
                    tokens.push(Token {
                        token_type: TokenType::Mod,
                        line: self.line,
                        column: start_column,
                        value: None
                    });
                },
                _ => {
                    tokens.push(Token {
                        token_type: TokenType::Identifier(keyword.clone()),
                        line: self.line,
                        column: start_column,
                        value: Some(keyword)
                    });
                }
            }
        }
    }

    fn check_number(&mut self, tokens: &mut Vec<Token>) {
        let mut number = String::new();
        let start_column = self.column;

        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            number.push(self.input[self.position]);
            self.position += 1;
            self.column += 1;
        }

        if let Ok(value) = number.parse::<u64>() {
            tokens.push(Token {
                token_type: TokenType::Value(value),
                line: self.line,
                column: start_column,
                value: Some(number)
            });
        }
    }

    pub fn lexing(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.position < self.input.len() {
            self.check_whitespace();

            if self.position >= self.input.len() {
                break;
            }

            if self.input[self.position].is_alphabetic() {
                self.check_keyword(&mut tokens);
            } else if self.input[self.position].is_numeric() {
                self.check_number(&mut tokens);
            } else {
                self.check_punctuation(&mut tokens);
            }
        }

        tokens
    }
}
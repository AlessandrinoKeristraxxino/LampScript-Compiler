// ast.rs

use std::mem::discriminant;

#![allow(dead_code)]

use crate::lexer::token::*;

pub enum Expr {
    Number(u64),
    Identifier(String),
}

pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Print(Expr),
    Println(Expr),
}

pub struct Program {
    pub statements: Vec<Stmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn get_current_token(&self) -> &Token {
        if self.position >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1];
        }

        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        if self.position >= self.tokens.len() {
            return;
        }

        self.position += 1;
    }

    fn expect_token(&mut self, expected: TokenType) {
        let current_token = self.get_current_token();

        if discriminant(&current_token.token_type) == discriminant(&expected) {
            self.advance();
        } else {
            panic!(
                "Syntax Error at line {}, column {}.\n\
                Expected Token: {:?}\n\
                Found: {:?}",
                current_token.line, current_token.column, expected, current_token
            );
        }
    }

    fn parse_expression(&mut self) -> Expr {
        let current_token = self.get_current_token();

        match &current_token.token_type {
            TokenType::Value(value) => {
                self.advance();
                Expr::Number(*value)
            },
            TokenType::Identifier(name) => {
                self.advance();
                Expr::Identifier(name.clone())
            },
            _ => {
                panic!(
                    "Syntax Error at line {}, column {}.\n\
                    This expression is not supported: {:?}",
                    current_token.line, current_token.column, current_token
                );
            }
        }
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        let current_token = self.get_current_token();

        match current_token {
            &Token { token_type: TokenType::Let, .. } => {
                self.advance();
                let current_token = self.get_current_token();

                if let &Token { token_type: TokenType::Identifier(ref name), .. } = current_token {
                    self.advance();
                    let variable_name = name.clone();
                    let current_token = self.get_current_token();

                    if let &Token { token_type: TokenType::Assign, .. } = current_token {
                        self.advance();

                        let def_value = self.parse_expression();

                        self.expect_token(TokenType::Semicolon);

                        return Some(Stmt::Let { name: variable_name, value: def_value });

                    } else {
                        panic!("
                            Syntax Error at line {}, column {}. \n\
                            Unexpected Token {:?} \n\
                            Expected Assign `let variablename = value` (TokenType::Assign) \n\
                                                                          + <- here",
                            current_token.line, current_token.column, current_token    
                        );
                    }
                } else {
                    panic!("
                        Syntax Error at line {}, column {}.\n\
                        Unexpected Token {:?} \n\
                        Expected Identifier `let variable_name = value` (TokenType::Identifier(String)) \n\
                                             +-+-+-+-+-+-+ <- here",
                        current_token.line, current_token.column, current_token
                    );
                }
            },
            &Token { token_type: TokenType::Print, .. } => {
                self.expect_token(TokenType::LParen);

                let expr = self.parse_expression();

                self.expect_token(TokenType::RParen);
                self.expect_token(TokenType::Semicolon);

                Some(Stmt::Print(expr))
            },
            &Token { token_type: TokenType::Println, .. } => {
                self.expect_token(TokenType::LParen);

                let expr = self.parse_expression();

                self.expect_token(TokenType::RParen);
                self.expect_token(TokenType::Semicolon);

                Some(Stmt::Println(expr))
            }
            _ => None
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                self.advance(); 
            }
        }

        Program { statements }
    }
}
// ast.rs

#![allow(dead_code)]

use crate::lexer::token::Token;

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

    fn parse_statements(&mut self) -> Self {

    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            let current = self.get_current_token();

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                self.advance(); 
            }
        }

        Program { statements }
    }
}
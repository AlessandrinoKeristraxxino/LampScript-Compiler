use std::mem::discriminant;

use crate::lexer::token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Unary {
        op: TokenType,
        expr: Box<Expr>,
    },
    Root {
        degree: Option<f64>,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Print(Expr),
    Println(Expr),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn get_current_token(&self) -> Token {
        if self.position >= self.tokens.len() {
            if let Some(token) = self.tokens.last() {
                return token.clone();
            }

            return Token {
                token_type: TokenType::Value(0.0),
                line: 0,
                column: 0,
                value: None,
            };
        }

        self.tokens[self.position].clone()
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
                "Syntax Error at line {}, column {}.\nExpected Token: {:?}\nFound: {:?}",
                current_token.line,
                current_token.column,
                expected,
                current_token
            );
        }
    }

    fn parse_expression(&mut self) -> Expr {
        let current_token = self.get_current_token();

        match &current_token.token_type {
            TokenType::Value(value) => {
                self.advance();

                if matches!(self.get_current_token().token_type, TokenType::Sqrt) {
                    self.advance();
                    let expr = self.parse_expression();
                    Expr::Root {
                        degree: Some(*value),
                        expr: Box::new(expr),
                    }
                } else {
                    Expr::Number(*value)
                }
            }
            TokenType::Sqrt => {
                self.advance();
                let expr = self.parse_expression();
                Expr::Root {
                    degree: None,
                    expr: Box::new(expr),
                }
            }
            TokenType::Identifier(name) => {
                self.advance();
                Expr::Identifier(name.clone())
            }
            _ => panic!(
                "Syntax Error at line {}, column {}.\nThis expression is not supported: {:?}",
                current_token.line,
                current_token.column,
                current_token
            ),
        }
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        let current_token = self.get_current_token();

        match &current_token.token_type {
            TokenType::Let => {
                self.advance();
                let current_token = self.get_current_token();

                if let TokenType::Identifier(name) = &current_token.token_type {
                    self.advance();
                    let variable_name = name.clone();
                    let current_token = self.get_current_token();

                    if let TokenType::Equal = &current_token.token_type {
                        self.advance();
                        let def_value = self.parse_expression();
                        self.expect_token(TokenType::Semicolon);

                        return Some(Stmt::Let {
                            name: variable_name,
                            value: def_value,
                        });
                    }

                    panic!(
                        "Syntax Error at line {}, column {}.\nUnexpected Token {:?}\nExpected Equal `let variable_name = value`",
                        current_token.line,
                        current_token.column,
                        current_token
                    );
                }

                panic!(
                    "Syntax Error at line {}, column {}.\nUnexpected Token {:?}\nExpected Identifier `let variable_name = value`",
                    current_token.line,
                    current_token.column,
                    current_token
                );
            }
            TokenType::Print => {
                self.advance();
                self.expect_token(TokenType::LParen);

                let expr = self.parse_expression();

                self.expect_token(TokenType::RParen);
                self.expect_token(TokenType::Semicolon);

                Some(Stmt::Print(expr))
            }
            TokenType::Println => {
                self.advance();
                self.expect_token(TokenType::LParen);

                let expr = self.parse_expression();

                self.expect_token(TokenType::RParen);
                self.expect_token(TokenType::Semicolon);

                Some(Stmt::Println(expr))
            }
            _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_simple_let_statement() {
        let tokens = vec![
            Token {
                token_type: TokenType::Let,
                line: 1,
                column: 1,
                value: None,
            },
            Token {
                token_type: TokenType::Identifier("x".to_string()),
                line: 1,
                column: 5,
                value: Some("x".to_string()),
            },
            Token {
                token_type: TokenType::Equal,
                line: 1,
                column: 7,
                value: None,
            },
            Token {
                token_type: TokenType::Value(10.0),
                line: 1,
                column: 9,
                value: Some("10".to_string()),
            },
            Token {
                token_type: TokenType::Semicolon,
                line: 1,
                column: 11,
                value: None,
            },
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Let { name, value } => {
                assert_eq!(name, "x");
                assert!(matches!(value, Expr::Number(10.0)));
            }
            other => panic!("Expected let statement, got {other:?}"),
        }
    }
}

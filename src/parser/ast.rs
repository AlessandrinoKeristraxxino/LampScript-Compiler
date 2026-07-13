use std::mem::discriminant;

use crate::lexer::token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    F8, F16, F32, F64,
    Bool, Char, String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
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
        is_mutable: bool,
        var_type: Option<Type>,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Print(Vec<Expr>),
    Println(Vec<Expr>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
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

    fn parse_type(&mut self) -> Option<Type> {
        let current_token = self.get_current_token();
        let t = match current_token.token_type {
            TokenType::TypeU8 => Some(Type::U8),
            TokenType::TypeU16 => Some(Type::U16),
            TokenType::TypeU32 => Some(Type::U32),
            TokenType::TypeU64 => Some(Type::U64),
            TokenType::TypeI8 => Some(Type::I8),
            TokenType::TypeI16 => Some(Type::I16),
            TokenType::TypeI32 => Some(Type::I32),
            TokenType::TypeI64 => Some(Type::I64),
            TokenType::TypeF8 => Some(Type::F8),
            TokenType::TypeF16 => Some(Type::F16),
            TokenType::TypeF32 => Some(Type::F32),
            TokenType::TypeF64 => Some(Type::F64),
            TokenType::TypeBool => Some(Type::Bool),
            TokenType::TypeChar => Some(Type::Char),
            TokenType::TypeString => Some(Type::String),
            _ => None,
        };
        if t.is_some() {
            self.advance();
        }
        t
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Expr {
        let mut expr = self.parse_logical_and();
        while matches!(self.get_current_token().token_type, TokenType::Or) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_logical_and();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_logical_and(&mut self) -> Expr {
        let mut expr = self.parse_not_both();
        while matches!(self.get_current_token().token_type, TokenType::And) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_not_both();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_not_both(&mut self) -> Expr {
        let mut expr = self.parse_equality();
        while matches!(self.get_current_token().token_type, TokenType::NotBoth) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_equality();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();
        while matches!(self.get_current_token().token_type, TokenType::EqualEqual | TokenType::NotEqual) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_comparison();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        while matches!(self.get_current_token().token_type, TokenType::LessThan | TokenType::GreaterThan | TokenType::LessThanOrEqual | TokenType::GreaterThanOrEqual) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_term();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        while matches!(self.get_current_token().token_type, TokenType::Plus | TokenType::Minus) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_factor();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while matches!(self.get_current_token().token_type, TokenType::Asterisk | TokenType::Slash) {
            let op = self.get_current_token().token_type.clone();
            self.advance();
            let right = self.parse_primary();
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        let current_token = self.get_current_token();
        match &current_token.token_type {
            TokenType::Value(value) => {
                self.advance();
                Expr::Number(*value)
            }
            TokenType::True => {
                self.advance();
                Expr::Number(1.0)
            }
            TokenType::False => {
                self.advance();
                Expr::Number(0.0)
            }
            TokenType::Identifier(name) => {
                self.advance();
                Expr::Identifier(name.clone())
            }
            TokenType::Content(text) => {
                self.advance();
                Expr::StringLiteral(text.clone())
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect_token(TokenType::RParen);
                expr
            }
            TokenType::Sqrt => {
                self.advance();
                let expr = self.parse_expression();
                Expr::Root { degree: None, expr: Box::new(expr) }
            }
            _ => panic!(
                "Syntax Error at line {}, column {}.\nUnexpected expression token: {:?}",
                current_token.line, current_token.column, current_token
            )
        }
    }

    fn parse_macro_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        if matches!(self.get_current_token().token_type, TokenType::RParen) {
            return args;
        }
        loop {
            args.push(self.parse_expression());
            if matches!(self.get_current_token().token_type, TokenType::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        args
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        let current_token = self.get_current_token();

        match &current_token.token_type {
            TokenType::LBrace => {
                self.advance(); // consume '{'
                let mut stmts = Vec::new();
                while !matches!(self.get_current_token().token_type, TokenType::RBrace) && self.position < self.tokens.len() {
                    if let Some(stmt) = self.parse_statement() {
                        stmts.push(stmt);
                    } else {
                        self.advance();
                    }
                }
                self.expect_token(TokenType::RBrace);
                Some(Stmt::Block(stmts))
            }
            TokenType::If => {
                self.advance();
                let condition = self.parse_expression();
                let then_branch = self.parse_statement().expect("Expected statement after if condition");
                
                let mut else_branch = None;
                if matches!(self.get_current_token().token_type, TokenType::Else) {
                    self.advance();
                    else_branch = Some(Box::new(self.parse_statement().expect("Expected statement after else")));
                }
                
                Some(Stmt::If {
                    condition,
                    then_branch: Box::new(then_branch),
                    else_branch,
                })
            }
            TokenType::While => {
                self.advance();
                let condition = self.parse_expression();
                let body = self.parse_statement().expect("Expected statement after while condition");
                Some(Stmt::While { condition, body: Box::new(body) })
            }
            TokenType::Let => {
                self.advance();
                let current_token = self.get_current_token();

                if let TokenType::Identifier(name) = &current_token.token_type {
                    self.advance();
                    let variable_name = name.clone();
                    
                    let mut is_mutable = false;
                    let mut var_type = None;
                    
                    if matches!(self.get_current_token().token_type, TokenType::Colon) {
                        self.advance(); // consume ':'
                        if matches!(self.get_current_token().token_type, TokenType::Mod) {
                            is_mutable = true;
                            self.advance(); // consume 'mod'
                        }
                        var_type = self.parse_type();
                    }

                    let current_token = self.get_current_token();
                    if let TokenType::Equal = &current_token.token_type {
                        self.advance();
                        let def_value = self.parse_expression();
                        self.expect_token(TokenType::Semicolon);

                        return Some(Stmt::Let {
                            name: variable_name,
                            is_mutable,
                            var_type,
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
            TokenType::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                let current_token = self.get_current_token();
                if let TokenType::Equal = &current_token.token_type {
                    self.advance();
                    let value = self.parse_expression();
                    self.expect_token(TokenType::Semicolon);
                    Some(Stmt::Assign { name: var_name, value })
                } else {
                    // Not an assignment
                    panic!("Syntax Error at line {}, column {}.\nUnexpected Token {:?}\nExpected Equal for assignment.", current_token.line, current_token.column, current_token);
                }
            }
            TokenType::Print => {
                self.advance();
                self.expect_token(TokenType::LParen);
                let args = self.parse_macro_args();
                self.expect_token(TokenType::RParen);
                self.expect_token(TokenType::Semicolon);
                Some(Stmt::Print(args))
            }
            TokenType::Println => {
                self.advance();
                self.expect_token(TokenType::LParen);
                let args = self.parse_macro_args();
                self.expect_token(TokenType::RParen);
                self.expect_token(TokenType::Semicolon);
                Some(Stmt::Println(args))
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
            Stmt::Let { name, is_mutable, var_type, value } => {
                assert_eq!(name, "x");
                assert_eq!(*is_mutable, false);
                assert_eq!(*var_type, None);
                assert!(matches!(value, Expr::Number(10.0)));
            }
            other => panic!("Expected let statement, got {other:?}"),
        }
    }
}

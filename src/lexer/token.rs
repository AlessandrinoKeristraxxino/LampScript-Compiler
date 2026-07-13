// token.rs

#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // macros
    Print,
    Println,
    
    // variabili
    Let,
    Identifier(String),
    Content(String), // for doublequotes
    Mod,
    
    // punctuation
    Semicolon,
    Dot,
    Colon,
    DoubleQuote,
    Equal,
    LParen,
    RParen,

    // operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleAsterisk,
    Sqrt,

    // comments
    SingleComment,
    MultiComment,

    Value(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub value: Option<String>,
}


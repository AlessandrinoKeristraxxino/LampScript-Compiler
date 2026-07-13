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
    DoubleSlash,

    Value(u64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub value: Option<String>,
}


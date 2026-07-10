// token.rs

#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // funzioni
    Print,
    Println,
    
    // variabili
    Let,
    Identifier(String),
    Content(String), // per le doublequotes
    Mod,
    
    // punteggiatura
    Semicolon,
    Colon,
    DoubleQuote,
    Assign,
    LParen,
    RParen,

    Value(u64),
}

pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub value: Option<String>,
}


// token.rs

#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // macros
    Print,
    Println,
    
    // control flow
    If, Else, While,
    True, False,
    
    // variabili
    Let,
    Identifier(String),
    Content(String), // for doublequotes
    Mod,
    
    // types
    TypeU8, TypeU16, TypeU32, TypeU64,
    TypeI8, TypeI16, TypeI32, TypeI64,
    TypeF8, TypeF16, TypeF32, TypeF64,
    TypeBool, TypeChar, TypeString,

    // punctuation
    Semicolon,
    Comma,
    Dot,
    Colon,
    DoubleQuote,
    Equal,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleAsterisk,
    Sqrt,
    EqualEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    NotBoth, // !!

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


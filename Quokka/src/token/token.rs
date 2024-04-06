use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TokenType {
    Ident,
    Illegal,
    EOF,
    Int(i32),
    Plus,
    Minus,
    EQ,
    NotEQ,
    Not,
    Larrow,
    Rarrow,
    Fslash,
    Assign,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrack,
    Rbrack,
    Asterisk,
    Let,
    Function,
    If,
    Else,
    True,
    False,
    Return,
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.tok_type {
            TokenType::Ident => write!(f, "Ident[{}]", self.literal),
            TokenType::Int(s) => write!(f, "Int[{}]", s),
            _ => write!(f, "{:?}", self.tok_type),
        }
    }
}

//for testing purposes, notice could fix the problm using lazy_static, but pay attention as it
//could throw a runtime error.
lazy_static! {
    pub static ref keywords: HashMap<&'static str, TokenType> = HashMap::from([
        ("fn", TokenType::Function),
        ("let", TokenType::Let),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("return", TokenType::Return)
    ]);
}
pub fn lookup_ident(ident: &str) -> TokenType {
    keywords.get(ident).unwrap_or(&TokenType::Ident).clone()
}

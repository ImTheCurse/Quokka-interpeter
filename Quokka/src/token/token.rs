use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Ident,
    Illegal,
    EOF,
    Int(i32),
    Plus,
    Assign,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrack,
    Rbrack,
    Let,
    Function,
}

pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

//for testing purposes, notice could fix the problm using lazy_static, but pay attention as it
//could throw a runtime error.
lazy_static! {
    pub static ref keywords: HashMap<&'static str, TokenType> =
        HashMap::from([("fn", TokenType::Function), ("let", TokenType::Let),]);
}
pub fn lookup_ident(ident: &str) -> TokenType {
    keywords.get(ident).unwrap_or(&TokenType::Ident).clone()
}

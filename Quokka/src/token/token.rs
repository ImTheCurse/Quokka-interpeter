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

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EQ => write!(f, "EQ"),
            Self::NotEQ => write!(f, "NotEQ"),
            Self::If => write!(f, "If"),
            Self::Function => write!(f, "Function"),
            Self::EOF => write!(f, "EOF"),
            Self::Else => write!(f, "Else"),
            Self::Let => write!(f, "Let"),
            Self::Plus => write!(f, "Plus"),
            Self::True => write!(f, "True"),
            Self::Ident => write!(f, "Ident"),
            Self::Minus => write!(f, "Minus"),
            Self::Comma => write!(f, "Comma"),
            Self::False => write!(f, "False"),
            Self::Larrow => write!(f, "<"),
            Self::Rarrow => write!(f, ">"),
            Self::Fslash => write!(f, "/"),
            Self::Assign => write!(f, "Assign"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Lbrack => write!(f, "Left Bracket"),
            Self::Rbrack => write!(f, "Right Bracket"),
            Self::Return => write!(f, "Return"),
            Self::Illegal => write!(f, "Illegal"),
            Self::Int(num) => write!(f, "{}", num),
            Self::Asterisk => write!(f, "Astrisk"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Not => write!(f, "Not"),
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

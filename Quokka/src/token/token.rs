use std::collections::HashMap;

use lazy_static::lazy_static;
#[derive(Debug, PartialEq, Clone)]
pub struct TokenType(pub String);

pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

//Identifiers + literlars
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

//operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

//Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACK: &str = "{";
pub const RBRACK: &str = "}";

//for testing purposes, notice could fix the problm using lazy_static, but pay attention as it
//could throw a runtime error.
lazy_static! {
    pub static ref LET: String = "let".to_string();
    pub static ref FUNCTION: String = "fn".to_string();
    pub static ref keywords: HashMap<&'static str, TokenType> = HashMap::from([
        ("fn", TokenType(FUNCTION.to_string())),
        ("let", TokenType(LET.to_string())),
    ]);
}
pub fn lookup_ident(ident: &str) -> TokenType {
    keywords
        .get(ident)
        .unwrap_or(&TokenType(IDENT.to_string()))
        .clone()
}

pub struct TokenType(pub String);

pub struct Token {
    tok_type: TokenType,
    literal: String,
}

pub struct Lexer {
    input: String,
    ch: char,
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

impl Lexer {
    fn new(&mut self, input: String) -> Self {
        Self {
            input: input.clone(),
            ch: input.chars().next().unwrap(),
        }
    }
}

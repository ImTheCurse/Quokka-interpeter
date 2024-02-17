#[derive(Debug, PartialEq)]
pub struct TokenType(pub String);

pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

pub struct Lexer {
    pub input: String,
    pub ch: char,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

//Identifiers + literlars
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

//operators
pub const ASSIGN: char = '=';
pub const PLUS: char = '+';

//Delimiters
pub const COMMA: char = ',';
pub const SEMICOLON: char = ';';
pub const LPAREN: char = '(';
pub const RPAREN: char = ')';
pub const LBRACK: char = '{';
pub const RBRACK: char = '}';

impl Lexer {
    pub fn new(&mut self, input: String) -> Self {
        Self {
            input: input.clone(),
            ch: input.chars().next().unwrap(),
        }
    }
    pub fn next_token(&mut self) -> Token {
        //let tokens: Vec<&str> = self.input.split(' ').collect();
        //TODO use split.once(' ') to get the first token and use split.remainder() to consume and assign it to self.input
        let binding = self.input.clone();
        let first_token = binding
            .split_whitespace()
            .next()
            .unwrap_or("")
            .chars()
            .next()
            .unwrap_or('~');
        self.ch = first_token;
        if self.input.len() > 0 {
            self.input.remove(0);
        }
        match first_token {
            COMMA => Token {
                tok_type: TokenType(COMMA.to_string()),
                literal: COMMA.to_string(),
            },
            SEMICOLON => Token {
                tok_type: TokenType(SEMICOLON.to_string()),
                literal: SEMICOLON.to_string(),
            },
            LPAREN => Token {
                tok_type: TokenType(LPAREN.to_string()),
                literal: LPAREN.to_string(),
            },
            RPAREN => Token {
                tok_type: TokenType(RPAREN.to_string()),
                literal: RPAREN.to_string(),
            },
            LBRACK => Token {
                tok_type: TokenType(LBRACK.to_string()),
                literal: LBRACK.to_string(),
            },
            RBRACK => Token {
                tok_type: TokenType(RBRACK.to_string()),
                literal: RBRACK.to_string(),
            },
            PLUS => Token {
                tok_type: TokenType(PLUS.to_string()),
                literal: PLUS.to_string(),
            },
            ASSIGN => Token {
                tok_type: TokenType(ASSIGN.to_string()),
                literal: ASSIGN.to_string(),
            },
            _ => Token {
                tok_type: TokenType(EOF.to_string()),
                literal: "".to_string(),
            },
        }
    }
}

use crate::token::token::*;

pub struct Lexer {
    pub input: String,
    pub ch: char,
}

fn remove_first_word(input: &str) -> String {
    if let Some(index) = input.find(' ') {
        let remaining = &input[index + 1..];
        return String::from(remaining);
    }

    // If there is no space, return an empty string or the original string based on your requirement.
    String::new()
}

impl Lexer {
    pub fn new(&mut self, input: String) -> Self {
        Self {
            input: input.clone(),
            ch: input.chars().next().unwrap(),
        }
    }
    pub fn next_token(&mut self) -> Token {
        //let tokens: Vec<&str> = self.input.split(" ").collect();
        //TODO use split.once(" ") to get the first token and use split.remainder() to consume and assign it to self.input
        let binding = self.input.clone();
        let first_token = binding.split_whitespace().next().unwrap_or("");
        self.ch = first_token.chars().next().unwrap_or('~');
        self.input = remove_first_word(&self.input);
        println!("lexer:{}", first_token);
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
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    Token {
                        literal: first_token.to_string(),
                        tok_type: lookup_ident(first_token),
                    }
                } else if self.ch.is_numeric() {
                    Token {
                        //when parsing, note for other chars such as ;. there is a need to check
                        //each character in the token is actually an int.
                        literal: first_token.parse().unwrap(),
                        tok_type: TokenType(INT.to_string()),
                    }
                } else {
                    Token {
                        tok_type: TokenType(ILLEGAL.to_string()),
                        literal: "".to_string(),
                    }
                }
            }
        }
    }
}

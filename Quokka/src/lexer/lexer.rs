use crate::token::token::*;

#[derive(Clone)]
pub struct Lexer {
    pub input: String,
    pub ch: char,
}
fn consume_first_word(input: &str) -> String {
    if let Some(index) = input.find(' ') {
        let remaining = &input[index + 1..];
        return String::from(remaining);
    }
    String::new()
}
fn consume_white_space(input: &str) -> String {
    let mut idx = 0;
    for c in input.chars() {
        if c != '\n' && c != ' ' && c != '\t' && c != '\r' {
            let remaining = &input[idx..];
            return String::from(remaining);
        }
        idx += 1;
    }
    String::new()
}

//to use only on "Constructor", could be used on nextToken but not preferable.
#[allow(unused_assignments)]
fn split_special_chars(inp: &str) -> String {
    let mut s = String::new();
    s = inp.replace('(', " ( ");
    s = s.replace(')', " ) ");
    s = s.replace('{', " { ");
    s = s.replace('}', " } ");
    s = s.replace(',', " , ");
    println!("{}", s);
    return s;
}

fn is_token_number(tok: &str) -> bool {
    for c in tok.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

impl Lexer {
    pub fn new(&mut self, input: String) -> Self {
        Self {
            input: input.clone(),
            ch: input.chars().next().unwrap(),
        };
        self.input = split_special_chars(&mut self.input);
        return self.clone();
    }
    pub fn next_token(&mut self) -> Token {
        //let tokens: Vec<&str> = self.input.split(" ").collect();
        //TODO use split.once(" ") to get the first token and use split.remainder() to consume and assign it to self.input

        let binding = self.input.clone();
        let mut first_token = binding.split_whitespace().next().unwrap_or("");

        self.input = consume_white_space(&self.input);
        self.input = consume_first_word(&self.input);

        if first_token.chars().last().unwrap_or(' ') == ';' && first_token.len() > 1 {
            first_token = &first_token[0..first_token.len() - 1];
            self.input.insert_str(0, "; ");
        }
        self.ch = first_token.chars().next().unwrap_or('~');

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
                } else if first_token == "" {
                    Token {
                        literal: first_token.to_string(),
                        tok_type: TokenType(EOF.to_string()),
                    }
                } else if is_token_number(first_token) {
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

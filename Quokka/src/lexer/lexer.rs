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

fn insert_str_at_start(append_to: &str, to_append: &str) -> String {
    let mut temp: String = String::from(to_append);
    temp = temp + append_to;
    temp
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
    s = s.replace(';', " ; ");
    s = s.replace('-', " - ");
    s = s.replace('*', " * ");
    s = s.replace('/', " / ");
    s = s.replace('<', " < ");
    s = s.replace('>', " > ");
    s = s.replace('!', " ! ");
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
        let binding = self.input.clone();
        let first_token = binding.split_whitespace().next().unwrap_or("");
        let mut itr = first_token.chars();

        self.input = consume_white_space(&self.input);
        self.input = consume_first_word(&self.input);

        self.ch = itr.next().unwrap_or('~');
        let peek_char = itr.next().unwrap_or('~');

        match first_token {
            "," => Token {
                tok_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            ";" => Token {
                tok_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            "(" => Token {
                tok_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            ")" => Token {
                tok_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            "{" => Token {
                tok_type: TokenType::Lbrack,
                literal: "{".to_string(),
            },
            "}" => Token {
                tok_type: TokenType::Rbrack,
                literal: "}".to_string(),
            },
            "+" => Token {
                tok_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            "*" => Token {
                tok_type: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            "/" => Token {
                tok_type: TokenType::Fslash,
                literal: "/".to_string(),
            },
            "-" => Token {
                tok_type: TokenType::Minus,
                literal: "-".to_string(),
            },
            ">" => Token {
                tok_type: TokenType::Rarrow,
                literal: ">".to_string(),
            },
            "<" => Token {
                tok_type: TokenType::Larrow,
                literal: "<".to_string(),
            },

            _ => {
                //Check for != or !IDENT
                if self.ch == '!' {
                    let mut eq = binding.chars();
                    eq.next();
                    eq.next();
                    eq.next();

                    if eq.next().unwrap_or('~') == '=' {
                        self.input = consume_white_space(&self.input);
                        self.input = consume_first_word(&self.input);
                        return Token {
                            tok_type: TokenType::NotEQ,
                            literal: "!=".to_string(),
                        };
                    }

                    let mut temp: String = String::from(first_token);
                    temp.remove(0);
                    self.input = insert_str_at_start(&self.input, &temp);
                    return Token {
                        tok_type: TokenType::Not,
                        literal: "!".to_string(),
                    };
                }

                //Check for assignment or equality
                if self.ch == '=' {
                    if peek_char == '=' {
                        let mut temp: String = String::from(first_token);
                        temp.remove(0);
                        temp.remove(0);
                        self.input = insert_str_at_start(&self.input, &temp);
                        return Token {
                            tok_type: TokenType::EQ,
                            literal: "==".to_string(),
                        };
                    }
                    let mut temp: String = String::from(first_token);
                    temp.remove(0);
                    self.input = insert_str_at_start(&self.input, &temp);

                    return Token {
                        tok_type: TokenType::Assign,
                        literal: "=".to_string(),
                    };
                }

                //TODO if a char is alphabetic and contains = or != than seprate them and and add
                //back the == or != or = to the input.
                if first_token.contains("==") {}

                if first_token.contains("!=") {}

                if first_token.contains("=") {}

                if self.ch.is_ascii_alphabetic() {
                    Token {
                        literal: first_token.to_string(),
                        tok_type: lookup_ident(first_token),
                    }
                } else if first_token == "" {
                    Token {
                        literal: first_token.to_string(),
                        tok_type: TokenType::EOF,
                    }
                } else if is_token_number(first_token) {
                    let num: i32 = first_token.parse().unwrap();
                    Token {
                        //when parsing, note for other chars such as ;. there is a need to check
                        //each character in the token is actually an int.
                        literal: num.to_string(),
                        tok_type: TokenType::Int(num),
                    }
                } else {
                    Token {
                        tok_type: TokenType::Illegal,
                        literal: "".to_string(),
                    }
                }
            }
        }
    }
}

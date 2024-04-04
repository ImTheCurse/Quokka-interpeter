use crate::ast;
use crate::Lexer;
use crate::TokenType;

pub struct Parser {
    pub lexer: Lexer,
    pub curr_token: TokenType,
    pub peek_token: TokenType,
}

impl Parser {
    fn new(lex: Lexer) -> Parser {
        let mut p = Parser {
            lexer: lex,
            curr_token: TokenType::EOF,
            peek_token: TokenType::EOF,
        };
        p.next_token_parser();
        p.next_token_parser();

        p
    }

    fn next_token_parser(&mut self) {
        let tok = self.lexer.next_token().tok_type;
        self.curr_token = self.peek_token;
        self.peek_token = tok;
    }
}

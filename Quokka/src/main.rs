use crate::lexer::lexer::Lexer;
use crate::token::token::TokenType;
use crate::AST::ast;
use std::io::{self, Write};

pub(crate) mod AST;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod token;

fn main() -> io::Result<()> {
    let mut input = String::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut lex = Lexer::new(
            &mut Lexer {
                ch: ' ',
                input: input.clone(),
            },
            input.clone(),
        );
        let mut tok = lex.next_token();
        while tok.tok_type != TokenType::EOF {
            println!("{}", tok);
            tok = lex.next_token();
        }
        input.clear();
    }
}

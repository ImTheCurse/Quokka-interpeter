use crate::AST::ast;
use crate::{lexer::lexer::Lexer, parser::parser::Parser};
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
        let lex = Lexer::new(
            &mut Lexer {
                ch: ' ',
                input: input.clone(),
            },
            input.clone(),
        );
        let mut parser = Parser::new(lex);
        let program = parser.parse_program();

        if parser.errors().len() != 0 {
            print_parser_errors(parser.errors);
            continue;
        }
        println!(
            "{}\n",
            program
                .unwrap_or(ast::Program {
                    statments: Vec::new()
                })
                .to_string()
        );
        input.clear();

        /*
        let mut tok = lex.next_token();
        while tok.tok_type != TokenType::EOF {
            println!("{}", tok);
            tok = lex.next_token();
        }
        input.clear();
        */
    }
}

fn print_parser_errors(errors: Vec<String>) {
    let monkey = "     
            __,__
   .--.  .-'     '-.  .--.
  / .. \\/  .-. .-.  \\/ .. \
 |
 | \\   \\  \\ 0 | 0 /  /   / |
  \\ '- ,\\.-'`` ``'-./, -' /
   `'-' /_   ^ ^   _\\ '-'`
       |  \\._   _./  |
       \\   \\ `~` /   /
        '._ '-=-' _.'
           '~---~'
           ";
    println!("{}", monkey);
    println!("Whoops! We ran into some monkey business here!\n");
    println!("parser errors:\n");
    for msg in &errors {
        println!("\t{}\n", msg);
    }
}

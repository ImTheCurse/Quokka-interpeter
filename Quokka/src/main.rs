use crate::evaluator::eval::eval;
use crate::evaluator::object::{Enviornment, Object};
use crate::AST::ast::Program;
use crate::{lexer::lexer::Lexer, parser::parser::Parser};
use std::io::{self, Write};

pub(crate) mod AST;
pub(crate) mod evaluator;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod token;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut env = Enviornment::new();

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

        for s in program
            .unwrap_or(Program {
                statments: Vec::new(),
            })
            .statments
            .iter()
        {
            let evaluated = eval(s, &mut env);
            if evaluated.is_some() {
                println!("{}", evaluated.clone().unwrap().to_string());
                if let Object::Error(_) = evaluated.clone().unwrap() {
                    break;
                }
                if let Object::ReturnValue(_) = evaluated.clone().unwrap() {
                    break;
                }
            }
        }

        input.clear()
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

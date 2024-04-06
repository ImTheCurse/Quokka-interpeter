use crate::token::token::{Token, TokenType};
use crate::Lexer;
use crate::AST::ast::{Expression, Identifier, LetStatment, Literal, Program, Statment};

#[derive(Clone)]
pub struct Parser {
    pub lexer: Lexer,
    pub curr_token: Token,
    pub peek_token: Token,
}

impl<'a> Parser {
    pub fn new(lex: Lexer) -> Parser {
        let mut tok = Token{
            literal: "".to_string(),
            tok_type: TokenType::EOF,
        };
        let mut p = Parser {
            lexer: lex,
            curr_token: tok.clone(),
            peek_token: tok.clone(),
        };
        p.next_token_parser();
        p.next_token_parser();

        p
    }

    pub fn next_token_parser(&mut self) {
        let tok = self.lexer.next_token();
        self.curr_token = self.peek_token.clone();
        self.peek_token = tok;
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statments: Vec::new(),
        };

        while self.curr_token.tok_type != TokenType::EOF {
            if let Some(statment) = self.parse_statment(self.curr_token.clone()) {
                program.statments.push(statment);
            }
            self.next_token_parser();
        }
        Some(program)
    }

    pub fn parse_statment(&mut self, curr_tok: Token) -> Option<Statment> {
        return match curr_tok.tok_type {
            TokenType::Let => self.parse_let_statment(),
            _ => None,
        };
    }

    fn parse_let_statment(&mut self) -> Option<Statment> {
        //Current state:
        //curr_token = let
        //peek_token = identifier.
        if self.peek_token.tok_type != TokenType::Ident {
            return None;
        }
        let ident = Identifier {
            value: self.peek_token.literal.to_string(),
        };
        self.next_token_parser();

        //State:
        //curr_token = identifier
        //peek_token = assignment

        if self.peek_token.tok_type != TokenType::Assign {
            return None;
        }
        self.next_token_parser();

        //TODO: we're skipping the expression untill we encounter semicolon.
        //TODO: Notice to change expression aswell
        //state:
        //curr_token = assignment
        //peek_token = value / expression
        let val = &self.peek_token.literal;
        if self.peek_token.tok_type != TokenType::Int(val.parse::<i32>().unwrap()) {
            return None;
        }
        let lit = Literal {
            value: val.to_string(),
        };
        let stmt = LetStatment {
            ident: ident,
            value: Expression::Literal(lit),
        };
        while self.curr_token.tok_type != TokenType::Semicolon {
            self.next_token_parser();
        }
        return Some(Statment::Let(stmt));
    }
}

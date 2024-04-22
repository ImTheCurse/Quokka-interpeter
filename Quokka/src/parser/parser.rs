use crate::token::token::{Token, TokenType};
use crate::Lexer;
use crate::AST::ast::{
    Boolen, Expression, Identifier, Infix, InfixExpression, IntLiteral, LetStatment, Literal,
    PrefixExpression, Program, Statment,
};
use c_enum::c_enum;
use std::fmt::Write;

#[derive(Clone)]
pub struct Parser {
    pub lexer: Lexer,
    pub curr_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

c_enum! {
#[derive(PartialEq,PartialOrd, Clone, Copy, Eq)]
    pub enum Precedence :i32 {
        Lowest = 1,
        Equals,      //==
        LessGreater, //< or >
        Sum,         // + or -
        Product,     //*
        Prefix,
        Call,        //func(x)

    }
}

impl Parser {
    pub fn new(lex: Lexer) -> Parser {
        let tok = Token {
            literal: "".to_string(),
            tok_type: TokenType::EOF,
        };
        let mut p = Parser {
            lexer: lex,
            curr_token: tok.clone(),
            peek_token: tok.clone(),
            errors: vec![],
        };
        p.next_token_parser();
        p.next_token_parser();

        p
    }

    fn curr_token_is(&self, tok: &TokenType) -> bool {
        self.curr_token.tok_type == *tok
    }

    fn token_to_precedence(tok: &TokenType) -> Precedence {
        match *tok {
            TokenType::EQ | TokenType::NotEQ => Precedence::Equals,
            TokenType::Rarrow | TokenType::Larrow => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Asterisk | TokenType::Fslash => Precedence::Product,
            TokenType::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.peek_token.tok_type)
    }

    fn next_token_is(&self, tok: &TokenType) -> bool {
        self.peek_token.tok_type == *tok
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
            TokenType::Return => self.parse_return_statments(),
            TokenType::Illegal => None,
            _ => self.parse_expr_statments(),

            _ => None,
        };
    }

    fn parse_expr_statments(&mut self) -> Option<Statment> {
        return match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&TokenType::Semicolon) {
                    self.next_token_parser();
                }
                Some(Statment::Expr(expr))
            }
            None => None,
        };
    }

    fn parse_bool_expr(&mut self) -> Expression {
        let bol = Boolen {
            tok_type: if self.curr_token_is(&TokenType::True) {
                TokenType::True
            } else {
                TokenType::False
            },
            value: self.curr_token_is(&TokenType::True),
        };
        Expression::BoolenExpr(bol)
    }

    fn parse_expr(&mut self, prec: Precedence) -> Option<Expression> {
        // prefix
        let mut lhs = match self.curr_token.tok_type {
            TokenType::Ident => self.parse_ident(),
            TokenType::Int(num) => self.parse_int(num),
            TokenType::Not => self.parse_prefix_expr(),
            TokenType::Minus => self.parse_prefix_expr(),
            TokenType::Plus => self.parse_prefix_expr(),
            TokenType::True | TokenType::False => self.parse_bool_expr(),
            _ => self.prefix_error(),
        };

        //infix

        while !self.next_token_is(&TokenType::Semicolon) && prec < self.next_token_precedence() {
            match self.peek_token.tok_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Fslash
                | TokenType::Asterisk
                | TokenType::EQ
                | TokenType::NotEQ
                | TokenType::Larrow
                | TokenType::Rarrow => {
                    self.next_token_parser();
                    lhs = self.parse_infix_expr(&lhs);
                }
                _ => return Some(lhs),
            };
        }
        Some(lhs)
    }
    fn parse_infix_expr(&mut self, left: &Expression) -> Expression {
        let curr_expr = Expression::Blank;
        let mut infix = InfixExpression {
            tok_type: self.curr_token.tok_type,
            operator: self.curr_token.literal.clone(),
            lhs: left.clone(),
            rhs: curr_expr,
        };
        let prec = Self::token_to_precedence(&self.curr_token.tok_type);
        self.next_token_parser();
        infix.rhs = self.parse_expr(prec).unwrap_or(Expression::Blank);
        Expression::Infix(Box::new(infix))
    }

    fn parse_prefix_expr(&mut self) -> Expression {
        let current_expr = Expression::Blank;
        let mut prefix_expr = PrefixExpression {
            tok_type: self.curr_token.tok_type,
            operator: self.curr_token.literal.clone(),
            rhs: current_expr,
        };

        self.next_token_parser();
        prefix_expr.rhs = self
            .parse_expr(Precedence::Prefix)
            .unwrap_or(Expression::Blank);
        return Expression::Prefix(Box::new(prefix_expr));
    }

    fn parse_int(&mut self, num: i32) -> Expression {
        let expr = Expression::Int(IntLiteral { value: num });
        expr
    }

    fn parse_ident(&mut self) -> Expression {
        let ident = self.curr_token.literal.clone();
        let expr = Expression::Identifier(Identifier {
            value: ident.to_string(),
        });
        expr
    }

    fn prefix_error(&mut self) -> Expression {
        panic!(
            "Prefix is incorrect, no prefix function to parse current prefix. got: {}",
            self.curr_token.literal
        );
    }

    fn parse_let_statment(&mut self) -> Option<Statment> {
        //expect_peek() also gets the next token
        //Current state:
        //curr_token = let
        //peek_token = identifier.
        if self.peek_token.tok_type != TokenType::Ident {
            self.peek_error(TokenType::Ident);
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
            self.peek_error(TokenType::Assign);
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

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn peek_error(&mut self, tok: TokenType) {
        let mut message: String = String::new();
        write!(
            message,
            "Expected next token: {}, got: {}",
            tok, self.peek_token.tok_type
        );
        self.errors.push(message);
    }

    pub fn parse_return_statments(&mut self) -> Option<Statment> {
        todo!()
    }
}

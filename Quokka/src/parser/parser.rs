use crate::token::token::{Token, TokenType};
use crate::Lexer;
use crate::AST::ast::{
    BlockStatment, Boolen, CallExpression, Expression, FunctionLiteral, Identifier, IfStatment,
    InfixExpression, IntLiteral, LetStatment, PrefixExpression, Program, ReturnStatment, Statment,
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
            TokenType::Lparen => self.parse_grouped_expr(),
            TokenType::If => self.parse_if_expr()?,
            TokenType::Function => self.parse_func_literal()?,
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
                TokenType::Lparen => {
                    self.next_token_parser();
                    lhs = self.parse_call_expr(&lhs);
                }
                _ => return Some(lhs),
            };
        }
        Some(lhs)
    }

    fn parse_call_expr(&mut self, func: &Expression) -> Expression {
        let expr = CallExpression {
            arguments: self.parse_call_arguments(),
            function: func.clone(),
        };
        Expression::Call(Box::new(expr))
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();
        if self.next_token_is(&TokenType::Rparen) {
            self.next_token_parser();
            return args;
        }
        self.next_token_parser();
        args.push(
            self.parse_expr(Precedence::Lowest)
                .unwrap_or(Expression::Blank),
        );

        while !self.next_token_is(&TokenType::Rparen) {
            self.next_token_parser();
            self.next_token_parser();
            args.push(
                self.parse_expr(Precedence::Lowest)
                    .unwrap_or(Expression::Blank),
            );
        }
        if self.next_token_is(&TokenType::Rparen) {
            self.next_token_parser();
        }
        return args;
    }

    fn parse_func_literal(&mut self) -> Option<Expression> {
        let mut lit = FunctionLiteral {
            params: Vec::new(),
            body: BlockStatment { stmts: Vec::new() },
        };
        if !self.next_token_is(&TokenType::Lparen) {
            return None;
        }
        self.next_token_parser();
        lit.params = self.parse_func_param().unwrap_or(Vec::new());

        if !self.next_token_is(&TokenType::Lbrack) {
            return None;
        }
        self.next_token_parser();
        lit.body = *self.parse_block_statment();
        return Some(Expression::Func(lit));
    }

    fn parse_func_param(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();
        if self.next_token_is(&TokenType::Rparen) {
            self.next_token_parser();
            return None;
        }
        self.next_token_parser();
        let ident = Identifier {
            value: self.curr_token.literal.clone(),
        };
        identifiers.push(ident);

        while self.next_token_is(&TokenType::Comma) {
            self.next_token_parser();
            self.next_token_parser();
            let ident = Identifier {
                value: self.curr_token.literal.clone(),
            };
            identifiers.push(ident);
        }
        if !self.next_token_is(&TokenType::Rparen) {
            return None;
        }
        self.next_token_parser();
        return Some(identifiers);
    }

    fn parse_if_expr(&mut self) -> Option<Expression> {
        let consq_block = BlockStatment { stmts: Vec::new() };

        let mut expr = IfStatment {
            condition: Expression::Blank,
            consequence: consq_block,
            alternative: None,
        };
        if self.peek_token.tok_type != TokenType::Lparen {
            return None;
        }
        self.next_token_parser();
        self.next_token_parser();
        expr.condition = self.parse_expr(Precedence::Lowest)?;
        if self.peek_token.tok_type != TokenType::Rparen {
            return None;
        }
        self.next_token_parser();
        if self.peek_token.tok_type != TokenType::Lbrack {
            return None;
        }
        self.next_token_parser();
        expr.consequence = *self.parse_block_statment();

        if self.next_token_is(&TokenType::Else) {
            self.next_token_parser();
            if self.peek_token.tok_type != TokenType::Lbrack {
                return None;
            }
            self.next_token_parser();
            expr.alternative = Some(*self.parse_block_statment());
        }

        return Some(Expression::If(Box::new(expr)));
    }

    fn parse_block_statment(&mut self) -> Box<BlockStatment> {
        let mut block = BlockStatment { stmts: Vec::new() };
        self.next_token_parser();
        while !self.curr_token_is(&TokenType::Rbrack) && !self.curr_token_is(&TokenType::EOF) {
            let stmt = self.parse_statment(self.curr_token.clone());
            if stmt.is_some() {
                block
                    .stmts
                    .push(stmt.unwrap_or(Statment::Expr(Expression::Blank)));
            }
            self.next_token_parser();
        }
        return Box::new(block);
    }

    fn parse_grouped_expr(&mut self) -> Expression {
        self.next_token_parser();
        let expr = self.parse_expr(Precedence::Lowest);
        if self.peek_token.tok_type != TokenType::Rparen {
            return Expression::Blank;
        }
        self.next_token_parser();

        return expr.unwrap_or(Expression::Blank);
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
        let s = "Prefix is incorrect, no prefix function to parse current prefix. got: "
            .to_string()
            + &self.curr_token.literal.to_string();
        self.errors.push(s);
        Expression::Blank
    }

    fn parse_let_statment(&mut self) -> Option<Statment> {
        let mut stmt = LetStatment {
            ident: Identifier {
                value: "".to_string(),
            },
            value: Expression::Blank,
        };
        if !self.next_token_is(&TokenType::Ident) {
            self.next_token_parser();
            return None;
        }
        self.next_token_parser();

        stmt.ident = Identifier {
            value: self.curr_token.literal.clone(),
        };
        if !self.next_token_is(&TokenType::Assign) {
            self.next_token_parser();
            return None;
        }
        self.next_token_parser();
        self.next_token_parser();

        stmt.value = self.parse_expr(Precedence::Lowest)?;
        if self.next_token_is(&TokenType::Semicolon) {
            self.next_token_parser();
        }
        Some(Statment::Let(stmt))
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
        let mut stmt = ReturnStatment {
            return_value: Expression::Blank,
        };

        self.next_token_parser();
        stmt.return_value = self.parse_expr(Precedence::Lowest)?;

        if self.next_token_is(&TokenType::Semicolon) {
            self.next_token_parser();
        }
        Some(Statment::Return(stmt))
    }
}

use crate::TokenType;

pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

pub enum Statment {
    Let(LetStatment),
}

pub struct Program {
    statments: Vec<Statment>,
}

pub struct Literal {
    pub value: String,
}

pub struct LetStatment {
    pub token: TokenType, // Let token
    pub ident: Identifier,
    pub value: Expression,
}

pub struct Identifier {
    pub value: String,
    pub token: TokenType, //Ident token
}

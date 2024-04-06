use crate::TokenType;

#[derive(Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Clone)]
pub enum Statment {
    Let(LetStatment),
}

#[derive(Clone)]
pub struct Program {
    pub statments: Vec<Statment>,
}

#[derive(Clone)]
pub struct Literal {
    pub value: String,
}

#[derive(Clone)]
pub struct LetStatment {
    pub ident: Identifier,
    pub value: Expression,
}

#[derive(Clone)]
pub struct Identifier {
    pub value: String,
}

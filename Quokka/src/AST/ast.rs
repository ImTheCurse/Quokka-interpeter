use std::fmt::{write, Display};

#[derive(Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Clone)]
pub enum Statment {
    Let(LetStatment),
    Return(ReturnStatment),
    Expression(ExpressionStatment),
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

#[derive(Clone)]
pub struct ReturnStatment {
    pub return_value: Expression,
}

#[derive(Clone)]
pub struct ExpressionStatment {
    pub expr: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Not,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit.value),
            Expression::Identifier(ident) => write!(f, "{}", ident.value),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for LetStatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "let {} = {};",
            self.ident.to_string(),
            self.value.to_string()
        )
    }
}

impl Display for ReturnStatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {}", self.return_value.to_string());
        write!(f, ";")
    }
}

impl Display for Statment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Statment::Let(l) => write!(f, "{}", l.to_string()),
            Statment::Return(ret) => write!(f, "{}", ret.to_string()),
            Statment::Expression(expr) => write!(f, "{}", expr.expr),
        };
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statments {
            write!(f, "{}", stmt.to_string());
        }
        write!(f, "")
    }
}

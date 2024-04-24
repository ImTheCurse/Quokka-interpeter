use std::fmt::Display;

use crate::token::token::TokenType;

#[derive(Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Int(IntLiteral),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    BoolenExpr(Boolen),
    If(Box<IfStatment>),
    Blank,
}

#[derive(Clone)]
pub enum Statment {
    Let(LetStatment),
    Return(ReturnStatment),
    Expr(Expression),
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
pub struct IntLiteral {
    pub value: i32,
}

#[derive(Clone)]
pub struct Boolen {
    pub value: bool,
    pub tok_type: TokenType,
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
pub struct IfStatment {
    pub condition: Expression,
    pub consequence: BlockStatment,
    pub alternative: Option<BlockStatment>,
}

#[derive(Clone)]
pub struct BlockStatment {
    pub stmts: Vec<Statment>,
}

#[derive(Clone)]
pub struct ExpressionStatment {
    pub expr: Expression,
}

#[derive(Clone)]
pub struct PrefixExpression {
    pub tok_type: TokenType,
    pub operator: String,
    pub rhs: Expression,
}
#[derive(Clone)]
pub struct InfixExpression {
    pub tok_type: TokenType,
    pub lhs: Expression,
    pub operator: String,
    pub rhs: Expression,
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

impl Display for IfStatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.condition, self.consequence);

        if self.alternative.is_some() {
            write!(f, "{}", self.alternative.clone().unwrap());
        }
        Ok(())
    }
}
impl Display for BlockStatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in self.stmts.iter() {
            write!(f, "{}", stmt);
        }
        Ok(())
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rhs.to_string())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit.value),
            Expression::Identifier(ident) => write!(f, "{}", ident.value),
            Expression::Int(num) => write!(f, "{}", num.value),
            Expression::Prefix(p_ex) => {
                write!(f, "({}", p_ex.operator);
                write!(f, "{})", p_ex.rhs)
            }
            Expression::Infix(i_ex) => {
                write!(f, "({}", i_ex.lhs);
                write!(f, " {} ", i_ex.operator);
                write!(f, "{})", i_ex.rhs)
            }
            Expression::Blank => write!(f, ""),
            Expression::BoolenExpr(bool) => write!(f, "{}", bool.value),
            Expression::If(stmt) => write!(f, "{}", stmt),
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
            Statment::Expr(expr) => write!(f, "{}", expr),
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

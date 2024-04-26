use crate::evaluator::object::Object;
use crate::AST::ast::{Expression, LetStatment, Program, ReturnStatment, Statment};

pub fn eval(stmt: &Statment) -> Option<Object> {
    match stmt {
        Statment::Expr(e) => eval_expr(e),
        Statment::Let(l) => eval_let_stmt(l),
        Statment::Return(r) => eval_return_stmt(r),
    }
}
pub fn eval_expr(expr: &Expression) -> Option<Object> {
    match expr {
        Expression::Int(i) => return Some(Object::Integer(i.value)),
        Expression::BoolenExpr(b) => return Some(Object::Boolean(b.value)),
        _ => return None,
    };
}

pub fn eval_let_stmt(s: &LetStatment) -> Option<Object> {
    todo!()
}

pub fn eval_return_stmt(r: &ReturnStatment) -> Option<Object> {
    todo!()
}

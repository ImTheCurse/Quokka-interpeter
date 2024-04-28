use crate::evaluator::object::Object;
use crate::AST::ast::{Expression, LetStatment, ReturnStatment, Statment};

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
        Expression::Prefix(pre) => {
            let right = eval_expr(&pre.rhs);
            return eval_prefix_expr(&pre.operator, &right.unwrap());
        }
        _ => return None,
    };
}

pub fn eval_prefix_expr(op: &str, rhs: &Object) -> Option<Object> {
    match op {
        "!" => return eval_bang_expr(&rhs),
        _ => None,
    }
}

pub fn eval_bang_expr(rhs: &Object) -> Option<Object> {
    match rhs {
        Object::Boolean(b) => return Some(Object::Boolean(!b)),
        Object::Integer(val) => {
            return Some(Object::Boolean(if val.is_positive() {
                false
            } else {
                true
            }));
        }
        _ => None,
    }
}

pub fn eval_let_stmt(s: &LetStatment) -> Option<Object> {
    todo!()
}

pub fn eval_return_stmt(r: &ReturnStatment) -> Option<Object> {
    todo!()
}

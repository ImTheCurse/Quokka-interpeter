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
            return eval_prefix_expr(&pre.operator, &right.unwrap_or(Object::Null));
        }
        Expression::Infix(infix) => {
            let lhs = eval_expr(&infix.lhs);
            let rhs = eval_expr(&infix.rhs);

            return eval_infix_expr(
                &lhs.unwrap_or(Object::Null),
                &rhs.unwrap_or(Object::Null),
                &infix.operator,
            );
        }
        _ => return None,
    };
}

pub fn eval_infix_expr(lhs: &Object, rhs: &Object, op: &str) -> Option<Object> {
    if let Object::Integer(first) = rhs {
        if let Object::Integer(sec) = lhs {
            return eval_int_infix_expr(*sec, *first, op);
        }
    }
    None
}

pub fn eval_int_infix_expr(lhs: i32, rhs: i32, op: &str) -> Option<Object> {
    match op {
        "+" => return Some(Object::Integer(lhs + rhs)),
        "-" => return Some(Object::Integer(lhs - rhs)),
        "/" => {
            if rhs == 0 && lhs == 0 || rhs == 0 && lhs != 0 {
                return None;
            }
            return Some(Object::Integer(lhs / rhs));
        }
        "*" => return Some(Object::Integer(lhs * rhs)),
        _ => None,
    }
}

pub fn eval_prefix_expr(op: &str, rhs: &Object) -> Option<Object> {
    match op {
        "!" => return eval_bang_expr(&rhs),
        "-" => return eval_minus_prefix(&rhs),
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

pub fn eval_minus_prefix(rhs: &Object) -> Option<Object> {
    if let Object::Integer(i) = rhs {
        return Some(Object::Integer(-i));
    }
    None
}

pub fn eval_let_stmt(s: &LetStatment) -> Option<Object> {
    todo!()
}

pub fn eval_return_stmt(r: &ReturnStatment) -> Option<Object> {
    todo!()
}

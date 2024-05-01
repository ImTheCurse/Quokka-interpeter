use crate::evaluator::object::Object;
use crate::AST::ast::{BlockStatment, Expression, Identifier, IfStatment, LetStatment, Statment};

use super::object::{Enviornment, Obj};
use crate::new_error;

pub fn eval(stmt: &Statment, env: &mut Enviornment) -> Option<Object> {
    match stmt {
        Statment::Expr(e) => Some(eval_expr(e, env)),
        Statment::Let(l) => eval_let_stmt(l, env),
        Statment::Return(r) => Some(Object::ReturnValue(Box::new(eval_expr(
            &r.return_value,
            env,
        )))),
    }
}
fn eval_expr(expr: &Expression, env: &mut Enviornment) -> Object {
    match expr {
        Expression::Int(i) => return Object::Integer(i.value),
        Expression::BoolenExpr(b) => return Object::Boolean(b.value),
        Expression::Prefix(pre) => {
            let right = eval_expr(&pre.rhs, env);
            return eval_prefix_expr(&pre.operator, &right);
        }
        Expression::Infix(infix) => {
            let lhs = eval_expr(&infix.lhs, env);
            let rhs = eval_expr(&infix.rhs, env);

            return eval_infix_expr(&lhs, &rhs, &infix.operator);
        }
        Expression::If(if_stmt) => return eval_if_expr(if_stmt, env),
        Expression::Identifier(ident) => return eval_ident(ident.clone(), env),
        _ => return Object::Error("unknown expression, @eval_expr".to_string()),
    };
}

fn eval_ident(ident: Identifier, env: &mut Enviornment) -> Object {
    env.get(&ident.to_string())
}

fn eval_if_expr(stmt: &IfStatment, env: &mut Enviornment) -> Object {
    let cond = eval_expr(&stmt.condition, env);
    if is_truthy(&cond) {
        return eval_statments(&stmt.consequence.stmts, env);
    }
    if stmt.alternative.is_some() {
        return eval_statments(
            &stmt
                .alternative
                .clone()
                .unwrap_or(BlockStatment { stmts: Vec::new() })
                .stmts,
            env,
        );
    }
    Object::Null
}

fn eval_statments(stmts: &Vec<Statment>, env: &mut Enviornment) -> Object {
    let mut result = Some(Object::Null);
    for stmt in stmts {
        result = eval(stmt, env);

        if result.is_some() {
            if let Object::ReturnValue(v) = result.clone().unwrap_or(Object::Null) {
                return Object::ReturnValue(v);
            }
            if let Object::Error(err) = result.clone().unwrap_or(Object::Null) {
                return Object::Error(err);
            }
        }
    }
    result.unwrap_or(Object::Null)
}

fn is_truthy(obj: &Object) -> bool {
    if let Object::Integer(i) = obj {
        if i.is_positive() {
            return true;
        }
    }
    if let Object::Boolean(b) = obj {
        return *b;
    }
    return false;
}

fn eval_infix_expr(lhs: &Object, rhs: &Object, op: &str) -> Object {
    if let Object::Integer(first) = rhs {
        if let Object::Integer(sec) = lhs {
            return eval_int_infix_expr(*sec, *first, op);
        }
    }
    if lhs.Type() != rhs.Type() {
        return create_new_error(new_error!(
            "type mismatch:".to_string(),
            lhs.Type(),
            op.to_string(),
            rhs.Type()
        ));
    }
    match op {
        "==" => return Object::Boolean(lhs == rhs),
        "!=" => return Object::Boolean(lhs != rhs),
        _ => {
            return create_new_error(new_error!(
                "unknown operator:".to_string(),
                lhs.Type(),
                op.to_string(),
                rhs.Type()
            ));
        }
    };
}

fn eval_int_infix_expr(lhs: i32, rhs: i32, op: &str) -> Object {
    match op {
        "+" => Object::Integer(lhs + rhs),
        "-" => Object::Integer(lhs - rhs),
        "/" => {
            if rhs == 0 && lhs == 0 || rhs == 0 && lhs != 0 {
                return Object::Error("Division by zero is not allowed.".to_string());
            }
            return Object::Integer(lhs / rhs);
        }
        "*" => Object::Integer(lhs * rhs),
        "<" => Object::Boolean(lhs < rhs),
        ">" => Object::Boolean(lhs > rhs),
        "==" => Object::Boolean(lhs == rhs),
        "!=" => Object::Boolean(lhs != rhs),
        _ => {
            return create_new_error(new_error!(
                "unknown operator: ",
                lhs.to_string(),
                op.to_string(),
                rhs.to_string()
            ))
        }
    }
}

fn eval_prefix_expr(op: &str, rhs: &Object) -> Object {
    match op {
        "!" => return eval_bang_expr(&rhs),
        "-" => return eval_minus_prefix(&rhs),
        _ => create_new_error(new_error!("unknown operator: ", op.to_string(), rhs.Type())),
    }
}

fn eval_bang_expr(rhs: &Object) -> Object {
    match rhs {
        Object::Boolean(b) => return Object::Boolean(!b),
        Object::Integer(val) => {
            return Object::Boolean(if val.is_positive() { false } else { true });
        }

        _ => create_new_error(new_error!(
            "unknown operator: ",
            "!".to_string(),
            rhs.Type()
        )),
    }
}

fn eval_minus_prefix(rhs: &Object) -> Object {
    if let Object::Integer(i) = rhs {
        return Object::Integer(-i);
    }
    create_new_error(new_error!("unknown operator: -".to_string(), rhs.Type()))
}

fn eval_let_stmt(s: &LetStatment, env: &mut Enviornment) -> Option<Object> {
    let value = eval_expr(&s.value, env);
    if let Object::Error(_) = &value {
        return Some(value);
    }
    env.set(s.ident.to_string(), &value);
    Some(value)
}

#[macro_use]
mod error_macros {
    #[macro_export]
    macro_rules! new_error {
        ($s:expr) => {
            $s
        };
        ($s:expr,$($additonal_expr:expr),+) => {
            {
                let out : String = format!("{} {}",$s,new_error!($($additonal_expr),+));
                out
            }
        }
    }
}
fn create_new_error(message: String) -> Object {
    Object::Error(message)
}

use crate::AST::ast::FunctionLiteral;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

pub type ObjectType = String;

pub trait Obj {
    fn Type(&self) -> ObjectType;
}
#[derive(Clone, PartialEq, Eq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
    Function(FunctionLiteral, Enviornment),
}

#[derive(PartialEq, Eq, Clone)]
pub struct Enviornment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Enviornment>>>,
}

impl Enviornment {
    pub fn new() -> Enviornment {
        Enviornment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, ident: &String) -> Object {
        let obj = self.store.get(ident);
        match obj {
            Some(val) => val.clone(),
            None => match self.outer {
                Some(ref out) => Rc::try_unwrap(out.clone())
                    .unwrap_or(RefCell::new(Enviornment::new()))
                    .borrow_mut()
                    .get(ident),
                None => Object::Error(format!("identifier not found: {}", ident)),
            },
        }
    }

    pub fn set(&mut self, ident: String, obj: &Object) {
        self.store.insert(ident, obj.clone());
    }

    pub fn new_enclosed_env(outer: &mut Enviornment) -> Enviornment {
        let mut env = Enviornment::new();
        env.outer = Some(Rc::new(RefCell::new(outer.clone())));
        return env;
    }
}

impl Obj for Object {
    fn Type(&self) -> ObjectType {
        match *self {
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::Null => "NULL".to_string(),
            Object::ReturnValue(_) => "RETURN_VALUE".to_string(),
            Object::Error(_) => "ERROR".to_string(),
            Object::Function(_, _) => "FUNCTION".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "{}", "null"),
            Object::ReturnValue(val) => write!(f, "{}", val),
            Object::Error(err) => write!(f, "Error: {}", err),
            Object::Function(func, _) => {
                let mut params = Vec::new();
                for param in &func.params {
                    params.push(param.to_string());
                }
                write!(
                    f,
                    "fn({}){{\n{}\n}}",
                    params.join(", "),
                    func.body.to_string()
                )
            }
        }
    }
}

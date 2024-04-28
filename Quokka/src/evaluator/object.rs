use std::fmt::Display;

pub type ObjectType = String;

pub trait Obj<T>
where
    T: Display,
{
    fn r#type(&self) -> ObjectType;
}
#[derive(PartialEq, Eq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
}
impl<T: std::fmt::Display> Obj<T> for Object {
    fn r#type(&self) -> ObjectType {
        match *self {
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::Null => "NULL".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "{}", "null"),
        }
    }
}

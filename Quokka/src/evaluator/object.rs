use std::fmt::Display;

pub type ObjectType = String;

pub trait Obj<T>
where
    T: Display,
{
    fn r#type(&self) -> ObjectType;
}

pub enum Object {
    Integer(i32),
}

impl<T: std::fmt::Display> Obj<T> for Object {
    fn r#type(&self) -> ObjectType {
        match *self {
            Object::Integer(_) => "INTEGER".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Object::Integer(i) => write!(f, "{}", i),
        }
    }
}

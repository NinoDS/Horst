use std::fmt::{Display, Formatter};
use crate::vm::program::Program;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Function(Function),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Function(fun) => write!(f, "{}", fun),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => !b,
            _ => true
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub program: Program,
    pub arity: usize,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::program::Program;
    use crate::vm::value::{Function, Value};

    #[test]
    fn test_value_format() {
        // Number.
        assert_eq!(format!("{}", Value::Number(1.0)), "1".to_string());
        assert_eq!(format!("{}", Value::Number(4.2)), "4.2".to_string());

        // Boolean.
        assert_eq!(format!("{}", Value::Boolean(true)), "true".to_string());
        assert_eq!(format!("{}", Value::Boolean(false)), "false".to_string());

        // String.
        assert_eq!(format!("{}", Value::String("Hello, World!".to_string())), "Hello, World!".to_string());

        // Function.
        assert_eq!(format!("{}", Value::Function(Function{
            program: Program{
                instructions: vec![],
                constants: vec![]
            }, arity: 0
        })), "function".to_string());
    }
}

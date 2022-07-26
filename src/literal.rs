use std::fmt::Display;

#[derive(PartialEq)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Bool(bool) => bool.fmt(f),
            // Using the Display implementation of f64 causes numbers like
            // 5.0 to be printed as 5, causing tests to fail. The Debug implementation
            // doesn't do this, so we use that instead.
            Literal::Number(num) => write!(f, "{num:?}"),
            Literal::String(string) => string.fmt(f),
            Literal::Nil => write!(f, "null"),
        }
    }
}

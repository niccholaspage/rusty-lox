use std::fmt::Display;

pub enum Literal {
    Number(f64),
    String(String),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Number(num) => num.fmt(f),
            Literal::String(string) => string.fmt(f),
            Literal::Nil => write!(f, "null"),
        }
    }
}

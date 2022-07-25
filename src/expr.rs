use crate::{literal::Literal, token::Token};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

use crate::{literal::Literal, token::Token};

pub enum Expr<'a> {
    Binary {
        left: &'a Expr<'a>,
        operator: &'a Token,
        right: &'a Expr<'a>,
    },
    Grouping {
        expression: &'a Expr<'a>,
    },
    Literal(Literal),
    Unary {
        operator: &'a Token,
        right: &'a Expr<'a>,
    },
}

use crate::{literal::Literal, token::Token};

pub trait Visitor<R> {
    fn visit_expr(&mut self, expr: &Expr) -> R;
}

pub enum Expr<'a> {
    Assign {
        name: &'a Token,
        value: &'a Expr<'a>
    },
    Binary {
        left: &'a Expr<'a>,
        operator: &'a Token,
        right: &'a Expr<'a>,
    },
    Grouping {
        expression: &'a Expr<'a>,
    },
    Literal(&'a Literal),
    Unary {
        operator: &'a Token,
        right: &'a Expr<'a>,
    },
    Variable(&'a Token)
}

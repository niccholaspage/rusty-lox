use crate::{expr::Expr, token::Token};

pub trait Visitor<R> {
    fn visit_stmt(&mut self, stmt: &Stmt) -> R;
}

pub enum Stmt<'a> {
    Block {
        statements: Vec<Stmt<'a>>,
    },
    Expression(&'a Expr<'a>),
    Print(&'a Expr<'a>),
    Var {
        name: &'a Token,
        initializer: Option<&'a Expr<'a>>,
    },
}

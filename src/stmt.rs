use crate::expr::Expr;

pub trait Visitor<R> {
    fn visit_stmt(&mut self, stmt: &Stmt) -> R;
}

pub enum Stmt<'a> {
    Expression(&'a Expr<'a>),
    Print(&'a Expr<'a>)
}
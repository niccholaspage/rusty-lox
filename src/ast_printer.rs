use crate::expr::Expr;

use crate::literal::Literal;
use crate::visitor::Visitor;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        self.visit_expr(expr)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut string = format!("({name}");

        for expr in exprs {
            string.push(' ');
            string.push_str(&self.visit_expr(expr));
        }

        string.push(')');

        string
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary {
                ref left,
                operator,
                ref right,
            } => self.parenthesize(&operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => self.parenthesize("group", &[expression]),
            Expr::Literal(value) => {
                if value == &Literal::Nil {
                    "nil".to_string()
                } else {
                    format!("{value}")
                }
            }
            Expr::Unary {
                operator,
                ref right,
            } => self.parenthesize(&operator.lexeme, &[right]),
        }
    }
}
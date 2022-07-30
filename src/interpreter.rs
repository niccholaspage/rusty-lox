use std::cell::RefCell;

use crate::{
    expr::{self, Expr},
    literal::Literal,
    stmt::{self, Stmt},
    token::Token,
    token_type::TokenType,
    Context,
};

#[derive(PartialEq)]
enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

pub struct RuntimeError {
    pub token_line: usize,
    pub message: &'static str,
}

impl RuntimeError {
    fn new(token: &Token, message: &'static str) -> RuntimeError {
        RuntimeError {
            token_line: token.line,
            message,
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, context: &RefCell<Context>, statements: Vec<Stmt>) {
        for statement in statements {
            let result = self.execute(&statement);
            if let Err(error) = result {
                context.borrow_mut().runtime_error(error);
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr::Visitor::visit_expr(self, expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt::Visitor::visit_stmt(self, stmt)
    }

    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(bool) => *bool,
            _ => true,
        }
    }

    fn is_equal(a: &Value, b: &Value) -> bool {
        a == b
    }

    fn stringify(value: Value) -> String {
        match value {
            Value::Nil => "nil".to_string(),
            Value::Number(number) => format!("{number}"),
            Value::Bool(bool) => format!("{bool}"),
            Value::String(str) => str,
        }
    }

    fn check_number_operand(operator: &Token, operand: Value) -> Result<f64, RuntimeError> {
        match operand {
            Value::Number(num) => Ok(num),
            _ => Err(RuntimeError {
                token_line: operator.line,
                message: "Operand must be a number.",
            }),
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &Value,
        right: &Value,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok((*left, *right)),
            _ => Err(RuntimeError::new(operator, "Operands must be numbers.")),
        }
    }
}

impl expr::Visitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(literal) => {
                match literal {
                    Literal::Bool(bool) => Ok(Value::Bool(*bool)),
                    Literal::Number(f64) => Ok(Value::Number(*f64)),
                    Literal::String(string) => Ok(Value::String(string.clone())), // Optimize away this clone
                    Literal::Nil => Ok(Value::Nil),
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                if operator.r#type == TokenType::Bang {
                    return Ok(Value::Bool(!Interpreter::is_truthy(&right)));
                } else if operator.r#type == TokenType::Minus {
                    let number = Interpreter::check_number_operand(operator, right)?;
                    return Ok(Value::Number(-number));
                }

                // Unreachable
                todo!("Handle this case later!")
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.r#type {
                    TokenType::BangEqual => {
                        return Ok(Value::Bool(!Interpreter::is_equal(&left, &right)))
                    }
                    TokenType::EqualEqual => {
                        return Ok(Value::Bool(Interpreter::is_equal(&left, &right)))
                    }
                    TokenType::Greater => {
                        let (left, right) =
                            Interpreter::check_number_operands(operator, &left, &right)?;
                        return Ok(Value::Bool(left > right));
                    }
                    TokenType::GreaterEqual => {
                        let (left, right) =
                            Interpreter::check_number_operands(operator, &left, &right)?;
                        return Ok(Value::Bool(left >= right));
                    }
                    TokenType::Less => {
                        let (left, right) =
                            Interpreter::check_number_operands(operator, &left, &right)?;
                        return Ok(Value::Bool(left < right));
                    }
                    TokenType::LessEqual => {
                        let (left, right) =
                            Interpreter::check_number_operands(operator, &left, &right)?;
                        return Ok(Value::Bool(left <= right));
                    }
                    TokenType::Minus => {
                        return {
                            let (left, right) =
                                Interpreter::check_number_operands(operator, &left, &right)?;
                            Ok(Value::Number(left - right))
                        }
                    }
                    TokenType::Plus => {
                        let result = Interpreter::check_number_operands(operator, &left, &right);

                        if let Ok((left, right)) = result {
                            return Ok(Value::Number(left + right));
                        } else if let (Value::String(left), Value::String(right)) = &(&left, &right)
                        {
                            if operator.r#type == TokenType::Plus {
                                return Ok(Value::String(format!("{left}{right}")));
                            }
                        } else {
                            return Err(RuntimeError::new(
                                operator,
                                "Operands must be two numbers or two strings.",
                            ));
                        }
                    }
                    TokenType::Slash => {
                        let (left, right) =
                            Interpreter::check_number_operands(operator, &left, &right)?;
                        return Ok(Value::Number(left / right));
                    }
                    TokenType::Star => {
                        let (left, right) =
                            Interpreter::check_number_operands(operator, &left, &right)?;
                        return Ok(Value::Number(left * right));
                    }
                    _ => (),
                }

                // Unreachable
                todo!("Handle this case later!")
            }
        }
    }
}

impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expression) => {
                self.evaluate(expression)?;
                Ok(())
            },
            Stmt::Print(expression) => {
                let value = self.evaluate(expression)?;
                println!("{}", Interpreter::stringify(value));
                Ok(())
            }
        }
    }
}

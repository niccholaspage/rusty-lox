use std::cell::{RefCell, Cell};

use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, Context};

use typed_arena::Arena;

pub struct Parser<'a> {
    context: &'a RefCell<Context>,
    tokens: Vec<Token>,
    current: Cell<usize>,
}

struct ParseError;

const FALSE_LITERAL: Literal = Literal::Bool(false);
const TRUE_LITERAL: Literal = Literal::Bool(true);
const NIL_LITERAL: Literal = Literal::Nil;

impl<'a> Parser<'a> {
    pub fn new(context: &RefCell<Context>, tokens: Vec<Token>) -> Parser {
        Parser {
            context,
            tokens,
            current: Cell::new(0),
        }
    }

    pub fn parse(&mut self, arena: &'a Arena<Expr<'a>>) -> Option<&'a Expr> {
        if let Ok(expr) = self.expression(arena) {
            Some(expr)
        } else {
            None
        }
    }

    fn expression(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        self.equality(arena)
    }

    fn equality(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        let mut expr = self.comparison(arena)?;

        while self.r#match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison(arena)?;
            expr = arena.alloc(Expr::Binary {
                left: expr,
                operator: self.get_token_at_index(operator),
                right,
            });
        }

        Ok(expr)
    }

    fn comparison(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        let mut expr = self.term(arena)?;

        while self.r#match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term(arena)?;
            expr = arena.alloc(Expr::Binary {
                left: expr,
                operator: self.get_token_at_index(operator),
                right,
            });
        }

        Ok(expr)
    }

    fn term(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        let mut expr = self.factor(arena)?;

        while self.r#match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor(arena)?;
            expr = arena.alloc(Expr::Binary {
                left: expr,
                operator: self.get_token_at_index(operator),
                right,
            });
        }

        Ok(expr)
    }

    fn factor(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        let mut expr = self.unary(arena)?;

        while self.r#match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary(arena)?;
            expr = arena.alloc(Expr::Binary {
                left: expr,
                operator: self.get_token_at_index(operator),
                right,
            });
        }

        Ok(expr)
    }

    fn unary(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        if self.r#match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary(arena)?;
            return Ok(arena.alloc(Expr::Unary {
                operator: self.get_token_at_index(operator),
                right,
            }));
        }

        self.primary(arena)
    }

    fn primary(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        if self.r#match(&[TokenType::False]) {
            return Ok(arena.alloc(Expr::Literal(&FALSE_LITERAL)));
        }
        if self.r#match(&[TokenType::True]) {
            return Ok(arena.alloc(Expr::Literal(&TRUE_LITERAL)));
        }
        if self.r#match(&[TokenType::Nil]) {
            return Ok(arena.alloc(Expr::Literal(&NIL_LITERAL)));
        }

        if self.r#match(&[TokenType::Number, TokenType::String]) {
            return Ok(arena.alloc(Expr::Literal(&self.get_token_at_index(self.previous()).literal)));
        }

        if self.r#match(&[TokenType::LeftParen]) {
            let expr = self.expression(arena)?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(arena.alloc(Expr::Grouping {
                expression: expr,
            }));
        }

        Err(self.error("Expect expression."))
    }

    fn r#match(&self, types: &[TokenType]) -> bool {
        for r#type in types {
            if self.check(*r#type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&self, r#type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(r#type) {
            return Ok(self.advance());
        }

        Err(self.error(message))
    }

    fn error(&self, message: &str) -> ParseError {
        self.context.borrow_mut().error_with_token(self.peek(), message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.get_token_at_index(self.previous()).r#type == TokenType::Semicolon {
                return;
            }

            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn check(&self, r#type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().r#type == r#type
        }
    }

    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }

        self.get_token_at_index(self.previous())
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current.get()]
    }

    fn previous(&self) -> usize {
        self.current.get() - 1
    }

    fn get_token_at_index(&self, index: usize) -> &Token {
        &self.tokens[index]
    }
}

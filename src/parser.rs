use std::cell::{RefCell, Cell};

use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, Context, stmt::Stmt};

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

    pub fn parse(&'a mut self, arena: &'a Arena<Expr<'a>>) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let statement = self.declaration(arena);

            match statement {
                Some(statement) => statements.push(statement),
                None => ()
            }
        }

        Some(statements)
    }

    fn expression(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        self.assignment(arena)
    }

    fn declaration(&'a self, arena: &'a Arena<Expr<'a>>) -> Option<Stmt<'_>> {
        if self.r#match(&[TokenType::Var]) {
            return match self.var_declaration(arena) {
                Ok(statement) => Some(statement),
                Err(_) => {
                    self.synchronize();
                    None
                }
            };
        }

        match self.statement(arena) {
            Ok(statement) => Some(statement),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn statement(&'a self, arena: &'a Arena<Expr<'a>>) -> Result<Stmt<'_>, ParseError> {
        if self.r#match(&[TokenType::Print]) {
            return self.print_statement(arena);
        }

        if self.r#match(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block { statements: self.block(arena)? });
        }
 
        self.expression_statement(arena)
    }

    fn print_statement(&'a self, arena: &'a Arena<Expr<'a>>) -> Result<Stmt<'_>, ParseError> {
        let value = self.expression(arena)?;

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print(value))
    }

    fn var_declaration(&'a self, arena: &'a Arena<Expr<'a>>) -> Result<Stmt<'_>, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer = None;

        if self.r#match(&[TokenType::Equal]) {
            initializer = Some(self.expression(arena)?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, initializer })
    }

    fn expression_statement(&'a self, arena: &'a Arena<Expr<'a>>) -> Result<Stmt<'_>, ParseError> {
        let value = self.expression(arena)?;

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Expression(value))
    }

    fn block(&'a self, arena: &'a Arena<Expr<'a>>) -> Result<Vec<Stmt<'_>>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(statement) = self.declaration(arena) {
                statements.push(statement);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&self, arena: &'a Arena<Expr<'a>>) -> Result<&'a Expr, ParseError> {
        let expr = self.equality(arena)?;

        if self.r#match(&[TokenType::Equal]) {
          let equals = self.previous();
          let value = self.assignment(arena)?;

          if let Expr::Variable(name) = expr {
            return Ok(arena.alloc(Expr::Assign{ name, value }));
          }
    
          return Err(self.error(self.get_token_at_index(equals), "Invalid assignment target."));
        }
    
        Ok(expr)
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

        if self.r#match(&[TokenType::Identifier]) {
            return Ok(arena.alloc(Expr::Variable(self.get_token_at_index(self.previous()))));
        }

        if self.r#match(&[TokenType::LeftParen]) {
            let expr = self.expression(arena)?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(arena.alloc(Expr::Grouping {
                expression: expr,
            }));
        }

        Err(self.error(self.peek(), "Expect expression."))
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

        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        self.context.borrow_mut().error_with_token(token, message);
        ParseError
    }

    fn synchronize(&self) {
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

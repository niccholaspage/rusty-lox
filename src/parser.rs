use crate::{token::Token, expr::Expr, token_type::TokenType, literal::Literal, Context};

pub struct Parser<'a> {
    context: &'a mut Context,
    tokens: Vec<Token>,
    current: usize,
}

struct ParseError;

impl<'a> Parser<'a> {
    pub fn new(context: &'a mut Context, tokens: Vec<Token>) -> Parser<'a> {
        Parser { context, tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        if let Ok(expr) = self.expression() {
            Some(expr)
        } else {
            None
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.r#match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.r#match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
          let operator = self.previous();
          let right = self.term()?;
          expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
    
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.r#match(&[TokenType::Minus, TokenType::Plus]) {
          let operator = self.previous();
          let right = self.factor()?;
          expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
    
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.r#match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.r#match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.r#match(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.r#match(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.r#match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
    
        if self.r#match(&[TokenType::Number, TokenType::String]) {
          return Ok(Expr::Literal(self.previous().literal));
        }
    
        if self.r#match(&[TokenType::LeftParen]) {
          let expr = self.expression()?;
          self.consume(TokenType::RightParen, "Expect ')' after expression.");
          return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn r#match(&mut self, types: &[TokenType]) -> bool {
        for r#type in types {
            if self.check(*r#type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume(&mut self, r#type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(r#type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        self.context.error_with_token(token, message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return
            }

            match self.peek().r#type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
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

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}

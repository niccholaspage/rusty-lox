use crate::literal::Literal;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::Context;
use std::collections::HashMap;
use std::str;

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<Vec<u8>, TokenType>, // Optimize this to be static somehow
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        let mut keywords = HashMap::new();

        keywords.insert(b"and".to_vec(), TokenType::And);
        keywords.insert(b"class".to_vec(), TokenType::Class);
        keywords.insert(b"else".to_vec(), TokenType::Else);
        keywords.insert(b"false".to_vec(), TokenType::False);
        keywords.insert(b"for".to_vec(), TokenType::For);
        keywords.insert(b"fun".to_vec(), TokenType::Fun);
        keywords.insert(b"if".to_vec(), TokenType::If);
        keywords.insert(b"nil".to_vec(), TokenType::Nil);
        keywords.insert(b"or".to_vec(), TokenType::Or);
        keywords.insert(b"print".to_vec(), TokenType::Print);
        keywords.insert(b"return".to_vec(), TokenType::Return);
        keywords.insert(b"super".to_vec(), TokenType::Super);
        keywords.insert(b"this".to_vec(), TokenType::This);
        keywords.insert(b"true".to_vec(), TokenType::True);
        keywords.insert(b"var".to_vec(), TokenType::Var);
        keywords.insert(b"while".to_vec(), TokenType::While);

        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self, context: &mut Context) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(context);
        }

        self.tokens.push(Token {
            r#type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: Literal::Nil,
            line: self.line,
        });

        &self.tokens
    }

    fn scan_token(&mut self, context: &mut Context) {
        let c = self.advance();

        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            b'!' => {
                let token = self.match_check(b'=', TokenType::BangEqual, TokenType::Bang);
                self.add_token(token);
            }
            b'=' => {
                let token = self.match_check(b'=', TokenType::EqualEqual, TokenType::Equal);
                self.add_token(token);
            }
            b'<' => {
                let token = self.match_check(b'=', TokenType::LessEqual, TokenType::Less);
                self.add_token(token);
            }
            b'>' => {
                let token = self.match_check(b'=', TokenType::GreaterEqual, TokenType::Greater);
                self.add_token(token);
            }
            b'/' => {
                if self.r#match(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(context),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    context.error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let r#type = self.keywords.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token(*r#type);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number = str::from_utf8(&self.source[self.start..self.current]).unwrap();

        let number: f64 = number.parse().unwrap();

        self.add_token_with_literal(TokenType::Number, Literal::Number(number));
    }

    fn string(&mut self, context: &mut Context) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            context.error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_literal(
            TokenType::String,
            Literal::String(str::from_utf8(value).unwrap().to_string()),
        );
    }

    fn r#match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_alpha(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn is_alpha_numeric(c: u8) -> bool {
        Scanner::is_alpha(c) || c.is_ascii_digit()
    }

    fn match_check(
        &mut self,
        expected: u8,
        match_type: TokenType,
        no_match_type: TokenType,
    ) -> TokenType {
        if self.r#match(expected) {
            match_type
        } else {
            no_match_type
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let current = self.current;
        self.current += 1;
        self.source[current]
    }

    fn add_token(&mut self, r#type: TokenType) {
        self.add_token_with_literal(r#type, Literal::Nil);
    }

    fn add_token_with_literal(&mut self, r#type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            r#type,
            lexeme: str::from_utf8(text).unwrap().to_string(),
            literal,
            line: self.line,
        });
    }
}

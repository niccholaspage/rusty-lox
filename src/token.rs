use std::fmt::Display;

use crate::{token_type::TokenType, literal::Literal};

pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r#type, self.lexeme, self.literal)
    }
}

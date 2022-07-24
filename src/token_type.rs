use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Display for TokenType {
    // To allow us to pass the test suite, we need to implement Display
    // and return back the token type in screaming snake case.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug_name = format!("{:?}", self);

        let first_letter = &debug_name[0..1];

        let mut name_parts: Vec<String> = debug_name[1..].split_inclusive(char::is_uppercase).map(str::to_string).collect();

        for name in &mut name_parts {
            name.make_ascii_uppercase();
        }

        let display_name = name_parts.join("_");

        write!(f, "{first_letter}{display_name}")
    }
}
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

        let debug_indices = debug_name[1..].chars();

        let mut display_name = debug_name[..1].to_string();

        for char in debug_indices {
            if char.is_uppercase() {
                display_name.push('_');
                display_name.push(char);
            } else {
                display_name.push(char.to_ascii_uppercase());
            }
        }

        write!(f, "{}", display_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType{
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Equal,
    Dot,
    Comma,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParen,
    RightParen,

    // Keywords
    And,
    Or,
    False,
    Else,
    If,
    True,
    Let,
    Fn,

    String,
    Number,
    Identifier,
    Eof
}


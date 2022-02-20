use crate::common::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Special
    EndOfFile,
    Newline,
    Name(String),
    Integer(u128),

    // Keywords
    Let,

    // Brackets
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,

    // Symbols
    LeftArrow,
    RightArrow,

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    ExclamationMark,

    // Comparison Operators
    EqualEqual,
    ExclamationMarkEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    // Assignment Operators
    Equal,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: SourceLocation,
    pub length: usize,
}

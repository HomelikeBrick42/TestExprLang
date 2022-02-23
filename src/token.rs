use crate::common::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Special
    EndOfFile,
    Newline,
    Name(String),
    Integer(u128),

    // Keywords
    Export,
    Let,

    // Brackets
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,

    // Symbols
    LeftArrow,
    RightArrow,
    Comma,

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

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            // Special
            TokenKind::EndOfFile => "the end of file".to_string(),
            TokenKind::Newline => "a newline".to_string(),
            TokenKind::Name(_) => "a name".to_string(),
            TokenKind::Integer(_) => "an integer".to_string(),

            // Keywords
            TokenKind::Export => "export".to_string(),
            TokenKind::Let => "let".to_string(),

            // Brackets
            TokenKind::OpenParenthesis => "(".to_string(),
            TokenKind::CloseParenthesis => ")".to_string(),
            TokenKind::OpenBrace => "{".to_string(),
            TokenKind::CloseBrace => "}".to_string(),

            // Symbols
            TokenKind::LeftArrow => "<-".to_string(),
            TokenKind::RightArrow => "->".to_string(),
            TokenKind::Comma => ",".to_string(),

            // Operators
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Asterisk => "*".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::ExclamationMark => "!".to_string(),

            // Comparison Operators
            TokenKind::EqualEqual => "==".to_string(),
            TokenKind::ExclamationMarkEqual => "!=".to_string(),
            TokenKind::LessThan => "<".to_string(),
            TokenKind::GreaterThan => ">".to_string(),
            TokenKind::LessThanEqual => "<=".to_string(),
            TokenKind::GreaterThanEqual => ">=".to_string(),

            // Assignment Operators
            TokenKind::Equal => "=".to_string(),
            TokenKind::PlusEqual => "+=".to_string(),
            TokenKind::MinusEqual => "-=".to_string(),
            TokenKind::AsteriskEqual => "*=".to_string(),
            TokenKind::SlashEqual => "/=".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: SourceLocation,
    pub length: usize,
}

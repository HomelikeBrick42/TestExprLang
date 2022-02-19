use crate::common::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: SourceLocation,
    pub length: usize,
}

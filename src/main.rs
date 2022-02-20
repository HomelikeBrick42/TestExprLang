#![allow(dead_code)]

mod common;
mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod lexer_tests {
    use crate::{lexer::Lexer, token::TokenKind};

    #[test]
    fn empty_file() {
        let filepath = "Empty.fpl".to_string();
        let source = "";
        let mut lexer = Lexer::new(filepath.clone(), source);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::EndOfFile);
    }

    #[test]
    fn integer() {
        let filepath = "Integer.fpl".to_string();
        let source = "123 0x856 0d543 0b0100101 0o5674 0b135";
        let mut lexer = Lexer::new(filepath, source);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Integer(123));
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Integer(0x856));
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Integer(543));
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::Integer(0b0100101)
        );
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Integer(0o5674));
        lexer.next_token().unwrap_err();
        // TODO: allow the lexer to keep going after an error
    }

    #[test]
    fn name() {
        let filepath = "Integer.fpl".to_string();
        let source = "a123 _5_5aayufwuadvwuadvWADWauDYwYUDwa";
        let mut lexer = Lexer::new(filepath, source);
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::Name("a123".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::Name("_5_5aayufwuadvwuadvWADWauDYwYUDwa".to_string())
        );
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::EndOfFile);
    }
}

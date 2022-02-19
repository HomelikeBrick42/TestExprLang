#![allow(dead_code)]

mod common;
mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod lexer_checks {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenKind},
    };

    #[test]
    fn empty_file_test() {
        let filepath = "Empty.fpl".to_string();
        let mut lexer = Lexer::new(filepath.clone(), "");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                kind: TokenKind::EndOfFile,
                location: crate::common::SourceLocation {
                    filepath: filepath.clone(),
                    position: 0,
                    line: 1,
                    column: 1
                },
                length: 0
            }
        );
    }
}

#![allow(dead_code)]

mod ast;
mod common;
mod lexer;
mod parsing;
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

#[cfg(test)]
mod parser_tests {
    use crate::{ast::Ast, lexer::Lexer, parsing::parse_file, token::TokenKind};

    #[test]
    fn empty_file() {
        let filepath = "Empty.fpl".to_string();
        let source = "";
        let mut lexer = Lexer::new(filepath.clone(), source);
        let file = parse_file(&mut lexer).unwrap();
        assert_eq!(file.expressions.len(), 0);
        assert_eq!(file.end_of_file_token.kind, TokenKind::EndOfFile);
    }

    #[test]
    fn expression_test() {
        let filepath = "Expression.fpl".to_string();
        let source = "1 + 2 * 3";
        let mut lexer = Lexer::new(filepath.clone(), source);
        let file = parse_file(&mut lexer).unwrap();
        assert_eq!(file.expressions.len(), 1);
        assert_eq!(file.end_of_file_token.kind, TokenKind::EndOfFile);

        let binary_plus = if let Ast::Binary(binary) = &file.expressions[0] {
            binary
        } else {
            panic!("Not a binary operator")
        };
        assert_eq!(binary_plus.operator_token.kind, TokenKind::Plus);
        let integer_1 = if let Ast::Integer(integer) = &binary_plus.left as &Ast {
            integer
        } else {
            panic!("Not an integer")
        };
        assert_eq!(integer_1.integer_token.kind, TokenKind::Integer(1));
        let binary_asterisk = if let Ast::Binary(binary) = &binary_plus.right as &Ast {
            binary
        } else {
            panic!("Not a binary operator")
        };
        assert_eq!(binary_asterisk.operator_token.kind, TokenKind::Asterisk);
        let integer_2 = if let Ast::Integer(integer) = &binary_asterisk.left as &Ast {
            integer
        } else {
            panic!("Not an integer")
        };
        assert_eq!(integer_2.integer_token.kind, TokenKind::Integer(2));
        let integer_3 = if let Ast::Integer(integer) = &binary_asterisk.right as &Ast {
            integer
        } else {
            panic!("Not an integer")
        };
        assert_eq!(integer_3.integer_token.kind, TokenKind::Integer(3));
    }
}

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
    use crate::{lexer::Lexer, parsing::parse_file, token::TokenKind};

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

        let binary_plus = file.expressions[0].unwrap_binary();
        assert_eq!(binary_plus.operator_token.kind, TokenKind::Plus);

        let integer_1 = binary_plus.left.unwrap_integer();
        assert_eq!(integer_1.integer_token.kind, TokenKind::Integer(1));

        let binary_asterisk = binary_plus.right.unwrap_binary();
        assert_eq!(binary_asterisk.operator_token.kind, TokenKind::Asterisk);

        let integer_2 = binary_asterisk.left.unwrap_integer();
        assert_eq!(integer_2.integer_token.kind, TokenKind::Integer(2));

        let integer_3 = binary_asterisk.right.unwrap_integer();
        assert_eq!(integer_3.integer_token.kind, TokenKind::Integer(3));
    }

    #[test]
    fn let_test() {
        let filepath = "Let.fpl".to_string();
        let source = "
			let a
			let b = 5
		";
        let mut lexer = Lexer::new(filepath.clone(), source);
        let file = parse_file(&mut lexer).unwrap();
        assert_eq!(file.expressions.len(), 2);
        assert_eq!(file.end_of_file_token.kind, TokenKind::EndOfFile);

        let a = file.expressions[0].unwrap_let();
        assert_eq!(a.name_token.kind, TokenKind::Name("a".to_string()));
        assert_eq!(a.value, None);

        let b = file.expressions[1].unwrap_let();
        assert_eq!(b.name_token.kind, TokenKind::Name("b".to_string()));
        let b_value = b.value.clone().unwrap();
        let integer_5 = b_value.unwrap_integer();
        assert_eq!(integer_5.integer_token.kind, TokenKind::Integer(5));
    }

    #[test]
    fn block_test() {
        let filepath = "Block.fpl".to_string();
        let source = "
		let foo =
		{
			let a
			5
		}";
        let mut lexer = Lexer::new(filepath.clone(), source);
        let file = parse_file(&mut lexer).unwrap();
        assert_eq!(file.expressions.len(), 1);
        assert_eq!(file.end_of_file_token.kind, TokenKind::EndOfFile);

        let foo = file.expressions[0].unwrap_let();
        assert_eq!(foo.name_token.kind, TokenKind::Name("foo".to_string()));
        let foo_value = foo.value.clone().unwrap();

        let block = foo_value.unwrap_block();
        assert_eq!(block.expressions.len(), 2);

        let a = block.expressions[0].unwrap_let();
        assert_eq!(a.name_token.kind, TokenKind::Name("a".to_string()));
        assert_eq!(a.value, None);

        let integer_5 = block.expressions[1].unwrap_integer();
        assert_eq!(integer_5.integer_token.kind, TokenKind::Integer(5));
    }

    #[test]
    fn export_test() {
        let filepath = "Block.fpl".to_string();
        let source = "
		export foo =
		{
			let a
			export b = 5
		}";
        let mut lexer = Lexer::new(filepath.clone(), source);
        let file = parse_file(&mut lexer).unwrap();
        assert_eq!(file.expressions.len(), 1);
        assert_eq!(file.end_of_file_token.kind, TokenKind::EndOfFile);

        let foo_export = file.expressions[0].unwrap_export();
        assert_eq!(
            foo_export.name_token.kind,
            TokenKind::Name("foo".to_string())
        );

        let block = foo_export.value.unwrap_block();
        assert_eq!(block.expressions.len(), 2);

        let a = block.expressions[0].unwrap_let();
        assert_eq!(a.value, None);

        let export_5 = block.expressions[1].unwrap_export();
        let integer_5 = export_5.value.unwrap_integer();
        assert_eq!(integer_5.integer_token.kind, TokenKind::Integer(5));
    }
}

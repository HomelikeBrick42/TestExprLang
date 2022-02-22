#![allow(dead_code)]

use std::{collections::VecDeque, io::Write, process::exit};

use crate::{
    ast::{AstFile, AstTrait},
    lexer::Lexer,
    parsing::parse_file,
};

mod ast;
mod bound_nodes;
mod common;
mod lexer;
mod parsing;
mod token;
mod types;

fn print_usage(stream: &mut dyn Write) -> Result<(), std::io::Error> {
    let program_str = std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
        .unwrap();
    writeln!(stream, "Usage: {} <command> [options]", program_str)?;
    writeln!(stream, "Commands:")?;
    writeln!(stream, "    {} help: Prints this message", program_str)?;
    writeln!(
        stream,
        "    {} dump_ast <file>: Dumps the ast of the program",
        program_str,
    )?;
    Ok(())
}

fn parse_ast_or_error(filepath: String) -> AstFile {
    let source = std::fs::read_to_string(filepath.clone()).unwrap_or_else(|_| {
        writeln!(std::io::stderr(), "Unable to open file: '{}'", filepath).unwrap();
        exit(1)
    });
    let mut lexer = Lexer::new(filepath, &source);
    parse_file(&mut lexer).unwrap_or_else(|error| {
        writeln!(
            std::io::stderr(),
            "{}:{}:{}: Compile Error: {}",
            error.location.filepath,
            error.location.line,
            error.location.column,
            error.message,
        )
        .unwrap();
        for note in error.notes {
            if let Some(location) = &note.location {
                writeln!(
                    std::io::stderr(),
                    "{}:{}:{}: ",
                    location.filepath,
                    location.line,
                    location.column,
                )
                .unwrap();
            }
            writeln!(std::io::stderr(), "Note: {}", note.message).unwrap();
        }
        exit(1)
    })
}

fn main() {
    let mut args: VecDeque<String> = std::env::args().into_iter().collect();
    args.pop_front().unwrap();
    let command = args.pop_front().unwrap_or_else(|| {
        let mut stderr = std::io::stderr();
        writeln!(stderr, "Please specify a command").unwrap();
        print_usage(&mut stderr).unwrap();
        exit(1)
    });
    match &command as &str {
        "help" => {
            print_usage(&mut std::io::stdout()).unwrap();
        }

        "dump_ast" => {
            let filepath = args.pop_front().unwrap_or_else(|| {
                let mut stderr = std::io::stderr();
                writeln!(stderr, "Please specify a file").unwrap();
                print_usage(&mut stderr).unwrap();
                exit(1)
            });
            let file = parse_ast_or_error(filepath);
            writeln!(std::io::stdout(), "{}", file.dump(0)).unwrap();
        }

        _ => {
            let mut stderr = std::io::stderr();
            writeln!(stderr, "Unknown command: '{}'", command).unwrap();
            print_usage(&mut stderr).unwrap();
            exit(1)
        }
    }
    return;
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
        assert_eq!(a.name_token.kind, TokenKind::Name("a".to_string()));
        assert_eq!(a.value, None);

        let export_b = block.expressions[1].unwrap_export();
        assert_eq!(export_b.name_token.kind, TokenKind::Name("b".to_string()));
        let integer_5 = export_b.value.unwrap_integer();
        assert_eq!(integer_5.integer_token.kind, TokenKind::Integer(5));
    }
}

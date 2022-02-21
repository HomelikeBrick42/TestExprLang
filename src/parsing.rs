use crate::{
    ast::{Ast, AstBinary, AstFile, AstInteger, AstLet, AstName, AstUnary},
    common::CompileError,
    lexer::Lexer,
    token::TokenKind,
};

fn allow_newline(lexer: &mut Lexer) -> Result<(), CompileError> {
    if lexer.peek_kind()? == TokenKind::Newline {
        lexer.next_token()?;
    }
    Ok(())
}

pub fn parse_file(lexer: &mut Lexer) -> Result<AstFile, CompileError> {
    let mut expressions = vec![];
    while lexer.peek_kind()? != TokenKind::EndOfFile {
        while lexer.peek_kind()? == TokenKind::Newline {
            lexer.next_token()?;
        }
        expressions.push(parse_expression(lexer)?);
        if lexer.peek_kind()? != TokenKind::EndOfFile {
            let newline = lexer.next_token()?;
            if newline.kind != TokenKind::Newline {
                return Err(CompileError {
                    location: newline.location.clone(),
                    message: format!(
                        "Expected {} at the end of the expression, but got {}",
                        TokenKind::Newline.to_string(),
                        newline.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
        }
    }
    let end_of_file_token = lexer.next_token()?;
    assert_eq!(end_of_file_token.kind, TokenKind::EndOfFile);
    Ok(AstFile {
        expressions,
        end_of_file_token,
    })
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Ast, CompileError> {
    parse_binary_expression(lexer, 0)
}

fn parse_binary_expression(
    lexer: &mut Lexer,
    parent_precedence: usize,
) -> Result<Ast, CompileError> {
    fn get_unary_precedence(kind: TokenKind) -> usize {
        match kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::ExclamationMark => 4,
            _ => 0,
        }
    }

    fn get_binary_precedence(kind: TokenKind) -> usize {
        match kind {
            TokenKind::Asterisk | TokenKind::Slash => 3,
            TokenKind::Plus | TokenKind::Minus => 2,
            TokenKind::EqualEqual
            | TokenKind::ExclamationMarkEqual
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThanEqual => 1,
            _ => 0,
        }
    }

    let mut left;

    let unary_precedence = get_unary_precedence(lexer.peek_kind()?);
    if unary_precedence > 0 {
        let operator_token = lexer.next_token()?;
        allow_newline(lexer)?;
        let operand = parse_binary_expression(lexer, unary_precedence)?;
        left = Ast::Unary(AstUnary {
            operator_token,
            operand: Box::new(operand),
        });
    } else {
        left = parse_primary_expression(lexer)?;
    }

    'main_loop: loop {
        let binary_precedence = get_binary_precedence(lexer.peek_kind()?);
        if binary_precedence <= parent_precedence {
            break 'main_loop;
        }

        let operator_token = lexer.next_token()?;
        allow_newline(lexer)?;
        let right = parse_binary_expression(lexer, binary_precedence)?;
        left = Ast::Binary(AstBinary {
            left: Box::new(left),
            operator_token,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn parse_primary_expression(lexer: &mut Lexer) -> Result<Ast, CompileError> {
    match lexer.peek_kind()? {
        TokenKind::Name(_) => {
            let name_token = lexer.next_token()?;
            Ok(Ast::Name(AstName { name_token }))
        }

        TokenKind::Integer(_) => {
            let integer_token = lexer.next_token()?;
            Ok(Ast::Integer(AstInteger { integer_token }))
        }

        TokenKind::Let => {
            let let_token = lexer.next_token()?;
            let name_token = lexer.next_token()?;
            if let TokenKind::Name(_) = name_token.kind {
            } else {
                return Err(CompileError {
                    location: name_token.location.clone(),
                    message: format!(
                        "Expected {}, but got {}",
                        TokenKind::Name(String::new()).to_string(),
                        name_token.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
            let equal_token;
            let value;
            if lexer.peek_kind()? == TokenKind::Equal {
                equal_token = Some(lexer.next_token()?);
                allow_newline(lexer)?;
                value = Some(Box::new(parse_expression(lexer)?));
            } else {
                equal_token = None;
                value = None;
            }
            Ok(Ast::Let(AstLet {
                let_token,
                name_token,
                equal_token,
                value,
            }))
        }

        _ => {
            let token = lexer.next_token()?;
            Err(CompileError {
                location: token.location.clone(),
                message: format!("Expected an expression but got {}", token.kind.to_string()),
                notes: vec![],
            })
        }
    }
}

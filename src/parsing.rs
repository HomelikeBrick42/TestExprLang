use crate::{
    ast::{
        Ast, AstBinary, AstBlock, AstCall, AstExport, AstFile, AstInteger, AstLet, AstName,
        AstUnary,
    },
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
        while lexer.peek_kind()? == TokenKind::OpenParenthesis {
            let open_parenthesis_token = lexer.next_token()?;
            allow_newline(lexer)?;
            let mut first = true;
            let mut arguments = vec![];
            while lexer.peek_kind()? != TokenKind::CloseParenthesis
                && lexer.peek_kind()? != TokenKind::EndOfFile
            {
                if first {
                    first = false;
                } else {
                    let comma = lexer.next_token()?;
                    if comma.kind != TokenKind::Comma {
                        return Err(CompileError {
                            location: comma.location.clone(),
                            message: format!(
                                "Expected {} to seperate arguments in the call, but got {}",
                                TokenKind::Comma.to_string(),
                                comma.kind.to_string(),
                            ),
                            notes: vec![],
                        });
                    }
                    allow_newline(lexer)?;
                    if lexer.peek_kind()? == TokenKind::CloseParenthesis {
                        break;
                    }
                }
                arguments.push(parse_expression(lexer)?);
            }
            let close_parenthesis_token = lexer.next_token()?;
            if close_parenthesis_token.kind != TokenKind::CloseParenthesis {
                return Err(CompileError {
                    location: close_parenthesis_token.location.clone(),
                    message: format!(
                        "Expected {} at the end of the call, but got {}",
                        TokenKind::CloseParenthesis.to_string(),
                        close_parenthesis_token.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
            left = Ast::Call(AstCall {
                operand: Box::new(left),
                open_parenthesis_token,
                arguments,
                close_parenthesis_token,
            })
        }

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

        TokenKind::OpenBrace => Ok(Ast::Block(parse_block(lexer)?)),

        TokenKind::OpenParenthesis => {
            lexer.next_token()?;
            let expression = parse_expression(lexer)?;
            let close_parenthesis_token = lexer.next_token()?;
            if close_parenthesis_token.kind != TokenKind::CloseParenthesis {
                return Err(CompileError {
                    location: close_parenthesis_token.location.clone(),
                    message: format!(
                        "Expected {} to close the opening (, but got {}",
                        TokenKind::CloseParenthesis.to_string(),
                        close_parenthesis_token.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
            Ok(expression)
        }

        TokenKind::Export => {
            let export_token = lexer.next_token()?;
            let name_token = lexer.next_token()?;
            if let TokenKind::Name(_) = name_token.kind {
            } else {
                return Err(CompileError {
                    location: name_token.location.clone(),
                    message: format!(
                        "Expected {} for export, but got {}",
                        TokenKind::Name(String::new()).to_string(),
                        name_token.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
            let equals_token = lexer.next_token()?;
            if equals_token.kind != TokenKind::Equal {
                return Err(CompileError {
                    location: equals_token.location.clone(),
                    message: format!(
                        "Expected {} for export value, but got {}",
                        TokenKind::Name(String::new()).to_string(),
                        equals_token.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
            allow_newline(lexer)?;
            let value = parse_expression(lexer)?;
            Ok(Ast::Export(AstExport {
                export_token,
                name_token,
                equals_token,
                value: Box::new(value),
            }))
        }

        TokenKind::Let => {
            let let_token = lexer.next_token()?;
            let name_token = lexer.next_token()?;
            if let TokenKind::Name(_) = name_token.kind {
            } else {
                return Err(CompileError {
                    location: name_token.location.clone(),
                    message: format!(
                        "Expected {} for let, but got {}",
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

fn parse_block(lexer: &mut Lexer) -> Result<AstBlock, CompileError> {
    let open_brace_token = lexer.next_token()?;
    if open_brace_token.kind != TokenKind::OpenBrace {
        return Err(CompileError {
            location: open_brace_token.location.clone(),
            message: format!(
                "Expected {}, but got a {}",
                TokenKind::OpenBrace.to_string(),
                open_brace_token.kind.to_string(),
            ),
            notes: vec![],
        });
    }

    let mut expressions = vec![];
    while lexer.peek_kind()? != TokenKind::CloseBrace && lexer.peek_kind()? != TokenKind::EndOfFile
    {
        while lexer.peek_kind()? == TokenKind::Newline {
            lexer.next_token()?;
        }
        expressions.push(parse_expression(lexer)?);
        if lexer.peek_kind()? != TokenKind::CloseBrace && lexer.peek_kind()? != TokenKind::EndOfFile
        {
            let newline = lexer.next_token()?;
            if newline.kind != TokenKind::Newline {
                return Err(CompileError {
                    location: newline.location.clone(),
                    message: format!(
                        "Expected {} or {} at the end of the expression, but got {}",
                        TokenKind::Newline.to_string(),
                        TokenKind::CloseBrace.to_string(),
                        newline.kind.to_string(),
                    ),
                    notes: vec![],
                });
            }
        }
    }

    let close_brace_token = lexer.next_token()?;
    if close_brace_token.kind != TokenKind::CloseBrace {
        return Err(CompileError {
            location: close_brace_token.location.clone(),
            message: format!(
                "Expected {}, but got a {}",
                TokenKind::CloseBrace.to_string(),
                close_brace_token.kind.to_string(),
            ),
            notes: vec![],
        });
    }

    Ok(AstBlock {
        open_brace_token,
        expressions,
        close_brace_token,
    })
}

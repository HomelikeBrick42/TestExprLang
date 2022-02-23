use crate::{
    common::{CompileError, SourceLocation},
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub struct Lexer {
    filepath: String,
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(filepath: String, source: &str) -> Lexer {
        Lexer {
            filepath,
            source: source.chars().into_iter().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current_char(&self) -> char {
        if self.position < self.source.len() {
            self.source[self.position]
        } else {
            '\0'
        }
    }

    fn next_char(&mut self) -> char {
        let current = self.current_char();

        self.position += 1;
        self.column += 1;

        if current == '\n' {
            self.line += 1;
            self.column = 1;
        }

        current
    }

    fn get_current_location(&self) -> SourceLocation {
        SourceLocation {
            filepath: self.filepath.clone(),
            position: self.position,
            line: self.line,
            column: self.column,
        }
    }

    fn single_char_token(&mut self, kind: TokenKind) -> Token {
        let start_location = self.get_current_location();
        self.next_char();
        Token {
            kind,
            length: self.position - start_location.position,
            location: start_location,
        }
    }

    fn double_char_token(
        &mut self,
        kind: TokenKind,
        second_char: char,
        second_kind: TokenKind,
    ) -> Token {
        let start_location = self.get_current_location();
        self.next_char();
        if self.current_char() == second_char {
            self.next_char();
            Token {
                kind: second_kind,
                length: self.position - start_location.position,
                location: start_location,
            }
        } else {
            Token {
                kind,
                length: self.position - start_location.position,
                location: start_location,
            }
        }
    }

    fn double_char_token_2_choice(
        &mut self,
        kind: TokenKind,
        second_char_1: char,
        second_kind_1: TokenKind,
        second_char_2: char,
        second_kind_2: TokenKind,
    ) -> Token {
        let start_location = self.get_current_location();
        self.next_char();
        if self.current_char() == second_char_1 {
            self.next_char();
            Token {
                kind: second_kind_1.clone(),
                length: self.position - start_location.position,
                location: start_location,
            }
        } else if self.current_char() == second_char_2 {
            self.next_char();
            Token {
                kind: second_kind_2.clone(),
                length: self.position - start_location.position,
                location: start_location,
            }
        } else {
            Token {
                kind,
                length: self.position - start_location.position,
                location: start_location,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, CompileError> {
        'main_loop: loop {
            let start_location = self.get_current_location();
            return match self.current_char() {
                '\0' => Ok(Token {
                    kind: TokenKind::EndOfFile,
                    length: self.position - start_location.position,
                    location: start_location,
                }),

                ' ' | '\t' => {
                    self.next_char();
                    continue 'main_loop;
                }

                '\n' => {
                    self.next_char();
                    if self.current_char() == '\r' {
                        self.next_char();
                    }
                    Ok(Token {
                        kind: TokenKind::Newline,
                        length: self.position - start_location.position,
                        location: start_location,
                    })
                }

                '\r' => {
                    self.next_char();
                    if self.current_char() == '\n' {
                        self.next_char();
                    }
                    Ok(Token {
                        kind: TokenKind::Newline,
                        length: self.position - start_location.position,
                        location: start_location,
                    })
                }

                'A'..='Z' | 'a'..='z' | '_' => {
                    let mut value = String::new();
                    'name_loop: loop {
                        match self.current_char() {
                            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => value.push(self.next_char()),
                            _ => break 'name_loop,
                        }
                    }
                    match &value as &str {
                        "export" => Ok(Token {
                            kind: TokenKind::Export,
                            length: self.position - start_location.position,
                            location: start_location,
                        }),

                        "let" => Ok(Token {
                            kind: TokenKind::Let,
                            length: self.position - start_location.position,
                            location: start_location,
                        }),

                        _ => Ok(Token {
                            kind: TokenKind::Name(value),
                            length: self.position - start_location.position,
                            location: start_location,
                        }),
                    }
                }

                '0'..='9' => {
                    let base: u128 = if self.current_char() == '0' {
                        self.next_char();
                        match self.current_char() {
                            'b' => {
                                self.next_char();
                                2
                            }

                            'o' => {
                                self.next_char();
                                8
                            }

                            'd' => {
                                self.next_char();
                                10
                            }

                            'x' => {
                                self.next_char();
                                16
                            }

                            _ => 10,
                        }
                    } else {
                        10
                    };

                    let mut int_value: u128 = 0;
                    'int_loop: loop {
                        match self.current_char() {
                            '0'..='9' | 'A'..='Z' | 'a'..='z' => {
                                let value = match self.current_char() {
                                    '0'..='9' => self.current_char() as u128 - '0' as u128,
                                    'A'..='Z' => self.current_char() as u128 - 'A' as u128 + 10,
                                    'a'..='z' => self.current_char() as u128 - 'a' as u128 + 10,
                                    _ => unreachable!(),
                                };

                                if value >= base {
                                    return Err(CompileError {
                                        location: self.get_current_location(),
                                        message: format!(
                                            "Character '{}' is too big for base '{}'",
                                            self.current_char(),
                                            base
                                        ),
                                        notes: vec![],
                                    });
                                }

                                int_value *= base;
                                int_value += value;

                                self.next_char();
                            }

                            '_' => {
                                self.next_char();
                            }

                            _ => break 'int_loop,
                        }
                    }

                    Ok(Token {
                        kind: TokenKind::Integer(int_value),
                        length: self.position - start_location.position,
                        location: start_location,
                    })
                }

                '(' => Ok(self.single_char_token(TokenKind::OpenParenthesis)),
                ')' => Ok(self.single_char_token(TokenKind::CloseParenthesis)),
                '{' => Ok(self.single_char_token(TokenKind::OpenBrace)),
                '}' => Ok(self.single_char_token(TokenKind::CloseBrace)),

                ',' => Ok(self.single_char_token(TokenKind::Comma)),

                '+' => Ok(self.double_char_token(TokenKind::Plus, '=', TokenKind::PlusEqual)),
                '-' => Ok(self.double_char_token_2_choice(
                    TokenKind::Minus,
                    '=',
                    TokenKind::MinusEqual,
                    '>',
                    TokenKind::RightArrow,
                )),
                '*' => {
                    Ok(self.double_char_token(TokenKind::Asterisk, '=', TokenKind::AsteriskEqual))
                }

                '/' => {
                    self.next_char();
                    if self.current_char() == '/' {
                        while self.current_char() != '\n' && self.current_char() != '\0' {
                            self.next_char();
                        }
                        if self.current_char() == '\r' {
                            self.next_char();
                        }
                        continue 'main_loop;
                    } else if self.current_char() == '=' {
                        self.next_char();
                        Ok(Token {
                            kind: TokenKind::SlashEqual,
                            length: self.position - start_location.position,
                            location: start_location,
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Slash,
                            length: self.position - start_location.position,
                            location: start_location,
                        })
                    }
                }

                '=' => Ok(self.double_char_token(TokenKind::Equal, '=', TokenKind::EqualEqual)),
                '!' => Ok(self.double_char_token(
                    TokenKind::ExclamationMark,
                    '=',
                    TokenKind::ExclamationMarkEqual,
                )),
                '<' => Ok(self.double_char_token_2_choice(
                    TokenKind::LessThan,
                    '=',
                    TokenKind::LessThanEqual,
                    '-',
                    TokenKind::RightArrow,
                )),
                '>' => Ok(self.double_char_token(
                    TokenKind::GreaterThan,
                    '=',
                    TokenKind::LessThanEqual,
                )),

                _ => {
                    let chr = self.next_char();
                    Err(CompileError {
                        location: start_location,
                        message: format!("Unexpected '{}'", chr),
                        notes: vec![],
                    })
                }
            };
        }
    }

    pub fn peek_kind(&self) -> Result<TokenKind, CompileError> {
        // maybe dont clone the entire lexer+source tokens every time?
        Ok(self.clone().next_token()?.kind)
    }
}

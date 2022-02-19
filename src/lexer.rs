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

    pub fn next_token(&mut self) -> Result<Token, CompileError> {
        'main_loop: loop {
            let start_location = self.get_current_location();
            return match self.current_char() {
                '\0' => Ok(Token {
                    kind: TokenKind::EndOfFile,
                    length: self.position - start_location.position,
                    location: start_location,
                }),

                _ => {
                    let chr = self.next_char();
                    Err(CompileError {
                        location: start_location,
                        message: format!("Unexpected '{}'", chr),
                        notes: Vec::new(),
                    })
                }
            };
        }
    }
}

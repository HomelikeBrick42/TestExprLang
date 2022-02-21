use std::fmt::Debug;

use crate::{common::SourceLocation, token::Token};

// is there a better name for this?
pub trait AstTrait: Debug + Clone + PartialEq {
    fn get_location(&self) -> SourceLocation;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    File(AstFile),
    Unary(AstUnary),
    Binary(AstBinary),
    Name(AstName),
    Integer(AstInteger),
}

impl AstTrait for Ast {
    fn get_location(&self) -> SourceLocation {
        match self {
            Ast::File(file) => file.get_location(),
            Ast::Unary(unary) => unary.get_location(),
            Ast::Binary(binary) => binary.get_location(),
            Ast::Name(name) => name.get_location(),
            Ast::Integer(integer) => integer.get_location(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstFile {
    pub expressions: Vec<Ast>,
    pub end_of_file_token: Token,
}

impl AstTrait for AstFile {
    fn get_location(&self) -> SourceLocation {
        self.end_of_file_token.location.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstUnary {
    pub operator_token: Token,
    pub operand: Box<Ast>,
}

impl AstTrait for AstUnary {
    fn get_location(&self) -> SourceLocation {
        self.operator_token.location.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstBinary {
    pub left: Box<Ast>,
    pub operator_token: Token,
    pub right: Box<Ast>,
}

impl AstTrait for AstBinary {
    fn get_location(&self) -> SourceLocation {
        self.operator_token.location.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstName {
    pub name_token: Token,
}

impl AstTrait for AstName {
    fn get_location(&self) -> SourceLocation {
        self.name_token.location.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstInteger {
    pub integer_token: Token,
}

impl AstTrait for AstInteger {
    fn get_location(&self) -> SourceLocation {
        self.integer_token.location.clone()
    }
}

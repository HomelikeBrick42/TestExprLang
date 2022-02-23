use std::fmt::Debug;

use crate::{
    common::SourceLocation,
    token::{Token, TokenKind},
};

// is there a better name for this?
pub trait AstTrait: Debug + Clone + PartialEq {
    fn get_location(&self) -> SourceLocation;
    fn pretty_print(&self, indent: usize) -> String;
}

fn get_indent(indent: usize) -> String {
    let mut result = String::new();
    for _ in 0..indent {
        result += "    ";
    }
    result
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    File(AstFile),
    Block(AstBlock),
    Export(AstExport),
    Let(AstLet),
    Unary(AstUnary),
    Binary(AstBinary),
    Name(AstName),
    Integer(AstInteger),
    Call(AstCall),
}

impl Ast {
    pub fn unwrap_file(&self) -> &AstFile {
        if let Ast::File(file) = self {
            file
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_block(&self) -> &AstBlock {
        if let Ast::Block(block) = self {
            block
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_export(&self) -> &AstExport {
        if let Ast::Export(export) = self {
            export
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_let(&self) -> &AstLet {
        if let Ast::Let(lett) = self {
            lett
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_unary(&self) -> &AstUnary {
        if let Ast::Unary(unary) = self {
            unary
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_binary(&self) -> &AstBinary {
        if let Ast::Binary(binary) = self {
            binary
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_name(&self) -> &AstName {
        if let Ast::Name(name) = self {
            name
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_integer(&self) -> &AstInteger {
        if let Ast::Integer(integer) = self {
            integer
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_call(&self) -> &AstCall {
        if let Ast::Call(call) = self {
            call
        } else {
            unreachable!()
        }
    }
}

impl AstTrait for Ast {
    fn get_location(&self) -> SourceLocation {
        match self {
            Ast::File(file) => file.get_location(),
            Ast::Block(block) => block.get_location(),
            Ast::Export(export) => export.get_location(),
            Ast::Let(lett) => lett.get_location(),
            Ast::Unary(unary) => unary.get_location(),
            Ast::Binary(binary) => binary.get_location(),
            Ast::Name(name) => name.get_location(),
            Ast::Integer(integer) => integer.get_location(),
            Ast::Call(call) => call.get_location(),
        }
    }

    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Ast::File(file) => file.pretty_print(indent),
            Ast::Block(block) => block.pretty_print(indent),
            Ast::Export(export) => export.pretty_print(indent),
            Ast::Let(lett) => lett.pretty_print(indent),
            Ast::Unary(unary) => unary.pretty_print(indent),
            Ast::Binary(binary) => binary.pretty_print(indent),
            Ast::Name(name) => name.pretty_print(indent),
            Ast::Integer(integer) => integer.pretty_print(indent),
            Ast::Call(call) => call.pretty_print(indent),
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

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        for expression in &self.expressions {
            result.push('\n');
            result += &get_indent(indent);
            result += &expression.pretty_print(indent);
        }
        result.push('\n');
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstBlock {
    pub open_brace_token: Token,
    pub expressions: Vec<Ast>,
    pub close_brace_token: Token,
}

impl AstTrait for AstBlock {
    fn get_location(&self) -> SourceLocation {
        self.open_brace_token.location.clone()
    }

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push('{');
        for expression in &self.expressions {
            result.push('\n');
            result += &get_indent(indent + 1);
            result += &expression.pretty_print(indent + 1);
        }
        result.push('\n');
        result += &get_indent(indent);
        result.push('}');
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstExport {
    pub export_token: Token,
    pub name_token: Token,
    pub equals_token: Token,
    pub value: Box<Ast>,
}

impl AstTrait for AstExport {
    fn get_location(&self) -> SourceLocation {
        self.name_token.location.clone()
    }

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        result += "export ";
        result += if let TokenKind::Name(name) = &self.name_token.kind {
            name
        } else {
            unreachable!()
        };
        result += " = ";
        result += &self.value.pretty_print(indent);
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLet {
    pub let_token: Token,
    pub name_token: Token,
    pub equal_token: Option<Token>,
    pub value: Option<Box<Ast>>,
}

impl AstTrait for AstLet {
    fn get_location(&self) -> SourceLocation {
        self.name_token.location.clone()
    }

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        result += "let ";
        result += if let TokenKind::Name(name) = &self.name_token.kind {
            name
        } else {
            unreachable!()
        };
        if let Some(value) = &self.value {
            result += " = ";
            result += &value.pretty_print(indent);
        }
        result
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

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        result += &self.operator_token.kind.to_string();
        result += &self.operand.pretty_print(indent);
        result
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

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        result += &self.left.pretty_print(indent);
        result.push(' ');
        result += &self.operator_token.kind.to_string();
        result.push(' ');
        result += &self.right.pretty_print(indent);
        result
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

    fn pretty_print(&self, _indent: usize) -> String {
        if let TokenKind::Name(name) = &self.name_token.kind {
            name.clone()
        } else {
            unreachable!()
        }
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

    fn pretty_print(&self, _indent: usize) -> String {
        if let TokenKind::Integer(integer) = &self.integer_token.kind {
            integer.to_string()
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCall {
    pub operand: Box<Ast>,
    pub open_parenthesis_token: Token,
    pub arguments: Vec<Ast>,
    pub close_parenthesis_token: Token,
}

impl AstTrait for AstCall {
    fn get_location(&self) -> SourceLocation {
        self.open_parenthesis_token.location.clone()
    }

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        result += &self.operand.pretty_print(indent);
        result.push('(');
        for (i, expression) in self.arguments.iter().enumerate() {
            if i > 0 {
                result += ", ";
            }
            result += &expression.pretty_print(indent);
        }
        result.push(')');
        result
    }
}

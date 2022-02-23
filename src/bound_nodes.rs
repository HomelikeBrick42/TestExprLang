use std::{
    collections::HashMap,
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::{
    common::SourceLocation,
    types::{ProcType, Type},
};

pub trait BoundNodeTrait: Debug + Clone {
    fn get_location(&self) -> SourceLocation;
    fn get_type(&self) -> Type;
}

#[derive(Debug, Clone)]
pub enum BoundNode {
    Block(BoundBlock),
    Export(BoundExport),
    Let(BoundLet),
    Unary(BoundUnary),
    Binary(BoundBinary),
    Name(BoundName),
    Integer(BoundInteger),
    Call(BoundCall),
    PrintInteger(BoundPrintInteger),
}

impl BoundNode {
    pub fn unwrap_block(&self) -> &BoundBlock {
        if let BoundNode::Block(block) = self {
            block
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_export(&self) -> &BoundExport {
        if let BoundNode::Export(export) = self {
            export
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_let(&self) -> &BoundLet {
        if let BoundNode::Let(lett) = self {
            lett
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_unary(&self) -> &BoundUnary {
        if let BoundNode::Unary(unary) = self {
            unary
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_binary(&self) -> &BoundBinary {
        if let BoundNode::Binary(binary) = self {
            binary
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_name(&self) -> &BoundName {
        if let BoundNode::Name(name) = self {
            name
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_integer(&self) -> &BoundInteger {
        if let BoundNode::Integer(integer) = self {
            integer
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_call(&self) -> &BoundCall {
        if let BoundNode::Call(call) = self {
            call
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_print_integer(&self) -> &BoundPrintInteger {
        if let BoundNode::PrintInteger(print_integer) = self {
            print_integer
        } else {
            unreachable!()
        }
    }
}

impl BoundNodeTrait for BoundNode {
    fn get_location(&self) -> SourceLocation {
        match self {
            BoundNode::Block(block) => block.get_location(),
            BoundNode::Export(export) => export.get_location(),
            BoundNode::Let(lett) => lett.get_location(),
            BoundNode::Unary(unary) => unary.get_location(),
            BoundNode::Binary(binary) => binary.get_location(),
            BoundNode::Name(name) => name.get_location(),
            BoundNode::Integer(integer) => integer.get_location(),
            BoundNode::Call(call) => call.get_location(),
            BoundNode::PrintInteger(print_integer) => print_integer.get_location(),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            BoundNode::Block(block) => block.get_type(),
            BoundNode::Export(export) => export.get_type(),
            BoundNode::Let(lett) => lett.get_type(),
            BoundNode::Unary(unary) => unary.get_type(),
            BoundNode::Binary(binary) => binary.get_type(),
            BoundNode::Name(name) => name.get_type(),
            BoundNode::Integer(integer) => integer.get_type(),
            BoundNode::Call(call) => call.get_type(),
            BoundNode::PrintInteger(print_integer) => print_integer.get_type(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundBlock {
    pub location: SourceLocation,
    pub expressions: Vec<Rc<BoundNode>>,
    pub exported_expressions: HashMap<String, Weak<BoundNode>>,
    pub block_type: Type,
}

impl BoundNodeTrait for BoundBlock {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.block_type.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundExport {
    pub location: SourceLocation,
    pub name: String,
    pub value: Rc<BoundNode>,
}

impl BoundNodeTrait for BoundExport {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.value.get_type()
    }
}

#[derive(Debug, Clone)]
pub struct BoundLet {
    pub location: SourceLocation,
    pub name: String,
    pub value: Option<Rc<BoundNode>>,
}

impl BoundNodeTrait for BoundLet {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        if let Some(value) = &self.value {
            value.get_type()
        } else {
            Type::Void
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperatorKind {
    Identity,
    Negation,
}

#[derive(Debug, Clone)]
pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub operand: Type,
    pub result: Type,
}

#[derive(Debug, Clone)]
pub struct BoundUnary {
    pub location: SourceLocation,
    pub operator: UnaryOperator,
    pub operand: Rc<BoundNode>,
}

impl BoundNodeTrait for BoundUnary {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.operator.result.clone()
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperatorKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub left: Type,
    pub right: Type,
    pub result: Type,
}

#[derive(Debug, Clone)]
pub struct BoundBinary {
    pub location: SourceLocation,
    pub left: Rc<BoundNode>,
    pub operator: BinaryOperator,
    pub right: Rc<BoundNode>,
}

impl BoundNodeTrait for BoundBinary {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.operator.result.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundName {
    pub location: SourceLocation,
    pub name: String,
    pub resolved_expression: Weak<BoundNode>,
}

impl BoundNodeTrait for BoundName {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.resolved_expression.upgrade().unwrap().get_type()
    }
}

#[derive(Debug, Clone)]
pub struct BoundInteger {
    pub location: SourceLocation,
    pub value: u128,
}

impl BoundNodeTrait for BoundInteger {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        Type::Integer
    }
}

#[derive(Debug, Clone)]
pub struct BoundCall {
    pub location: SourceLocation,
    pub operand: Rc<BoundNode>,
    pub arguments: Vec<Rc<BoundNode>>,
    pub proc_type: Type,
}

impl BoundNodeTrait for BoundCall {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.proc_type.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundPrintInteger {
    pub location: SourceLocation,
}

impl BoundNodeTrait for BoundPrintInteger {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        Type::Proc(ProcType {
            parameter_types: vec![Type::Integer],
            return_type: Box::new(Type::Void),
        })
    }
}

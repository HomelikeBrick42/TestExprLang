use std::{
    collections::HashMap,
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::{common::SourceLocation, types::Type};

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
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundBlock {
    pub location: SourceLocation,
    pub expressions: Vec<Rc<BoundNode>>,
    pub exported_expressions: HashMap<String, Weak<BoundExport>>,
    pub type_: Type,
}

impl BoundNodeTrait for BoundBlock {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.type_.clone()
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
pub struct UnaryOperator {
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
pub struct BinaryOperator {
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
    pub resolved_ast: Weak<BoundNode>,
}

impl BoundNodeTrait for BoundName {
    fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    fn get_type(&self) -> Type {
        self.resolved_ast.upgrade().unwrap().get_type()
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

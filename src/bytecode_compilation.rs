use std::rc::Rc;

use crate::{
    bound_nodes::{
        BinaryOperatorKind, BoundBinary, BoundBlock, BoundCall, BoundExport, BoundInteger,
        BoundLet, BoundName, BoundNode, BoundNodeTrait, BoundPrintInteger, BoundUnary,
        UnaryOperatorKind,
    },
    bytecode::{Bytecode, BytecodeValue},
};

trait Compilable: BoundNodeTrait {
    fn compile(&self, bytecode: &mut Vec<Bytecode>);
}

pub fn compile_bytecode(node: &Rc<BoundNode>, bytecode: &mut Vec<Bytecode>) {
    node.compile(bytecode);
}

impl Compilable for BoundNode {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        match self {
            BoundNode::Block(block) => block.compile(bytecode),
            BoundNode::Export(export) => export.compile(bytecode),
            BoundNode::Let(lett) => lett.compile(bytecode),
            BoundNode::Unary(unary) => unary.compile(bytecode),
            BoundNode::Binary(binary) => binary.compile(bytecode),
            BoundNode::Name(name) => name.compile(bytecode),
            BoundNode::Integer(integer) => integer.compile(bytecode),
            BoundNode::Call(call) => call.compile(bytecode),
            BoundNode::PrintInteger(print_integer) => print_integer.compile(bytecode),
        }
    }
}

impl Compilable for BoundBlock {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        for expression in &self.expressions {
            expression.compile(bytecode);
            bytecode.push(Bytecode::Pop);
        }
    }
}

impl Compilable for BoundExport {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        self.value.compile(bytecode);
        bytecode.push(Bytecode::Dup);
        bytecode.push(Bytecode::Store(self.name.clone()));
    }
}

impl Compilable for BoundLet {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        if let Some(value) = &self.value {
            value.compile(bytecode);
            bytecode.push(Bytecode::Dup);
        } else {
            bytecode.push(Bytecode::Push(BytecodeValue::Void));
        }
        bytecode.push(Bytecode::Store(self.name.clone()));
    }
}

impl Compilable for BoundUnary {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        self.operand.compile(bytecode);
        match &self.operator.kind {
            UnaryOperatorKind::Identity => {}
            UnaryOperatorKind::Negation => bytecode.push(Bytecode::NegateInteger),
        }
    }
}

impl Compilable for BoundBinary {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        self.left.compile(bytecode);
        self.right.compile(bytecode);
        match &self.operator.kind {
            BinaryOperatorKind::Addition => bytecode.push(Bytecode::AddInteger),
            BinaryOperatorKind::Subtraction => bytecode.push(Bytecode::SubInteger),
            BinaryOperatorKind::Multiplication => bytecode.push(Bytecode::MulInteger),
            BinaryOperatorKind::Division => bytecode.push(Bytecode::DivInteger),
        }
    }
}

impl Compilable for BoundName {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        bytecode.push(Bytecode::Load(self.name.clone()));
    }
}

impl Compilable for BoundInteger {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        bytecode.push(Bytecode::Push(BytecodeValue::Integer(self.value as i64)));
    }
}

impl Compilable for BoundCall {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        self.operand.compile(bytecode);
        for argument in &self.arguments {
            argument.compile(bytecode);
        }
        bytecode.push(Bytecode::Call {
            argument_count: self.arguments.len(),
        });
    }
}

impl Compilable for BoundPrintInteger {
    fn compile(&self, bytecode: &mut Vec<Bytecode>) {
        // TODO: Maybe dont create a new function every time print_integer is referenced
        bytecode.push(Bytecode::Push(BytecodeValue::Procedure(Vec::from([
            Bytecode::PrintInteger,
            Bytecode::Return,
        ]))));
    }
}

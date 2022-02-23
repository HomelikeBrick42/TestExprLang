use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    ast::{
        Ast, AstBinary, AstBlock, AstCall, AstExport, AstFile, AstInteger, AstLet, AstName,
        AstTrait, AstUnary,
    },
    bound_nodes::{
        BinaryOperator, BinaryOperatorKind, BoundBinary, BoundBlock, BoundCall, BoundExport,
        BoundInteger, BoundLet, BoundName, BoundNode, BoundNodeTrait, BoundUnary, UnaryOperator,
        UnaryOperatorKind,
    },
    common::{CompileError, CompileNote},
    token::TokenKind,
    types::{BlockType, Type},
};

trait BindingTrait {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError>;
}

pub fn bind_ast(
    ast: &Ast,
    names: &mut HashMap<String, Weak<BoundNode>>,
) -> Result<Rc<BoundNode>, CompileError> {
    ast.bind(names)
}

impl BindingTrait for Ast {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        match self {
            Ast::File(file) => file.bind(names),
            Ast::Block(block) => block.bind(names),
            Ast::Export(export) => export.bind(names),
            Ast::Let(lett) => lett.bind(names),
            Ast::Unary(unary) => unary.bind(names),
            Ast::Binary(binary) => binary.bind(names),
            Ast::Name(name) => name.bind(names),
            Ast::Integer(integer) => integer.bind(names),
            Ast::Call(call) => call.bind(names),
        }
    }
}

impl BindingTrait for AstFile {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let mut new_names = names.clone();

        let mut expressions = vec![];
        let mut exported_expressions = HashMap::new();
        for expression in &self.expressions {
            let bound_expression = expression.bind(&mut new_names)?;
            expressions.push(bound_expression.clone());

            if let BoundNode::Export(export) = &bound_expression as &BoundNode {
                exported_expressions.insert(export.name.clone(), Rc::downgrade(&bound_expression));
            }
        }

        let mut exported_types = HashMap::new();
        for (name, expression) in &exported_expressions {
            exported_types.insert(name.clone(), expression.upgrade().unwrap().get_type());
        }

        Ok(Rc::new(BoundNode::Block(BoundBlock {
            location: self.get_location(),
            expressions,
            exported_expressions,
            block_type: Type::Block(BlockType { exported_types }),
        })))
    }
}

impl BindingTrait for AstBlock {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let mut new_names = names.clone();

        let mut expressions = vec![];
        let mut exported_expressions = HashMap::new();
        for expression in &self.expressions {
            let bound_expression = expression.bind(&mut new_names)?;
            expressions.push(bound_expression.clone());

            if let BoundNode::Export(export) = &bound_expression as &BoundNode {
                exported_expressions.insert(export.name.clone(), Rc::downgrade(&bound_expression));
            }
        }

        let mut exported_types = HashMap::new();
        for (name, expression) in &exported_expressions {
            exported_types.insert(name.clone(), expression.upgrade().unwrap().get_type());
        }

        Ok(Rc::new(BoundNode::Block(BoundBlock {
            location: self.get_location(),
            expressions,
            exported_expressions,
            block_type: Type::Block(BlockType { exported_types }),
        })))
    }
}

impl BindingTrait for AstExport {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let name = if let TokenKind::Name(name) = &self.name_token.kind {
            name.clone()
        } else {
            unreachable!()
        };

        let value = self.value.bind(names)?;

        if let Some(expression) = names.get(&name.clone()) {
            Err(CompileError {
                location: self.get_location(),
                message: format!("{} is already defined", name),
                notes: vec![CompileNote {
                    location: Some(expression.upgrade().unwrap().get_location()),
                    message: format!("{} was previously defined here", name),
                }],
            })
        } else {
            let export = Rc::new(BoundNode::Export(BoundExport {
                location: self.get_location(),
                name: name.clone(),
                value,
            }));
            names.insert(name, Rc::downgrade(&export));
            Ok(export)
        }
    }
}

impl BindingTrait for AstLet {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let name = if let TokenKind::Name(name) = &self.name_token.kind {
            name.clone()
        } else {
            unreachable!()
        };

        let value = if let Some(value) = &self.value {
            Some(value.bind(names)?)
        } else {
            None
        };

        if let Some(expression) = names.get(&name.clone()) {
            Err(CompileError {
                location: self.get_location(),
                message: format!("{} is already defined", name),
                notes: vec![CompileNote {
                    location: Some(expression.upgrade().unwrap().get_location()),
                    message: format!("{} was previously defined here", name),
                }],
            })
        } else {
            let lett = Rc::new(BoundNode::Let(BoundLet {
                location: self.get_location(),
                name: name.clone(),
                value,
            }));
            names.insert(name, Rc::downgrade(&lett));
            Ok(lett)
        }
    }
}

static UNARY_OPERATORS: &[(TokenKind, UnaryOperator)] = &[
    (
        TokenKind::Plus,
        UnaryOperator {
            kind: UnaryOperatorKind::Identity,
            operand: Type::Integer,
            result: Type::Integer,
        },
    ),
    (
        TokenKind::Minus,
        UnaryOperator {
            kind: UnaryOperatorKind::Negation,
            operand: Type::Integer,
            result: Type::Integer,
        },
    ),
];

impl BindingTrait for AstUnary {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let operand = self.operand.bind(names)?;

        let mut operator = None;
        for (kind, unary_operator) in UNARY_OPERATORS {
            if &self.operator_token.kind == kind && unary_operator.operand == operand.get_type() {
                operator = Some(unary_operator.clone());
                break;
            }
        }

        if let Some(operator) = operator {
            Ok(Rc::new(BoundNode::Unary(BoundUnary {
                location: self.get_location(),
                operator,
                operand,
            })))
        } else {
            // TODO: Print type properly
            Err(CompileError {
                location: self.get_location(),
                message: format!(
                    "Unable to find unary operator {} for type {:?}",
                    self.operator_token.kind.to_string(),
                    operand.get_type(),
                ),
                notes: vec![],
            })
        }
    }
}

static BINARY_OPERATORS: &[(TokenKind, BinaryOperator)] = &[
    (
        TokenKind::Plus,
        BinaryOperator {
            kind: BinaryOperatorKind::Addition,
            left: Type::Integer,
            right: Type::Integer,
            result: Type::Integer,
        },
    ),
    (
        TokenKind::Minus,
        BinaryOperator {
            kind: BinaryOperatorKind::Subtraction,
            left: Type::Integer,
            right: Type::Integer,
            result: Type::Integer,
        },
    ),
    (
        TokenKind::Asterisk,
        BinaryOperator {
            kind: BinaryOperatorKind::Multiplication,
            left: Type::Integer,
            right: Type::Integer,
            result: Type::Integer,
        },
    ),
    (
        TokenKind::Slash,
        BinaryOperator {
            kind: BinaryOperatorKind::Division,
            left: Type::Integer,
            right: Type::Integer,
            result: Type::Integer,
        },
    ),
];

impl BindingTrait for AstBinary {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let left = self.left.bind(names)?;
        let right = self.right.bind(names)?;

        let mut operator = None;
        for (kind, binary_operator) in BINARY_OPERATORS {
            if &self.operator_token.kind == kind
                && binary_operator.left == left.get_type()
                && binary_operator.right == right.get_type()
            {
                operator = Some(binary_operator.clone());
                break;
            }
        }

        if let Some(operator) = operator {
            Ok(Rc::new(BoundNode::Binary(BoundBinary {
                location: self.get_location(),
                left,
                operator,
                right,
            })))
        } else {
            // TODO: Print type properly
            Err(CompileError {
                location: self.get_location(),
                message: format!(
                    "Unable to find binary operator {} for types {:?} and {:?}",
                    self.operator_token.kind.to_string(),
                    left.get_type(),
                    right.get_type(),
                ),
                notes: vec![],
            })
        }
    }
}

impl BindingTrait for AstName {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let name = if let TokenKind::Name(name) = &self.name_token.kind {
            name.clone()
        } else {
            unreachable!()
        };

        if let Some(expression) = names.get(&name) {
            Ok(Rc::new(BoundNode::Name(BoundName {
                location: self.get_location(),
                name,
                resolved_expression: expression.clone(),
            })))
        } else {
            Err(CompileError {
                location: self.get_location(),
                message: format!("Unable to find {}", name),
                notes: vec![],
            })
        }
    }
}

impl BindingTrait for AstInteger {
    fn bind(
        &self,
        _names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let value = if let TokenKind::Integer(value) = self.integer_token.kind {
            value
        } else {
            unreachable!()
        };

        if value > i64::MAX as u128 {
            Err(CompileError {
                location: self.integer_token.location.clone(),
                message: format!("Integer {} is too big for a 64 bit signed integer", value),
                notes: vec![],
            })
        } else {
            Ok(Rc::new(BoundNode::Integer(BoundInteger {
                location: self.get_location(),
                value,
            })))
        }
    }
}

impl BindingTrait for AstCall {
    fn bind(
        &self,
        names: &mut HashMap<String, Weak<BoundNode>>,
    ) -> Result<Rc<BoundNode>, CompileError> {
        let operand = self.operand.bind(names)?;
        let proc_type = if let Type::Proc(proc_type) = operand.get_type() {
            proc_type
        } else {
            return Err(CompileError {
                location: self.close_parenthesis_token.location.clone(),
                message: format!("Cannot call a non procedure"),
                notes: vec![CompileNote {
                    location: Some(operand.get_location()),
                    message: format!("The type was {:?}", operand.get_type()),
                }],
            });
        };

        if proc_type.parameter_types.len() != self.arguments.len() {
            return Err(CompileError {
                location: self.close_parenthesis_token.location.clone(),
                message: format!(
                    "Invalid number of arguments for procedure, expected {} arguments but got {}",
                    proc_type.parameter_types.len(),
                    self.arguments.len(),
                ),
                notes: vec![],
            });
        }

        let mut arguments = vec![];
        for (i, expression) in self.arguments.iter().enumerate() {
            let argument = expression.bind(names)?;
            if argument.get_type() != proc_type.parameter_types[i] {
                return Err(CompileError {
                    location: self.close_parenthesis_token.location.clone(),
                    message: format!(
                        "Wrong argument type for procedure, expected type {:?} but got type {:?}",
                        proc_type.parameter_types[i],
                        argument.get_type(),
                    ),
                    notes: vec![],
                });
            }
            arguments.push(argument);
        }

        Ok(Rc::new(BoundNode::Call(BoundCall {
            location: self.get_location(),
            operand,
            arguments,
            proc_type: Type::Proc(proc_type),
        })))
    }
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::bytecode::{Bytecode, BytecodeValue};

pub fn execute_bytecode(
    bytecode: &Vec<Bytecode>,
    mut stack: Vec<Rc<RefCell<BytecodeValue>>>,
) -> Option<Rc<RefCell<BytecodeValue>>> {
    let mut ip = 0;
    let mut vars: HashMap<String, Rc<RefCell<BytecodeValue>>> = HashMap::new();
    stack.insert(0, Rc::new(RefCell::new(BytecodeValue::Void)));
    loop {
        match &bytecode[ip] {
            Bytecode::Exit => return None,

            Bytecode::Push(value) => stack.push(Rc::new(RefCell::new(value.clone()))),

            Bytecode::Pop => {
                stack.pop().unwrap();
            }

            Bytecode::Dup => stack.push(stack.last().unwrap().clone()),

            Bytecode::Call { argument_count } => {
                let mut new_stack = vec![];
                for _ in 0..*argument_count {
                    new_stack.push(stack.pop().unwrap());
                }
                let procedure = stack.pop().unwrap();
                stack.push(
                    execute_bytecode(&procedure.borrow().unwrap_procedure(), new_stack).unwrap(),
                );
            }

            Bytecode::Return => return Some(stack.pop().unwrap()),

            Bytecode::Load(name) => stack.push(vars.get(name).unwrap().clone()),

            Bytecode::Store(name) => {
                vars.insert(name.clone(), stack.pop().unwrap());
            }

            Bytecode::AddInteger => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Rc::new(RefCell::new(BytecodeValue::Integer(
                    a.borrow().unwrap_integer() + b.borrow().unwrap_integer(),
                ))));
            }

            Bytecode::SubInteger => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Rc::new(RefCell::new(BytecodeValue::Integer(
                    a.borrow().unwrap_integer() - b.borrow().unwrap_integer(),
                ))));
            }

            Bytecode::MulInteger => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Rc::new(RefCell::new(BytecodeValue::Integer(
                    a.borrow().unwrap_integer() * b.borrow().unwrap_integer(),
                ))));
            }

            Bytecode::DivInteger => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Rc::new(RefCell::new(BytecodeValue::Integer(
                    a.borrow().unwrap_integer() / b.borrow().unwrap_integer(),
                ))));
            }

            Bytecode::PrintInteger => {
                println!("{}", &stack.pop().unwrap().borrow().unwrap_integer());
            }
        }
        ip += 1;
    }
}

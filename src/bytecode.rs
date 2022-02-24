use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Bytecode {
    Exit,
    Push(BytecodeValue),
    Pop,
    Dup,
    Call { argument_count: usize },
    Return,
    Load(String),
    Store(String),
    AddInteger,
    SubInteger,
    MulInteger,
    DivInteger,
    PrintInteger,
}

#[derive(Debug, Clone)]
pub enum BytecodeValue {
    Void,
    Integer(i64),
    Procedure(Vec<Bytecode>),
    Block(HashMap<String, BytecodeValue>),
}

impl BytecodeValue {
    pub fn unwrap_integer(&self) -> &i64 {
        if let BytecodeValue::Integer(integer) = self {
            integer
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_procedure(&self) -> &Vec<Bytecode> {
        if let BytecodeValue::Procedure(procedure) = self {
            procedure
        } else {
            unreachable!()
        }
    }

    pub fn unwrap_block(&self) -> &HashMap<String, BytecodeValue> {
        if let BytecodeValue::Block(block) = self {
            block
        } else {
            unreachable!()
        }
    }
}

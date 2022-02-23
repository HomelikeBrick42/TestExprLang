use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Type,
    Integer,
    Block(BlockType),
    Proc(ProcType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockType {
    pub exported_types: HashMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcType {
    pub parameter_types: Vec<Type>,
    pub return_type: Box<Type>,
}

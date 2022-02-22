use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Type,
    Integer,
    BlockType(BlockType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockType {
    pub exported_types: HashMap<String, Type>,
}

use derive::OpCodeTable;
use utils::OpCodeTable;

pub mod types {
    pub type Value = f32;
}

#[derive(Debug, OpCodeTable)]
pub enum OpCode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Constant(types::Value),
    Negate,
    Return,   
}
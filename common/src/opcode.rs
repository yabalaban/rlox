use crate::types::OpCode;

pub struct ByteCode;

impl ByteCode {
    pub const RETURN: OpCode    = 0;
    pub const NEGATE: OpCode    = 1;
    pub const ADD: OpCode       = 2;
    pub const SUBTRACT: OpCode  = 3;
    pub const MULTIPLY: OpCode  = 4;
    pub const DIVIDE: OpCode    = 5;
    pub const CONSTANT: OpCode  = 6;
}

pub fn offset(code: u8) -> u8 {
    match code {
        ByteCode::RETURN..=ByteCode::DIVIDE => 1,
        ByteCode::CONSTANT => 2,
        _ => panic!("OpCode \"{}\" is not defined", code),
    }
}
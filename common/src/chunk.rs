use crate::types;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<types::OpCode>,
    pub constants: Vec<types::Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn write(&mut self, opcode: types::OpCode, line: usize) {
        self.code.push(opcode);
        self.lines.push(line)
    }

    pub fn add_constant(&mut self, value: types::Value) -> types::OpCode {
        let pos = self.constants.len();
        self.constants.push(value);
        pos as types::OpCode
    }
}

pub fn make() -> Chunk {
    Chunk {
        code: Vec::new(),
        constants: Vec::new(),
        lines: Vec::new(),
    }
}
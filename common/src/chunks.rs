use crate::opcode::OpCode;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn write(&mut self, opcode: OpCode, line: i32) {
        self.code.push(opcode);
        self.lines.push(line);
    }
}

pub fn make() -> Chunk {
    Chunk {
        code: Vec::new(),
        lines: Vec::new(),
    }
}
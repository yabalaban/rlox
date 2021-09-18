use common::chunk;
use common::types;

use crate::InterpretResult;
use crate::Interpreter;

#[derive(Debug)]
pub struct VM {
    pub chunk: chunk::Chunk,
    pub stack: Vec<types::Value>,
    pub ip: usize,
}

pub(crate) fn make() -> VM {
    VM {
        chunk: chunk::make(),
        stack: Vec::new(),
        ip: 0,
    }
}

impl Interpreter for VM {
    fn interpret(&mut self, source: &String) -> InterpretResult {
        let mut chunk = chunk::make();
        if !compiler::compile(&mut chunk, source) {
            InterpretResult::CompilationError
        } else {
            self.reset(chunk);
            self.run()
        }
    }
}

impl VM {
    fn read_byte(&mut self) -> types::OpCode {
        let ip = self.ip;
        self.ip += 1;
        self.chunk.code[ip]
    }

    fn read_constant(&mut self) -> types::Value {
        let code = self.read_byte() as usize;
        self.chunk.constants[code]
    }
}

impl VM {
    pub(crate) fn run(&mut self) -> InterpretResult {
        use common::opcode::ByteCode;
        loop {
            let opcode = self.read_byte();
            match opcode {
                ByteCode::RETURN => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok
                },
                ByteCode::NEGATE => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-1.0 * value)
                },
                ByteCode::CONSTANT => {
                    let value = self.read_constant();
                    self.stack.push(value)
                },
                ByteCode::ADD => self.binary(std::ops::Add::add),
                ByteCode::SUBTRACT => self.binary(std::ops::Sub::sub),
                ByteCode::MULTIPLY => self.binary(std::ops::Mul::mul),
                ByteCode::DIVIDE => self.binary(std::ops::Div::div),
                _ => (), 
            }
        }
    }

    pub(crate) fn reset(&mut self, chunk: chunk::Chunk) {
        self.chunk = chunk;
        self.stack = Vec::new();
        self.ip = 0;
    }

    pub(crate) fn binary(&mut self, op: fn(types::Value, types::Value) -> types::Value) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
    }
}
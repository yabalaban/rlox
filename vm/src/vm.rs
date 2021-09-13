use common::chunks;
use common::chunks::Chunk;
use common::opcode::OpCode;
use common::opcode::types::Value;
use utils::OpCodeTable;

use crate::InterpretResult;
use crate::Interpreter;
use crate::InterpreterDebug;

#[derive(Debug)]
pub(crate) struct VM {
    pub(crate) chunk: Chunk,
    pub(crate) stack: Vec<Value>,
}

impl Interpreter for VM {
    fn interpret(&mut self, source: &String) -> InterpretResult {
        let mut chunk = chunks::make();
        compiler::compile(source);
        InterpretResult::Ok
    }
}

impl InterpreterDebug for VM {
    fn print_code(&self) {
        println!("== Chunk ==");
        let mut offset = 0;
        let chunk = &self.chunk;
        for idx in 0..chunk.code.len() {
            let opcode = &chunk.code[idx];
            println!("{:04}\t{}\t{:?}", offset, chunk.lines[idx], opcode);
            offset += opcode.offset();
        }
    }

    fn print_stack(&self) {
        println!("== Stack ==");
        let stack = &self.stack;
        for idx in (0..stack.len()).rev() {
            println!("{}\t{}", idx, stack[idx]);
        }
    }
}

impl VM {
    pub fn run(&mut self) -> InterpretResult {
        let chunk = &self.chunk;
        for opcode in &chunk.code {
            match opcode {
                OpCode::Add => {
                    match self.stack.pop() {
                        Some(val1) => {
                            if let Some(val) = self.stack.last_mut() {
                                *val = *val + val1;
                            }
                        },
                        None => panic!("Stack underflow"),
                    }
                },
                OpCode::Subtract => {
                    match self.stack.pop() {
                        Some(val1) => {
                            if let Some(val) = self.stack.last_mut() {
                                *val = *val - val1;
                            }
                        },
                        None => panic!("Stack underflow"),
                    }
                },
                OpCode::Multiply => {
                    match self.stack.pop() {
                        Some(val1) => {
                            if let Some(val) = self.stack.last_mut() {
                                *val =  *val * val1;
                            }
                        },
                        None => panic!("Stack underflow"),
                    }
                },
                OpCode::Divide => {
                    match self.stack.pop() {
                        Some(val1) => {
                            if val1 == 0.0 {
                                return InterpretResult::RuntimeError;
                            } else if let Some(val) = self.stack.last_mut() {
                                *val = *val / val1;
                            }
                        },
                        None => panic!("Stack underflow"),
                    }
                }
                OpCode::Return => return InterpretResult::Ok,
                OpCode::Negate => {
                    if let Some(val) = self.stack.last_mut() {
                        *val = -1.0 * *val;
                    }
                }
                OpCode::Constant(val) => {
                    self.stack.push(*val);
                },
            }
        }
        InterpretResult::RuntimeError
    }
}
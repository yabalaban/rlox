pub mod vm;

use common::chunks;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompilationError,
    RuntimeError,
}

pub trait Interpreter {
    fn interpret(&mut self, source: &String) -> InterpretResult;
}

pub trait InterpreterDebug {
    fn print_code(&self);
    fn print_stack(&self);
}

pub fn make() -> impl Interpreter + InterpreterDebug {
    vm::VM {
        chunk: chunks::make(),
        stack: Vec::new(),
    }
}
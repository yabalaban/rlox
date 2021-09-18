mod result;
mod vm;

pub use result::InterpretResult;
pub use vm::VM;

pub trait Interpreter {
    fn interpret(&mut self, source: &String) -> InterpretResult;
}

pub fn make() -> vm::VM {
    vm::make()
}

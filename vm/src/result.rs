#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompilationError,
    RuntimeError,
}
use crate::interpreter::{Interpreter, InterpreterError};

pub type RunError = InterpreterError;

type Result<T> = std::result::Result<T, RunError>;

pub fn execute(interpreter: &mut Interpreter, code: &str) -> Result<()> {
    interpreter.run(code)
}
